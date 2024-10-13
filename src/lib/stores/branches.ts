import { getLocalBranches } from '@/integrated-backend/browse/branches';
import type { BranchInfo } from '@/integrated-backend/browse/branches/types';
import { derived, writable } from 'svelte/store';

export const branches = (() => {
	const store = writable<Array<BranchInfo>>([]);

	async function reload(repoPath: string) {
		try {
			const branches = (await getLocalBranches(repoPath)).sort((a, b) => {
				if (a.commitTime === b.commitTime) return a.name.localeCompare(b.name);
				else return b.commitTime - a.commitTime;
			});

			store.set(branches);
		} catch (error) {
			console.error(error);
		}
	}

	return {
		...store,
		reload
	};
})();

export const currentBranch = derived(branches, (value) => {
	const filtered = value.filter((value) => value.isHead === true);
	return filtered[0] as BranchInfo | undefined;
});
