// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod _backend_specific;
mod browse;
mod error;
mod home;
mod init;

use crate::{
    browse::branches::{
        actions::{delete_branch, fetch_branch, pull_branch, push_branch, switch_branch},
        get_local_branches,
    },
    home::repositories::{
        checks::{assert_dot_git_folder, assert_origin_head, get_origin_fetch_url},
        saved::{add_repo, list_repos, remove_repo, reorder_repo},
    },
    init::is_git_available,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .invoke_handler(tauri::generate_handler![
            // init
            is_git_available,
            // home:repositories:checks
            assert_dot_git_folder,
            assert_origin_head,
            get_origin_fetch_url,
            // home:repositories:store
            add_repo,
            list_repos,
            remove_repo,
            reorder_repo,
            // browse:branches
            get_local_branches,
            // browse:branches:actions
            delete_branch,
            fetch_branch,
            pull_branch,
            push_branch,
            switch_branch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
