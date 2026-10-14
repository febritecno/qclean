<script lang="ts">
	import Dialog from '$lib/components/ui/Dialog.svelte';
	import Logo from '$lib/components/Logo.svelte';
	import { formatBytesCompact } from '$lib/utils/format';
	import { system } from '$lib/stores/system.svelte';

	let { open = $bindable(false) }: { open?: boolean } = $props();
	const info = $derived(system.info);
</script>

<Dialog bind:open title="About QClean" size="sm">
	<div class="flex flex-col items-center gap-3 text-center">
		<Logo size={64} animated />
		<div>
			<p class="text-lg font-semibold tracking-[-0.02em] text-ink">QClean</p>
			<p class="text-sm text-muted">Version {info?.appVersion ?? '—'}</p>
		</div>
		<p class="max-w-xs text-sm text-muted">
			A native cleanup utility for macOS, Linux and Windows. The Rust engine owns every filesystem
			operation; deletion is only ever allowed for items found in the current scan.
		</p>
		{#if info}
			<dl class="mt-1 grid w-full grid-cols-2 gap-x-4 gap-y-1 text-left text-xs text-muted">
				<dt class="text-faint">Architecture</dt>
				<dd class="text-right text-ink">{info.arch}</dd>
				<dt class="text-faint">User</dt>
				<dd class="truncate text-right text-ink">
					{info.currentUser}{info.isAdmin ? ' (root)' : ''}
				</dd>
				<dt class="text-faint">Disk free</dt>
				<dd class="text-right text-ink">
					{formatBytesCompact(info.availableDiskBytes)} / {formatBytesCompact(info.totalDiskBytes)}
				</dd>
			</dl>
		{/if}
		<p class="mt-2 text-xs text-faint">Apache-2.0 · Based on MacClean, © 2026 bkrajendra</p>
	</div>
</Dialog>
