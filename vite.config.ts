import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import laravel from "laravel-vite-plugin";
import tailwind from "@tailwindcss/vite";

const config = defineConfig({
    plugins: [
        tailwind(),
        react(),
        laravel({
            input: ["www/app.tsx"],
            buildDirectory: "build",
        }),
    ],
});

export default config;