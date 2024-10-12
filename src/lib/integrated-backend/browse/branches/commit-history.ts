import { repoPath as repoPathStore } from '@/stores/repo';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';
import type { ParentCommits } from './types';

export async function getParentCommits(
	repoPath: string,
	commitHash: string,
	/**
	 * type: `u8`
	 */
	noOfCommits: number
) {
	if (noOfCommits < 0 || noOfCommits > 255) {
		throw Error('Number of commits can be from 0 to 255 only.');
	}

	return await invoke<ParentCommits>('get_parent_commits', {
		repoPath,
		commitHash,
		noOfCommits
	});
}

export async function cherryPickCommit(commitHash: string) {
	const repoPath = repoPathStore.get();

	if (repoPath !== null) {
		try {
			const output = await invoke<string>('cherry_pick_commit', {
				repoPath,
				commitHash
			});
			toast.success(output);
		} catch (error) {
			toast.error(error as string);
		}
	}
}

export async function revertCommit(commitHash: string) {
	const repoPath = repoPathStore.get();

	if (repoPath !== null) {
		try {
			const output = await invoke<string>('revert_commit', {
				repoPath,
				commitHash
			});

			toast.success(output);
		} catch (error) {
			toast.error(error as string);
		}
	}
}
