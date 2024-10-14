<script lang="ts">
	import type { BranchInfo } from '@/integrated-backend/browse/branches/types';

	interface Props {
		branch: BranchInfo;
	}

	let { branch }: Props = $props();
</script>

<div class="w-full p-[6px]">
	<button
		class="w-full border rounded-[6px] hover:bg-accent hover:text-accent-foreground text-md"
		onclick={() => {
			console.log(branch.name);
		}}
	>
		Start Committing
	</button>
</div>
