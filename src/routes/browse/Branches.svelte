<script lang="ts">
  import Separator from "@/shadcn-svelte-components/ui/separator/separator.svelte";
  import { branches } from "@/stores/Branches";
  import { repoPath } from "@/stores/Repo";
  import { onDestroy, onMount } from "svelte";
  import Branch from "./Branch.svelte";

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
    if ($repoPath) {
      branches.reload($repoPath);
    }

    branchesContainerElement.addEventListener("scroll", handleScroll);
    branchesContainerElement.addEventListener("resize", handleScroll);
  });

  onDestroy(() => {
    branchesContainerElement.removeEventListener("scroll", handleScroll);
    branchesContainerElement.removeEventListener("resize", handleScroll);
    currentBranchElement = null;
  });
</script>

<div
  bind:this={branchesContainerElement}
  class="branches w-[calc(100dvw - 4.25rem)] h-full flex items-center overflow-x-auto"
>
  {#if $branches.length !== 0}
    {#each $branches as branch (branch.name)}
      {#if branch.isHead}
        <div
          class={`h-full min-w-[390px] max-w-[390px] bg-background p-2
          sticky ${defaultBranchShadowOn ? `shadow-${defaultBranchShadowOn}` : ""}`}
          bind:this={currentBranchElement}
        >
          <Branch {...{ branch }} />
        </div>
      {:else}
        <div class="h-full min-w-[390px] max-w-[390px] bg-background p-2">
          <Branch {...{ branch }} />
        </div>
      {/if}
      <Separator orientation="vertical" />
    {/each}
  {:else}
    <!-- option to create new branch -->
  {/if}
</div>

<style lang="scss">
  .branches {
    -ms-overflow-style: none;
    scrollbar-width: none;
    &::-webkit-scrollbar {
      display: none;
    }

    div {
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
