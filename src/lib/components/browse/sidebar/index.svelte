<script>
	import { goto } from '$app/navigation';
	import ArrowSmallLeftIcon from '@/codicons/arrow-small-left-icon.svelte';
	import PruneIcon from '@/codicons/prune-icon.svelte';
	import { pruneRepository } from '@/integrated-backend/browse/sidebar';
</script>

<div
	class="w-[4.25rem] h-full p-[14px] flex flex-col items-center justify-between border-r-[0.5px] border-separate"
>
	<div class="flex flex-col items-center gap-2">
		<button
			class="aspect-square border w-[36px] rounded-[6px] hover:bg-accent hover:text-accent-foreground"
			onclick={async () => {
				await goto('/home');
			}}
		>
			<ArrowSmallLeftIcon />
		</button>
	</div>
	<div class="flex flex-col items-center gap-2">
		<!-- repo actions -->
		<button
			class="aspect-square border w-[36px] rounded-[6px] hover:bg-accent hover:text-accent-foreground"
			title="Prune repository"
			onclick={async () => {
				await pruneRepository();
			}}
		>
			<PruneIcon class="m-auto h-3/5" />
		</button>
	</div>
</div>
