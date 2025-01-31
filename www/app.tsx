import "./app.css";

import { createInertiaApp } from "@inertiajs/react";
import { createRoot } from "react-dom/client";

createInertiaApp({
    progress: { includeCSS: true, color: "#fff800" },
    
    title: (title: string) => title ?? "Inertia Rust",
    
    resolve: (pageName: string) => {
        const pages = import.meta.glob("./pages/**/*.tsx", { eager: true });
        const page = pages[`./pages/${ pageName }.tsx`];
        return page;
    },
    
    setup: ({ App, el,props }) => {
        createRoot(el).render(<App {...props} />)
    },
})
