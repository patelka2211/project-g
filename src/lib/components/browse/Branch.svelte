<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse";
  import BranchActions from "./Branch-Actions.svelte";

  export let branch: BranchInfo;
</script>

<div class="p-2 border rounded-[12px]">
  <BranchActions {branch} />
  {branch.name}
</div>
