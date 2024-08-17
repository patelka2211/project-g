use super::utilities::get_remote_origin;
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
pub fn does_repo_has_remote_origin(repo_path: String) -> core::result::Result<String, String> {
    let remote_origin = match get_remote_origin(repo_path) {
        Ok(origin) => origin,
        Err(error) => return Err(error.to_string()),
    };

    let output = match serde_json::to_string(&remote_origin) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    Ok(output)
}
