<script lang="ts">
  import EllipsisIcon from "@/codicons/ellipsis-icon.svelte";
  import type { BranchInfo } from "@/integrated-backend/browse/branches";

  export let branch: BranchInfo;
</script>

<div class="h-[28px] flex items-center justify-between mb-[6px]">
  <div
    class="w-[calc(100%-28px-6px)] h-full rounded-[6px] px-[6px] hover:bg-accent hover:cursor-pointer"
  >
    {branch.name}
  </div>
  <button
    class="aspect-square h-full border rounded-[6px] hover:bg-accent hover:shadow-inner hover:border-accent"
  >
    <EllipsisIcon class="aspect-square h-3/4 m-auto" />
  </button>
</div>
