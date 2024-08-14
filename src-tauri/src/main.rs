// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod error;
mod browse;
mod initialization;
mod onboarding;

use crate::browse::local_branches;
use crate::initialization::is_git_available;
use crate::onboarding::{does_repo_has_remote_origin, is_it_repository};

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
