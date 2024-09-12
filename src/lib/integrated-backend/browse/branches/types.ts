export type BranchType = "Local" | "Remote";

export type CommitType = "Merged" | "Normal" | "First";

export interface RemoteBranchInfo {
  remote: string;
  name: string;
}

export interface AuthorInfo {
  name: string | null;
  email: string | null;
}

export interface CommitInfo {
  hash: string;
  msg: string;
  author: AuthorInfo;
  commitType: CommitType;
}

export interface BranchData {
  name: String;
  branch_type: BranchType;
}

export interface BranchInfo {
  name: string;
  upstream: string | null;
  isHead: boolean;
  commitTime: number;
}
