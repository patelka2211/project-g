import { repoPath as repoPathStore } from '@/stores/repo';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';

export async function createBranch(startPoint: string | null) {
	const repoPath = repoPathStore.get();

	if (repoPath !== null) {
		try {
			const newBranch = await invoke<string>('create_branch', {
				repoPath,
				startPoint
			});

			toast.success(`Branch "${newBranch}" created.`);
		} catch (error) {
			toast.error(error as string);
		}
	}
}

export async function mergeBranch(branchName: string) {
	const repoPath = repoPathStore.get();

	if (repoPath !== null) {
		try {
			const output = await invoke<string>('merge_branch', {
				repoPath,
				branchName
			});

			toast.success(output);
		} catch (error) {
			toast.error(error as string);
		}
	}
}

export async function rebaseBranch(branchName: string) {
	const repoPath = repoPathStore.get();

	if (repoPath !== null) {
		try {
			const output = await invoke<string>('rebase_branch', {
				repoPath,
				branchName
			});

			toast.success(output);
		} catch (error) {
			toast.error(error as string);
		}
	}
}
