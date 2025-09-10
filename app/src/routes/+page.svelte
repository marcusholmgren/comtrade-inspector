<script lang="ts">
import init, { add } from 'comtrade_rust';
import { onMount } from 'svelte';

let result = $state<bigint | null>(null);
let initialized = $state(false);
let error = $state<string | null>(null);

onMount(async () => {
	try {
		// Initialize WASM module with explicit path to static file
		await init('/comtrade_rust_bg.wasm');
		initialized = true;
		result = add(2n, 3n);
		console.log("Addition:", result);
	} catch (err) {
		console.error("Failed to initialize WASM:", err);
		error = err instanceof Error ? err.message : String(err);
	}
});
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

{#if error}
	<div class="error">
		<h2>Error loading WASM:</h2>
		<p>{error}</p>
	</div>
{:else if initialized && result !== null}
	<div>
		<h2>WASM Result:</h2>
		<p>2 + 3 = {result.toString()}</p>
	</div>
{:else}
	<p>Loading WASM module...</p>
{/if}

<style>
	.error {
		background-color: #fee;
		border: 1px solid #fcc;
		padding: 1rem;
		border-radius: 4px;
		margin: 1rem 0;
	}
</style>
