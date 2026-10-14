<script lang="ts">
	import type { Snippet } from 'svelte';
	import { clamp } from '$lib/utils/format';

	let {
		size = 260,
		stroke = 6,
		/** 0..1 for a determinate arc; null for an indeterminate spinner */
		progress = null,
		tone = 'brand',
		children
	}: {
		size?: number;
		stroke?: number;
		progress?: number | null;
		tone?: 'brand' | 'magenta';
		children?: Snippet;
	} = $props();

	const r = $derived((size - stroke) / 2);
	const circumference = $derived(2 * Math.PI * r);
	const frac = $derived(progress === null ? 0.28 : clamp(progress, 0, 1));
	const dash = $derived(circumference * frac);
	const color = $derived(tone === 'magenta' ? 'rgb(var(--danger))' : 'rgb(var(--brand))');
</script>

<div class="relative grid place-items-center" style="width:{size}px;height:{size}px">
	<svg
		width={size}
		height={size}
		viewBox="0 0 {size} {size}"
		overflow="visible"
		class={progress === null
			? 'motion-safe:animate-[orbit_1.4s_linear_infinite] motion-reduce:animate-pulse'
			: ''}
	>
		<circle
			cx={size / 2}
			cy={size / 2}
			{r}
			fill="none"
			stroke="rgb(var(--line))"
			stroke-width={stroke}
		/>
		<circle
			cx={size / 2}
			cy={size / 2}
			{r}
			fill="none"
			stroke={color}
			stroke-width={stroke}
			stroke-linecap="round"
			pathLength={progress === null ? 1 : undefined}
			stroke-dasharray="{dash} {circumference}"
			transform="rotate(-90 {size / 2} {size / 2})"
			class={progress === null
				? 'indeterminate'
				: 'transition-[stroke-dasharray] duration-500 ease-out'}
		/>
	</svg>
	<div class="absolute inset-0 grid place-items-center text-center">
		{@render children?.()}
	</div>
</div>

<style>
	/* Indeterminate: the arc grows and shrinks while the whole ring turns, so it
	   reads as working rather than as a fixed slice. Lengths are in pathLength=1. */
	.indeterminate {
		animation: breathe 1.6s cubic-bezier(0.65, 0, 0.35, 1) infinite;
	}
	@keyframes breathe {
		0% {
			stroke-dasharray: 0.04 1;
			stroke-dashoffset: 0;
		}
		50% {
			stroke-dasharray: 0.45 1;
			stroke-dashoffset: -0.12;
		}
		100% {
			stroke-dasharray: 0.04 1;
			stroke-dashoffset: -0.96;
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.indeterminate {
			animation: none;
			stroke-dasharray: 0.28 1;
		}
	}
</style>
