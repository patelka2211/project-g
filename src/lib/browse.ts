import { invoke } from "@tauri-apps/api/tauri";

export interface Branch {
  name: string;
  upstream?: string;
  isHead: boolean;
  commitTime: number;
}

export async function getLocalBranches(repoPath: String) {
  let branches = await invoke<string>("get_local_branches", {
    repoPath,
  });

  return JSON.parse(branches) as Array<Branch>;
}
