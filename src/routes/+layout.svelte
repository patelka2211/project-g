<script lang="ts">
	import RootLayoutMacOS from '@/components/root-layout.macos.svelte';
	import RootLayout from '@/components/root-layout.svelte';
	import Sonner from '@/shadcn-svelte-components/ui/sonner/sonner.svelte';
	import { type as getOS } from '@tauri-apps/plugin-os';
	import { ModeWatcher } from 'mode-watcher';
	import { onMount, type Snippet } from 'svelte';
	import '../app.css';

	interface Props {
		children?: Snippet;
	}

	let { children }: Props = $props();

	let isMacOS = $state(false);

	onMount(() => {
		isMacOS = getOS() === 'macos';
	});
</script>

<ModeWatcher />

<Sonner richColors expand />

{#if isMacOS}
	<RootLayoutMacOS>
		{@render children?.()}
	</RootLayoutMacOS>
{:else}
	<RootLayout>
		{@render children?.()}
	</RootLayout>
{/if}
