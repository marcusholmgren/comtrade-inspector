<!--
// app/src/lib/components/Upload.svelte
// This file contains the Svelte component for the file upload view.
// This file exists to provide a user interface for uploading COMTRADE files to be analyzed.
// RELEVANT FILES: app/src/routes/+page.svelte, comtrade_rust/src/lib.rs
-->
<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	let { parse_comtrade } = $props();

	let cfgFile = $state<File | null>(null);
	let datFile = $state<File | null>(null);
	let cffFile = $state<File | null>(null);
	let isDragging = $state(false);
	let error = $state<string | null>(null);
	let selectedEncoding = $state('utf-8');

	const dispatch = createEventDispatcher<{
		analyse: {
			data: unknown;
			cfgFileName?: string;
			datFileName?: string;
			cffFileName?: string;
		};
	}>();

	function handleFiles(files: FileList) {
		for (const file of files) {
			const lowerCaseName = file.name.toLowerCase();
			if (lowerCaseName.endsWith('.cfg')) {
				cfgFile = file;
				cffFile = null;
			} else if (lowerCaseName.endsWith('.dat')) {
				datFile = file;
				cffFile = null;
			} else if (lowerCaseName.endsWith('.cff')) {
				cffFile = file;
				cfgFile = null;
				datFile = null;
			}
		}
	}

	function onDrop(event: DragEvent) {
		event.preventDefault();
		isDragging = false;
		if (event.dataTransfer?.files) {
			handleFiles(event.dataTransfer.files);
		}
	}

	function onDragOver(event: DragEvent) {
		event.preventDefault();
		isDragging = true;
	}

	function onDragLeave() {
		isDragging = false;
	}

	function onFileSelected(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files) {
			handleFiles(input.files);
		}
	}

	async function analyseFiles() {
		error = null;
		try {
			let result;
			let fileInfo;

			if (cffFile) {
				const cffData = new Uint8Array(await cffFile.arrayBuffer());
				result = parse_comtrade(null, null, cffData, null);
				fileInfo = { cffFileName: cffFile.name };
			} else if (cfgFile && datFile) {
				const cfgData = new Uint8Array(await cfgFile.arrayBuffer());
				const datData = new Uint8Array(await datFile.arrayBuffer());
				result = parse_comtrade(cfgData, datData, null, selectedEncoding);
				fileInfo = { cfgFileName: cfgFile.name, datFileName: datFile.name };
			} else {
				return;
			}

			dispatch('analyse', {
				data: result,
				...fileInfo
			});
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		}
	}
</script>

<div class="mx-auto flex max-w-4xl flex-col items-center">
	<div class="mb-12 text-center">
		<h1 class="text-4xl leading-tight font-bold tracking-tighter text-white">
			Upload COMTRADE Files
		</h1>
		<p class="mt-4 max-w-2xl text-lg leading-relaxed font-normal text-gray-400">
			Drag and drop your .CFG and .DAT files as a pair, or a single .CFF file. You can also click
			the button to browse your computer.
		</p>
	</div>
	<div class="w-full max-w-2xl">
		<div
			class="flex h-80 w-full flex-col items-center justify-center rounded-lg border-2 border-dashed p-8 transition-colors {isDragging
				? 'border-blue-500 bg-gray-800/80'
				: 'border-gray-700 bg-gray-800/50'}"
			ondragover={onDragOver}
			ondragleave={onDragLeave}
			ondrop={onDrop}
		>
			<input
				type="file"
				id="file-input"
				class="hidden"
				multiple
				onchange={onFileSelected}
				accept=".cfg,.dat,.cff"
			/>
			<label for="file-input" class="flex cursor-pointer flex-col items-center gap-4">
				<div class="text-blue-500">
					<span class="material-symbols-outlined text-6xl"> upload_file </span>
				</div>
				<p class="text-xl leading-tight font-bold text-white">Drag &amp; drop files here</p>
				<p class="text-base font-normal text-gray-400">Supported file types: .CFG, .DAT, .CFF</p>
				<p class="text-sm text-gray-500">or</p>
				<span
					class="hover:bg-opacity-90 flex h-12 items-center justify-center gap-2 rounded-md bg-blue-500 px-6 text-base leading-normal font-bold tracking-wide text-white shadow-lg transition-all"
				>
					<span class="material-symbols-outlined"> folder_open </span>
					<span class="truncate">Browse Files</span>
				</span>
			</label>
		</div>
		<div class="mt-4 space-y-2">
			{#if cffFile}
				<div class="flex items-center justify-between rounded bg-gray-800 p-2">
					<span class="text-white">{cffFile.name}</span>
					<button onclick={() => (cffFile = null)} class="text-red-500 hover:text-red-400"
						>Remove</button
					>
				</div>
			{/if}
			{#if cfgFile}
				<div class="flex items-center justify-between rounded bg-gray-800 p-2">
					<span class="text-white">{cfgFile.name}</span>
					<button onclick={() => (cfgFile = null)} class="text-red-500 hover:text-red-400"
						>Remove</button
					>
				</div>
			{/if}
			{#if datFile}
				<div class="flex items-center justify-between rounded bg-gray-800 p-2">
					<span class="text-white">{datFile.name}</span>
					<button onclick={() => (datFile = null)} class="text-red-500 hover:text-red-400"
						>Remove</button
					>
				</div>
			{/if}
		</div>

		{#if cfgFile && datFile}
			<div class="mt-4">
				<label for="encoding-select" class="block text-sm font-medium text-gray-300"
					>CFG File Encoding:</label
				>
				<select
					id="encoding-select"
					bind:value={selectedEncoding}
					class="mt-1 block w-full rounded-md border-gray-600 bg-gray-700 text-white shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
				>
					<option value="utf-8">UTF-8</option>
					<option value="latin1">ISO-8859-1 (Latin-1)</option>
				</select>
			</div>
		{/if}

		{#if (cfgFile && datFile) || cffFile}
			<div class="mt-6 text-center">
				<button
					onclick={analyseFiles}
					class="rounded-md bg-green-600 px-8 py-3 text-lg font-semibold text-white shadow-lg transition-colors hover:bg-green-500 disabled:cursor-not-allowed disabled:bg-gray-600"
				>
					Analyse
				</button>
			</div>
		{/if}

		{#if error}
			<div class="mt-4 text-center text-red-400">
				<p>Error: {error}</p>
			</div>
		{/if}
	</div>
</div>
