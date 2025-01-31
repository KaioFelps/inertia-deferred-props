import { defineConfig, PluginOption } from "vite";
import react from "@vitejs/plugin-react";
import laravel from "laravel-vite-plugin";
import tailwind from "@tailwindcss/vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

import "dotenv/config";

const client = process.env.CLIENT;

let plugins: PluginOption[] = [
    tailwind(),
    laravel({
        input: ["www/app.tsx"],
        buildDirectory: "build",
    })
];

if (client === "react") plugins.push(react());
if (client === "svelte") plugins.push(svelte());

const config = defineConfig({
    plugins,
});

export default config;