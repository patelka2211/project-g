<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse/branches/types";
  import Actions from "./actions.svelte";
  import CommitHistory from "./commit-history.svelte";
  import NameAndMenu from "./name-and-menu.svelte";

  export let branch: BranchInfo;
</script>

<div class="p-[6px] border rounded-[12px] h-full">
  <NameAndMenu {branch} />
  <Actions {branch} />
  <!-- start committing -->
  <CommitHistory {branch} />
</div>
