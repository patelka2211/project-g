<script lang="ts">
	import CloseIcon from '@/codicons/close-icon.svelte';
	import { zIndex } from '@/stores/z-index';
	import type { Snippet } from 'svelte';

	interface Props {
		showModal: boolean;
		children?: Snippet;
	}

	let { showModal = $bindable(), children }: Props = $props();

	let modalContainer: HTMLDivElement | null = $state(null);

	let localZIndex: number | null = $state(null);

	$effect(() => {
		if (showModal === true && modalContainer !== null) {
			document.body.appendChild(modalContainer);
			localZIndex = zIndex.getNext();
		} else if (showModal === false) {
			zIndex.decrease();
			localZIndex = null;
		}
	});
</script>

{#if showModal}
	<div
		bind:this={modalContainer}
		class={`${localZIndex ? `z-[${localZIndex}` : 'z-[-10000]'}] fixed top-0 left-0 w-dvw h-full flex justify-center items-center transition-color bg-black/20 backdrop-blur`}
		onclick={() => (showModal = false)}
		onkeypress={() => (showModal = false)}
		role="button"
		tabindex="0"
	>
		<div
			class={`bg-background aspect-[960/660] border w-[366px] rounded-[12px] shadow-lg scale-100 transition-all p-2`}
			onclick={(event) => {
				event.stopPropagation();
			}}
			onkeypress={(event) => {
				event.stopPropagation();
			}}
			role="button"
			tabindex="0"
		>
			<button
				class="absolute right-2 top-2 w-[28px] h-[28px] border rounded-[4px]"
				onclick={() => (showModal = false)}
			>
				<CloseIcon class="aspect-square w-3/4 m-auto" />
			</button>
			{@render children?.()}
		</div>
	</div>
{/if}
