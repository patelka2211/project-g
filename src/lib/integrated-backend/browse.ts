import { invoke } from "@tauri-apps/api/tauri";

export interface BranchInfo {
  name: string;
  upstream?: string;
  isHead: boolean;
  commitTime: number;
}

export async function getLocalBranches(repoPath: String) {
  return await invoke<Array<BranchInfo>>("get_local_branches", {
    repoPath,
  });
}
