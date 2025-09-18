<!--
	/app/src/lib/components/Sidebar.svelte
	This file defines the sidebar component.
	This file exists to provide a reusable sidebar for the application.
	RELEVANT FILES:
	- /app/src/routes/+layout.svelte
-->
<script lang="ts">
	import { base } from '$app/paths';
	import { analysisResult } from '$lib/store';
	import { page } from '$app/stores';

	let {
		isSidebarOpen,
		toggleSidebar
	} = $props<{ isSidebarOpen: boolean; toggleSidebar: () => void }>();

	function handleLinkClick() {
		if (window.innerWidth < 768) {
			toggleSidebar();
		}
	}
</script>

<aside
	class="fixed z-10 h-full w-64 flex-col justify-between bg-[#181C21] p-6 print:hidden md:static md:flex"
	class:hidden={!isSidebarOpen}
>
	<div>
		<div class="mb-8 flex items-center gap-2">
			<span class="material-symbols-outlined text-3xl text-[var(--primary-color)]"> insights </span>
			<h1 class="text-xl font-bold">File Analyzer</h1>
		</div>
		<nav class="flex flex-col gap-2">
			<a
				class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]"
				href={base || '/'}
				class:bg-[var(--primary-color)]={$page.url.pathname === `${base}/`}
				class:text-white={$page.url.pathname === `${base}/`}
				on:click={handleLinkClick}
			>
				<span class="material-symbols-outlined">upload_file</span>
				<span>Upload</span>
			</a>
			{#if $analysisResult}
				<a
					class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]"
					href={`${base}/info`}
					class:bg-[var(--primary-color)]={$page.url.pathname === `${base}/info`}
					class:text-white={$page.url.pathname === `${base}/info`}
					on:click={handleLinkClick}
				>
					<span class="material-symbols-outlined">description</span>
					<span>File Info</span>
				</a>
				<a
					class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]"
					href={`${base}/waveform`}
					class:bg-[var(--primary-color)]={$page.url.pathname === `${base}/waveform`}
					class:text-white={$page.url.pathname === `${base}/waveform`}
					on:click={handleLinkClick}
				>
					<span class="material-symbols-outlined">show_chart</span>
					<span>Waveform</span>
				</a>
			{/if}
		</nav>
	</div>
	<div>
		<a
			class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]"
			href="#"
			on:click={handleLinkClick}
		>
			<span class="material-symbols-outlined">logout</span>
			<span>Logout</span>
		</a>
	</div>
</aside>