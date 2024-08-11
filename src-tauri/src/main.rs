// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod fs;
mod git_utilities;
mod shell;

use crate::git_utilities::{
    branches::{get_branch_info, local_branches},
    is_git_available, is_it_repository,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_git_available,
            is_it_repository,
            local_branches,
            get_branch_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
