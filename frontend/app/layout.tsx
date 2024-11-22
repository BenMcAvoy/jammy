import type { Metadata } from "next";
import "./globals.css";

import { config } from '@fortawesome/fontawesome-svg-core'
import '@fortawesome/fontawesome-svg-core/styles.css'
config.autoAddCss = false

import { QueryClientProvider, QueryClient } from "@tanstack/react-query";

import Header from "./components/Header";
import RootLayout from "./rootlayout";


export const metadata: Metadata = {
  title: "jammy",
  description: "A project jamming competition",
	favicon: "/jammy.ico",
};

export default RootLayout;
