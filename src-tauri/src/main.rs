// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod browse;
mod error;
mod init;
mod repository_checks;
mod saved_repos;

use browse::handlers::get_local_branches;
use init::handlers::is_git_available;
use repository_checks::handlers::{
    assert_dot_git_folder, assert_origin_head, get_origin_fetch_url,
};
use saved_repos::{add_repo, delete_repo, get_repos};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // init
            is_git_available,
            // repository_checks
            assert_dot_git_folder,
            assert_origin_head,
            get_origin_fetch_url,
            // saved_repos
            add_repo,
            delete_repo,
            get_repos,
            // browse
            get_local_branches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
