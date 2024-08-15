<script lang="ts">
  import { page } from "$app/stores";
  import { getLocalBranches } from "$lib/browse";
  import { onMount } from "svelte";
  import NoRepoPath from "./no-repo-path.svelte";

  let repoPath: string | null = null;

  let branches: Awaited<ReturnType<typeof getLocalBranches>> = undefined;

  onMount(async () => {
    repoPath = $page.url.searchParams.get("repo");

    if (repoPath !== null) {
      let _branches = await getLocalBranches(repoPath);

      branches = _branches;
    } else {
      console.log("no repo");
    }
  });
</script>

{#if repoPath === null}
  <NoRepoPath />
{:else}
  <!-- root -->
  <div class="flex flex-col items-center">hi</div>
  <!-- {:else if branches !== undefined}
  {#each branches as branch (branch.name)}
    {branch.name}, {branch.upstream}
    <br />
  {/each} -->
{/if}

<br />
<a href="/">home</a>
