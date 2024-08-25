import { invoke } from "@tauri-apps/api/tauri";

export interface RepoInfo {
  id: string;
  name: string;
  dir: string;
}

export async function addRepo(dir: string, name: string) {
  try {
    return await invoke<string>("add_repo", {
      dir,
      name,
    });
  } catch (error) {
    console.log(error);

    throw Error("Cannot add repository.");
  }
}

export async function listRepos() {
  try {
    return await invoke<Array<RepoInfo>>("list_repos");
  } catch (error) {
    console.log(error);
    throw Error("Cannot read saved repositories.");
  }
}

export async function deleteRepo(repoId: string) {
  try {
    return await invoke<RepoInfo>("delete_repo", { repoId });
  } catch (error) {
    console.log(error);
    throw Error("Cannot delete repository.");
  }
}

async function reorderRepo(repoId: string) {
  try {
    return await invoke<void>("reorder_repo", { repoId });
  } catch (error) {
    console.log(error);
    throw Error("Cannot reorder repository.");
  }
}
