<script lang="ts">
  import { goto } from "$app/navigation";
  import { Branches } from "@/components/browse/branches";
  import { Sidebar } from "@/components/browse/sidebar";
  import { repoPath } from "@/stores/repo";
  import { onDestroy, onMount } from "svelte";
  import { toast } from "svelte-sonner";

  onMount(async () => {
    if ($repoPath === null) {
      toast.error("Repository path not provided.");
      repoPath.clear();
      await goto("/home");
    }
  });

  onDestroy(() => {
    repoPath.clear();
  });
</script>

<div class="flex h-full">
  <Sidebar />
  <Branches />
</div>
