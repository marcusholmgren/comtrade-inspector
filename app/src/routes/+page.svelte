<!--
// app/src/routes/+page.svelte
// This file contains the main page of the application.
// This file exists to provide the main entry point for the user.
// RELEVANT FILES: app/src/lib/components/Upload.svelte, app/src/routes/info/+page.svelte
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import init, { parse_comtrade, init_panic_hook } from 'comtrade_rust';
	import Upload from '$lib/components/Upload.svelte';
	import { analysisResult } from '$lib/store';

	let initialized = $state(false);
	let error = $state<string | null>(null);

	onMount(async () => {
		try {
			await init('/comtrade_rust_bg.wasm');
			init_panic_hook();
			initialized = true;
		} catch (err) {
			console.error('Failed to initialize WASM:', err);
			error = err instanceof Error ? err.message : String(err);
		}
	});

	function handleAnalyse(
		event: CustomEvent<{
			data: unknown;
			cfgFileName?: string;
			datFileName?: string;
			cffFileName?: string;
		}>
	) {
		const result = {
			...(event.detail.data as object),
			cfgFileName: event.detail.cfgFileName,
			datFileName: event.detail.datFileName,
			cffFileName: event.detail.cffFileName
		};
		analysisResult.set(result);
		goto('/info');
	}
</script>

<div class="container mx-auto p-8">
	{#if error}
		<div class="rounded-lg border border-red-500/50 bg-red-500/20 p-4 text-red-400">
			<h2 class="font-bold">Error loading WASM module</h2>
			<p>{error}</p>
		</div>
	{:else if initialized}
		<Upload {parse_comtrade} on:analyse={handleAnalyse} />
	{:else}
		<p>Loading WASM module...</p>
	{/if}
</div>
