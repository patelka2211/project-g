import { invoke } from "@tauri-apps/api/tauri";

export async function getLocalBranches(repoPath: String) {
  try {
    let branches = await invoke<string>("local_branches", {
      repoPath,
    });

    return JSON.parse(branches) as Array<{
      name: string;
      upstream?: string;
      isHead: boolean;
      commitTime: number;
    }>;
  } catch (error) {
    console.log(error);
  }
}
