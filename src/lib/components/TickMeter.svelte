<!--
	A row of ticks that fill left to right. Used for disk usage and for progress:
	each tick lights slightly after its neighbour, so a change sweeps across.
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { clamp } from '$lib/utils/format';

	let {
		value,
		ticks = 48,
		label,
		class: klass = ''
	}: {
		/** 0..1 */
		value: number;
		ticks?: number;
		label: string;
		class?: string;
	} = $props();

	// Starts empty so the first paint animates up to the real value.
	let mounted = $state(false);
	onMount(() => {
		const raf = requestAnimationFrame(() => (mounted = true));
		return () => cancelAnimationFrame(raf);
	});

	const lit = $derived(mounted ? Math.round(clamp(value, 0, 1) * ticks) : 0);
</script>

<div
	role="meter"
	aria-label={label}
	aria-valuemin={0}
	aria-valuemax={100}
	aria-valuenow={Math.round(clamp(value, 0, 1) * 100)}
	class="flex h-9 items-stretch gap-[3px] {klass}"
>
	{#each { length: ticks } as _, i (i)}
		<span class="tick flex-1 rounded-[2px]" class:lit={i < lit} style="transition-delay:{i * 9}ms"
		></span>
	{/each}
</div>

<style>
	.tick {
		background: rgb(var(--line));
		transform: scaleY(0.55);
		transition:
			background-color 0.25s var(--ease-out),
			transform 0.35s var(--ease-spring);
	}
	.tick.lit {
		background: rgb(var(--brand));
		transform: scaleY(1);
	}
	@media (prefers-reduced-motion: reduce) {
		.tick {
			transition: none;
		}
	}
</style>
