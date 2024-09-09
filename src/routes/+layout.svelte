<script lang="ts">
  import Sonner from "@/shadcn-svelte-components/ui/sonner/sonner.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { ModeWatcher } from "mode-watcher";
  import { onDestroy, onMount } from "svelte";
  import "../app.css";

  let isFullscreen = false;
  let unlisten: Awaited<ReturnType<typeof appWindow.onResized>>;

  async function checkFullscreen() {
    isFullscreen = await appWindow.isFullscreen();
  }

  onMount(async () => {
    await checkFullscreen();
    unlisten = await appWindow.onResized(checkFullscreen);
  });

  onDestroy(() => {
    unlisten();
  });
</script>

<ModeWatcher />

<Sonner richColors expand />

<!-- Title bar -->
{#if !isFullscreen}
  <div
    data-tauri-drag-region
    class="fixed z-[10000] top-0 left-0 w-dvw h-[28px] border-b border-separate bg-background select-none"
  ></div>
  <div class="h-[28px]"></div>
{/if}

<div class={`w-dvw ${isFullscreen ? "h-dvh" : "h-[calc(100dvh-28px)]"}`}>
  <slot></slot>
</div>
