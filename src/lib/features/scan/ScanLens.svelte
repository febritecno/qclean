<!-- The QClean lens gliding over a grid of folders; dots under the glass light up.
     The bright grid is masked by a circle riding the same motion path as the lens,
     so the two stay in lockstep without script. Reduced motion parks the lens. -->
<script lang="ts">
	import { prefersReducedMotion } from '$lib/utils/motion';

	const id = $props.id();
	const still = prefersReducedMotion();

	const COLS = 13;
	const ROWS = 7;
	const dots = Array.from({ length: COLS * ROWS }, (_, i) => ({
		x: 12 + (i % COLS) * 18,
		y: 16 + Math.floor(i / COLS) * 18
	}));
	// A figure-eight that keeps the lens inside the grid.
	const PATH =
		'M120 70 C160 18, 222 30, 204 78 C190 118, 150 112, 120 70 C90 28, 52 22, 38 62 C26 104, 82 122, 120 70 Z';
	const PINGS = [
		{ x: 66, y: 52, d: 0.4 },
		{ x: 174, y: 34, d: 1.9 },
		{ x: 192, y: 106, d: 3.1 },
		{ x: 48, y: 106, d: 4.6 }
	];
</script>

<svg viewBox="0 0 240 140" class="h-full w-auto" aria-hidden="true">
	<defs>
		<mask id="{id}-glass" maskUnits="userSpaceOnUse" x="0" y="0" width="240" height="140">
			<circle r="24" fill="white" transform={still ? 'translate(120 70)' : undefined}>
				{#if !still}
					<animateMotion dur="8s" repeatCount="indefinite" path={PATH} />
				{/if}
			</circle>
		</mask>
	</defs>

	<g fill="rgb(var(--line))">
		{#each dots as d, i (i)}<circle cx={d.x} cy={d.y} r="1.7" />{/each}
	</g>
	<g fill="rgb(var(--brand))" mask="url(#{id}-glass)">
		{#each dots as d, i (i)}<circle cx={d.x} cy={d.y} r="2.8" />{/each}
	</g>

	{#if !still}
		<g fill="none" stroke="rgb(var(--brand))">
			{#each PINGS as p (p.x)}
				<circle class="ping" cx={p.x} cy={p.y} r="9" style="animation-delay:{p.d}s" />
			{/each}
		</g>
	{/if}

	<!-- The lens: the Q from the app icon -->
	<g transform={still ? 'translate(120 70)' : undefined}>
		{#if !still}
			<animateMotion dur="8s" repeatCount="indefinite" path={PATH} />
		{/if}
		<circle r="24" fill="rgb(var(--brand) / 0.07)" />
		<circle r="24" fill="none" stroke="rgb(var(--brand))" stroke-width="5" />
		<path d="M19 19 L31 31" stroke="rgb(var(--brand))" stroke-width="6" stroke-linecap="round" />
		<path
			d="M-12 -12 A17 17 0 0 1 4 -16"
			fill="none"
			stroke="rgb(var(--surface-2))"
			stroke-opacity="0.7"
			stroke-width="2.5"
			stroke-linecap="round"
		/>
	</g>
</svg>

<style>
	.ping {
		transform-box: fill-box;
		transform-origin: center;
		opacity: 0;
		stroke-width: 1.5;
		animation: ping 6s var(--ease-out) infinite;
	}
	@keyframes ping {
		0% {
			opacity: 0.9;
			transform: scale(0.2);
		}
		18% {
			opacity: 0;
			transform: scale(1.6);
		}
		100% {
			opacity: 0;
		}
	}
</style>
