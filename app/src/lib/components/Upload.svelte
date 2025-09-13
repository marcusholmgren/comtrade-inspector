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
    let isDragging = $state(false);
    let error = $state<string | null>(null);

    const dispatch = createEventDispatcher<{
        analyse: { data: unknown };
    }>();

    function handleFiles(files: FileList) {
        for (const file of files) {
            const lowerCaseName = file.name.toLowerCase();
            if (lowerCaseName.endsWith('.cfg')) {
                cfgFile = file;
            } else if (lowerCaseName.endsWith('.dat')) {
                datFile = file;
            } else if (lowerCaseName.endsWith('.cff')) {
                // For CFF files, we can extract both CFG and DAT, but that's a future enhancement.
                // For now, we'll just note that it's a CFF file.
                console.log("CFF file selected, but not yet supported for automatic parsing.");
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
        if (!cfgFile || !datFile) {
            return;
        }
        error = null;
        try {
            const cfgData = new Uint8Array(await cfgFile.arrayBuffer());
            const datData = new Uint8Array(await datFile.arrayBuffer());
            const result = parse_comtrade(cfgData, datData);
            dispatch('analyse', {
                data: result,
                cfgFileName: cfgFile.name,
                datFileName: datFile.name
            });
        } catch (e) {
            error = e instanceof Error ? e.message : String(e);
        }
    }
</script>

<div class="mx-auto flex max-w-4xl flex-col items-center">
    <div class="mb-12 text-center">
        <h1 class="text-white text-4xl font-bold leading-tight tracking-tighter">
            Upload COMTRADE Files
        </h1>
        <p class="mt-4 max-w-2xl text-gray-400 text-lg font-normal leading-relaxed">
            Drag and drop your .CFG and .DAT files as a pair, or a single .CFF file. You can also click the button to browse your computer.
        </p>
    </div>
    <div class="w-full max-w-2xl">
        <div
            class="flex flex-col items-center justify-center w-full h-80 rounded-lg border-2 border-dashed p-8 transition-colors {isDragging
                ? 'border-blue-500 bg-gray-800/80'
                : 'border-gray-700 bg-gray-800/50'}"
            on:dragover={onDragOver}
            on:dragleave={onDragLeave}
            on:drop={onDrop}
        >
            <input type="file" id="file-input" class="hidden" multiple on:change={onFileSelected} accept=".cfg,.dat,.cff" />
            <label for="file-input" class="flex flex-col items-center gap-4 cursor-pointer">
                <div class="text-blue-500">
                    <span class="material-symbols-outlined text-6xl">
                        upload_file
                    </span>
                </div>
                <p class="text-white text-xl font-bold leading-tight">
                    Drag &amp; drop files here
                </p>
                <p class="text-gray-400 text-base font-normal">
                    Supported file types: .CFG, .DAT, .CFF
                </p>
                <p class="text-gray-500 text-sm">or</p>
                <span class="flex items-center justify-center gap-2 rounded-md h-12 px-6 bg-blue-500 text-white text-base font-bold leading-normal tracking-wide shadow-lg hover:bg-opacity-90 transition-all">
                    <span class="material-symbols-outlined">
                        folder_open
                    </span>
                    <span class="truncate">Browse Files</span>
                </span>
            </label>
        </div>
        <div class="mt-4 space-y-2">
            {#if cfgFile}
                <div class="flex justify-between items-center bg-gray-800 p-2 rounded">
                    <span class="text-white">{cfgFile.name}</span>
                    <button on:click={() => cfgFile = null} class="text-red-500 hover:text-red-400">Remove</button>
                </div>
            {/if}
            {#if datFile}
                <div class="flex justify-between items-center bg-gray-800 p-2 rounded">
                    <span class="text-white">{datFile.name}</span>
                    <button on:click={() => datFile = null} class="text-red-500 hover:text-red-400">Remove</button>
                </div>
            {/if}
        </div>

        {#if cfgFile && datFile}
            <div class="mt-6 text-center">
                <button
                    on:click={analyseFiles}
                    class="rounded-md bg-green-600 px-8 py-3 text-lg font-semibold text-white shadow-lg hover:bg-green-500 transition-colors disabled:bg-gray-600 disabled:cursor-not-allowed"
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
