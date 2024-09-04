import { invoke } from "@tauri-apps/api/tauri";

export async function isGitAvailable() {
  return await invoke<boolean>("is_git_available");
}
