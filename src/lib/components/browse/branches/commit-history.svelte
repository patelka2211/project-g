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

  onMount(async () => {
    if ($repoPath !== null) {
      try {
        const parentCommits = await getParentCommits(
          $repoPath,
          {
            name: branch.name,
            branch_type: "Local",
          },
          20
        );

        commitHistory = parentCommits.list;
        endOfCommits = parentCommits.endOfCommits;
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
  end of commits: {endOfCommits}
</div>
