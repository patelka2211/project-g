<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { documentDir } from "@tauri-apps/api/path";
  
    let selectedFolder: Awaited<ReturnType<typeof open>> = null;
  
    async function folderSelector() {
      selectedFolder = await open({
        defaultPath: await documentDir(),
        directory: true,
        multiple: false,
      });
    }
  </script>
  
  <button on:click={folderSelector}>Select folder</button>
  
  {#if selectedFolder !== null}
    <br />
  
    Selected folder:
    {selectedFolder}
  {/if}
  
  <style>
  </style>
  