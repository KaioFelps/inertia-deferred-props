<script lang="ts">
	import { Link } from "@inertiajs/svelte";
</script>

<main class="main-container">
	<h1 class="mb-8 p-20 bg-amber-100 rounded-4xl">Testing Deferred Props</h1>
	<p>
		The bug with React's Deferred component doesn't seem to affect the Svelte adapter. You can
		navigate to <Link href="built-in">Built-in Deferred</Link> and to the
		<Link href="custom">Custom Deferred page</Link> and check that they act exactly equal. No bug at
		all.
	</p>
</main>
