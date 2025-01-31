import { Link } from "@inertiajs/react";

export default function Index() {
    return (
        <main className="main-container">
            <h1 className="mb-8 p-20 bg-amber-100 rounded-4xl">
                Testing Deferred Props
            </h1>
            <p>
                There's nothing to see here. Navigate to <Link href="built-in">Built-in Deferred</Link> to see the buggy
                behavior of the built-in Deferred component.
            </p>
            <p>
                Then, head to <Link href="custom">Custom Deferred page</Link> to see how it should behave.
            </p>
        </main>
    )
}