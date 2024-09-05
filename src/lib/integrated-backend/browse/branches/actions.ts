import { invoke } from "@tauri-apps/api/tauri";

interface RemoteBranchInfo {
  remote: string;
  name: string;
}

export type BranchType = "Local" | "Remote";

export async function switchBranch(repoPath: string, branchName: string) {
  return await invoke<void>("switch_branch", { repoPath, branchName });
}

export async function fetchBranch(
  repoPath: string,
  remoteBranch: RemoteBranchInfo
) {
  return await invoke<void>("fetch_branch", { repoPath, remoteBranch });
}

export async function pullBranch(
  repoPath: string,
  remoteBranch: RemoteBranchInfo
) {
  return await invoke<void>("pull_branch", { repoPath, remoteBranch });
}

export async function pushBranch(
  repoPath: string,
  remoteBranch: RemoteBranchInfo,
  branchType: BranchType
) {
  return await invoke<void>("push_branch", {
    repoPath,
    remoteBranch,
    branchType,
  });
}

export async function deleteBranch(
  repoPath: string,
  localBranch: string,
  remoteBranch: string | null,
  force: boolean | null
) {
  return await invoke<void>("delete_branch", {
    repoPath,
    localBranch,
    remoteBranch,
    force,
  });
}
