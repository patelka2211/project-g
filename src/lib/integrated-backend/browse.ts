import { invoke } from "@tauri-apps/api/tauri";

export interface Branch {
  name: string;
  upstream?: string;
  isHead: boolean;
  commitTime: number;
}

export async function getLocalBranches(repoPath: String) {
  return await invoke<Array<Branch>>("get_local_branches", {
    repoPath,
  });
}
