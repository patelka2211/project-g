mod utilities {
    use git2::Repository;

    use std::{fs::File, io::Write, path::Path};

    use crate::{_backend_specific::git::executor::run_command, error::Result};

    pub fn get_origin_fetch_url(repo_path: String) -> Result<Option<String>> {
        let repo = Repository::open(repo_path)?;

        let must_have_remote = "origin";

        let remote_origin = repo.find_remote(&must_have_remote)?;

        Ok(match remote_origin.url() {
            Some(fetch_url) => Some(fetch_url.to_string()),
            None => None,
        })
    }

    pub fn assert_origin_head(repo_path: String) -> Result<()> {
        let remote_name = "origin";

        let remote_head_path = format!("{}/.git/refs/remotes/{}/HEAD", repo_path, remote_name);
        let remote_head_path = Path::new(&remote_head_path);

        if remote_head_path.exists() && remote_head_path.is_file() {
            return Ok(());
        }

        let stdout = match run_command(&repo_path, "remote", &vec!["show", "origin"]) {
            Ok(output) => output,
            Err(_) => return Err("Cannot find \"origin/HEAD\"".into()),
        };
        let mut branch_name: Option<&str> = None;
        let identifier = "HEAD branch:";

        for line in stdout.lines() {
            match line.find(&identifier) {
                Some(position) => {
                    branch_name = Some(&line[position + identifier.len()..].trim());
                    break;
                }
                None => continue,
            }
        }

        let branch_name = match branch_name {
            Some(branch_name) => branch_name,
            None => return Err("Cannot find default branch of remote origin.".into()),
        };

        let mut origin_head_file = File::create(remote_head_path)?;

        origin_head_file
            .write_all(format!("ref: refs/remotes/origin/{}", branch_name).as_bytes())?;

        Ok(())
    }
}

use std::{fs::metadata, path::Path};

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
