<script lang="ts">
	import Glyph from '$lib/components/Glyph.svelte';
	import { CheckmarkCircle02Icon as CircleCheck } from '@hugeicons/core-free-icons';
	import type { Snippet } from 'svelte';

	let {
		title,
		subtitle,
		complete = false,
		actions
	}: {
		title: string;
		subtitle?: string;
		complete?: boolean;
		actions?: Snippet;
	} = $props();
</script>

<div class="flex items-start justify-between gap-4">
	<div>
		<div class="flex items-center gap-2">
			{#if complete}
				<Glyph icon={CircleCheck} class="h-6 w-6 text-brand" />
			{/if}
			<h2 class="text-2xl font-semibold tracking-[-0.03em] text-ink">
				{title}
			</h2>
		</div>
		{#if subtitle}
			<p class="mt-1 text-sm text-muted">{subtitle}</p>
		{/if}
	</div>
	{#if actions}
		<div class="no-drag flex shrink-0 items-center gap-2">{@render actions()}</div>
	{/if}
</div>
