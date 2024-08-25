mod helpers;

use helpers::RepoInfo;

#[tauri::command]
pub fn add_repo(name: String, dir: String) -> core::result::Result<String, String> {
    match helpers::add_repo(name, dir) {
        Ok(id) => Ok(id),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn list_repos() -> core::result::Result<Vec<RepoInfo>, String> {
    match helpers::list_repos() {
        Ok(repo_list) => Ok(repo_list),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn delete_repo(repo_id: String) -> core::result::Result<RepoInfo, String> {
    match helpers::delete_repo(repo_id) {
        Ok(deleted_repo) => Ok(deleted_repo),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn reorder_repo(repo_id: String) -> core::result::Result<(), String> {
    match helpers::reorder_repo(repo_id) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
