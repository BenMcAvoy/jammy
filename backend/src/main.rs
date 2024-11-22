// TODO: Refactor this entire file into mutliple files
// and add comments

use std::collections::HashSet;
use std::sync::Mutex;

use argon2::Algorithm;
use argon2::Params;
use argon2::Version;
use surrealdb::engine::local::Db;
use surrealdb::RecordId;
use surrealdb::Surreal;

use tracing;

use poem_openapi::Tags;
use rand::RngCore;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use poem::{
    error::InternalServerError, listener::TcpListener, web::Data, EndpointExt, Request, Result,
    Route,
};
use poem_openapi::{
    auth::ApiKey,
    payload::{Json, PlainText},
    Object, OpenApi, OpenApiService, SecurityScheme,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use chrono::{Duration, Utc};

type ServerKey = Hmac<Sha256>;

use surrealdb::engine::local::RocksDb;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    exp: i64, // expiration time as a Unix timestamp
}

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "header",
    checker = "api_checker"
)]

struct Authorizor(User);

struct TokenBlacklist {
    tokens: HashSet<String>,
}

impl TokenBlacklist {
    fn validate(&self, value: &str) -> poem::Result<()> {
        if self.tokens.contains(value) {
            Err(poem::Error::from_status(StatusCode::CONFLICT))
        } else {
            Ok(())
        }
    }
}

lazy_static::lazy_static! {
    static ref BLACKLIST: Mutex<TokenBlacklist> = Mutex::new(TokenBlacklist {
        tokens: HashSet::new(),
    });
}

/// ApiKey authorization, this is the actual authorization function
/// that checks the API key and returns the user if one exists.
async fn api_checker(req: &Request, api_key: ApiKey) -> Option<User> {
    let server_key = req.data::<ServerKey>().unwrap();
    let user = VerifyWithKey::<User>::verify_with_key(api_key.key.as_str(), server_key).ok();

    if let Some(ref us) = user {
        if us.exp < Utc::now().timestamp() {
            return None;
        }
    }

    // Check the blacklist of dead tokens (e.g. expired tokens or if the user has been deleted)
    // We don't need dead tokens to be stored persistently because when the server restarts, the
    // tokens will be regenerated.
    if (BLACKLIST
        .lock()
        .as_ref()
        .ok()?
        .validate(api_key.key.as_str()))
    .is_err()
    {
        return None;
    }

    user
}

#[derive(Object)]
struct LoginRequest {
    username: String,
    password: String,
}

struct Api;

#[derive(Debug, Serialize, Deserialize)]
struct UserRecord {
    username: String,
    hash: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: RecordId,
}

#[derive(Tags)]
enum Tag {
    User,
    Test,
}

#[OpenApi]
#[allow(unused_variables)]
impl Api {
    /// Creates a new user account and returns the token, returns a conflict or internal server
    /// error if something goes wrong.
    #[oai(path = "/user/register", method = "post", tag = "Tag::User")]
    async fn register(
        &self,
        server_key: Data<&ServerKey>,
        db: Data<&Surreal<Db>>,
        req: Json<LoginRequest>,
    ) -> Result<PlainText<String>> {
        let user: Option<UserRecord> = db
            .select(("user", req.0.username.as_str()))
            .await
            .map_err(InternalServerError)?;

        if let Some(_user) = user {
            return Err(poem::Error::from_status(StatusCode::CONFLICT));
        }

        // Goodies, we don't have this user yet!
        let password = req.password.clone();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(InternalServerError)?
            .to_string();

        let record: Option<Record> = db
            .create(("user", req.username.as_str()))
            .content(UserRecord {
                username: req.username.clone(),
                hash,
            })
            .await
            .map_err(InternalServerError)?;

        let exp = (Utc::now() + Duration::hours(5)).timestamp();

        let claims = User {
            username: req.username.clone(),
            exp,
        };

        let token = claims
            .sign_with_key(server_key.0)
            .map_err(InternalServerError)?;

        Ok(PlainText(token))
    }

