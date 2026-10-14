<script lang="ts">
	let {
		size = 40,
		animated = false,
		class: klass = ''
	}: { size?: number; animated?: boolean; class?: string } = $props();
</script>

<!-- The animated SVG plays its intro on load and respects prefers-reduced-motion itself. -->
<img
	src={animated ? '/app-icon-animated.svg' : '/app-icon.png'}
	alt="QClean"
	width={size}
	height={size}
	class="rounded-[24%] shadow-tile {klass}"
	style="width:{size}px;height:{size}px"
/>
