import { invoke } from "@tauri-apps/api/tauri";

/**
 *
 * @param dir
 * @param name
 * @returns id
 */
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

export interface RepoInfo {
  name: string;
  dir: string;
}

export async function getRepos() {
  try {
    return await invoke<{ [id: string]: RepoInfo }>("get_repos");
  } catch (error) {
    console.log(error);
    throw Error("Cannot read saved repositories.");
  }
}
