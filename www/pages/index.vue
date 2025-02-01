<script setup lang="ts">
    import { Link } from "@inertiajs/vue3";
</script>

<template>
    <main className="main-container">
        <h1 className="mb-8 p-20 bg-amber-100 rounded-4xl">
            Testing Deferred Props
        </h1>
        <p>
            Deferred component is working fine in Vue 3 either. Navigate to
            <Link href="built-in">Built-in Deferred</Link> and click the button to see.
        </p>
        <p>
            As it's working as expected, there is no reason for trying a custom Deferred
            component.
        </p>
    </main>
</template>