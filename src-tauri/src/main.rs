// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browse;
mod error;
mod initialization;
mod onboarding;

use crate::browse::handlers::local_branches;
use crate::initialization::handlers::is_git_available;
use crate::onboarding::handlers::{does_repo_has_remote_origin, is_it_repository};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Initialization
            is_git_available,
            // Onboarding
            is_it_repository,
            does_repo_has_remote_origin,
            // Browse
            local_branches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
