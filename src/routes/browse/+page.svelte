<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { getLocalBranches, type Branch } from "$lib/browse";
  import Button from "@/components/ui/button/button.svelte";
  import { onMount } from "svelte";

  let repoPath = $page.url.searchParams.get("repo");
  let branches: {
    ahead: Array<Branch>;
    current: Array<Branch>;
    behind: Array<Branch>;
  } = {
    ahead: [],
    current: [],
    behind: [],
  };

  function processBranches(unprocessedBranches: Array<Branch>) {
    let processedBranches: typeof branches = {
      ahead: [],
      current: [],
      behind: [],
    };

    unprocessedBranches = unprocessedBranches.sort((a, b) => {
      if (a.commitTime === b.commitTime) return a.name.localeCompare(b.name);
      else return b.commitTime - a.commitTime;
    });

    let insertionType: keyof typeof branches = "ahead";

    unprocessedBranches.forEach((branch) => {
      if (branch.isHead === true && insertionType === "ahead")
        insertionType = "current";
      else if (branch.isHead === false && insertionType === "current")
        insertionType = "behind";

      processedBranches[insertionType] = [
        ...processedBranches[insertionType],
        branch,
      ];
    });

    return processedBranches;
  }

  async function refreshBranches() {
    if (repoPath) {
      try {
        branches = processBranches(await getLocalBranches(repoPath));
      } catch (error) {
        console.log(error);
      }
    }
  }

  onMount(refreshBranches);
</script>

<div
  class="flex flex-row items-center justify-between w-full p-2 h-12 border-b border-separate"
>
  <Button
    variant="ghost"
    size="sm"
    class="aspect-square"
    on:click={() => {
      goto("/home");
    }}
  >
    B
  </Button>
  {#if repoPath}
    <span>{repoPath.split("/").at(-1)}</span>
  {/if}
  <Button
    variant="ghost"
    size="sm"
    class="aspect-square"
    on:click={refreshBranches}
  >
    R
  </Button>
</div>

<div class="branches flex flex-row items-center">
  <!-- ahead -->
  <!-- current -->
  <!-- behind -->

  {#if branches.ahead.length !== 0}
    {#each branches.ahead as branch}
      ahead: {branch.name}, {branch.commitTime}
      <br />
    {/each}
  {/if}
  <br />
  {#if branches.current.length !== 0}
    {#each branches.current as branch}
      current: {branch.name}, {branch.commitTime}
      <br />
    {/each}
  {/if}
  <br />
  {#if branches.behind.length !== 0}
    {#each branches.behind as branch}
      behind: {branch.name}, {branch.commitTime}
      <br />
    {/each}
  {/if}
</div>

<style>
  .branches {
    height: calc(100% - 48px);
  }
</style>
