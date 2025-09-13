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

	interface AnalogChannel {
		index: number;
		name: string;
		units: string;
		min_value: number;
		max_value: number;
		multiplier: number;
		offset_adder: number;
	}

	interface ComtradeInfo {
		station: string;
		recording_device_id: string;
		start_time: string;
		trigger_time: string;
		data_format: string;
		frequency: number;
		analog_channels: AnalogChannel[];
	}

	let result: ComtradeInfo | null = null;

	onMount(() => {
		const unsubscribe = analysisResult.subscribe((value) => {
			if (value) {
				result = value as ComtradeInfo;
			} else {
				// If there's no result, redirect back to the upload page.
				goto('/');
			}
		});

		return () => unsubscribe();
	});

	const dateFormater = new Intl.DateTimeFormat('en-US', {
		year: 'numeric',
		month: '2-digit',
		day: '2-digit',
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit',
		fractionalSecondDigits: 3,
		timeZoneName: 'short'
	});

	const formatDate = (dateString: string) => {
		return dateFormater.format(new Date(dateString));
	};
</script>

<header class="mb-8">
	<h2 class="text-4xl font-bold tracking-tight">COMTRADE File Information</h2>
</header>
<div class="space-y-8">
	<section class="rounded-lg bg-[#181C21] p-6">
		<h3 class="mb-4 text-xl font-semibold">File Information (from .CFG)</h3>
		{#if result}
			<div class="grid grid-cols-1 gap-x-8 gap-y-4 md:grid-cols-2">
				<div class="flex justify-between border-b border-[#3b4754] pb-2">
					<p class="text-[#9dabb9]">Station</p>
					<p>{result.station}</p>
				</div>
				<div class="flex justify-between border-b border-[#3b4754] pb-2">
					<p class="text-[#9dabb9]">Recorder ID</p>
					<p>{result.recording_device_id}</p>
				</div>
				<div class="flex justify-between border-b border-[#3b4754] pb-2">
					<p class="text-[#9dabb9]">Start Time</p>
					<p>{formatDate(result.start_time)}</p>
				</div>
				<div class="flex justify-between border-b border-[#3b4754] pb-2">
					<p class="text-[#9dabb9]">File Type</p>
					<p>{result.data_format}</p>
				</div>
				<div class="flex justify-between border-b border-[#3b4754] pb-2">
					<p class="text-[#9dabb9]">Trigger Time</p>
					<p>{formatDate(result.trigger_time)}</p>
				</div>
				<div class="flex justify-between border-b border-[#3b4754] pb-2">
					<p class="text-[#9dabb9]">Frequency</p>
					<p>{result.frequency} Hz</p>
				</div>
			</div>
		{:else}
			<p>No analysis result available. Please upload a file first.</p>
		{/if}
	</section>
	<section>
		<h3 class="mb-4 text-xl font-semibold">Analog Channels</h3>
		<div class="overflow-x-auto rounded-lg bg-[#181C21]">
			<table class="min-w-full text-sm">
				<thead class="bg-[#283039] text-left">
					<tr>
						<th class="p-3">Channel</th>
						<th class="p-3">Name</th>
						<th class="p-3">Units</th>
						<th class="p-3">Min</th>
						<th class="p-3">Max</th>
						<th class="p-3">Multiplier</th>
						<th class="p-3">Offset</th>
					</tr>
				</thead>
				<tbody>
					{#if result}
						{#each result.analog_channels as channel, i}
							<tr class="border-b border-[#3b4754]">
								<td class="p-3 font-medium">{channel.index}</td>
								<td class="p-3 text-[#9dabb9]">{channel.name}</td>
								<td class="p-3 text-[#9dabb9]">{channel.units}</td>
								<td class="p-3 text-[#9dabb9]">{channel.min_value}</td>
								<td class="p-3 text-[#9dabb9]">{channel.max_value}</td>
								<td class="p-3 text-[#9dabb9]">{channel.multiplier}</td>
								<td class="p-3 text-[#9dabb9]">{channel.offset_adder}</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		</div>
	</section>
	<section>
		<h3 class="mb-4 text-xl font-semibold">Digital Channels</h3>
		<div class="overflow-x-auto rounded-lg bg-[#181C21]">
			<table class="min-w-full text-sm">
				<thead class="bg-[#283039] text-left">
					<tr>
						<th class="w-1/3 p-3">Channel</th>
						<th class="w-1/3 p-3">Name</th>
						<th class="w-1/3 p-3">Initial Value</th>
					</tr>
				</thead>
				<tbody>
					<tr class="border-b border-[#3b4754]">
						<td class="p-3 font-medium">1</td>
						<td class="p-3 text-[#9dabb9]"> Breaker Status </td>
						<td class="p-3 text-[#9dabb9]">0</td>
					</tr>
					<tr>
						<td class="p-3 font-medium">2</td>
						<td class="p-3 text-[#9dabb9]"> Trip Signal </td>
						<td class="p-3 text-[#9dabb9]">0</td>
					</tr>
				</tbody>
			</table>
		</div>
	</section>
</div>
