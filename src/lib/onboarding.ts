import { invoke } from "@tauri-apps/api/tauri";

export async function isItRepository(repoPath: string) {
  try {
    return JSON.parse(
      await invoke<string>("is_it_repository", { repoPath })
    ) as boolean;
  } catch (error) {
    console.log(error);
  }
}

export async function doesRepoHasRemoteOrigin(repoPath: string) {
  try {
    return JSON.parse(
      await invoke<string>("does_repo_has_remote_origin", { repoPath })
    ) as {
      fetch?: string;
      push?: string;
    };
  } catch (error) {
    console.log(error);
  }
}
