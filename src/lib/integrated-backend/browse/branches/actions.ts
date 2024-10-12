import { repoPath as repoPathStore } from '@/stores/repo';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';
import type { BranchType, RemoteBranchInfo } from './types';

export async function switchBranch(branchName: string) {
	const repoPath = repoPathStore.get();
	if (repoPath !== null) {
		try {
			await invoke<void>('switch_branch', { repoPath, branchName });
		} catch (error) {
			if (typeof error === 'string') toast.error(error);
		}
	}
}

export async function fetchBranch(repoPath: string, remoteBranch: RemoteBranchInfo) {
	return await invoke<void>('fetch_branch', { repoPath, remoteBranch });
}

export async function pullBranch(repoPath: string, remoteBranch: RemoteBranchInfo) {
	return await invoke<void>('pull_branch', { repoPath, remoteBranch });
}

export async function pushBranch(
	repoPath: string,
	remoteBranch: RemoteBranchInfo,
	branchType: BranchType
) {
	return await invoke<void>('push_branch', {
		repoPath,
		remoteBranch,
		branchType
	});
}

export async function deleteBranch(
	repoPath: string,
	localBranch: string,
	remoteBranch: string | null,
	force: boolean | null
) {
	return await invoke<void>('delete_branch', {
		repoPath,
		localBranch,
		remoteBranch,
		force
	});
}
