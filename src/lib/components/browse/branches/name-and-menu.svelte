<script lang="ts">
  import EllipsisIcon from "@/codicons/ellipsis-icon.svelte";
  import { Modal } from "@/components/modal";
  import type { BranchInfo } from "@/integrated-backend/browse/branches/types";

  export let branch: BranchInfo;

  let showBranchModal = false;
</script>

<Modal bind:showModal={showBranchModal}>
  {branch.name}
  <br />
  {branch.upstream}
</Modal>

<div class="h-[28px] flex items-center justify-between mb-[6px]">
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="w-[calc(100%-28px-6px)] h-full rounded-[6px] px-[6px] hover:bg-accent hover:cursor-pointer"
    on:click={() => (showBranchModal = true)}
  >
    {branch.name}
  </div>
  <button
    class="aspect-square h-full border rounded-[6px] hover:bg-accent hover:shadow-inner hover:border-accent"
  >
    <EllipsisIcon class="aspect-square h-3/4 m-auto" />
  </button>
</div>
