<script lang="ts">
  import type { Branch } from "@/integrated-backend/browse";
  import { onDestroy, onMount } from "svelte";

  let branchesContainerElement: HTMLDivElement;
  let currentBranchElement: HTMLDivElement | null;
  let defaultBranchShadowOn: "left" | "right" | undefined = undefined;

  function handleScroll() {
    if (currentBranchElement === null) return;

    const containerRect = branchesContainerElement.getBoundingClientRect();
    const branchRect = currentBranchElement.getBoundingClientRect();

    if (branchRect.left <= containerRect.left) {
      currentBranchElement.style.left = "0px";
      currentBranchElement.style.right = "unset";
      defaultBranchShadowOn = "right";
    } else if (branchRect.right >= containerRect.right) {
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
    branchesContainerElement.addEventListener("scroll", handleScroll);
    branchesContainerElement.addEventListener("resize", handleScroll);
  });

  onDestroy(() => {
    branchesContainerElement.removeEventListener("scroll", handleScroll);
    branchesContainerElement.removeEventListener("resize", handleScroll);
  });

  export let branches: Array<Branch>;
</script>

<div
  bind:this={branchesContainerElement}
  class={`branches w-full flex items-center overflow-x-auto${branches.length === 0 ? " justify-around" : ""}`}
>
  {#if branches.length === 0}
    <div class=" flex flex-col items-center gap-4">
      <span> No branches </span>
    </div>
  {:else}
    {#each branches as branch}
      {#if branch.isHead}
        <div
          bind:this={currentBranchElement}
          class={`branch p-2 sticky${defaultBranchShadowOn === undefined ? "" : ` shadow-${defaultBranchShadowOn}`}`}
        >
          {branch.name}
        </div>
      {:else}
        <div class="branch p-2">
          {branch.name}
        </div>
      {/if}
    {/each}
  {/if}
</div>

<style lang="scss">
  .branches {
    height: calc(100% - 48px);

    .branch {
      height: 100%;
      @apply min-w-96 bg-background;

      &.sticky {
        transition: box-shadow 0.2s ease-in-out;
        &.shadow-right {
          box-shadow: 22px 0 70px 4px hsl(var(--foreground) / 0.08);
        }
        &.shadow-left {
          box-shadow: -22px 0 70px 4px hsl(var(--foreground) / 0.08);
        }
      }
    }
  }
</style>
