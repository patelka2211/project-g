<script lang="ts">
  import RootLayoutMacOS from "@/components/root-layout.macos.svelte";
  import RootLayout from "@/components/root-layout.svelte";
  import Sonner from "@/shadcn-svelte-components/ui/sonner/sonner.svelte";
  import { type as getOS } from "@tauri-apps/api/os";
  import { ModeWatcher } from "mode-watcher";
  import { onMount } from "svelte";
  import "../app.css";

  let isMacOS = false;

  onMount(async () => {
    isMacOS = (await getOS()) === "Darwin";
  });
</script>

<ModeWatcher />

<Sonner richColors expand />

{#if isMacOS}
  <RootLayoutMacOS>
    <slot></slot>
  </RootLayoutMacOS>
{:else}
  <RootLayout>
    <slot></slot>
  </RootLayout>
{/if}
