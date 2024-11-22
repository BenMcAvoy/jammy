"use client";

import tkn from "../lib/jammy";
import { useState } from "react";
import useLogin from "../hooks/useLogin";

export default function Account() {
    const [email, setEmail] = useState('');
    const [pass, setPass] = useState('');

    // Declare the login function here at the top level
    const { mutate: login, isError } = useLogin();

    const [dummy, setDummy] = useState(0);

    const loginHandler = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        console.log("email: " + email + " pass: " + pass);

        // Ensure you're calling the mutate function properly
        try {
            login({ email, pass });  // Make sure login expects these parameters
            setDummy(dummy + 1);
        } catch (error) {
            console.error("Error during login:", error);
        }
    };

    // Check if error
    //if (isError) {
    //    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
    //        <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
    //            return <p>Error: {isError}</p>;
    //        </main>
    //    </div>
    //}

    // Render login form if token is invalid
    if (!tkn.isValid) {
        return (
            <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
                <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
                    {isError && <p>Something went wrong, check your credentials or try again later.</p>}
                    <form
                        onSubmit={loginHandler}
                        className="flex flex-col gap-4"
                    >
                        <input
                            type="text"
                            placeholder="Username"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            className="rounded-full border-secondary border-opacity-50 shadow-xl border-b-2 bg-primary text-white px-4 py-2 placeholder-gray-100"
                        />
                        <input
                            type="password"
                            placeholder="Password"
                            value={pass}
                            onChange={(e) => setPass(e.target.value)}
                            className="rounded-full border-secondary border-opacity-50 shadow-xl border-b-2 bg-primary text-white px-4 py-2 placeholder-white"
                        />
                        <button
                            type="submit"
                            className="bg-primary text-white px-4 py-2 rounded-full hover:bg-primary-med transition-all ease-out duration-300 transform hover:scale-110 border-secondary border-opacity-50 shadow-xl border-b-2"
                        >
                            Login
                        </button>
                    </form>
                </main>
            </div>
        );
    } else {
        return (
            <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
                <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
                    <p>Signed in as {tkn.token}</p>
                </main>
            </div>
        );
    }
}

