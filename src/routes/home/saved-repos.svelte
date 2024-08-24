<script lang="ts">
  import {
    getRepos,
    type RepoInfo,
  } from "@/integrated-backend/repository-store";
  import { onMount } from "svelte";

  type RepoListItem = RepoInfo & { id: string };

  let savedRepos: Array<RepoListItem> = [];

  // async function deleteRepo(id: string) {}

  function browseRepo(repoPath: string) {
    // check if repo is available or not. if not then delete from repo store and show error.
    console.log(repoPath);
    // goto(`/browse?repo=${repoPath}`);
  }

  onMount(async () => {
    let repos = await getRepos();
    savedRepos = Object.entries(repos).map(
      ([id, { dir, name }]): RepoListItem => {
        return {
          id,
          name,
          dir,
        };
      }
    );
  });
</script>

<div
  class="repo-list flex flex-col items-center border w-[329px] max-h-[203px] mb-6 overflow-y-auto rounded-[12px]"
>
  {#each savedRepos as repo (repo.id)}
    <div
      class="repo flex p-2 items-center justify-between w-full h-12 hover:bg-accent hover:cursor-pointer border-b"
      tabindex="0"
      role="button"
      on:keydown={() => {
        browseRepo(`${repo.dir}/${repo.name}`);
      }}
      on:click={() => {
        browseRepo(`${repo.dir}/${repo.name}`);
      }}
    >
      <div class="flex flex-col justify-between">
        <div class="truncate max-w-[273px] text-[1.053rem]" title={repo.name}>
          {repo.name}
        </div>
        <div
          class="truncate max-w-[273px] text-[0.632rem] text-muted-foreground"
          title={repo.dir}
        >
          {repo.dir}
        </div>
      </div>
      <button
        class="w-[2rem] h-[2rem] border bg-background rounded-[4px] hover:bg-destructive hover:text-destructive-foreground"
        on:click|stopPropagation|preventDefault={() => {
          console.log("hello");
        }}
      >
        D
      </button>
    </div>
  {/each}
</div>

<style lang="scss">
  .repo-list {
    -ms-overflow-style: none;
    scrollbar-width: none;
    &::-webkit-scrollbar {
      display: none;
    }

    .repo {
      &:last-child {
        border-bottom: none;
      }
    }
  }
</style>
