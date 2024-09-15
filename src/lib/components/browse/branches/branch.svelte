<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse/branches/types";
  import Separator from "@/shadcn-svelte-components/ui/separator/separator.svelte";
  import Actions from "./actions.svelte";
  import CommitHistory from "./commit-history.svelte";
  import NameAndMenu from "./name-and-menu.svelte";
  import StartCommitting from "./start-committing.svelte";

  export let branch: BranchInfo;
</script>

<div class="w-full border rounded-[12px] h-full flex flex-col items-center">
  <div class="w-full p-[6px] flex flex-col items-center gap-[6px]">
    <NameAndMenu {branch} />
    <Actions {branch} />
  </div>
  {#if branch.isHead === true}
    <Separator orientation="horizontal" />
    <StartCommitting {branch} />
  {/if}
  <Separator orientation="horizontal" />
  <CommitHistory {branch} />
</div>
