use std::process::Command;

#[tauri::command]
/// returns `true` if git is available, `false` otherwise
pub fn is_git_available() -> core::result::Result<bool, String> {
    match Command::new("git").arg("--version").output() {
        Ok(output) => Ok(output.status.success()),
        Err(error) => return Err(error.to_string()),
    }
}
