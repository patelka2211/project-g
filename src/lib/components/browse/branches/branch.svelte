<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse/branches";
  import Actions from "./actions.svelte";

  export let branch: BranchInfo;
</script>

<div class="p-2 border rounded-[12px]">
  <Actions {branch} />
  {branch.name}
</div>
