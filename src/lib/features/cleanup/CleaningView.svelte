<script lang="ts">
	import Glyph from '$lib/components/Glyph.svelte';
	import {
		Clock01Icon as Clock,
		FileMinusIcon as FileMinus,
		PieChartIcon as PieChart,
		SecurityCheckIcon as ShieldCheck,
		SparklesIcon as Sparkles
	} from '@hugeicons/core-free-icons';
	import StepHeading from '$lib/components/StepHeading.svelte';
	import CircularProgress from '$lib/components/CircularProgress.svelte';
	import OrbitField, { type OrbitItem } from '$lib/components/OrbitField.svelte';
	import StatTile from '$lib/components/StatTile.svelte';
	import { iconFor } from '$lib/constants/categories';
	import {
		formatBytesCompact,
		formatCount,
		formatDuration,
		formatPercent
	} from '$lib/utils/format';
	import { scan } from '$lib/stores/scan.svelte';
	import { system } from '$lib/stores/system.svelte';
	import TickMeter from '$lib/components/TickMeter.svelte';

	let now = $state(Date.now());
	const startedAt = Date.now();
	$effect(() => {
		const t = setInterval(() => (now = Date.now()), 500);
		return () => clearInterval(t);
	});
	const elapsed = $derived(Math.max(0, now - startedAt));

	const p = $derived(scan.cleanupProgress);
	const fraction = $derived(p && p.total > 0 ? p.done / p.total : 0);

	// selected categories, marked done as the run progresses through the list
	const orbitItems = $derived.by<OrbitItem[]>(() => {
		const cats = new Map<string, number>();
		for (const c of scan.selectedItems)
			cats.set(c.category, (cats.get(c.category) ?? 0) + c.sizeBytes);
		const done = fraction;
		return [...cats.entries()]
			.sort((a, b) => b[1] - a[1])
			.slice(0, 6)
			.map(([cat, bytes], i, arr) => ({
				label: cat,
				icon: iconFor(cat as never),
				detail: formatBytesCompact(bytes),
				status: (i / arr.length < done ? 'done' : 'active') as 'done' | 'active'
			}));
	});
</script>

<div class="flex h-full min-h-0 animate-screen-in flex-col">
	<StepHeading
		title="Cleaning your {system.device}"
		subtitle="Removing the items you selected — please don't quit QClean."
	/>

	<div class="flex min-h-0 flex-1 flex-col overflow-y-auto">
		<div class="my-auto flex flex-col items-center gap-7 py-4">
			<OrbitField items={orbitItems}>
				{#snippet ring()}
					<CircularProgress size={240} progress={fraction}>
						<div class="flex flex-col items-center">
							<Glyph icon={Sparkles} class="mb-3 h-5 w-5 text-brand" />
							<p class="num text-[2.5rem] font-medium leading-none text-ink">
								{formatPercent(fraction)}
							</p>
							<p class="mt-2 text-sm text-muted">removed</p>
						</div>
					</CircularProgress>
				{/snippet}
			</OrbitField>

			<TickMeter value={fraction} ticks={56} label="Cleanup progress" class="w-full max-w-md" />

			<div
				class="mx-auto flex max-w-md items-center justify-center gap-2 rounded-xl border border-line bg-surface-2 px-4 py-2.5 text-sm text-muted"
			>
				<Glyph icon={ShieldCheck} class="h-4 w-4 text-emerald-500" />
				QClean is removing only the items you selected.
			</div>
		</div>
	</div>

	<div class="grid shrink-0 gap-3 pt-4 sm:grid-cols-2 lg:grid-cols-4">
		<StatTile
			icon={FileMinus}
			label="Items processed"
			value="{formatCount(p?.done ?? 0)} / {formatCount(p?.total ?? 0)}"
			hint="selected for removal"
		/>
		<StatTile
			icon={PieChart}
			label="Space freed"
			value={formatBytesCompact(p?.freedBytes ?? 0)}
			hint="so far"
			accent="ok"
		/>
		<StatTile icon={Clock} label="Time elapsed" value={formatDuration(elapsed)} hint="cleaning" />
		<StatTile
			icon={ShieldCheck}
			label="Status"
			value="Cleaning safely"
			hint="don't quit the app"
			accent="ok"
		/>
	</div>
</div>
