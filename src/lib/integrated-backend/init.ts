import { invoke } from '@tauri-apps/api/core';

export async function isGitAvailable() {
	return await invoke<boolean>('is_git_available');
}
