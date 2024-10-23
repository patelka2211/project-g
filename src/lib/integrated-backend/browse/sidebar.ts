import { repoPath as repoPathStore } from '@/stores/repo';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'svelte-sonner';
import { get } from 'svelte/store';

export async function pruneRepository() {
	const repoPath = get(repoPathStore);

	if (repoPath !== null) {
		try {
			await invoke<void>('prune_repository', {
				repoPath
			});
			toast.success('Repository pruned successfully.');
		} catch (error) {
			toast.error(typeof error === 'string' ? error : (error as Error).toString());
		}
	}
}
