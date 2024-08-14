use git2::Repository;
use serde::Serialize;
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

#[derive(Serialize)]
struct RemoteOrigin {
    fetch: Option<String>,
    push: Option<String>,
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
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(error) => return Err(error.to_string()),
    };

    let must_have_remote = "origin";

    let remote_origin = match repo.find_remote(&must_have_remote) {
        Ok(origin) => origin,
        Err(error) => return Err(error.to_string()),
    };

    let fetch = match remote_origin.url() {
        Some(url) => Some(url.to_string()),
        None => None,
    };

    let push = match remote_origin.pushurl() {
        Some(url) => Some(url.to_string()),
        None => None,
    };

    let output = match serde_json::to_string(&RemoteOrigin { fetch, push }) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    Ok(output)
}
