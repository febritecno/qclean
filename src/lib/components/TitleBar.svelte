<script lang="ts">
	import type { Snippet } from 'svelte';

	// macOS overlay title bar: the traffic lights float over the content, so we
	// reserve a draggable strip. `data-tauri-drag-region` is Tauri's drag
	// mechanism (and gives double-click-to-zoom for free); the CSS class is a
	// belt-and-braces `-webkit-app-region: drag`.
	import { system } from '$lib/stores/system.svelte';

	// Linux and Windows draw their own title bar, so only a little breathing
	// room is needed there.
	let { actions }: { actions?: Snippet } = $props();
</script>

<div
	data-tauri-drag-region
	class="drag flex shrink-0 items-center justify-between px-4 {system.isMac ? 'h-11' : 'h-4'}"
>
	<div data-tauri-drag-region class="h-full flex-1"></div>
	<div class="no-drag flex items-center gap-1">
		{@render actions?.()}
	</div>
</div>
