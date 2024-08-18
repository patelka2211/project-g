import { invoke } from "@tauri-apps/api/tauri";

export async function isGitAvailable() {
  return JSON.parse(await invoke<string>("is_git_available")) as boolean;
}
