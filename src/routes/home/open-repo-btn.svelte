<script lang="ts">
  import { verifyRepository } from "@/integrated-backend/repository-checks";
  import { addRepo } from "@/integrated-backend/repository-store";
  import Button from "@/shadcn-svelte-components/ui/button/button.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { documentDir } from "@tauri-apps/api/path";
  import { toast } from "svelte-sonner";

  export let buttonLabel: string;

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
      toast.error("No folder selected.");
    } else {
      toast.error("Cannot select multiple folders.");
    }
  }
</script>

<Button on:click={folderSelector}>{buttonLabel}</Button>
