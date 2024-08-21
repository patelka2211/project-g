import { invoke } from "@tauri-apps/api/tauri";

export async function checkForDotGitFolder(repoPath: string) {
  try {
    await invoke<void>("check_for_dot_git_folder", { repoPath });
  } catch {
    throw Error("Not a repository.");
  }
}

export async function getRemoteOrigin(repoPath: string) {
  try {
    return JSON.parse(
      await invoke<string>("get_remote_origin", { repoPath })
    ) as {
      fetch?: string;
      push?: string;
    };
  } catch {
    throw Error('Remote "origin" not found.');
  }
}

export async function checkOriginHead(repoPath: string) {
  try {
    await invoke<void>("check_origin_head", {
      repoPath,
    });
  } catch {
    throw Error('Branch "origin/HEAD" not found.');
  }
}

export async function verifyRepository(repoPath: string) {
  await checkForDotGitFolder(repoPath);

  let remoteOrigin = await getRemoteOrigin(repoPath);

  if (remoteOrigin.fetch === undefined || remoteOrigin.push === undefined) {
    throw Error(`Remote "origin" must have "fetch" and "push" URLs.`);
  }

  await checkOriginHead(repoPath);
}
