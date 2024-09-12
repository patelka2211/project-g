import { invoke } from "@tauri-apps/api/tauri";
import type { BranchInfo } from "./types";

export async function getLocalBranches(repoPath: String) {
  return await invoke<Array<BranchInfo>>("get_local_branches", {
    repoPath,
  });
}
