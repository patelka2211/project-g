<script lang="ts">
	import AKPVERSEInitiative from '@/assets/a-kpverse-initiative.svelte';
	import { SavedRepos } from '@/components/home/saved-repos';
</script>

<div class="min-h-full flex flex-col items-center justify-around">
	<div class="flex flex-col items-center">
		<img src="/icon.png" alt="Project G" class="h-[140px] aspect-square mb-4" />
		<h1 class="font-semibold text-3xl mb-6">Welcome to Project G</h1>
		<SavedRepos />
	</div>
</div>

<div class="absolute overflow-hidden bottom-[24px] h-[10px] left-[calc(50%-5*24.33px)]">
	<AKPVERSEInitiative class="h-full" />
	<div class="shine-effect"></div>
</div>

<style lang="scss">
	* {
		&:hover {
			cursor: default;
		}
	}

	.shine-effect {
		background: linear-gradient(
			130deg,
			hsl(var(--background) / 0),
			hsl(var(--background) / 1),
			hsl(var(--background) / 0)
		);
		animation: shine 3s;
		position: absolute;
		top: 0;
		left: -100%;
		width: 100%;
		height: 100%;
		pointer-events: none;
	}

	@keyframes shine {
		0% {
			left: -100%;
		}
		100% {
			left: 100%;
		}
	}
</style>
