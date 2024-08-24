<script lang="ts">
  import { verifyRepository } from "@/integrated-backend/repository-checks";
  import { addRepo } from "@/integrated-backend/repository-store";
  import { Button } from "@/shadcn-svelte-components/ui/button";
  import { open } from "@tauri-apps/api/dialog";
  import { documentDir } from "@tauri-apps/api/path";
  import { toast } from "svelte-sonner";
  import SavedRepos from "./saved-repos.svelte";

  let errorMsg: undefined | string = undefined;

  async function folderSelector() {
    let selectedFolder = await open({
      defaultPath: await documentDir(),
      directory: true,
      multiple: false,
    });

    if (typeof selectedFolder === "string") {
      try {
        await verifyRepository(selectedFolder);

        let folders = selectedFolder.split("/");

        let [dir, name] = [
          folders.slice(0, folders.length - 1).join("/"),
          folders.at(-1) || "",
        ];

        // if adding repo to repo store fails then just redirect to browse page.
        let id = await addRepo(dir, name);

        console.log(id);
        // goto(`/browse?repo=${selectedFolder}`);
      } catch (error) {
        console.error(error);
        toast.error((error as Error).toString());
      }
    } else if (selectedFolder === null) {
      errorMsg = "No folder selected.";
    } else {
      errorMsg = "Cannot select multiple folders.";
    }
  }

  $: {
    if (errorMsg !== undefined) {
      toast.error(errorMsg);
      errorMsg = undefined;
    }
  }
</script>

<div class="min-h-full flex flex-col items-center justify-around">
  <div class="flex flex-col items-center">
    <h1 class="font-semibold text-3xl mb-6">Welcome to Project G</h1>
    <SavedRepos />
    <Button on:click={folderSelector}>Open another repository</Button>
  </div>
</div>

<style lang="scss">
  * {
    &:hover {
      cursor: default;
    }
  }
</style>
