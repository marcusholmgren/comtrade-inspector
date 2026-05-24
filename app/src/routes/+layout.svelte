<!-- app/src/routes/+layout.svelte -->
<!-- The root layout component of the SvelteKit application, providing the sidebar and main layout. -->
<!-- This file exists to define global styles, initialize common resources, log build details, and register the PWA service worker. -->
<!-- RELEVANT FILES: app/src/routes/+layout.ts, app/src/app.html, app/vite.config.ts -->
<script lang="ts">
	// app/src/routes/+layout.svelte
	// Defines the main layout structure, sidebar integration, and PWA manifest loading.
	// This file exists to establish a consistent shell across all routes and load the service worker manifest.
	// RELEVANT FILES: app/src/routes/+layout.ts, app/vite.config.ts, app/src/app.d.ts

	import '../app.css';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { asset } from '$app/paths';
	import { pwaInfo } from 'virtual:pwa-info';

	let { children } = $props();

	let isSidebarOpen = $state(false);

	function toggleSidebar() {
		isSidebarOpen = !isSidebarOpen;
	}

	const webManifestLink = pwaInfo ? pwaInfo.webManifest.linkTag : '';

	import { onMount } from 'svelte';
	onMount(async () => {
		// Log app build info on load
		console.log(`comtrade-inspector v${__APP_VERSION__} (${__GIT_HASH__})`);

		// Register service worker if supported and not in dev environment
		if ('serviceWorker' in navigator && !import.meta.env.DEV) {
			try {
				const { registerSW } = await import('virtual:pwa-register');
				registerSW({
					immediate: true,
					onNeedRefresh() {
						console.log('New app version is available. Please reload.');
					},
					onOfflineReady() {
						console.log('COMTRADE Inspector is ready for offline use!');
					}
				});
			} catch (error) {
				console.error('Failed to register service worker:', error);
			}
		}
	});
</script>

<svelte:head>
	<link rel="icon" type="image/svg+xml" href={asset('/favicon.svg')} />
	<link rel="alternate icon" href={asset('/favicon.svg')} />
	<!-- eslint-disable-next-line svelte/no-at-html-tags -->
	{@html webManifestLink}
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
