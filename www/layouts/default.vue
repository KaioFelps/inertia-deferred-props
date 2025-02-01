<script setup lang="ts">
  import { Link } from "@inertiajs/vue3";
</script>

<template>    
    <header class="w-full p-8 bg-purple-100 flex items-center gap-4 text-2xl mb-12">
        <Link href="/" class="font-bold hover:bg-purple-200 active:bg-purple-300">
            Home
        </Link>
        <Link
            href="built-in"
            class="font-bold hover:bg-purple-200 active:bg-purple-300"
        >
            Built-in
        </Link>
        <Link
            disabled
            href=""
            class="font-bold opacity-50 line-through decoration-solid pointer-events-none"
        >
            Custom
        </Link>
    </header>
    
    <slot></slot>
</template>
