<script lang="ts">
  import { getParentCommits } from "@/integrated-backend/browse/branches/commit-history";
  import type {
    BranchInfo,
    ParentCommits,
  } from "@/integrated-backend/browse/branches/types";
  import { repoPath } from "@/stores/repo";
  import { onMount } from "svelte";

  export let branch: BranchInfo;

  let commitHistory: ParentCommits["list"] = [];

  let endOfCommits: ParentCommits["endOfCommits"] = false;

  async function loadParentCommits() {
    if ($repoPath !== null) {
      try {
        const commitHash =
            commitHistory.length == 0
              ? branch.commitHash
              : commitHistory[commitHistory.length - 1].hash,
          skippedCommit = commitHistory.length === 0 ? 0 : 1;

        let parentCommits = await getParentCommits(
          $repoPath,
          commitHash,
          20 + skippedCommit
        );

        commitHistory = [
          ...commitHistory,
          ...parentCommits.list.slice(skippedCommit),
        ];

        endOfCommits = parentCommits.endOfCommits;
      } catch (error) {
        console.log(error);
      }
    }
  }

  onMount(loadParentCommits);
</script>

<div class="w-full flex flex-col items-center h-full overflow-x-auto">
  {#each commitHistory as commit (`${commit.hash}-${branch.name}`)}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      class="w-full flex flex-col p-[6px] border-b last:border-b-0"
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

  {#if !endOfCommits}
    <div class="w-full p-[6px]">
      <button
        class="w-full border rounded-[6px] hover:bg-accent hover:text-accent-foreground text-md"
        on:click={loadParentCommits}
      >
        load more commits
      </button>
    </div>
  {:else if commitHistory.length === 0}
    <div>no commits</div>
  {/if}
</div>
