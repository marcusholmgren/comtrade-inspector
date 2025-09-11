<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Sidebar from '$lib/components/Sidebar.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<style>
		:root {
			--primary-color: #1173d4;
		}
	</style>
</svelte:head>

<div class="flex min-h-screen">
	<Sidebar />
	<main class="flex-1 p-8">
		{@render children?.()}
	</main>
</div>
