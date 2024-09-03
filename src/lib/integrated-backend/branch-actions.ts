import { invoke } from "@tauri-apps/api/tauri";

export async function switchBranch(repoPath: string, branchName: string) {
  await invoke<void>("switch_branch", { repoPath, branchName });
}
