<!--
	The living background: one accent light and one neutral field, drifting
	slowly under a film of grain.
	Purely decorative, sits behind everything and never takes pointer events.
-->
<div class="backdrop" aria-hidden="true">
	<span class="field field-a"></span>
	<span class="field field-b"></span>
	<span class="grain"></span>
</div>

<style>
	.backdrop {
		position: fixed;
		inset: 0;
		z-index: -1;
		overflow: hidden;
		pointer-events: none;
		background: rgb(var(--surface));
	}
	.field {
		position: absolute;
		border-radius: 50%;
		filter: blur(90px);
		opacity: var(--field-opacity);
		will-change: transform;
	}
	.field-a {
		top: -22%;
		left: -12%;
		width: 58vw;
		height: 58vw;
		background: rgb(var(--brand));
		animation: drift-a 26s var(--ease-in-out) infinite alternate;
	}
	.field-b {
		right: -16%;
		bottom: -30%;
		width: 62vw;
		height: 62vw;
		background: rgb(var(--faint));
		animation: drift-b 32s var(--ease-in-out) infinite alternate;
	}
	.grain {
		position: absolute;
		inset: 0;
		opacity: var(--grain-opacity);
		mix-blend-mode: overlay;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='220' height='220'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='.9' numOctaves='2' stitchTiles='stitch'/%3E%3CfeColorMatrix values='0 0 0 0 .5 0 0 0 0 .5 0 0 0 0 .5 0 0 0 .9 0'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E");
	}

	@keyframes drift-a {
		to {
			transform: translate3d(14vw, 10vh, 0) scale(1.15);
		}
	}
	@keyframes drift-b {
		to {
			transform: translate3d(-16vw, -12vh, 0) scale(0.88);
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.field {
			animation: none;
		}
	}
</style>
