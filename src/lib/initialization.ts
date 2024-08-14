import { invoke } from "@tauri-apps/api/tauri";

export async function isGitAvailable() {
  try {
    return JSON.parse(await invoke<string>("is_git_available")) as boolean;
  } catch (error) {
    console.log(error);
  }
}
