<script lang="ts">
	import type { BranchInfo } from '@/integrated-backend/browse/branches/types';

	export let branch: BranchInfo;
</script>

<div class="w-full p-[6px]">
	<button
		class="w-full border rounded-[6px] hover:bg-accent hover:text-accent-foreground text-md"
		on:click={() => {
			console.log(branch.name);
		}}
	>
		Start Committing
	</button>
</div>
