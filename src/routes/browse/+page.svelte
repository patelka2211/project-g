<script lang="ts">
  import { goto } from "$app/navigation";
  import Branches from "@/components/browse/Branches.svelte";
  import Sidebar from "@/components/browse/Sidebar.svelte";
  import { repoPath } from "@/stores/Repo";
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
