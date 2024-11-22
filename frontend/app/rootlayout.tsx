"use client";

import React from 'react';
import { QueryClientProvider, QueryClient } from '@tanstack/react-query';
import Header from './components/Header';

import { Inter } from "next/font/google";
const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
	const qp = new QueryClient();

	return (
		<html lang="en">
			<link rel="icon" href="/jammy.ico" sizes="any" />
			<body className={inter.variable}>
				<QueryClientProvider client={qp}>
					<Header />
					{children}
				</QueryClientProvider>
			</body>
		</html>
	);
}

