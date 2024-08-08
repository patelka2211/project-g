// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::Path, process::Command};

#[tauri::command]
fn is_git_available() -> bool {
    let git_check = Command::new("git").arg("--version").output();

    match git_check {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
fn is_it_repository(path: &str) -> bool {
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_git_available, is_it_repository])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
