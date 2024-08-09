use tauri::command as tauri_command;

#[tauri_command]
pub fn local_branches(repo_path: String) {}
