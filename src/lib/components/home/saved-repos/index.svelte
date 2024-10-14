<script lang="ts">
	import CloseIcon from '@/codicons/close-icon.svelte';
	import { verifyRepository } from '@/integrated-backend/home/repositories/checks';
	import { listRepos, removeRepo, reorderRepo } from '@/integrated-backend/home/repositories/saved';
	import type { RepoInfo } from '@/integrated-backend/home/repositories/types';
	import { repoPath } from '@/stores/repo';
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import OpenRepoBtn from './open-repo-btn.svelte';

	let savedRepos = $state<Array<RepoInfo>>([]);

	async function browseRepo(index: number) {
		try {
			let repo = savedRepos[index],
				_repoPath = `${repo.dir}/${repo.name}`;

			await verifyRepository(_repoPath);
			await reorderRepo(repo.id);
			await repoPath.setAndBrowse(_repoPath);
		} catch (error) {
			console.log(error);
			await removeRepo_UIUtility(index, false);
			toast.error('Not able to browse the repository.');
		}
	}

	async function removeRepo_UIUtility(index: number, showToast = true) {
		try {
			let repo = savedRepos[index];

			savedRepos = [...savedRepos.slice(0, index), ...savedRepos.slice(index + 1)];

			let deletedRepo = await removeRepo(repo.id);
			if (showToast === true) {
				toast.success(`Repository "${deletedRepo.name}" removed successfully.`);
			}
		} catch (error) {
			console.log(error);
			if (showToast === true) {
				toast.error('Not able to remove the repository.');
			}
		}
	}

	onMount(async () => {
		try {
			savedRepos = await listRepos();
		} catch (error) {
			console.log(error);
			toast.error('Not able to read saved repositories.');
		}
	});
</script>

{#if savedRepos.length !== 0}
	<div
		class="repo-list flex flex-col items-center border w-[329px] max-h-[208px] mb-2 overflow-y-auto rounded-[12px]"
	>
		{#each savedRepos as repo, index (repo.id)}
			<div
				class="repo flex p-2 items-center justify-between w-full h-12 hover:bg-accent hover:cursor-pointer border-b text-left"
				onclick={async () => {
					await browseRepo(index);
				}}
				onkeypress={async () => {
					await browseRepo(index);
				}}
				tabindex="0"
				role="button"
			>
				<div class="flex flex-col justify-between">
					<div class="truncate max-w-[273px] text-[1.053rem]" title={repo.name}>
						{repo.name}
					</div>
					<div
						class="truncate max-w-[273px] text-[0.632rem] text-muted-foreground"
						title={repo.dir}
					>
						{repo.dir}
					</div>
				</div>
				<div
					class="w-[2rem] h-[2rem] border bg-secondary text-secondary-foreground rounded-[4px] hover:bg-destructive hover:text-destructive-foreground"
					onclick={async (event) => {
						event.stopPropagation();
						event.preventDefault();
						await removeRepo_UIUtility(index);
					}}
					onkeypress={async (event) => {
						event.stopPropagation();
						event.preventDefault();
						await removeRepo_UIUtility(index);
					}}
					role="button"
					tabindex="0"
				>
					<CloseIcon />
				</div>
			</div>
		{/each}
	</div>
	<div class="mb-2">or</div>
	<OpenRepoBtn buttonLabel="Add another repository" />
{:else}
	<OpenRepoBtn buttonLabel="Add repository" />
{/if}

<style lang="scss">
	.repo-list {
		-ms-overflow-style: none;
		scrollbar-width: none;
		&::-webkit-scrollbar {
			display: none;
		}

		.repo {
			&:last-child {
				border-bottom: none;
			}
		}
	}
</style>
