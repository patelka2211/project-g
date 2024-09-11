<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse/branches";
  import {
    getParentCommits,
    type CommitInfo,
  } from "@/integrated-backend/browse/branches/commit-history";
  import { repoPath } from "@/stores/repo";
  import { onMount } from "svelte";

  export let branch: BranchInfo;

  let commitHistory: Array<CommitInfo> = [];

  onMount(async () => {
    if ($repoPath !== null) {
      try {
        commitHistory = await getParentCommits(
          $repoPath,
          branch.commitHash,
          20
        );
      } catch (error) {
        console.log(error);
      }
    }
  });
</script>

<div class="w-full">
  {#each commitHistory as commit (`${commit.hash}-${branch.name}`)}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="flex flex-col bg-accent border p-[6px]"
      on:contextmenu|preventDefault={() => {
        console.log(commit);
      }}
    >
      <div class="w-full truncate h-[28px] font-medium">{commit.msg}</div>
      <div class="h-[12px] text-[0.75rem] flex items-center">
        <div>Time</div>
        <div class="mx-[4.5px]">· by</div>
        <a
          class="h-full flex items-center"
          href={`mailto:${commit.author.email}`}
        >
          <img
            class="aspect-square h-full mr-[6px] rounded-full"
            src={`https://avatars.githubusercontent.com/u/e?email=${encodeURI(commit.author.email ? commit.author.email : "null")}&s=56`}
            alt={commit.author.name}
          />
          <div>{commit.author.name}</div>
        </a>
      </div>
    </div>
  {/each}
</div>
