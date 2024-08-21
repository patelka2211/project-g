<script>
  import { appWindow } from "@tauri-apps/api/window";
  import { ModeWatcher } from "mode-watcher";
  import { onMount } from "svelte";
  import "../app.css";
  import Sonner from "@/shadcn-svelte-components/ui/sonner/sonner.svelte";

  let titlebarHidden = false;

  async function checkFullscreen() {
    titlebarHidden = await appWindow.isFullscreen();
  }

  onMount(() => {
    appWindow.onResized(checkFullscreen);
  });
</script>

<ModeWatcher />

<Sonner richColors expand />

{#if !titlebarHidden}
  <div data-tauri-drag-region class="draggable border-b border-separate"></div>
{/if}

<div class={`window-content${titlebarHidden ? " fullscreen" : ""}`}>
  <slot></slot>
</div>

<style>
  .draggable {
    height: 28px;
    width: 100vw;
  }

  .window-content {
    width: 100dvw;
    height: calc(100dvh - 28px);
  }
  .window-content.fullscreen {
    height: 100dvh;
  }
</style>
