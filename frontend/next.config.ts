import type { NextConfig } from "next";

const rewrites = async () => {
    return [
        {
            source: '/api/:path*',
            destination: 'http://localhost:3000/api/:path*',
        },
    ];
};

const nextConfig: NextConfig = {
    rewrites: rewrites,
};

export default nextConfig;
