<!-- "Nothing left to clean": a drawn ring and check, dust drifting away, sparkles twinkling.
     Inline so it follows the theme tokens; reduced motion shows the final frame. -->
<svg viewBox="0 0 160 120" class="h-28 w-36" role="img" aria-label="All clean">
	<g fill="none" stroke-linecap="round" stroke-linejoin="round">
		<circle cx="80" cy="60" r="44" class="halo" />
		<circle cx="80" cy="60" r="34" pathLength="1" class="ring draw" transform="rotate(-90 80 60)" />
		<path d="M66 61 l9 9 l18 -19" pathLength="1" class="check draw" />
	</g>

	<g class="dust">
		<circle cx="38" cy="86" r="2.5" style="--dx:-10px;--d:0s" />
		<circle cx="124" cy="90" r="2" style="--dx:8px;--d:.5s" />
		<circle cx="30" cy="54" r="1.6" style="--dx:-6px;--d:1.1s" />
		<circle cx="132" cy="58" r="2.2" style="--dx:10px;--d:1.6s" />
	</g>

	{#each [{ x: 124, y: 24, s: 1, d: 0.9 }, { x: 34, y: 28, s: 0.65, d: 1.5 }, { x: 118, y: 104, s: 0.5, d: 2.1 }] as p (p.x)}
		<g transform="translate({p.x} {p.y}) scale({p.s})">
			<path
				class="spark"
				style="--d:{p.d}s"
				d="M0 -11 L3 -3 L11 0 L3 3 L0 11 L-3 3 L-11 0 L-3 -3 Z"
			/>
		</g>
	{/each}
</svg>

<style>
	.halo {
		fill: rgb(var(--brand) / 0.08);
		stroke: rgb(var(--brand) / 0.18);
		stroke-width: 1;
		stroke-dasharray: 2 5;
		transform-origin: 80px 60px;
		animation: spin 24s linear infinite;
	}
	.ring {
		stroke: rgb(var(--brand));
		stroke-width: 4;
	}
	.check {
		stroke: rgb(var(--brand));
		stroke-width: 5;
	}
	.draw {
		stroke-dasharray: 1 1;
		stroke-dashoffset: 1;
		animation: draw 0.8s cubic-bezier(0.65, 0, 0.35, 1) forwards;
	}
	.check {
		animation-duration: 0.45s;
		animation-delay: 0.7s;
		animation-timing-function: var(--ease-out);
	}
	.dust circle {
		fill: rgb(var(--muted) / 0.5);
		animation: drift 3.2s var(--ease-out) var(--d) infinite;
	}
	.spark {
		fill: rgb(var(--brand));
		opacity: 0;
		animation: twinkle 2.6s ease-in-out var(--d) infinite;
	}
	@keyframes draw {
		to {
			stroke-dashoffset: 0;
		}
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
	@keyframes drift {
		0% {
			opacity: 0;
			transform: translate(0, 0);
		}
		20% {
			opacity: 1;
		}
		100% {
			opacity: 0;
			transform: translate(var(--dx), -26px);
		}
	}
	@keyframes twinkle {
		0%,
		100% {
			opacity: 0;
			scale: 0.3;
		}
		35%,
		55% {
			opacity: 1;
			scale: 1;
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.halo,
		.dust circle {
			animation: none;
		}
		.dust circle {
			opacity: 0;
		}
		.draw {
			animation: none;
			stroke-dashoffset: 0;
		}
		.spark {
			animation: none;
			opacity: 1;
		}
	}
</style>
