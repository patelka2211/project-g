// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browse;
mod error;
mod initialization;
mod onboarding;

use browse::handlers::local_branches;
use initialization::handlers::is_git_available;
use onboarding::handlers::{get_origin_head, get_remote_origin, is_it_repository};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Initialization
            is_git_available,
            // Onboarding
            get_origin_head,
            get_remote_origin,
            is_it_repository,
            // Browse
            local_branches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
