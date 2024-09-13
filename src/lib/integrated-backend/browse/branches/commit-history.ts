import { invoke } from "@tauri-apps/api/tauri";
import type { BranchData, ParentCommits } from "./types";

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

  return await invoke<ParentCommits>("get_parent_commits", {
    repoPath,
    branchData,
    noOfCommits,
  });
}