    /// This will connect this to the embedded database and check for real accounts.
    /// It will then return a JWT token for the user.
    #[oai(path = "/user/login", method = "post", tag = "Tag::User")]
    async fn login(
        &self,
        server_key: Data<&ServerKey>,
        db: Data<&Surreal<Db>>,
        req: Json<LoginRequest>,
    ) -> Result<PlainText<String>> {
        let user: Option<UserRecord> = db
            .select(("user", req.0.username.as_str()))
            .await
            .map_err(InternalServerError)?;

        if let Some(user) = user {
            let passed_hash = PasswordHash::new(&user.hash).unwrap();
            let res = Argon2::default().verify_password(req.0.password.as_bytes(), &passed_hash);

            if res.is_err() {
                return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
            }

            let exp = (Utc::now() + Duration::hours(5)).timestamp();

            let token = User {
                username: req.0.username,
                exp,
            }
            .sign_with_key(server_key.0)
            .map_err(InternalServerError)?;

            return Ok(PlainText(token));
        }

        Err(poem::Error::from_status(StatusCode::UNAUTHORIZED))
    }

    /// Deletes the currently logged in user from jammy.
    #[oai(path = "/user/delete", method = "delete", tag = "Tag::User")]
    async fn delete_user(
        &self,
        auth: Authorizor,
        db: Data<&Surreal<Db>>,
        req: &Request,
    ) -> Result<PlainText<String>> {
        let user = auth.0.username;
        let deleted_one: Option<UserRecord> = db
            .delete(("user", user.as_str()))
            .await
            .map_err(InternalServerError)?;

        let mut blacklist = BLACKLIST.lock().unwrap();

        let headers = req.headers();
        let api_key = headers.get("X-API-Key").unwrap();

        blacklist
            .tokens
            .insert(api_key.to_str().unwrap_or("None").to_string());

        Ok(PlainText(format!("Deleted user {}", user)))
    }

    /// This API say hello to the currently logged in user
    #[oai(path = "/test/hello", method = "get", tag = "Tag::Test")]
    async fn hello(&self, auth: Authorizor) -> PlainText<String> {
        PlainText(format!("Hello {}", auth.0.username))
    }

    /// This API returns a hello message, whether you are logged in or not.
    #[oai(path = "/test/hello-anon", method = "get", tag = "Tag::Test")]
    async fn hello_anon(&self) -> PlainText<String> {
        PlainText("Hello, Anonymous!".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug,jammy=debug");
    }

    tracing_subscriber::fmt::init();

    let db = Surreal::new::<RocksDb>("db").await?;
    db.use_ns("test").use_db("test").await?;

    let api_service =
        OpenApiService::new(Api, "Jammy Backend", "1.0").server("https://localhost:3000/api");

    let ui = api_service.swagger_ui();

    let now = std::time::Instant::now();

    let mut rng = rand::rngs::OsRng;
    let mut key = [0u8; 32];
    rng.fill_bytes(&mut key);
    let salt = SaltString::generate(&mut rng)
        .to_string()
        .as_bytes()
        .to_vec();

    let mut server_key = [0u8; 32];

    // Generating the big server key takes a while, so we only do it in release mode.
    // We should warn the user that this is going to take a while.
    #[cfg(not(debug_assertions))]
    tracing::warn!("Generating server key, this will take a while...");

    #[allow(unused)]
    let release_configs = || {
        return (
            Algorithm::Argon2id,
            Version::V0x13,
            1 << 18, // 256 MB of memory
            128,     // 128 iterations for time cost
            8,       // 8 lanes for parallelism
        );
    };

    #[allow(unused)]
    let debug_configs = || {
        return (
            Algorithm::Argon2id,
            Version::V0x13,
            1 << 16, // 64 MB of memory
            8,       // 8 iterations for time cost
            8,       // 8 lanes for parallelism
        );
    };

    #[cfg(debug_assertions)]
    let (alogirthm, version, memory, iterations, lanes) = debug_configs();
    #[cfg(not(debug_assertions))]
    let (alogirthm, version, memory, iterations, lanes) = release_configs();

    let params = Params::new(memory, iterations, lanes, None).unwrap();

    Argon2::new(alogirthm, version, params)
        .hash_password_into(&key, &salt, &mut server_key)
        .unwrap();

    tracing::info!("Server key generated in {:?}", now.elapsed());

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(server_key)
        .data(db);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
