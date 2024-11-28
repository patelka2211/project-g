<script lang="ts">
	import { goto } from '$app/navigation';
	import AddIcon from '@/codicons/add-icon.svelte';
	import ArrowSmallLeftIcon from '@/codicons/arrow-small-left-icon.svelte';
	import PruneIcon from '@/codicons/prune-icon.svelte';
	import { createBranch } from '@/integrated-backend/browse/branches/name-and-menu';
	import { pruneRepository } from '@/integrated-backend/browse/sidebar';
</script>

<div
	class="w-[4.25rem] h-full p-[14px] flex flex-col items-center justify-between border-r-[0.5px] border-separate"
>
	<div class="flex flex-col items-center gap-2">
		<button
			class="aspect-square border w-[36px] rounded-[6px] hover:bg-accent hover:text-accent-foreground"
			on:click={async () => {
				await goto('/home');
			}}
		>
			<ArrowSmallLeftIcon />
		</button>
	</div>
	<div class="flex flex-col items-center gap-2">
		<!-- repo actions -->

		<!-- create new branch -->
		<button
			class="aspect-square border w-[36px] rounded-[6px] hover:bg-accent hover:text-accent-foreground"
			title="Create new branch"
			on:click={async () => {
				await createBranch(null);
			}}
		>
			<AddIcon class="m-auto h-3/5" />
		</button>

		<!-- prune repository -->
		<button
			class="aspect-square border w-[36px] rounded-[6px] hover:bg-accent hover:text-accent-foreground"
			title="Prune repository"
			on:click={async () => {
				await pruneRepository();
			}}
		>
			<PruneIcon class="m-auto h-3/5" />
		</button>
	</div>
</div>
