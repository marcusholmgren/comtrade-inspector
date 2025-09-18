<script lang="ts">
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
	class="fixed z-10 h-full w-64 flex flex-col justify-between bg-[#181C21] p-6 md:static md:flex md:h-auto print:hidden"
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
		<a class="flex items-center gap-3 rounded-md px-3 py-2 hover:bg-[#283039]" href="#">
			<span class="material-symbols-outlined">logout</span>
			<span>🌳</span>
		</a>
	</div>
</aside>
