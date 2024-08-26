<script lang="ts">
  import { page } from "$app/stores";
  import {
    getLocalBranches,
    type BranchInfo,
  } from "@/integrated-backend/browse";
  import { onMount } from "svelte";
  import Branches from "./Branches.svelte";
  import Sidebar from "./Sidebar.svelte";

  let repoPath = $page.url.searchParams.get("repo");
  let branches: Array<BranchInfo> = [];

  function sortBranches(unprocessedBranches: Array<BranchInfo>) {
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

  // $: repoName = repoPath ? repoPath.split("/").at(-1) : repoPath;
</script>

<div class="flex h-full">
  <Sidebar />
  <Branches branches={[]} />
</div>
