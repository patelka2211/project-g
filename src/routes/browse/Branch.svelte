<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse";

  export let branch: BranchInfo;
</script>

<div class="p-2 border rounded-[14px]">
  <button class="aspect-square h-[36px] border rounded-[6px]">hi</button>
  <div class="truncate">
    {branch.name}
  </div>
</div>
