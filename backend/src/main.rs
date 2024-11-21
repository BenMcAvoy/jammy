use surrealdb::RecordId;
use surrealdb::Surreal;

use rand::Rng;

use hmac::{Hmac, Mac};
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

//const SERVER_KEY: &[u8] = b"123456";

type ServerKey = Hmac<Sha256>;

use surrealdb::engine::local::RocksDb;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
}

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "header",
    checker = "api_checker"
)]
struct MyApiKeyAuthorization(User);

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<User> {
    let server_key = req.data::<ServerKey>().unwrap();
    VerifyWithKey::<User>::verify_with_key(api_key.key.as_str(), server_key).ok()
}

#[derive(Object)]
struct LoginRequest {
    username: String,
    password: String,
}

struct Api;

#[OpenApi]
#[allow(unused_variables)]
impl Api {
    /// Example account: `admin`, `123456`
    /// Soon we will connect this to the embedded database for real accounts.
    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        server_key: Data<&ServerKey>,
        req: Json<LoginRequest>,
    ) -> Result<PlainText<String>> {
        // Here, we can check the password and username.
        // In a real application, you should check the password and username against the database.
        if req.0.username != "admin" || req.0.password != "123456" {
            return Err(poem::Error::from_status(StatusCode::UNAUTHORIZED));
        }

        let token = User {
            username: req.0.username,
        }
        .sign_with_key(server_key.0)
        .map_err(InternalServerError)?;
        Ok(PlainText(token))
    }

    /// This API returns the currently logged in user.
    #[oai(path = "/hello", method = "get")]
    async fn hello(&self, auth: MyApiKeyAuthorization) -> PlainText<String> {
        PlainText(auth.0.username)
    }

    /// This API returns a hello message, whether you are logged in or not.
    #[oai(path = "/hello-anon", method = "get")]
    async fn hello_anon(&self) -> PlainText<String> {
        PlainText("Hello, Anonymous!".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    tracing_subscriber::fmt::init();

    let db = Surreal::new::<RocksDb>("db").await?;
    db.use_ns("test").use_db("test").await?;

    let api_service =
        OpenApiService::new(Api, "Jammy Backend", "1.0").server("http://localhost:3000/api");

    let ui = api_service.swagger_ui();

    let mut rng = rand::thread_rng();
    let key_seed = format!("{:x}", rng.gen::<u64>()); // NOTE: Not cryptographically secure,
                                                      // replace with a real key derivation function in production.
    let server_key = Hmac::<Sha256>::new_from_slice(key_seed.as_ref()).unwrap();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(server_key);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
