<script lang="ts">
	// app/src/lib/components/Sidebar.svelte
	// Renders the responsive side navigation panel for the application.
	// This file exists to provide page routing navigation and brand presentation.
	// RELEVANT FILES: app/src/routes/+layout.svelte, app/vite.config.ts

	import { resolve } from '$app/paths';
	import { analysisResult } from '$lib/store';
	import { page } from '$app/state';

	let { isSidebarOpen, toggleSidebar } = $props<{
		isSidebarOpen: boolean;
		toggleSidebar: () => void;
	}>();

	function handleLinkClick() {
		if (window.innerWidth < 768) {
			toggleSidebar();
		}
	}
</script>

<aside
	class="fixed z-10 flex h-full w-64 flex-col justify-between border-r border-[#283039]/40 bg-[#181C21] p-6 md:static md:flex md:h-auto print:hidden"
	class:hidden={!isSidebarOpen}
>
	<div>
		<div class="mb-8 flex flex-col gap-1 border-b border-[#283039]/50 pb-6">
			<div class="flex items-center gap-3">
				<div
					class="flex h-10 w-10 items-center justify-center rounded-xl bg-gradient-to-tr from-[#1173d4] to-[#00d2ff] shadow-md ring-1 shadow-[#1173d4]/10 ring-white/15"
				>
					<span class="material-symbols-outlined text-2xl text-white">query_stats</span>
				</div>
				<div class="flex flex-col">
					<h1
						class="bg-gradient-to-r from-white via-[#f1f5f9] to-[#cbd5e1] bg-clip-text text-base leading-tight font-black tracking-wider text-transparent uppercase"
					>
						COMTRADE
					</h1>
					<span
						class="mt-0.5 text-xs leading-none font-bold tracking-widest text-[#00d2ff] uppercase"
					>
						Inspector
					</span>
				</div>
			</div>
		</div>
		<nav class="flex flex-col gap-2">
			<a
				class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]"
				href={resolve('/')}
				class:bg-[var(--primary-color)]={page.url.pathname === `${resolve('/')}`}
				class:text-white={page.url.pathname === `${resolve('/')}`}
				onclick={handleLinkClick}
			>
				<span class="material-symbols-outlined">upload_file</span>
				<span>Upload</span>
			</a>
			{#if $analysisResult}
				<a
					class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]"
					href={resolve('/info')}
					class:bg-[var(--primary-color)]={page.url.pathname === `${resolve('/info')}`}
					class:text-white={page.url.pathname === `${resolve('/info')}`}
					onclick={handleLinkClick}
				>
					<span class="material-symbols-outlined">description</span>
					<span>File Info</span>
				</a>
				<a
					class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]"
					href={resolve('/waveform')}
					class:bg-[var(--primary-color)]={page.url.pathname === `${resolve('/waveform')}`}
					class:text-white={page.url.pathname === `${resolve('/waveform')}`}
					onclick={handleLinkClick}
				>
					<span class="material-symbols-outlined">show_chart</span>
					<span>Waveform</span>
				</a>
			{/if}
		</nav>
	</div>
	<div>
		<button class="flex w-full items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]">
			<span class="material-symbols-outlined">logout</span>
			<span>🌳</span>
		</button>
	</div>
</aside>
