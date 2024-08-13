// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod git_utilities;
mod shell;

use crate::git_utilities::{
    branches::{current_branch, get_branch_info, list_local_branches},
    does_repo_have_remote_origin, is_git_available, is_it_repository,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Initialization
            is_git_available,
            // Onboarding
            is_it_repository,
            does_repo_have_remote_origin,
            // Browse
            list_local_branches,
            get_branch_info,
            current_branch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
