<script lang="ts">
  import CloudUploadIcon from "@/codicons/cloud-upload-icon.svelte";
  import FetchIcon from "@/codicons/fetch-icon.svelte";
  import PullIcon from "@/codicons/pull-icon.svelte";
  import PushIcon from "@/codicons/push-icon.svelte";
  import TargetIcon from "@/codicons/target-icon.svelte";
  import TrashIcon from "@/codicons/trash-icon.svelte";
  import type { BranchInfo } from "@/integrated-backend/browse";

  export let branch: BranchInfo;
</script>

<div class="p-2 border rounded-[12px]">
  <div class="h-[28px] flex items-center justify-between gap-[4px]">
    <button
      class={`action-button ${branch.isHead ? "bg-accent-foreground text-accent" : ""} rounded-full`}
    >
      <TargetIcon class="aspect-square h-full" />
    </button>
    <button class="action-button">
      <FetchIcon class="aspect-square h-full" />
      <span>Fetch</span>
    </button>
    <button class="action-button">
      <PullIcon class="aspect-square h-full" />
      <span>Pull</span>
    </button>
    {#if branch.upstream}
      <button class="action-button">
        <PushIcon class="aspect-square h-full" />
        <span>Push</span>
      </button>
    {:else}
      <button class="action-button">
        <CloudUploadIcon class="aspect-square h-full" />
        <span>Publish</span>
      </button>
    {/if}
    <button class="action-button">
      <TrashIcon class="aspect-square h-full" />
      <span>Delete</span>
    </button>
  </div>
  {branch.name}
</div>

<style lang="scss">
  .action-button {
    @apply h-[28px] border p-[4px] flex gap-[4px] items-center rounded-[9px] shadow-sm hover:shadow-lg;
  }
</style>
