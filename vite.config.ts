import { defineConfig, type PluginOption } from "vite";
import tailwind from "@tailwindcss/vite";
import tsConfigPaths from "vite-tsconfig-paths";
import laravel from "laravel-vite-plugin";
import react from "@vitejs/plugin-react";
import vue from "@vitejs/plugin-vue";
import { svelte } from "@sveltejs/vite-plugin-svelte";

import "dotenv/config";

const client = process.env.CLIENT;

const entries: string[] = [];

let plugins: PluginOption[] = [
    tsConfigPaths(),
    tailwind(),
    laravel({
        input: entries,
        buildDirectory: "build",
    })
];

if (client === "react") plugins.push(react()) && entries.push("www/react-app.tsx");
if (client === "svelte") plugins.push(svelte()) && entries.push("www/svelte-app.ts");
if (client === "vue") plugins.push(vue()) && entries.push("www/vue-app.ts");

const config = defineConfig({
    plugins,
});

export default config;