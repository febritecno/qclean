<script lang="ts">
	import Glyph from '$lib/components/Glyph.svelte';
	import { Tick02Icon as Check, Loading03Icon as LoaderCircle } from '@hugeicons/core-free-icons';
	import type { IconSvgElement } from '@hugeicons/svelte';
	import type { Snippet } from 'svelte';
	import { pop } from '$lib/utils/motion';

	export interface OrbitItem {
		label: string;
		icon: IconSvgElement;
		detail?: string;
		status?: 'idle' | 'active' | 'done';
	}

	let { items, ring }: { items: OrbitItem[]; ring: Snippet } = $props();

	const mid = $derived(Math.ceil(items.length / 2));
	const left = $derived(items.slice(0, mid));
	const right = $derived(items.slice(mid));
</script>

<div class="relative mx-auto flex max-w-4xl items-center justify-center gap-4 py-2 sm:gap-8">
	<svg
		class="pointer-events-none absolute inset-0 h-full w-full opacity-70"
		aria-hidden="true"
		preserveAspectRatio="none"
		viewBox="0 0 800 360"
	>
		<g
			fill="none"
			stroke="rgb(var(--line))"
			stroke-width="1.5"
			stroke-dasharray="2 7"
			stroke-linecap="round"
		>
			<g class="motion-safe:animate-orbit" style="transform-origin:400px 180px">
				<ellipse cx="400" cy="180" rx="330" ry="120" />
				<circle cx="730" cy="180" r="4" fill="rgb(var(--accent))" stroke="none" />
			</g>
			<g class="motion-safe:animate-orbit-rev" style="transform-origin:400px 180px">
				<ellipse cx="400" cy="180" rx="260" ry="95" />
				<circle cx="140" cy="180" r="3.5" fill="rgb(var(--brand))" stroke="none" />
			</g>
		</g>
	</svg>

	{#snippet chip(item: OrbitItem, align: 'left' | 'right', i: number)}
		{@const Icon = item.icon}
		<li
			in:pop|global={{ start: 0.94, y: 6, duration: 240, delay: 60 + i * 50 }}
			class="flex items-center gap-3 {align === 'right' ? 'flex-row-reverse text-right' : ''}"
		>
			<span
				class="grid h-11 w-11 shrink-0 place-items-center rounded-2xl border border-line bg-surface-2 text-brand shadow-tile"
			>
				<Glyph icon={Icon} class="h-5 w-5" />
			</span>
			<div class="min-w-0">
				<p class="truncate text-sm font-semibold text-ink">{item.label}</p>
				{#if item.detail}<p class="truncate text-xs text-muted">{item.detail}</p>{/if}
			</div>
			{#if item.status === 'done'}
				<span class="grid h-5 w-5 place-items-center rounded-full gradient-brand">
					<Glyph icon={Check} class="h-3 w-3" strokeWidth={3.5} />
				</span>
			{:else if item.status === 'active'}
				<Glyph icon={LoaderCircle} class="h-4 w-4 shrink-0 animate-spin text-brand" />
			{/if}
		</li>
	{/snippet}

	<ul class="relative z-10 flex w-44 flex-col gap-7 sm:w-52">
		{#each left as item, i (item.label)}{@render chip(item, 'left', i)}{/each}
	</ul>

	<div class="relative z-10 shrink-0 rounded-full">{@render ring()}</div>

	<ul class="relative z-10 flex w-44 flex-col items-end gap-7 sm:w-52">
		{#each right as item, i (item.label)}{@render chip(item, 'right', mid + i)}{/each}
	</ul>
</div>
