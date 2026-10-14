<!-- A one-shot ring of sparks thrown outward from the centre of its positioned parent. -->
<script lang="ts">
	let { count = 14, radius = 150 }: { count?: number; radius?: number } = $props();
</script>

<div class="pointer-events-none absolute inset-0 motion-reduce:hidden" aria-hidden="true">
	{#each { length: count } as _, i (i)}
		<span
			class="spark"
			style="--angle:{(360 / count) * i}deg;--reach:{radius + (i % 3) * 22}px;--size:{i % 2
				? 6
				: 9}px;animation-delay:{120 + (i % 4) * 45}ms"
		></span>
	{/each}
</div>

<style>
	.spark {
		position: absolute;
		top: 50%;
		left: 50%;
		width: var(--size);
		height: var(--size);
		margin: calc(var(--size) / -2);
		border-radius: 50%;
		background: rgb(var(--brand));
		opacity: 0;
		animation: spark 0.9s var(--ease-out) forwards;
	}
	@keyframes spark {
		0% {
			opacity: 0;
			transform: rotate(var(--angle)) translateY(-70px) scale(0.4);
		}
		25% {
			opacity: 1;
		}
		100% {
			opacity: 0;
			transform: rotate(var(--angle)) translateY(calc(var(--reach) * -1)) scale(1);
		}
	}
</style>
