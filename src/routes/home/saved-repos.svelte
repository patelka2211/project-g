<script lang="ts">
  import {
    deleteRepo,
    getRepos,
    type RepoInfo,
  } from "@/integrated-backend/repository-store";
  import { onMount } from "svelte";
  import OpenRepoBtn from "./open-repo-btn.svelte";

  type RepoListItem = RepoInfo & { id: string };

  let savedRepos: Array<RepoListItem> = [];

  // async function deleteRepo(id: string) {}

  function browseRepo(repoPath: string) {
    // check if repo is available or not. if not then delete from repo store and show error.
    console.log(repoPath);
    // goto(`/browse?repo=${repoPath}`);
  }

  onMount(async () => {
    try {
      let repos = await getRepos();
      savedRepos = Object.entries(repos)
        .sort(([, a], [, b]) =>
          `${a.dir}/${a.name}`.localeCompare(`${b.dir}/${b.name}`)
        )
        .map(([id, { dir, name }]): RepoListItem => {
          return {
            id,
            name,
            dir,
          };
        });
    } catch (error) {
      //
    }
  });
</script>

{#if savedRepos.length !== 0}
  <!-- <div class="mb-2">continue with opened repository.</div> -->
  <div
    class="repo-list flex flex-col items-center border w-[329px] max-h-[208px] mb-2 overflow-y-auto rounded-[12px]"
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
            deleteRepo(repo.id);
          }}
        >
          D
        </button>
      </div>
    {/each}
  </div>
  <div class="mb-2">or</div>
  <OpenRepoBtn buttonLabel="Add another repository" />
{:else}
  <div class="mb-2">Get started by adding repository</div>
  <OpenRepoBtn buttonLabel="Add repository" />
{/if}

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
