<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Sidebar from '$lib/components/Sidebar.svelte';

	export const prerender = true;
	export const ssr = false;

	let { children } = $props();

	let isSidebarOpen = $state(false);

	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
	}

	import { onMount } from 'svelte';
	onMount(() => {
		// Log app build info on load
		console.log(`comtrade-inspector v${__APP_VERSION__} (${__GIT_HASH__})`);
	});
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
	<Sidebar {isSidebarOpen} {toggleSidebar} />
	<main class="flex-1 p-8">
		<button
			class="fixed top-4 left-4 z-20 md:hidden"
			onclick={toggleSidebar}
			aria-label="Toggle sidebar"
		>
			<span class="material-symbols-outlined text-3xl">menu</span>
		</button>
		{@render children?.()}
	</main>
</div>
