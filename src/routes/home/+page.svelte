<script lang="ts">
	import { SavedRepos } from '@/components/home/saved-repos';
</script>

<div class="min-h-full flex flex-col items-center justify-around">
	<div class="flex flex-col items-center">
		<img src="/icon.png" alt="Project G" class="h-[140px] aspect-square mb-4" />
		<h1 class="font-semibold text-3xl mb-6">Welcome to Project G</h1>
		<SavedRepos />
	</div>
</div>

<style lang="scss">
	* {
		&:hover {
			cursor: default;
		}
	}
</style>
