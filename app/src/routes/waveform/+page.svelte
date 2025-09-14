<!--
// app/src/routes/waveform/+page.svelte
// This file contains the waveform page of the application.
// This file exists to display the waveform plots of the analog channels.
// RELEVANT FILES: app/src/lib/components/AnalogWaveform.svelte, app/src/lib/store.ts
-->
<script lang="ts">
	import { analysisResult } from '$lib/store';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { base } from '$app/paths';
	import AnalogWaveform from '$lib/components/AnalogWaveform.svelte';
	import Switch from '$lib/components/Switch.svelte';

	let result = $state<any>(null);
	let selectedChannels = $state<number[]>([]);
	let combineChannels = $state(false);

	const colors = [
		'#e6194b',
		'#3cb44b',
		'#ffe119',
		'#4363d8',
		'#f58231',
		'#911eb4',
		'#46f0f0',
		'#f032e6',
		'#bcf60c',
		'#fabebe',
		'#008080',
		'#e6beff',
		'#9a6324',
		'#fffac8',
		'#800000',
		'#aaffc3',
		'#808000',
		'#ffd8b1',
		'#000075',
		'#808080'
	];

	const combinedSeries = $derived(
		selectedChannels.map((channelIndex) => {
			const channel = result.analog_channels.find((c) => c.index === channelIndex);
			return {
				name: channel.name,
				values: channel.values,
				color: colors[channelIndex % colors.length]
			};
		})
	);

	onMount(() => {
		const unsubscribe = analysisResult.subscribe((value) => {
			if (value) {
				console.log(value);
				result = value;
			} else {
				goto(`${base}/`);
			}
		});

		return () => unsubscribe();
	});

	function toggleChannel(channelIndex: number) {
		if (selectedChannels.includes(channelIndex)) {
			selectedChannels = selectedChannels.filter((i) => i !== channelIndex);
		} else {
			selectedChannels = [...selectedChannels, channelIndex];
		}
	}
</script>

<svelte:head>
	<title>Analog Waveforms</title>
</svelte:head>

<div class="p-8">
	<h2 class="text-4xl font-bold tracking-tight">Analog Waveforms</h2>

	{#if result}
		<div class="my-8">
			<h3 class="text-xl font-semibold">Select Channels</h3>
			<div class="mt-4 grid grid-cols-2 gap-4 md:grid-cols-4 lg:grid-cols-6">
				{#each result.analog_channels as channel (channel.index)}
					<label class="flex items-center space-x-2">
						<input
							type="checkbox"
							checked={selectedChannels.includes(channel.index)}
							onchange={() => toggleChannel(channel.index)}
						/>
						<span>{channel.name}</span>
					</label>
				{/each}
			</div>
			<div class="mt-4">
				<Switch bind:checked={combineChannels} label="Combine channels in one plot" />
			</div>
		</div>

		<div class="space-y-8">
			{#if combineChannels && selectedChannels.length > 0}
				<div class="mt-4 rounded-lg bg-gray-800 p-4">
					<AnalogWaveform
						timestamps={result.timestamps}
						series={combinedSeries}
						title="Combined Analog Waveforms"
					/>
				</div>
			{:else}
				{#each selectedChannels as channelIndex (channelIndex)}
					{@const channel = result.analog_channels.find((c) => c.index === channelIndex)}
					{#if channel}
						<div>
							<h3 class="text-lg font-semibold">{channel.name}</h3>
							<p class="text-sm text-gray-400">
								{channel.circuit_component_being_monitored} - {channel.units}
							</p>
							<div class="mt-4 rounded-lg bg-gray-800 p-4">
								<AnalogWaveform
									timestamps={result.timestamps}
									series={[
										{
											name: channel.name,
											values: channel.values,
											color: colors[channelIndex % colors.length]
										}
									]}
									title={channel.name}
								/>
							</div>
						</div>
					{/if}
				{/each}
			{/if}
		</div>
	{:else}
		<p>Loading analysis results...</p>
	{/if}
</div>
