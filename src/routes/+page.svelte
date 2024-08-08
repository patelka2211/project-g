<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  async function gitExist() {
    let git_exist = await invoke<boolean>("is_git_available");

    if (git_exist === false) goto("/git-does-not-exist");
    else goto("/home");
  }

  onMount(() => {
    gitExist();
  });
</script>

<!-- Checking if Git is available or not. -->
