<script lang="ts">
	import { Link } from "@inertiajs/svelte";
	import type { Snippet } from "svelte";

	let { children }: { children?: Snippet } = $props();
</script>

<header class="w-full p-8 bg-purple-100 flex items-center gap-4 text-2xl mb-12">
	<Link href="/" class="font-bold hover:bg-purple-200 active:bg-purple-300">Home</Link>
	<Link href="built-in" class="font-bold hover:bg-purple-200 active:bg-purple-300">Built-in</Link>
	<Link href="custom" class="font-bold hover:bg-purple-200 active:bg-purple-300">Custom</Link>
</header>
{@render children?.()}
