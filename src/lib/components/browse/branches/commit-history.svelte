<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse/branches";
  import {
    getParentCommits,
    type CommitInfo,
  } from "@/integrated-backend/browse/branches/commit-history";
  import { repoPath } from "@/stores/repo";
  import { onMount } from "svelte";

  export let branch: BranchInfo;

  let commitHistory: Array<CommitInfo>;

  onMount(async () => {
    if ($repoPath !== null) {
      try {
        let commits = await getParentCommits($repoPath, branch.commitHash, 20);

        console.log(commits[0]);
      } catch (error) {
        console.log(error);
      }
    }
  });
</script>

{#each commitHistory as { msg, hash, committer } (`${hash}_${branch.name}`)}
  <img
    src={`https://avatars.githubusercontent.com/u/e?email=${committer.email}&s={image_size_px}`}
    alt={committer.name}
  />
  <div>{msg}</div>
{/each}
