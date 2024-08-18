<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "@/components/ui/button";
  import Sonner from "@/components/ui/sonner/sonner.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { documentDir } from "@tauri-apps/api/path";
  import { toast } from "svelte-sonner";

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

  $: {
    if (errorMsg !== undefined) {
      toast.error(errorMsg);
      errorMsg = undefined;
    }
  }
</script>

<Sonner richColors expand />

<div class="root flex flex-col items-center justify-around">
  <div class="flex flex-col items-center">
    <h1 class="font-semibold text-3xl mb-6">Welcome to Project G</h1>
    <Button on:click={folderSelector} class="mb-4">Select folder</Button>
  </div>
</div>

<style lang="scss">
  * {
    &:hover {
      cursor: default;
    }
  }
  .root {
    min-height: calc(100dvh - 28px);
  }
</style>
