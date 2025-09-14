<script lang="ts">
	let {
		label,
		fontSize = 16,
		checked = $bindable(false)
	} = $props<{
		label: string;
		fontSize?: number;
		checked?: boolean;
	}>();

	const uniqueID = `switch-${Math.floor(Math.random() * 1000)}`;
</script>

<div class="s s--slider" style="font-size:{fontSize}px">
	<span id={uniqueID}>{label}</span>
	<button
		role="switch"
		aria-checked={checked}
		aria-labelledby={uniqueID}
		on:click={() => (checked = !checked)}
	>
	</button>
</div>

<style>
	:root {
		--accent-color: CornflowerBlue;
		--gray: #ccc;
	}

	/* Slider Design Option */
	.s--slider {
		display: flex;
		align-items: center;
	}

	.s--slider button {
		width: 3em;
		height: 1.6em;
		position: relative;
		margin: 0 0 0 0.5em;
		background: var(--gray);
		border: none;
		border-radius: 1.5em;
	}

	.s--slider button::before {
		content: '';
		position: absolute;
		width: 1.3em;
		height: 1.3em;
		background: #fff;
		top: 0.13em;
		left: 0.15em;
		transition: transform 0.3s;
		border-radius: 100%;
	}

	.s--slider button[aria-checked='true'] {
		background-color: var(--accent-color);
	}

	.s--slider button[aria-checked='true']::before {
		transform: translateX(1.4em);
	}

	.s--slider button:focus {
		box-shadow: 0 0px 8px var(--accent-color);
	}
</style>