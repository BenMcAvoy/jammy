"use client";

import Image from "next/image";
import tkn from "../lib/jammy";
import { useState } from "react";
import useLogin from "../hooks/useLogin";

export default function Account() {
  const [email, setEmail] = useState('');
  const [pass, setPass] = useState('');

  // Declare the login function here at the top level
  const { mutate: login, isLoading, isError } = useLogin();

	const loginHandler = async (e: React.FormEvent<HTMLFormElement>) => {
  e.preventDefault();
  console.log("email: " + email + " pass: " + pass);

  // Ensure you're calling the mutate function properly
  try {
    await login({ email, pass });  // Make sure login expects these parameters
  } catch (error) {
    console.error("Error during login:", error);
  }
};

  // Check if loading
  if (isLoading) {
    return <p>Loading...</p>;
  }

  // Render login form if token is invalid
  if (!tkn.isValid) {
    return (
      <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
        <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
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
    return <p>Signed in as {tkn}</p>;
  }
}

