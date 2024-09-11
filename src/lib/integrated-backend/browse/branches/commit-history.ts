import { invoke } from "@tauri-apps/api/tauri";

export interface AuthorInfo {
  name: string | null;
  email: string | null;
}

export interface CommitInfo {
  hash: string;
  msg: string;
  author: AuthorInfo;
}

export async function getParentCommits(
  repoPath: string,
  commitHash: string,
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
    commitHash,
    noOfCommits,
  });
}
