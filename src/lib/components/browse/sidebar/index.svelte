<script>
  import { goto } from "$app/navigation";
  import ArrowSmallLeftIcon from "@/codicons/arrow-small-left-icon.svelte";
  import RefreshIcon from "@/codicons/refresh-icon.svelte";
  import { branches } from "@/stores/branches";
  import { repoPath } from "@/stores/repo";
</script>

<div
  class="w-[4.25rem] h-full p-[14px] flex flex-col items-center justify-between border-r-[0.5px] border-separate"
>
  <div class="flex flex-col items-center gap-2">
    <button
      class="aspect-square border w-[36px] rounded-[6px] hover:bg-accent hover:text-accent-foreground"
      on:click={() => {
        goto("/home");
      }}
    >
      <ArrowSmallLeftIcon />
    </button>
  </div>
  <div class="flex flex-col items-center gap-2">
    <button
      class="aspect-square border w-[36px] rounded-[6px] hover:bg-accent hover:text-accent-foreground"
      on:click={() => {
        if ($repoPath) {
          branches.reload($repoPath);
        }
      }}
    >
      <RefreshIcon class="w-3/4 m-auto" />
    </button>
  </div>
</div>
