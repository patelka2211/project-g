<script lang="ts">
  import {
    deleteRepo,
    listRepos,
    type RepoInfo,
  } from "@/integrated-backend/repository-store";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import OpenRepoBtn from "./open-repo-btn.svelte";

  let savedRepos: Array<RepoInfo> = [];

  function browseRepo(repoPath: string) {
    // Verify the repository. if something goes wrong then delete from repo store and show error.
    // if everything is okay then set as last opened the repository and then browse repository.
    console.log(repoPath);
    // goto(`/browse?repo=${repoPath}`);
  }

  onMount(async () => {
    try {
      savedRepos = await listRepos();
    } catch (error) {
      console.log(error);
      toast.error("Not able to read saved repositories.");
    }
  });
</script>

{#if savedRepos.length !== 0}
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
          on:click|stopPropagation|preventDefault={async () => {
            try {
              let deletedRepo = await deleteRepo(repo.id);
              toast.success(
                `Repository "${deletedRepo.name}" removed successfully.`
              );
            } catch (error) {
              toast.error("Not able to remove the repository.");
            }
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
