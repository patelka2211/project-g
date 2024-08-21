use super::utilities;
use std::{fs::metadata, path::Path};

#[tauri::command]
/// If provided repository path is not a repository then throws error.
pub fn check_for_dot_git_folder(repo_path: String) -> core::result::Result<(), String> {
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
/// ```ts
/// // returns
/// {
///     fetch: string | undefined,
///     push: string | undefined,
/// }
/// ```
pub fn get_remote_origin(repo_path: String) -> core::result::Result<String, String> {
    let remote_origin = match utilities::get_remote_origin(repo_path) {
        Ok(origin) => origin,
        Err(error) => return Err(error.to_string()),
    };

    match serde_json::to_string(&remote_origin) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
/// Checks if `origin/HEAD` is being tracked by the local repository.
/// If it is not tracked, the function queries the remote repository to identify the default branch
/// and sets `origin/HEAD` to track that default branch.
pub fn check_origin_head(repo_path: String) -> core::result::Result<(), String> {
    match utilities::check_origin_head(repo_path) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
