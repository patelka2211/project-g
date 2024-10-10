mod _backend_specific;
mod browse;
mod error;
mod home;
mod init;

use crate::{
    browse::branches::{
        actions::{delete_branch, fetch_branch, pull_branch, push_branch, switch_branch},
        commit_history::get_parent_commits,
        get_local_branches,
        name_and_menu::create_branch,
    },
    home::repositories::{
        checks::{assert_dot_git_folder, assert_origin_head, get_origin_fetch_url},
        saved::{add_repo, list_repos, remove_repo, reorder_repo},
    },
    init::is_git_available,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
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
            switch_branch,
            // browse:branches:commit_history
            get_parent_commits,
            // browse:branches:name_and_menu
            create_branch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
