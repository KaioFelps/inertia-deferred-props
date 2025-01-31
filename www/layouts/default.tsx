import { Link } from "@inertiajs/react";
import type { PropsWithChildren } from "react";

export function DefaultLayout({children}: PropsWithChildren) {
    return (
        <>
            <header className="w-full p-8 bg-purple-100 flex items-center gap-4 text-2xl mb-12">
                <Link href="/" className="font-bold hover:bg-purple-200 active:bg-purple-300">Home</Link>
                <Link href="built-in" className="font-bold hover:bg-purple-200 active:bg-purple-300">Built-in</Link>
                <Link href="custom" className="font-bold hover:bg-purple-200 active:bg-purple-300">Custom</Link>
            </header>
            {children}
        </>
    )
}