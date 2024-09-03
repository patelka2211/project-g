// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod branch_actions;
mod browse;
mod error;
mod git_executor;
mod git_utilities;
mod init;
mod repository_checks;
mod repository_store;

use crate::{
    branch_actions::handlers::{
        delete_branch, fetch_branch, pull_branch, push_branch, switch_branch,
    },
    browse::handlers::get_local_branches,
    init::handlers::is_git_available,
    repository_checks::handlers::{
        assert_dot_git_folder, assert_origin_head, get_origin_fetch_url,
    },
    repository_store::handlers::{add_repo, list_repos, remove_repo, reorder_repo},
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .invoke_handler(tauri::generate_handler![
            // init
            is_git_available,
            // repository_checks
            assert_dot_git_folder,
            assert_origin_head,
            get_origin_fetch_url,
            // repository_store
            add_repo,
            list_repos,
            remove_repo,
            reorder_repo,
            // browse
            get_local_branches,
            // branch_actions
            delete_branch,
            fetch_branch,
            pull_branch,
            push_branch,
            switch_branch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
