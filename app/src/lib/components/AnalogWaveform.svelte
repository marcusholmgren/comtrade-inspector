<!--
// app/src/lib/components/AnalogWaveform.svelte
// This file contains the AnalogWaveform component.
// This file exists to display a single analog waveform plot using uPlot.
// RELEVANT FILES: app/src/routes/waveform/+page.svelte
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import 'uplot/dist/uPlot.min.css';
	import type uPlot from 'uplot';

	let { timestamps, series, title, trigger_timestamp } = $props<{
		timestamps: number[];
		series: { name: string; values: number[]; color: string }[];
		title: string;
		trigger_timestamp: number;
	}>();

	let chartContainer: HTMLDivElement;
	let plot: uPlot | null = null;
	let uPlotClass: typeof uPlot | null = $state(null);

	$effect(() => {
		if (plot) {
			plot.destroy();
		}
		if (chartContainer && uPlotClass) {
			const data = [
				timestamps,
				...series.map((s: { name: string; values: number[]; color: string }) => s.values)
			];
			const uPlotSeries: uPlot.Series[] = [
				{},
				...series.map((s: { name: string; values: number[]; color: string }) => ({
					label: s.name,
					stroke: s.color,
					width: 2
				}))
			];
			const opts: uPlot.Options = {
				title: title,
				width: chartContainer ? chartContainer.clientWidth : 0,
				height: 400,
				series: uPlotSeries,
				axes: [{ label: 'Time (s)' }, { label: 'Value' }],
				hooks: {
					draw: [
						(u) => {
							if (trigger_timestamp !== undefined) {
								const x = u.valToPos(trigger_timestamp, 'x', true);
								const ctx = u.ctx;
								ctx.save();
								ctx.beginPath();
								ctx.setLineDash([5, 5]);
								ctx.strokeStyle = 'red';
								ctx.lineWidth = 2;
								ctx.moveTo(x, u.bbox.top);
								ctx.lineTo(x, u.bbox.top + u.bbox.height);
								ctx.stroke();
								ctx.restore();
							}
						}
					]
				}
			};
			plot = new uPlotClass(opts, data as uPlot.AlignedData, chartContainer);
		}
	});

	onMount(() => {
		let resizeObserver: ResizeObserver | null = null;

		(async () => {
			const module = await import('uplot');
			uPlotClass = module.default;

			resizeObserver = new ResizeObserver(() => {
				if (plot && chartContainer) {
					plot.setSize({
						width: chartContainer.clientWidth,
						height: 400
					});
				}
			});
			resizeObserver.observe(chartContainer);
		})();

		return () => {
			if (resizeObserver) {
				resizeObserver.disconnect();
			}
		};
	});

	onDestroy(() => {
		if (plot) {
			plot.destroy();
		}
	});
</script>

<div bind:this={chartContainer}></div>
