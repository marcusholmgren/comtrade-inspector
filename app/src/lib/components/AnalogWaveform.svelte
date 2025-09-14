<!--
// app/src/lib/components/AnalogWaveform.svelte
// This file contains the AnalogWaveform component.
// This file exists to display a single analog waveform plot using uPlot.
// RELEVANT FILES: app/src/routes/waveform/+page.svelte
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import uPlot from 'uplot';
	import 'uplot/dist/uPlot.min.css';

	export let timestamps: number[];
	export let values: number[];
	export let channelName: string;
	export let strokeColor: string = 'rgb(75, 192, 192)';

	let chartContainer: HTMLDivElement;
	let plot: uPlot;

	onMount(() => {
		const data = [timestamps, values];

		const opts: uPlot.Options = {
			title: channelName,
			width: 800,
			height: 400,
			series: [
				{},
				{
					label: channelName,
					stroke: strokeColor,
					width: 2
				}
			],
			axes: [
				{
					label: 'Time (s)'
				},
				{
					label: 'Value'
				}
			]
		};

		plot = new uPlot(opts, data, chartContainer);

		// Resize the chart when the window is resized
		const resizeObserver = new ResizeObserver(() => {
			if (plot) {
				plot.setSize({
					width: chartContainer.clientWidth,
					height: 400
				});
			}
		});
		resizeObserver.observe(chartContainer);

		return () => {
			resizeObserver.disconnect();
		};
	});

	onDestroy(() => {
		if (plot) {
			plot.destroy();
		}
	});
</script>

<div bind:this={chartContainer}></div>
