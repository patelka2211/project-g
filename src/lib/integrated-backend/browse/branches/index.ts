import { invoke } from '@tauri-apps/api/core';
import type { BranchInfo } from './types';

export async function getLocalBranches(repoPath: string) {
	return await invoke<Array<BranchInfo>>('get_local_branches', {
		repoPath
	});
}
