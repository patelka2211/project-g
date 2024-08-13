use std::{fs, path::Path, process::Command};
use tauri::command as tauri_command;

pub mod branches;

#[tauri_command]
pub fn is_git_available() -> core::result::Result<bool, String> {
    match Command::new("git").arg("--version").output() {
        Ok(output) => Ok(output.status.success()),
        Err(error) => return Err(error.to_string()),
    }
}

#[tauri_command]
pub fn is_it_repository(path: String) -> core::result::Result<bool, String> {
    let path = Path::new(&path).join(".git");

    match fs::metadata(path) {
        Ok(metadata) => Ok(metadata.is_dir()),
        Err(error) => Err(error.to_string()),
    }
}
