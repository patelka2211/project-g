import { invoke } from "@tauri-apps/api/tauri";

export async function isItRepository(repoPath: string) {
  return JSON.parse(
    await invoke<string>("is_it_repository", { repoPath })
  ) as boolean;
}

export async function getRemoteOrigin(repoPath: string) {
  return JSON.parse(
    await invoke<string>("get_remote_origin", { repoPath })
  ) as {
    fetch?: string;
    push?: string;
  };
}

export async function getOriginHead(repoPath: string) {
  return (await invoke<string>("get_origin_head", {
    repoPath,
  })) as "origin/HEAD";
}
