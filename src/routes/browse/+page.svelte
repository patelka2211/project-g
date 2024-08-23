<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { getLocalBranches, type Branch } from "@/integrated-backend/browse";
  import Button from "@/shadcn-svelte-components/ui/button/button.svelte";
  import { onMount } from "svelte";
  import Branches from "./Branches.svelte";

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

<Branches {...{ branches }} />
