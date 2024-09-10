<script lang="ts">
  import CloseIcon from "@/codicons/close-icon.svelte";
  import { zIndex } from "@/stores/z-index";

  export let showModal: boolean;

  let modalContainer: HTMLDivElement | null = null;

  let localZIndex: number | null;

  $: {
    if (showModal === true && modalContainer !== null) {
      document.body.appendChild(modalContainer);
      localZIndex = zIndex.getNext();
    } else if (showModal === false) {
      zIndex.decrease();
    }
  }
</script>

{#if showModal}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    bind:this={modalContainer}
    class={`${localZIndex ? `z-[${localZIndex}` : "z-[-10000]"}] fixed top-0 left-0 w-dvw h-full flex justify-center items-center transition-color bg-black/20 backdrop-blur`}
    on:click={() => (showModal = false)}
  >
    <div
      class={`bg-background aspect-[960/660] border w-[366px] rounded-[12px] shadow-lg scale-100 transition-all p-2`}
      on:click|stopPropagation
    >
      <button
        class="absolute right-2 top-2 w-[28px] h-[28px] border rounded-[4px]"
        on:click={() => (showModal = false)}
      >
        <CloseIcon class="aspect-square w-3/4 m-auto" />
      </button>
      <slot />
    </div>
  </div>
{/if}
