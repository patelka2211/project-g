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

export async function getRemoteOrigin(repoPath: string) {
  try {
    return JSON.parse(
      await invoke<string>("get_remote_origin", { repoPath })
    ) as {
      fetch?: string;
      push?: string;
    };
  } catch (error) {
    console.log(error);
  }
}
