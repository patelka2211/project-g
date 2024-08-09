use std::{fs, path::Path, process::Command};
use tauri::command as tauri_command;

pub mod branches;

#[tauri_command]
pub fn is_git_available() -> bool {
    let git_check = Command::new("git").arg("--version").output();

    match git_check {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri_command]
pub fn is_it_repository(path: &str) -> bool {
    let path = Path::new(path).join(".git");

    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                // The ".git" folder exists in the given path.
                true
            } else {
                // The path exists but ".git" is not a folder.
                false
            }
        }
        // The ".git" folder or the given path does not exist.
        Err(_) => false,
    }
}
