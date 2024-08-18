use super::utilities;
use std::{fs::metadata, path::Path};

#[tauri::command]
/// returns `true` if give path is git repository, `false` otherwise.
pub fn is_it_repository(repo_path: String) -> core::result::Result<bool, String> {
    let repo_path = Path::new(&repo_path).join(".git");

    match metadata(repo_path) {
        Ok(metadata) => Ok(metadata.is_dir()),
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

    let output = match serde_json::to_string(&remote_origin) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    Ok(output)
}

#[tauri::command]
/// Checks if `origin/HEAD` is being tracked by the local repository.
/// If it is not tracked, the function queries the remote repository to identify the default branch
/// and sets `origin/HEAD` to track this default branch.
///
/// # Returns
/// ```ts
/// "origin/HEAD" as const
/// ```
pub fn get_origin_head(repo_path: String) -> core::result::Result<String, String> {
    match utilities::get_origin_head(repo_path) {
        Ok(origin_head) => Ok(origin_head),
        Err(error) => Err(error.to_string()),
    }
}
