<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { getLocalBranches } from "$lib/browse";
  import Button from "@/components/ui/button/button.svelte";
  import { onMount } from "svelte";

  let repoPath = $page.url.searchParams.get("repo");
  let branches: Awaited<ReturnType<typeof getLocalBranches>> = [];

  async function refreshBranches() {
    if (repoPath) {
      try {
        branches = await getLocalBranches(repoPath);
      } catch (error) {
        console.log(error);
      }
    }
  }

  onMount(refreshBranches);
</script>

<div class="flex flex-row items-center justify-between w-full p-2">
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
