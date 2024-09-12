import { invoke } from "@tauri-apps/api/tauri";
import type { BranchType } from "./types";

export interface AuthorInfo {
  name: string | null;
  email: string | null;
}
type CommitType = "Merged" | "Normal" | "First";

export interface CommitInfo {
  hash: string;
  msg: string;
  author: AuthorInfo;
  commitType: CommitType;
}

interface BranchData {
  name: String;
  branch_type: BranchType;
}

export async function getParentCommits(
  repoPath: string,
  branchData: BranchData,
  /**
   * type: `u8`
   */
  noOfCommits: number
) {
  if (noOfCommits < 0 || noOfCommits > 255) {
    throw Error("Number of commits can be from 0 to 255 only.");
  }

  return await invoke<Array<CommitInfo>>("get_parent_commits", {
    repoPath,
    branchData,
    noOfCommits,
  });
}
