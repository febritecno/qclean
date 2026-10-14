<script lang="ts">
	import Glyph from '$lib/components/Glyph.svelte';
	import { Clock01Icon as Clock } from '@hugeicons/core-free-icons';
	import { untrack } from 'svelte';
	import { flip } from 'svelte/animate';
	import { Tween } from 'svelte/motion';
	import { linear } from 'svelte/easing';
	import Button from '$lib/components/ui/Button.svelte';
	import StepHeading from '$lib/components/StepHeading.svelte';
	import ScanLens from './ScanLens.svelte';
	import { CATEGORY_META, iconFor } from '$lib/constants/categories';
	import type { Category, ScanCandidate } from '$lib/types/ipc';
	import {
		formatBytesCompact,
		formatCount,
		formatDuration,
		shortenPath,
		truncateMiddle
	} from '$lib/utils/format';
	import { easeOut, pop, prefersReducedMotion } from '$lib/utils/motion';
	import { scan } from '$lib/stores/scan.svelte';
	import { system } from '$lib/stores/system.svelte';

	const reduce = prefersReducedMotion();

	// The engine sends candidates in bursts and progress ~10×/s. Rendering each of
	// those makes the screen jitter, so everything except the size count-up is
	// sampled on one steady clock instead.
	const TICK_MS = 300;
	const FEED_SIZE = 6;
	const SHADES = [1, 0.7, 0.48, 0.32, 0.18];

	let now = $state(Date.now());
	let currentDir = $state('');
	let counts = $state({ items: 0, dirs: 0 });
	let recent = $state<ScanCandidate[]>([]);
	let segments = $state<{ label: string; bytes: number; share: number; shade: number }[]>([]);

	let fed = 0; // candidates already shown in the feed
	let order: Category[] = []; // categories in the order they first appeared

	function tick() {
		now = Date.now();
		const p = scan.progress;
		currentDir = p?.currentDir ? shortenPath(p.currentDir, system.home) : '';
		counts = { items: p?.itemsFound ?? scan.candidates.length, dirs: p?.scannedDirs ?? 0 };

		// Feed: one new row per tick. A large backlog is skipped, not queued.
		const all = scan.candidates;
		if (all.length > fed) {
			fed = Math.max(fed, all.length - 3);
			recent = [all[fed], ...recent].slice(0, FEED_SIZE);
			fed += 1;
		}

		// Composition: a fixed order (first seen), so segments grow instead of swapping.
		const buckets = scan.byCategory;
		for (const b of buckets) if (!order.includes(b.key)) order.push(b.key);
		const bytesOf = new Map(buckets.map((b) => [b.key, b.bytes]));
		const total = buckets.reduce((sum, b) => sum + b.bytes, 0);
		if (total === 0) return;
		const parts = order
			.slice(0, 4)
			.map((k) => ({ label: CATEGORY_META[k]?.short ?? k, bytes: bytesOf.get(k) ?? 0 }));
		const rest = order.slice(4).reduce((sum, k) => sum + (bytesOf.get(k) ?? 0), 0);
		if (rest > 0) parts.push({ label: 'Everything else', bytes: rest });
		segments = parts.map((s, i) => ({ ...s, share: s.bytes / total, shade: SHADES[i] }));
	}

	$effect(() => {
		untrack(tick); // the timer drives updates; don't subscribe this effect to scan data
		const t = setInterval(tick, TICK_MS);
		return () => clearInterval(t);
	});
	const elapsed = $derived(Math.max(0, now - scan.startedAt));

	// Linear and a little longer than the update interval, so successive targets
	// chain into one steady count-up rather than a burst-and-stop per update.
	const bytes = new Tween(0, { duration: reduce ? 0 : 500, easing: linear });
	$effect(() => {
		bytes.target = scan.progress?.bytesFound ?? 0;
	});
	const [bytesValue, bytesUnit] = $derived(formatBytesCompact(bytes.current).split(' '));

	const baseName = (p: string) => p.replace(/\/+$/, '').split('/').pop() || p;
	const parentDir = (p: string) =>
		truncateMiddle(shortenPath(p.replace(/\/[^/]+\/?$/, '') || '/', system.home), 40);
</script>

