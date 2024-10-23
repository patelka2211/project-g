use crate::_backend_specific::git::executor::run_command;

#[tauri::command]
pub fn prune_repository(repo_path: String) -> Result<(), String> {
    match run_command(&repo_path, "prune", None) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
