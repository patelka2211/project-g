import { invoke } from '@tauri-apps/api/core';

async function assertDotGitFolder(repoPath: string) {
	try {
		await invoke<void>('assert_dot_git_folder', { repoPath });
	} catch {
		throw Error('Not a repository.');
	}
}

async function assertOriginHead(repoPath: string) {
	try {
		await invoke<void>('assert_origin_head', {
			repoPath
		});
	} catch {
		throw Error('Branch "origin/HEAD" not found.');
	}
}

async function getOriginFetchUrl(repoPath: string) {
	try {
		return await invoke<string | null>('get_origin_fetch_url', { repoPath });
	} catch {
		throw Error('Remote "origin" not found.');
	}
}

export async function verifyRepository(repoPath: string) {
	await assertDotGitFolder(repoPath);

	const originFetchUrl = await getOriginFetchUrl(repoPath);

	if (originFetchUrl === null) {
		throw Error(`Remote "origin" must have "fetch" URL.`);
	}

	await assertOriginHead(repoPath);
}
