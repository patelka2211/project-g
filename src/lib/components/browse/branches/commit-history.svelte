<script lang="ts">
	import AddIcon from '@/codicons/add-icon.svelte';
	import ArrowSwapIcon from '@/codicons/arrow-swap-icon.svelte';
	import CherryPickIcon from '@/codicons/cherry-pick-icon.svelte';
	import EllipsisIcon from '@/codicons/ellipsis-icon.svelte';
	import RevertIcon from '@/codicons/revert-icon.svelte';
	import { switchBranch } from '@/integrated-backend/browse/branches/actions';
	import {
		cherryPickCommit,
		getParentCommits,
		revertCommit
	} from '@/integrated-backend/browse/branches/commit-history';
	import { createBranch } from '@/integrated-backend/browse/branches/name-and-menu';
	import type { BranchInfo, ParentCommits } from '@/integrated-backend/browse/branches/types';
	import * as DropdownMenu from '@/shadcn-svelte-components/ui/dropdown-menu';
	import { repoPath } from '@/stores/repo';
	import { onMount } from 'svelte';
	import RelativeTime from 'svelte-relative-time';

	export let branch: BranchInfo;

	let commitHistory: ParentCommits['list'] = [];

	let endOfCommits: ParentCommits['endOfCommits'] = false;

	async function loadParentCommits() {
		if ($repoPath !== null) {
			try {
				const commitHash =
						commitHistory.length == 0
							? branch.commitHash
							: commitHistory[commitHistory.length - 1].hash,
					skippedCommit = commitHistory.length === 0 ? 0 : 1;

				let parentCommits = await getParentCommits($repoPath, commitHash, 20 + skippedCommit);

				commitHistory = [...commitHistory, ...parentCommits.list.slice(skippedCommit)];

				endOfCommits = parentCommits.endOfCommits;
			} catch (error) {
				console.log(error);
			}
		}
	}

	onMount(loadParentCommits);
</script>

<div class="w-full flex flex-col items-center h-full overflow-x-auto">
	{#each commitHistory as commit (`${commit.hash}-${branch.name}`)}
		<div
			class="w-full flex flex-col p-[6px] border-b last:border-b-0 hover:bg-accent hover:text-accent-foreground"
		>
			<div class="flex items-start justify-between">
				<div class="w-full truncate h-[28px] font-medium">{commit.msg}</div>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						<div class="border rounded-sm px-[2px] hover:bg-background cursor-pointer">
							<EllipsisIcon class="aspect-square h-[16px]" />
						</div>
					</DropdownMenu.Trigger>
					<DropdownMenu.Content align="start">
						<!-- new branch from a commit -->
						<DropdownMenu.Item
							class="cursor-pointer"
							on:click={async () => {
								await createBranch(commit.hash);
							}}
						>
							<div class="flex items-center">
								<AddIcon class="aspect-square h-[16px]" />
								<span class="ml-[4px]">Create new branch</span>
							</div>
						</DropdownMenu.Item>

						{#if branch.isHead}
							<!-- revert -->
							<DropdownMenu.Item
								class="cursor-pointer"
								on:click={async () => {
									await revertCommit(commit.hash);
								}}
							>
								<div class="flex items-center">
									<RevertIcon class="aspect-square h-[16px]" />
									<span class="ml-[4px]">Revert this commit</span>
								</div>
							</DropdownMenu.Item>
						{:else}
							<!-- cherry-pick -->
							<DropdownMenu.Item
								class="cursor-pointer"
								on:click={async () => {
									await cherryPickCommit(commit.hash);
								}}
							>
								<div class="flex items-center">
									<CherryPickIcon class="aspect-square h-[16px]" />
									<span class="ml-[4px]">Cherry-pick this commit</span>
								</div>
							</DropdownMenu.Item>

							<DropdownMenu.Separator></DropdownMenu.Separator>

							<DropdownMenu.Item
								class="cursor-pointer"
								on:click={() => {
									switchBranch(branch.name);
								}}
							>
								<div class="flex items-center">
									<ArrowSwapIcon class="aspect-square h-[16px]" />
									<span class="ml-[4px]">
										switch to "{branch.name}"
									</span>
								</div>
							</DropdownMenu.Item>
						{/if}
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
			<div class="h-[12px] text-[0.75rem] flex items-center">
				<div title={new Date(commit.time).toUTCString()}>
					<RelativeTime date={new Date(commit.time)} locale="en" />
				</div>
				<div class="mx-[4.5px]">· by</div>
				<a class="h-full flex items-center" href={`mailto:${commit.author.email}`}>
					<img
						class="aspect-square h-full mr-[6px] rounded-full"
						src={`https://avatars.githubusercontent.com/u/e?email=${encodeURI(commit.author.email ? commit.author.email : 'null')}&s=56`}
						alt={commit.author.name}
					/>
					<div>{commit.author.name}</div>
				</a>
			</div>
		</div>
	{/each}

	{#if !endOfCommits}
		<div class="w-full p-[6px]">
			<button
				class="w-full border rounded-[6px] hover:bg-accent hover:text-accent-foreground text-md"
				on:click={loadParentCommits}
			>
				load more commits
			</button>
		</div>
	{:else if commitHistory.length === 0}
		<div>no commits</div>
	{/if}
</div>
