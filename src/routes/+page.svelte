<script lang="ts">
  import { goto } from "$app/navigation";
  import { isGitAvailable } from "$lib/initialization";
  import { onMount } from "svelte";

  let gitExistError: undefined | string;

  onMount(async () => {
    try {
      let gitExist = await isGitAvailable();
      if (gitExist === true) goto("/home");
      else gitExistError = "Git not available.";
    } catch (error) {
      gitExistError = "Not able to check availablability of Git.";
    }
  });
</script>

{#if gitExistError !== undefined}
  {gitExistError}
{/if}
