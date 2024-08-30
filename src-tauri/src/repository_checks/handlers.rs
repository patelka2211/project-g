use std::{fs::metadata, path::Path};

use super::utilities;

#[tauri::command]
/// If provided repository path is not a repository then throws error.
pub fn assert_dot_git_folder(repo_path: String) -> core::result::Result<(), String> {
    let repo_path = Path::new(&repo_path).join(".git");

    match metadata(repo_path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err("Not a repository.".to_string())
            }
        }
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
/// Checks if the repository has origin remote.
///
/// ```ts
/// // returns
/// string | null
/// ```
pub fn get_origin_fetch_url(repo_path: String) -> core::result::Result<Option<String>, String> {
    match utilities::get_origin_fetch_url(repo_path) {
        Ok(remote_origin) => Ok(remote_origin),
        Err(error) => return Err(error.to_string()),
    }
}

#[tauri::command]
/// Checks if `origin/HEAD` is being tracked by the local repository.
/// If it is not tracked, the function queries the remote repository to identify the default branch
/// and sets `origin/HEAD` to track that default branch.
pub fn assert_origin_head(repo_path: String) -> core::result::Result<(), String> {
    match utilities::assert_origin_head(repo_path) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
