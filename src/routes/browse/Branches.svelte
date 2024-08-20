<script lang="ts">
  import type { Branch } from "$lib/browse";
  import { onDestroy, onMount } from "svelte";

  let branchesContainerElement: HTMLDivElement | null;
  let currentBranchElement: HTMLDivElement | null;
  let defaultBranchShadowOn: "left" | "right" | undefined = undefined;

  function handleScroll() {
    if (!(branchesContainerElement && currentBranchElement)) return;

    const containerRect = branchesContainerElement.getBoundingClientRect();
    const cardRect = currentBranchElement.getBoundingClientRect();

    if (cardRect.left <= containerRect.left) {
      currentBranchElement.style.left = "0px";
      currentBranchElement.style.right = "unset";
      defaultBranchShadowOn = "right";
    } else if (cardRect.right >= containerRect.right) {
      currentBranchElement.style.right = "0px";
      currentBranchElement.style.left = "unset";
      defaultBranchShadowOn = "left";
    } else {
      currentBranchElement.style.left = "unset";
      currentBranchElement.style.right = "unset";
      defaultBranchShadowOn = undefined;
    }
  }

  onMount(() => {
    branchesContainerElement?.addEventListener("scroll", handleScroll);
    branchesContainerElement?.addEventListener("resize", handleScroll);
  });

  onDestroy(() => {
    branchesContainerElement?.removeEventListener("scroll", handleScroll);
    branchesContainerElement?.removeEventListener("resize", handleScroll);
  });

  export let branches: Array<Branch>;
</script>

{#if branches.length === 0}
  No branches
{:else}
  <div
    bind:this={branchesContainerElement}
    class="branches w-full flex flex-row overflow-x-auto"
  >
    {#each branches as branch}
      {#if branch.isHead}
        <div
          bind:this={currentBranchElement}
          class={`branch sticky${defaultBranchShadowOn === undefined ? "" : ` shadow-${defaultBranchShadowOn}`}`}
        >
          {branch.name}
        </div>
      {:else}
        <div class="branch bg-accent">
          {branch.name}
        </div>
      {/if}
    {/each}
  </div>
{/if}

<style lang="scss">
  .branches {
    height: calc(100% - 48px);

    .branch {
      height: 100%;
      @apply border min-w-96 bg-background;

      &.sticky.shadow-right {
        box-shadow:
          4px 0 10px hsl(var(--background) / 1),
          8px 0 20px hsl(var(--background) / 1);
      }
      &.sticky.shadow-left {
        box-shadow:
          -4px 0 10px hsl(var(--background) / 1),
          -8px 0 20px hsl(var(--background) / 1);
      }
    }
  }
</style>
