import "./app.css";

import { createApp, type DefineComponent, h } from 'vue';
import { createInertiaApp } from '@inertiajs/vue3';
import DefaultLayout from './layouts/default.vue';

createInertiaApp({
    progress: { includeCSS: true, color: "var(--color-purple-500)" },
    
    title: (title: string) => title ?? "Inertia Rust",
    
    resolve: (name: string) => {
        console.log("Hello from Vue Inertia App!");

        const pages = import.meta.glob('./pages/**/*.vue', { eager: true });
        const page = pages[`./pages/${name}.vue`] as DefineComponent;

        page.default.layout ??= DefaultLayout;

        return page;
    },
    
    setup({ el, App, props, plugin }) {
        createApp({ render: () => h(App, props) })
            .use(plugin)
            .mount(el);
    },
})
