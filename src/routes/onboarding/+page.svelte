<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import {
    getOriginHead,
    getRemoteOrigin,
    isItRepository,
  } from "$lib/onboarding";
  import Button from "@/shadcn-svelte-components/ui/button/button.svelte";
  import { onMount } from "svelte";

  let errorMsg: string | undefined;

  onMount(async () => {
    const repoPath = $page.url.searchParams.get("repo");

    if (repoPath === null) {
      // no repo path
      errorMsg = "No repository path provided.";
      return;
    }

    let isRepository: Awaited<ReturnType<typeof isItRepository>>;

    try {
      isRepository = await isItRepository(repoPath);

      if (isRepository === false) {
        // not a repo
        errorMsg = "The selected folder is not a repository.";
        return;
      }
    } catch (error) {
      console.log(error);
      // error finding repo
      errorMsg = "Cannot find repository.";
      return;
    }

    let remoteOrigin: Awaited<ReturnType<typeof getRemoteOrigin>>;

    try {
      remoteOrigin = await getRemoteOrigin(repoPath);

      if (remoteOrigin.fetch === undefined || remoteOrigin.push === undefined) {
        // both fetch and push must be available
        errorMsg = "Both fetch and push url should be available.";
        return;
      }
    } catch (error) {
      console.log(error);
      // no remote origin found
      errorMsg = "No remote found.";
      return;
    }

    let originHead: Awaited<ReturnType<typeof getOriginHead>>;

    try {
      originHead = await getOriginHead(repoPath);
    } catch (error) {
      console.log(error);
      // could not found origin/HEAD
      errorMsg = `Cannot find "origin/HEAD".`;
      return;
    }

    await goto(`/browse?repo=${repoPath}`);
  });
</script>

{#if errorMsg}
  <div class="root flex flex-col items-center justify-around">
    <div class="flex flex-col items-center gap-2">
      <span>{errorMsg}</span>

      <Button
        variant="outline"
        size="sm"
        on:click={async () => {
          await goto("/home");
        }}
      >
        Back
      </Button>
    </div>
  </div>
{/if}

<style>
  .root {
    min-height: calc(100dvh - 28px);
  }
</style>
