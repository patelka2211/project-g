import { invoke } from "@tauri-apps/api/tauri";

export async function getLocalBranches(repoPath: String) {
  try {
    let branches = await invoke<string>("local_branches", {
      repoPath,
    });

    return JSON.parse(branches) as Array<string>;
  } catch (error) {
    console.log(error);
  }
}

async function getBranchInfo(repoPath: string, branchName: string) {
  try {
    let branchInfo = await invoke<string>("get_branch_info", {
      repoPath,
      branchName,
    });

    return JSON.parse(branchInfo) as {
      pointsAt: string;
      updatedAt: string;
      msg: string;
    };
  } catch (error) {
    console.log(error);
  }
}

async function getCurrentBranch(repoPath: string) {
  try {
    let currentBranch = await invoke<string>("current_branch", { repoPath });

    return currentBranch as string;
  } catch (error) {
    console.log(error);
  }
}
