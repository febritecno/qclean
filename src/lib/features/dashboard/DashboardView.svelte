<script lang="ts">
	import Glyph from '$lib/components/Glyph.svelte';
	import {
		ArrowRight01Icon as ArrowRight,
		ArrowDown01Icon as ChevronDown
	} from '@hugeicons/core-free-icons';
	import { slide } from 'svelte/transition';
	import Button from '$lib/components/ui/Button.svelte';
	import Segmented from '$lib/components/ui/Segmented.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import TickMeter from '$lib/components/TickMeter.svelte';
	import PermissionNotice from '$lib/features/permissions/PermissionNotice.svelte';
	import type { Mode, ScopeDescriptor } from '$lib/types/ipc';
	import { formatBytesCompact } from '$lib/utils/format';
	import { easeOut } from '$lib/utils/motion';
	import { settings } from '$lib/stores/settings.svelte';
	import { scan } from '$lib/stores/scan.svelte';
	import { system } from '$lib/stores/system.svelte';

	let {
		scopes,
		onShowRules,
		onShowPermissions
	}: {
		scopes: ScopeDescriptor[];
		onShowRules: () => void;
		onShowPermissions: () => void;
	} = $props();

	let extra = $state(settings.extraRoots.join(', '));
	let showOptions = $state(false);

	const modeOptions: ReadonlyArray<{ value: Mode; label: string }> = [
		{ value: 'safe', label: 'Safe' },
		{ value: 'aggressive', label: 'Aggressive' }
	];
	const scopeOptions = $derived(scopes.map((s) => ({ value: s.value, label: s.label })));
	const scope = $derived(scopes.find((s) => s.value === settings.scope));
	const modeHint = $derived(
		settings.mode === 'safe'
			? 'Caches and dependency folders — everything here is rebuilt on demand.'
			: 'Also build output, compiled files and *.pyc.'
	);

	const total = $derived(system.info?.totalDiskBytes ?? 0);
	const free = $derived(system.info?.availableDiskBytes ?? 0);
	const usedFraction = $derived(total > 0 ? (total - free) / total : 0);

	function startScan() {
		settings.extraRoots = extra
			.split(',')
			.map((s) => s.trim())
			.filter(Boolean);
		settings.persist();
		void scan.start(settings.options());
	}
</script>

<div class="flex h-full min-h-0 flex-col overflow-y-auto">
	<PermissionNotice scope={settings.scope} onDetails={onShowPermissions} />

	<div class="my-auto grid items-center gap-x-14 gap-y-10 py-6 lg:grid-cols-[1.15fr_1fr]">
		<section>
			<h2
				class="animate-rise text-[2.6rem] font-semibold leading-[1.04] tracking-[-0.04em] text-ink"
			>
				Free up space on your {system.device}
			</h2>
			<p
				class="mt-4 max-w-[46ch] animate-rise text-[0.95rem] leading-relaxed text-muted [animation-delay:70ms]"
			>
				QClean finds caches, build output and leftovers, shows you the list, and removes only what
				you tick.
			</p>

			<div
				class="mt-8 flex animate-rise flex-wrap items-center gap-x-5 gap-y-3 [animation-delay:140ms]"
			>
				<Button size="lg" class="group" onclick={startScan}>
					Scan now
					<Glyph
						icon={ArrowRight}
						class="h-4 w-4 transition-transform duration-200 ease-out group-hover:translate-x-0.5"
					/>
				</Button>
				<button
					type="button"
					class="no-drag text-sm font-medium text-muted underline decoration-line decoration-1 underline-offset-4 transition hover:text-ink hover:decoration-ink/40"
					onclick={onShowRules}
				>
					What gets cleaned
				</button>
			</div>

			<div class="mt-7 animate-rise [animation-delay:210ms]">
				<button
					type="button"
					class="no-drag group inline-flex items-center gap-2 rounded-lg py-1 text-sm text-muted transition hover:text-ink focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand/50"
					aria-expanded={showOptions}
					onclick={() => (showOptions = !showOptions)}
				>
					<span>
						<span class="font-medium text-ink"
							>{settings.mode === 'safe' ? 'Safe' : 'Aggressive'}</span
						>
						mode in
						<span class="font-medium text-ink">{scope?.label ?? settings.scope}</span>
					</span>
					<Glyph
						icon={ChevronDown}
						class="h-4 w-4 transition-transform duration-200 ease-out {showOptions
							? 'rotate-180'
							: ''}"
					/>
				</button>

				{#if showOptions}
					<div transition:slide={{ duration: 220, easing: easeOut }}>
						<div class="mt-3 grid max-w-xl gap-5 border-t border-line pt-5 sm:grid-cols-2">
							<div>
								<span class="field-label">Mode</span>
								<Segmented bind:value={settings.mode} options={modeOptions} class="w-full" />
								<p class="mt-2 text-xs text-muted">{modeHint}</p>
							</div>
							<div>
								<span class="field-label">Where to look</span>
								<Select bind:value={settings.scope} options={scopeOptions} ariaLabel="Scan scope" />
								<p class="mt-2 text-xs text-muted">{scope?.description ?? ''}</p>
							</div>
							<div class="sm:col-span-2">
								<label class="field-label" for="extra-folders">Extra folders</label>
								<input
									id="extra-folders"
									class="h-10 w-full rounded-xl border border-line bg-surface-2 px-3 text-sm text-ink transition placeholder:text-faint focus:border-brand/60 focus:outline-none focus:ring-2 focus:ring-brand/25"
									placeholder="~/work, another/folder — separated by commas"
									bind:value={extra}
								/>
							</div>
						</div>
					</div>
				{/if}
			</div>
		</section>

		<aside class="card animate-rise p-6 [animation-delay:120ms]">
			<p class="text-sm text-muted">Free on this disk</p>
			<p class="mt-1 flex items-baseline gap-2">
				<span class="num text-[2.75rem] font-medium leading-none text-ink">
					{total > 0 ? formatBytesCompact(free) : '—'}
				</span>
				{#if total > 0}
					<span class="text-sm text-muted">of {formatBytesCompact(total)}</span>
				{/if}
			</p>
			<TickMeter value={usedFraction} label="Disk space used" class="mt-6" />
			<div class="mt-3 flex justify-between text-xs text-muted">
				<span
					>{total > 0 ? `${Math.round(usedFraction * 100)}% used` : 'Disk size unavailable'}</span
				>
				<span>{scope?.description ?? ''}</span>
			</div>
		</aside>
	</div>
</div>
