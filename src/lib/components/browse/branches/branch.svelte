<script lang="ts">
  import type { BranchInfo } from "@/integrated-backend/browse/branches";
  import Actions from "./actions.svelte";
  import NameAndMenu from "./name-and-menu.svelte";

  export let branch: BranchInfo;
</script>

<div class="p-[6px] border rounded-[12px]">
  <NameAndMenu {branch} />
  <Actions {branch} />
</div>
