<script lang="ts">
	import AddIcon from '@/codicons/add-icon.svelte';
	import EllipsisIcon from '@/codicons/ellipsis-icon.svelte';
	import { Modal } from '@/components/modal';
	import type { BranchInfo } from '@/integrated-backend/browse/branches/types';
	import * as DropdownMenu from '@/shadcn-svelte-components/ui/dropdown-menu';

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
			<DropdownMenu.Item
				class="h-[28px] flex items-center cursor-pointer"
				on:click={() => {
					console.log(`Create new branch from branch "${branch.name}"`);
				}}
			>
				<AddIcon class="h-3/4 aspect-square mr-[6px]" />
				<span>Create new branch</span>
			</DropdownMenu.Item>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
</div>
