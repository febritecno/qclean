<script lang="ts">
	import Glyph from '$lib/components/Glyph.svelte';
	import type { IconSvgElement } from '@hugeicons/svelte';
	import type { Snippet } from 'svelte';

	let {
		icon,
		title,
		message,
		action,
		art
	}: {
		icon: IconSvgElement;
		title: string;
		message?: string;
		action?: Snippet;
		/** Replaces the icon tile with an illustration. */
		art?: Snippet;
	} = $props();
	const Icon = $derived(icon);
</script>

<div class="flex flex-col items-center justify-center px-6 py-14 text-center">
	{#if art}
		<div class="mb-3">{@render art()}</div>
	{:else}
		<span class="mb-4 grid h-14 w-14 place-items-center rounded-2xl bg-brand-soft text-brand">
			<Glyph icon={Icon} class="h-7 w-7" />
		</span>
	{/if}
	<p class="font-display text-base font-bold text-ink">{title}</p>
	{#if message}<p class="mt-1 max-w-sm text-sm text-muted">{message}</p>{/if}
	{#if action}<div class="mt-5">{@render action()}</div>{/if}
</div>
