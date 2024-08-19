<script>
  import { appWindow } from "@tauri-apps/api/window";
  import { ModeWatcher } from "mode-watcher";
  import { onMount } from "svelte";
  import "../app.css";

  let titlebarHidden = false;

  async function checkFullscreen() {
    titlebarHidden = await appWindow.isFullscreen();
  }

  onMount(() => {
    appWindow.onResized(checkFullscreen);
  });
</script>

<ModeWatcher />

{#if !titlebarHidden}
  <div data-tauri-drag-region class="border-b border-accent"></div>
{/if}

<slot></slot>

<style>
  div {
    height: 28px;
    width: 100vw;
  }
</style>
