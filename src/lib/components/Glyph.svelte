<!-- One icon from the Hugeicons set. Size it with `class` (e.g. `h-4 w-4`). -->
<script lang="ts">
	import { HugeiconsIcon, type IconSvgElement } from '@hugeicons/svelte';

	let {
		icon,
		strokeWidth = 1.75,
		class: klass = ''
	}: {
		icon: IconSvgElement;
		strokeWidth?: number;
		class?: string;
	} = $props();
</script>

<!-- HugeiconsIcon draws its paths once, on mount; re-key so a new `icon` redraws. -->
{#key icon}
	<HugeiconsIcon {icon} {strokeWidth} class={klass} aria-hidden="true" />
{/key}
