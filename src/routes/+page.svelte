<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  let gitExist: boolean = true;

  onMount(async () => {
    let result = await invoke<boolean>("is_git_available");

    if (result === false) gitExist = false;
    else goto("/home");
  });
</script>

{#if gitExist === false}
  Git is not available. :(
{/if}
