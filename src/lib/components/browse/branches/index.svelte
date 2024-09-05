<script lang="ts">
  import { Button } from "@/shadcn-svelte-components/ui/button";
  import { branches } from "@/stores/branches";
  import { repoPath } from "@/stores/repo";
  import { onDestroy, onMount } from "svelte";
  import { watch } from "tauri-plugin-fs-watch-api";
  import Branch from "./branch.svelte";

  let branchesContainerElement: HTMLDivElement;
  let currentBranchElement: HTMLDivElement | null = null;
  let unwatch: Awaited<ReturnType<typeof watch>> | undefined = undefined;

  function handleScroll() {
    if (currentBranchElement === null) return;

    const containerRect = branchesContainerElement.getBoundingClientRect();
    const branchRect = currentBranchElement.getBoundingClientRect();

    if (branchRect.left <= containerRect.left) {
      currentBranchElement.style.left = "0px";
      currentBranchElement.style.right = "unset";
    } else if (branchRect.right >= containerRect.right) {
      currentBranchElement.style.right = "0px";
      currentBranchElement.style.left = "unset";
    } else {
      currentBranchElement.style.left = "unset";
      currentBranchElement.style.right = "unset";
    }
  }

  onMount(async () => {
    branchesContainerElement.addEventListener("scroll", handleScroll);
    branchesContainerElement.addEventListener("resize", handleScroll);

    if ($repoPath) {
      branches.reload($repoPath);
      unwatch = await watch(
        [
          `${$repoPath}/.git/refs/heads`,
          `${$repoPath}/.git/refs/remotes`,
          `${$repoPath}/.git/HEAD`,
        ],
        (_event) => {
          branches.reload($repoPath);
          handleScroll();
        },
        { recursive: true, delayMs: 100 }
      );
    }
  });

  onDestroy(() => {
    branchesContainerElement.removeEventListener("scroll", handleScroll);
    branchesContainerElement.removeEventListener("resize", handleScroll);
    currentBranchElement = null;

    if (unwatch) unwatch();
  });
</script>

<div
  bind:this={branchesContainerElement}
  class="branches-container w-[calc(100dvw - 4.25rem)] h-full flex items-center overflow-x-auto"
>
  {#if $branches.length !== 0}
    {#each $branches as branch (branch.name)}
      {#if branch.isHead}
        <div class="branch-container sticky" bind:this={currentBranchElement}>
          <Branch {...{ branch }} />
        </div>
      {:else}
        <div class="branch-container">
          <Branch {...{ branch }} />
        </div>
      {/if}
    {/each}
  {/if}
  <div class="h-full border-r-[0.5px]"></div>
  {#if $branches.length === 0}
    <div class="h-full min-w-[calc(100dvw-68px)] flex">
      <div class="flex flex-col items-center m-auto gap-2">
        No branches!
        <Button>Create new branch</Button>
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  .branches-container {
    -ms-overflow-style: none;
    scrollbar-width: none;
    &::-webkit-scrollbar {
      display: none;
    }

    .branch-container {
      @apply h-full bg-background p-[14px] border-x-[0.5px];
    }
  }
</style>
