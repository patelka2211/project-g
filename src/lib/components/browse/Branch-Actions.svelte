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

<div class="flex items-center justify-between gap-[6px]">
  <!-- target branch button -->
  <button
    class={`action-button ${branch.isHead ? "bg-accent-foreground text-accent" : "action_target"}`}
    title={`${branch.isHead ? "Already on" : "Switch to"} ${branch.name}`}
    on:click={() => {
      if (branch.isHead === true) return;
      console.log(`Switch to ${branch.name}`);
    }}
  >
    <TargetIcon class="aspect-square h-full" />
  </button>

  <!-- fetch -->
  <button
    class={`action-button action_common${branch.upstream !== null ? undefined : " action_disabled"}`}
    title={branch.upstream !== null
      ? `Fetch from ${branch.upstream}`
      : undefined}
    on:click={() => {
      if (branch.upstream === null) return;

      console.log(`Fetch from ${branch.upstream}`);
    }}
  >
    <FetchIcon class="aspect-square h-full" />
    <span>Fetch</span>
  </button>

  <!-- pull -->
  <button
    class={`action-button action_common ${branch.upstream !== null ? undefined : "action_disabled"}`}
    title={branch.upstream !== null
      ? `Pull from ${branch.upstream}`
      : undefined}
    on:click={() => {
      if (branch.upstream === null) return;

      console.log(`Pull from ${branch.upstream}`);
    }}
  >
    <PullIcon class="aspect-square h-full" />
    <span>Pull</span>
  </button>

  <!-- publish/push -->
  <button
    class="action-button action_common"
    title={branch.upstream !== null
      ? `Push to ${branch.upstream}`
      : `Publish as origin/${branch.name}`}
    on:click={() => {
      if (branch.upstream !== null) {
        console.log(`Push to ${branch.upstream}`);
      } else {
        console.log(`Publish to origin/${branch.name}`);
      }
    }}
  >
    {#if branch.upstream !== null}
      <PushIcon class="aspect-square h-full" />
      <span>Push</span>
    {:else}
      <CloudUploadIcon class="aspect-square h-full" />
      <span>Publish</span>
    {/if}
  </button>

  <!-- delete -->
  <button
    class="action-button action_delete"
    on:click={() => {
      if (branch.upstream !== null) {
        console.log(
          `Compare with ${branch.upstream} and delete if up-to-date else throw error accordingly`
        );
      } else {
        console.log(
          "Compare with origin/HEAD and delete if up-to-date else throw error accordingly"
        );
      }
    }}
  >
    <TrashIcon class="aspect-square h-full" />
    <span>Delete</span>
  </button>
</div>

<style lang="scss">
  .action-button {
    @apply h-[28px];
    @apply p-[6px];
    @apply border rounded-[6px];
    @apply flex items-center gap-[6px];
    @apply shadow-sm hover:shadow-inner;

    &.action_target {
      @apply hover:bg-accent hover:text-accent-foreground;
    }

    &.action_common {
      @apply hover:bg-accent hover:text-accent-foreground;

      &.action_disabled {
        @apply opacity-50 cursor-not-allowed;
      }
    }

    &.action_delete {
      @apply hover:bg-destructive hover:text-destructive-foreground;
    }
  }
</style>
