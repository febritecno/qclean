<script lang="ts">
	import Glyph from '$lib/components/Glyph.svelte';
	import { Tick02Icon as Check, MinusSignIcon as Minus } from '@hugeicons/core-free-icons';
	import { cn } from '$lib/utils/cn';

	let {
		checked = false,
		indeterminate = false,
		disabled = false,
		label,
		class: klass = '',
		onchange
	}: {
		checked?: boolean;
		indeterminate?: boolean;
		disabled?: boolean;
		label: string;
		class?: string;
		onchange?: (checked: boolean) => void;
	} = $props();
</script>

<button
	type="button"
	role="checkbox"
	aria-checked={indeterminate ? 'mixed' : checked}
	aria-label={label}
	{disabled}
	onclick={() => onchange?.(!checked)}
	class={cn(
		'no-drag relative grid h-[18px] w-[18px] shrink-0 place-items-center overflow-hidden rounded-[6px] border text-on-brand transition duration-150 active:scale-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand/50 focus-visible:ring-offset-1 focus-visible:ring-offset-surface-2',
		checked || indeterminate
			? 'border-transparent'
			: 'border-line bg-surface-2 hover:border-brand/50',
		disabled && 'cursor-not-allowed opacity-40',
		klass
	)}
>
	<span
		class="gradient-brand absolute inset-0 transition-opacity duration-150"
		class:opacity-0={!(checked || indeterminate)}
		aria-hidden="true"
	></span>
	<span
		class="relative grid place-items-center transition duration-150 ease-out"
		class:opacity-0={!(checked || indeterminate)}
		class:scale-50={!(checked || indeterminate)}
	>
		{#if indeterminate}
			<Glyph icon={Minus} class="h-3 w-3" strokeWidth={3} />
		{:else}
			<Glyph icon={Check} class="h-3 w-3" strokeWidth={3.5} />
		{/if}
	</span>
</button>
