<!--
// app/src/routes/info/+page.svelte
// This file contains the information page of the application.
// This file exists to display the analysis results of the COMTRADE file.
// RELEVANT FILES: app/src/routes/+page.svelte, app/src/lib/store.ts
-->
<script lang="ts">
    import { analysisResult } from '$lib/store';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let result: unknown;

    onMount(() => {
        const unsubscribe = analysisResult.subscribe(value => {
            if (value) {
                result = value;
            } else {
                // If there's no result, redirect back to the upload page.
                goto('/');
            }
        });

        return () => unsubscribe();
    });
</script>

<header class="mb-8">
    <h2 class="text-4xl font-bold tracking-tight">
        COMTRADE File Information
    </h2>
</header>
<div class="space-y-8">
    <section class="bg-[#181C21] rounded-lg p-6">
        <h3 class="text-xl font-semibold mb-4">
            Analysis Result
        </h3>
        {#if result}
            <p>{result}</p>
        {:else}
            <p>No analysis result available. Please upload a file first.</p>
        {/if}
    </section>
</div>
