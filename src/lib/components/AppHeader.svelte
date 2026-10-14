<script lang="ts">
	import type { Snippet } from 'svelte';
	import Logo from './Logo.svelte';

	let { actions }: { actions?: Snippet } = $props();
</script>

<header data-tauri-drag-region class="drag flex items-center justify-between gap-4">
	<div data-tauri-drag-region class="flex items-center gap-2.5">
		<Logo size={30} />
		<h1 data-tauri-drag-region class="text-[1.05rem] font-semibold tracking-[-0.02em] text-ink">
			QClean
		</h1>
	</div>
	{#if actions}
		<div class="no-drag flex shrink-0 items-center gap-1">
			{@render actions()}
		</div>
	{/if}
</header>
