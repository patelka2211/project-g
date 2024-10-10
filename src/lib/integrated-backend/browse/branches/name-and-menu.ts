import { invoke } from '@tauri-apps/api/core';

export async function createBranch(repoPath: string, startPoint: string) {
	return await invoke<string>('create_branch', {
		repoPath,
		startPoint
	});
}
