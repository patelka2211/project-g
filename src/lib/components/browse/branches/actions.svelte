<script lang="ts">
	import CloudUploadIcon from '@/codicons/cloud-upload-icon.svelte';
	import FetchIcon from '@/codicons/fetch-icon.svelte';
	import PullIcon from '@/codicons/pull-icon.svelte';
	import PushIcon from '@/codicons/push-icon.svelte';
	import TargetIcon from '@/codicons/target-icon.svelte';
	import TrashIcon from '@/codicons/trash-icon.svelte';
	import {
		deleteBranch,
		fetchBranch,
		pullBranch,
		pushBranch,
		switchBranch
	} from '@/integrated-backend/browse/branches/actions';
	import type { BranchInfo, BranchType } from '@/integrated-backend/browse/branches/types';
	import { repoPath } from '@/stores/repo';
	import { toast } from 'svelte-sonner';

	export let branch: BranchInfo;

	async function switchOnClick() {
		if (branch.isHead === true) return;

		if ($repoPath) {
			try {
				await switchBranch($repoPath, branch.name);
			} catch (error) {
				if (typeof error === 'string') toast.error(error);
			}
		}
	}

	async function fetchOnClick() {
		if (branch.upstream === null) return;

		if ($repoPath) {
			const [remote, ...other] = branch.upstream.split('/');
			try {
				await fetchBranch($repoPath, { name: other.join('/'), remote });
			} catch (error) {
				if (typeof error === 'string') toast.error(error);
			}
		}
	}

	async function pullOnClick() {
		if (branch.upstream === null) return;

		if ($repoPath) {
			const [remote, ...other] = branch.upstream.split('/');
			try {
				await pullBranch($repoPath, { name: other.join('/'), remote });
			} catch (error) {
				if (typeof error === 'string') toast.error(error);
			}
		}
	}

	async function pushOnClick() {
		if ($repoPath) {
			let branchType: BranchType;
			const [remote, name] = branch.upstream
				? (() => {
						const [remote, ...other] = branch.upstream.split('/');

						branchType = 'Remote';

						return [remote, other.join('/')];
					})()
				: (() => {
						branchType = 'Local';

						return ['origin', branch.name];
					})();

			try {
				await pushBranch($repoPath, { name, remote }, branchType);
			} catch (error) {
				if (typeof error === 'string') toast.error(error);
			}
		}
	}

	async function deleteOnClick() {
		if (branch.isHead) return;

		if ($repoPath) {
			try {
				await deleteBranch($repoPath, branch.name, branch.upstream, false);
			} catch (error) {
				if (typeof error === 'string') toast.error(error);
			}
		}
	}
</script>

<div class="w-full flex items-center justify-between gap-[6px]">
	<!-- target branch button -->
	<button
		class={`action-button ${branch.isHead ? 'bg-accent-foreground text-accent' : 'action_common'}`}
		title={`${branch.isHead ? 'Already on' : 'Switch to'} ${branch.name}`}
		on:click={switchOnClick}
	>
		<TargetIcon class="aspect-square h-full" />
	</button>

	<!-- fetch -->
	<button
		class={`action-button action_common${branch.upstream === null ? ' action_disabled' : ''}`}
		title={branch.upstream !== null ? `Fetch from ${branch.upstream}` : undefined}
		on:click={fetchOnClick}
	>
		<FetchIcon class="aspect-square h-full" />
		<span>Fetch</span>
	</button>

	<!-- pull -->
	<button
		class={`action-button action_common${branch.upstream === null ? ' action_disabled' : ''}`}
		title={branch.upstream !== null ? `Pull from ${branch.upstream}` : undefined}
		on:click={pullOnClick}
	>
		<PullIcon class="aspect-square h-full" />
		<span>Pull</span>
	</button>

	<!-- publish/push -->
	<button
		class="action-button action_common"
		title={branch.upstream !== null
			? `Push to ${branch.upstream}`
			: `Publish as origin/${branch.name}`}
		on:click={pushOnClick}
	>
		{#if branch.upstream !== null}
			<PushIcon class="aspect-square h-full" />
			<span>Push</span>
		{:else}
			<CloudUploadIcon class="aspect-square h-full" />
			<span>Publish</span>
		{/if}
	</button>

	<!-- delete -->
	<button
		class={`action-button action_delete${branch.isHead === true ? ' action_disabled' : ''}`}
		on:click={deleteOnClick}
	>
		<TrashIcon class="aspect-square h-full" />
		<span>Delete</span>
	</button>
</div>

<style lang="scss">
	.action-button {
		@apply h-[28px];
		@apply p-[6px];
		@apply border rounded-[6px];
		@apply flex items-center gap-[6px];
		@apply shadow-sm hover:shadow-inner;

		&.action_common {
			@apply hover:bg-accent hover:text-accent-foreground;
		}

		&.action_delete {
			@apply hover:bg-destructive hover:text-destructive-foreground;
		}

		&.action_common,
		&.action_delete {
			&.action_disabled {
				@apply text-foreground/50;
				@apply cursor-not-allowed;
				@apply shadow-none hover:shadow-none;
				@apply hover:bg-background hover:text-foreground/50;
			}
		}
	}
</style>
