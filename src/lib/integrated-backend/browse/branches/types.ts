export type BranchType = 'Local' | 'Remote';

export interface RemoteBranchInfo {
	remote: string;
	name: string;
}

interface AuthorInfo {
	name: string | null;
	email: string | null;
}

interface CommitInfo {
	hash: string;
	msg: string;
	author: AuthorInfo;
}

export interface ParentCommits {
	list: Array<CommitInfo>;
	endOfCommits: boolean;
}

export interface BranchInfo {
	name: string;
	upstream: string | null;
	isHead: boolean;
	commitTime: number;
	commitHash: string;
}
