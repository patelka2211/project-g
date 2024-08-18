<script lang="ts">
  import { goto } from "$app/navigation";
  import Button from "@/components/ui/button/button.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { documentDir } from "@tauri-apps/api/path";

  let errorMsg: undefined | string = undefined;

  async function folderSelector() {
    let selectedFolder = await open({
      defaultPath: await documentDir(),
      directory: true,
      multiple: false,
    });

    if (typeof selectedFolder === "string") {
      goto(`/onboarding?repo=${selectedFolder}`);
    } else if (selectedFolder === null) {
      errorMsg = "No folder selected!";
    } else {
      errorMsg = "Cannot select multiple folders!";
    }
  }
</script>

<Button on:click={folderSelector}>Select folder</Button>

{#if typeof errorMsg === "string"}
  {errorMsg}
{/if}
