<script>
  import Sonner from "@/shadcn-svelte-components/ui/sonner/sonner.svelte";
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

<Sonner richColors expand />

<!-- Title bar -->
{#if !titlebarHidden}
  <div
    data-tauri-drag-region
    class="w-dvw h-[28px] border-b border-separate"
  ></div>
{/if}

<div class={`w-dvw ${titlebarHidden ? "h-dvh" : "h-[calc(100dvh-28px)]"}`}>
  <slot></slot>
</div>
