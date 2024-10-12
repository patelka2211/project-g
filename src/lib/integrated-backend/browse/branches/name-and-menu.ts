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