<div class="flex h-full min-h-0 animate-screen-in flex-col gap-5">
	<StepHeading
		title="Scanning your {system.device}"
		subtitle="Everything below is only a preview — nothing is removed until you confirm."
	>
		{#snippet actions()}
			<span
				class="num flex items-center gap-1.5 rounded-full border border-line bg-surface-2 px-3 py-1.5 text-xs text-muted"
			>
				<Glyph icon={Clock} class="h-3.5 w-3.5" />
				{formatDuration(elapsed)}
			</span>
			<Button variant="ghost" onclick={() => scan.cancel()} loading={scan.cancelling}>
				{scan.cancelling ? 'Stopping…' : 'Stop scan'}
			</Button>
		{/snippet}
	</StepHeading>

	<section class="card flex min-h-0 flex-1 flex-col overflow-hidden">
		<div class="grid min-h-0 flex-1 overflow-y-auto md:grid-cols-[1.2fr_1fr]">
			<!-- What has been found, in total -->
			<div class="flex min-w-0 flex-col justify-center gap-6 p-7">
				<!-- Sized by window height so the totals stay in view on short windows. -->
				<div class="mx-auto h-[clamp(5.5rem,22vh,11rem)] opacity-90"><ScanLens /></div>

				<div>
					<p class="text-xs font-medium uppercase tracking-[0.14em] text-muted">
						Reclaimable so far
					</p>
					<p class="mt-1 flex items-baseline gap-2 text-ink">
						<span class="num text-6xl font-medium leading-none tracking-[-0.04em]"
							>{bytesValue}</span
						>
						<span class="text-xl font-semibold text-muted">{bytesUnit}</span>
					</p>
					<p class="num mt-2 text-sm text-muted">
						{formatCount(counts.items)} items · {formatCount(counts.dirs)} folders checked
					</p>
				</div>

				<div>
					<div class="flex h-2.5 w-full gap-0.5 overflow-hidden rounded-full bg-line/60">
						{#each segments as s (s.label)}
							<span
								class="h-full rounded-full bg-brand transition-[flex-grow] duration-500 ease-out"
								style="flex-grow:{s.share};opacity:{s.shade}"
							></span>
						{/each}
					</div>
					<!-- Fixed grid with room for all five rows' worth, so nothing reflows as it fills. -->
					<ul class="mt-3 grid min-h-[3.9rem] grid-cols-2 content-start gap-x-4 gap-y-1.5 text-xs">
						{#each segments as s (s.label)}
							<li class="flex min-w-0 items-center gap-1.5">
								<span class="h-2 w-2 shrink-0 rounded-full bg-brand" style="opacity:{s.shade}"
								></span>
								<span class="truncate text-ink">{s.label}</span>
								<span class="num ml-auto shrink-0 text-muted">{formatBytesCompact(s.bytes)}</span>
							</li>
						{:else}
							<li class="col-span-2 text-muted">Sorting finds into categories as they turn up…</li>
						{/each}
					</ul>
				</div>
			</div>

			<!-- What was found most recently -->
			<div class="flex min-w-0 flex-col border-line p-5 max-md:border-t md:border-l">
				<p class="mb-3 px-2 text-xs font-medium uppercase tracking-[0.14em] text-muted">
					Found just now
				</p>
				<ul class="feed flex min-h-0 flex-1 flex-col gap-1.5 overflow-hidden">
					{#each recent as c (c.id)}
						<li
							animate:flip={{ duration: reduce ? 0 : 260, easing: easeOut }}
							in:pop={{ start: 0.98, y: -8, duration: 260 }}
							class="flex items-center gap-3 rounded-xl px-2 py-2"
						>
							<span
								class="grid h-9 w-9 shrink-0 place-items-center rounded-xl bg-brand-soft text-brand"
							>
								<Glyph icon={iconFor(c.category)} class="h-4 w-4" />
							</span>
							<div class="min-w-0 flex-1">
								<p class="truncate text-sm font-medium text-ink">{baseName(c.displayPath)}</p>
								<p class="truncate font-mono text-[11px] text-muted" title={c.displayPath}>
									{parentDir(c.displayPath)}
								</p>
							</div>
							<span class="num shrink-0 text-sm text-ink">{formatBytesCompact(c.sizeBytes)}</span>
						</li>
					{:else}
						{#each { length: 4 } as _, i (i)}
							<li class="flex items-center gap-3 px-2 py-2" style="opacity:{1 - i * 0.22}">
								<span class="skeleton h-9 w-9 rounded-xl"></span>
								<div class="flex-1 space-y-1.5">
									<span class="skeleton block h-3 w-2/5 rounded"></span>
									<span class="skeleton block h-2.5 w-3/5 rounded"></span>
								</div>
							</li>
						{/each}
					{/each}
				</ul>
			</div>
		</div>

		<!-- Where the engine is right now -->
		<footer class="relative shrink-0 border-t border-line px-5 py-3">
			<span class="track absolute inset-x-0 -top-px h-px" aria-hidden="true"></span>
			<div class="flex min-w-0 items-center gap-3 text-sm">
				<span class="relative flex h-2 w-2 shrink-0">
					<span class="absolute inset-0 rounded-full bg-brand opacity-60 motion-safe:animate-ping"
					></span>
					<span class="relative h-2 w-2 rounded-full bg-brand"></span>
				</span>
				<span class="shrink-0 font-semibold text-brand">Scanning:</span>
				<span class="min-w-0 flex-1 truncate font-mono text-muted" title={currentDir}>
					{currentDir ? truncateMiddle(currentDir, 72) : 'Preparing…'}
				</span>
				{#if scan.errors.length}
					<span class="num shrink-0 text-xs text-amber-600 dark:text-amber-300">
						{formatCount(scan.errors.length)} unreadable
					</span>
				{/if}
			</div>
		</footer>
	</section>
</div>

<style>
	/* The newest find fades in at the top; older ones fade out toward the bottom. */
	.feed {
		mask-image: linear-gradient(to bottom, #000 70%, transparent);
	}
	.skeleton {
		background: rgb(var(--line) / 0.7);
		animation: breathe 1.6s ease-in-out infinite;
	}
	/* A highlight running along the footer's top edge: the engine is working. */
	.track {
		background: linear-gradient(90deg, transparent, rgb(var(--brand) / 0.9) 50%, transparent);
		background-size: 40% 100%;
		background-repeat: no-repeat;
		animation: run 1.8s cubic-bezier(0.65, 0, 0.35, 1) infinite;
	}
	@keyframes run {
		from {
			background-position: -40% 0;
		}
		to {
			background-position: 140% 0;
		}
	}
	@keyframes breathe {
		50% {
			opacity: 0.45;
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.skeleton,
		.track {
			animation: none;
		}
	}
</style>
