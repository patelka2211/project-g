<script lang="ts">
	import AddIcon from '@/codicons/add-icon.svelte';
	import EllipsisIcon from '@/codicons/ellipsis-icon.svelte';
	import { Modal } from '@/components/modal';
	import {
		createBranch,
		mergeBranch,
		rebaseBranch
	} from '@/integrated-backend/browse/branches/name-and-menu';
	import type { BranchInfo } from '@/integrated-backend/browse/branches/types';
	import * as DropdownMenu from '@/shadcn-svelte-components/ui/dropdown-menu';
	import { currentBranch } from '@/stores/branches';

	export let branch: BranchInfo;

	let showBranchModal = false;
</script>

<Modal bind:showModal={showBranchModal}>
	{branch.name}
	<br />
	{branch.upstream}
</Modal>

<div class="w-full h-[28px] flex items-center justify-between">
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		class="w-[calc(100%-28px-6px)] h-full rounded-[6px] px-[6px] hover:bg-accent hover:cursor-pointer"
		on:click={() => (showBranchModal = true)}
	>
		{branch.name}
	</div>
	<DropdownMenu.Root>
		<DropdownMenu.Trigger
			class="aspect-square h-full border rounded-[6px] hover:bg-accent hover:shadow-inner hover:border-accent"
		>
			<EllipsisIcon class="aspect-square h-3/4 m-auto" />
		</DropdownMenu.Trigger>
		<DropdownMenu.Content>
			<!-- new branch -->
			<DropdownMenu.Item
				class="h-[28px] flex items-center cursor-pointer"
				on:click={async () => {
					await createBranch(branch.commitHash);
				}}
			>
				<AddIcon class="h-3/4 aspect-square mr-[6px]" />
				<span>Create new branch</span>
			</DropdownMenu.Item>

			{#if branch.isHead === false && $currentBranch !== undefined}
				<!-- merge -->
				<DropdownMenu.Item
					class="h-[28px] flex items-center cursor-pointer"
					on:click={async () => {
						await mergeBranch(branch.name);
					}}
				>
					<AddIcon class="h-3/4 aspect-square mr-[6px]" />
					<span>
						Merge <strong>{branch.name}</strong> into <strong>{$currentBranch.name}</strong>
					</span>
				</DropdownMenu.Item>

				{#if branch.upstream === null}
					<!-- rebase -->
					<DropdownMenu.Item
						class="h-[28px] flex items-center cursor-pointer"
						on:click={async () => {
							await rebaseBranch(branch.name);
						}}
					>
						<AddIcon class="h-3/4 aspect-square mr-[6px]" />
						<span>
							Rebase <strong>{$currentBranch.name}</strong> on <strong>{branch.name}</strong>
						</span>
					</DropdownMenu.Item>
				{/if}
			{/if}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
</div>
