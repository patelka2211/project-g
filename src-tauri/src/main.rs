// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browse;
mod error;
mod initialization;
mod repository_checks;

use browse::handlers::get_local_branches;
use initialization::handlers::is_git_available;
use repository_checks::handlers::{check_for_dot_git_folder, check_origin_head, get_remote_origin};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // initialization
            is_git_available,
            // repository_checks
            check_origin_head,
            get_remote_origin,
            check_for_dot_git_folder,
            // browse
            get_local_branches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
