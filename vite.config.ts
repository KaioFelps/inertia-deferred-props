import { defineConfig } from "vite"
import react from "@vitejs/plugin-react";
import laravel from "laravel-vite-plugin";

const config = defineConfig({
    plugins: [
        react(),
        laravel({
            input: ["www/app.tsx"],
            buildDirectory: "build",
        }),
    ],
});

export default config;