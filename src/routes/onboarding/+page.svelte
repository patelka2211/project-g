<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let repoPath = $page.url.searchParams.get("repo"),
    isItRepository: undefined | false = undefined;

  onMount(async () => {
    if (repoPath) {
      const result = await invoke<boolean>("is_it_repository", {
        repoPath,
      });

      if (result === true) {
        goto(`/browse?repo=${repoPath}`);
      } else {
        isItRepository = result;
      }
    }
  });
</script>

{#if repoPath === null}
  No repo provided!
  <br />
  <a href="/">home</a>
{:else if isItRepository === undefined}
  checking repo: {repoPath}
{:else if isItRepository === false}
  {repoPath} is not a git repo
  <br />
  <a href="/">home</a>
{/if}
