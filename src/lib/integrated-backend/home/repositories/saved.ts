import { APP_ENV } from "@/env";
import { appDataDir, documentDir } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api/tauri";
import type { RepoInfo } from "./types";

async function getAppDataDir() {
  return APP_ENV === "production"
    ? await appDataDir()
    : `${await documentDir()}project-g_dev_data/`;
}

export async function addRepo(dir: string, name: string) {
  try {
    return await invoke<string>("add_repo", {
      dir,
      name,
      appDataDir: await getAppDataDir(),
    });
  } catch (error) {
    console.log(error);

    throw Error("Cannot add repository.");
  }
}

export async function listRepos() {
  try {
    return await invoke<Array<RepoInfo>>("list_repos", {
      appDataDir: await getAppDataDir(),
    });
  } catch (error) {
    console.log(error);
    throw Error("Cannot read saved repositories.");
  }
}

export async function removeRepo(repoId: string) {
  try {
    return await invoke<RepoInfo>("remove_repo", {
      repoId,
      appDataDir: await getAppDataDir(),
    });
  } catch (error) {
    console.log(error);
    throw Error("Cannot delete repository.");
  }
}

export async function reorderRepo(repoId: string) {
  try {
    return await invoke<void>("reorder_repo", {
      repoId,
      appDataDir: await getAppDataDir(),
    });
  } catch (error) {
    console.log(error);
    throw Error("Cannot reorder repository.");
  }
}
