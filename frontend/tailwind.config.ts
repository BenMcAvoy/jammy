import type { Config } from "tailwindcss";

export default {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "var(--background)",
        foreground: "var(--foreground)",

        primary: {
					DEFAULT: "var(--primary)",
					med: "var(--primary-med)",
					dark: "var(--primary-dark)",
				},

				secondary: {
					DEFAULT: "var(--secondary)",
					med: "var(--secondary-med)",
					dark: "var(--secondary-dark)",
				}
      },
    },
  },
  plugins: [],
} satisfies Config;
