<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse";

  export let branch: BranchInfo;
</script>

<div class="m-4">
  {branch.name}
</div>
