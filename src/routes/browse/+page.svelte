<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { getLocalBranches, type Branch } from "@/integrated-backend/browse";
  import { onMount } from "svelte";
  import Branches from "./Branches.svelte";
  import ArrowSmallLeftIcon from "@/codicons/arrow-small-left-icon.svelte";
  import RefreshIcon from "@/codicons/refresh-icon.svelte";

  let repoPath = $page.url.searchParams.get("repo");
  let branches: Array<Branch> = [];

  function sortBranches(unprocessedBranches: Array<Branch>) {
    return unprocessedBranches.sort((a, b) => {
      if (a.commitTime === b.commitTime) return a.name.localeCompare(b.name);
      else return b.commitTime - a.commitTime;
    });
  }

  async function refreshBranches() {
    if (repoPath) {
      try {
        branches = sortBranches(await getLocalBranches(repoPath));
      } catch (error) {
        console.log(error);
      }
    }
  }

  onMount(refreshBranches);

  $: repoName = repoPath ? repoPath.split("/").at(-1) : repoPath;
</script>

<div
  class="flex flex-row items-center justify-between w-full p-2 h-12 border-b border-separate"
>
  <button
    class="
    rounded-md
    aspect-square h-8 hover:bg-accent hover:text-accent-foreground
    "
    on:click={() => {
      goto("/home");
    }}
  >
    <ArrowSmallLeftIcon />
  </button>
  {#if repoName}
    <span>{repoName}</span>
  {/if}
  <button
    class="
  rounded-md
  aspect-square h-8 hover:bg-accent hover:text-accent-foreground
  flex items-center justify-around
  "
    on:click={refreshBranches}
  >
    <RefreshIcon class="w-2/3" />
  </button>
</div>

<Branches {...{ branches }} />
