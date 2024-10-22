<script lang="ts">
	import AddIcon from '@/codicons/add-icon.svelte';
	import EllipsisIcon from '@/codicons/ellipsis-icon.svelte';
	import {
		createBranch,
		mergeBranch,
		rebaseBranch
	} from '@/integrated-backend/browse/branches/name-and-menu';
	import type { BranchInfo } from '@/integrated-backend/browse/branches/types';
	import {
		AlertDialog,
		AlertDialogContent,
		AlertDialogDescription,
		AlertDialogPortal,
		AlertDialogTitle,
		AlertDialogTrigger
	} from '@/shadcn-svelte-components/ui/alert-dialog';
	import {
		DropdownMenu,
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuTrigger
	} from '@/shadcn-svelte-components/ui/dropdown-menu';
	import { currentBranch } from '@/stores/branches';

	export let branch: BranchInfo;
</script>

<div class="w-full h-[28px] flex items-center justify-between">
	<AlertDialog closeOnEscape closeOnOutsideClick>
		<AlertDialogTrigger
			class="w-[calc(100%-28px-6px)] h-full rounded-[6px] px-[6px] hover:bg-accent hover:cursor-pointer text-start"
		>
			{branch.name}
		</AlertDialogTrigger>
		<AlertDialogPortal>
			<AlertDialogContent>
				<AlertDialogTitle>
					{branch.name}
				</AlertDialogTitle>
				<AlertDialogDescription>
					{#if branch.upstream !== null}
						Tracking "{branch.upstream}"
					{/if}
				</AlertDialogDescription>
			</AlertDialogContent>
		</AlertDialogPortal>
	</AlertDialog>
	<DropdownMenu>
		<DropdownMenuTrigger
			class="aspect-square h-full border rounded-[6px] hover:bg-accent hover:shadow-inner hover:border-accent"
		>
			<EllipsisIcon class="aspect-square h-3/4 m-auto" />
		</DropdownMenuTrigger>
		<DropdownMenuContent>
			<!-- new branch -->
			<DropdownMenuItem
				class="h-[28px] flex items-center cursor-pointer"
				on:click={async () => {
					await createBranch(branch.commitHash);
				}}
			>
				<AddIcon class="h-3/4 aspect-square mr-[6px]" />
				<span>Create new branch</span>
			</DropdownMenuItem>

			{#if branch.isHead === false && $currentBranch !== undefined}
				<!-- merge -->
				<DropdownMenuItem
					class="h-[28px] flex items-center cursor-pointer"
					on:click={async () => {
						await mergeBranch(branch.name);
					}}
				>
					<AddIcon class="h-3/4 aspect-square mr-[6px]" />
					<span>
						Merge <strong>{branch.name}</strong> into <strong>{$currentBranch.name}</strong>
					</span>
				</DropdownMenuItem>

				{#if branch.upstream === null}
					<!-- rebase -->
					<DropdownMenuItem
						class="h-[28px] flex items-center cursor-pointer"
						on:click={async () => {
							await rebaseBranch(branch.name);
						}}
					>
						<AddIcon class="h-3/4 aspect-square mr-[6px]" />
						<span>
							Rebase <strong>{$currentBranch.name}</strong> on <strong>{branch.name}</strong>
						</span>
					</DropdownMenuItem>
				{/if}
			{/if}
		</DropdownMenuContent>
	</DropdownMenu>
</div>
