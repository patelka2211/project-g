<script lang="ts">
  import { goto } from "$app/navigation";
  import CloseIcon from "@/codicons/close-icon.svelte";
  import { verifyRepository } from "@/integrated-backend/repository-checks";
  import {
    listRepos,
    removeRepo,
    reorderRepo,
    type RepoInfo,
  } from "@/integrated-backend/repository-store";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import OpenRepoBtn from "./open-repo-btn.svelte";

  let savedRepos: Array<RepoInfo> = [];

  async function browseRepo(index: number) {
    try {
      let repo = savedRepos[index],
        repoPath = `${repo.dir}/${repo.name}`;

      await verifyRepository(repoPath);
      await reorderRepo(repo.id);
      await goto(`/browse?repo=${repoPath}`);
    } catch (error) {
      console.log(error);
      await removeRepo_UIUtility(index, false);
      toast.error("Not able to browse the repository.");
    }
  }

  async function removeRepo_UIUtility(index: number, showToast = true) {
    try {
      let repo = savedRepos[index];

      savedRepos = [
        ...savedRepos.slice(0, index),
        ...savedRepos.slice(index + 1),
      ];

      let deletedRepo = await removeRepo(repo.id);
      if (showToast === true) {
        toast.success(`Repository "${deletedRepo.name}" removed successfully.`);
      }
    } catch (error) {
      console.log(error);
      if (showToast === true) {
        toast.error("Not able to remove the repository.");
      }
    }
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
    {#each savedRepos as repo, index (repo.id)}
      <button
        class="repo flex p-2 items-center justify-between w-full h-12 hover:bg-accent hover:cursor-pointer border-b text-left"
        on:click={async () => {
          await browseRepo(index);
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
          class="w-[2rem] h-[2rem] border bg-secondary text-secondary-foreground rounded-[4px] hover:bg-destructive hover:text-destructive-foreground"
          on:click|stopPropagation|preventDefault={async () => {
            await removeRepo_UIUtility(index);
          }}
        >
          <CloseIcon />
        </button>
      </button>
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
