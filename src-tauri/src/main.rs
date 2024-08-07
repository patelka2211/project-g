// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn is_git_available() -> bool {
    let git_check = Command::new("git").arg("--version").output();

    match git_check {
        Ok(_output) => true,
        Err(_) => false,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![is_git_available])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
