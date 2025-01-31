import "./app.css";

import { createInertiaApp } from "@inertiajs/svelte";
import { mount } from "svelte";
import DefaultLayout from "./layouts/default.svelte";

createInertiaApp({
    progress: { includeCSS: true, color: "var(--color-purple-500)" },
    
    resolve: (pageName: string) => {
        console.log("Hello from Svelte Inertia App!");

        const pages = import.meta.glob("./pages/**/*.svelte", { eager: true });
        const page = pages[`./pages/${ pageName }.svelte`] as SveltePage;

        if (!page) throw new Error(`Could not find page ${pageName}.`);
        
        return {default: page.default, layout: page.layout ?? DefaultLayout};
    },
    
    setup({ el, App, props }) {
      mount(App, { target: el!, props })
    },
})

type SveltePage = {
    default: any,
    layout?: any
}