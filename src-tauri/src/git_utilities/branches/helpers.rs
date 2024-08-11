use crate::error::Result;
use serde::Serialize;
use std::{
    fs::read_dir,
    os::unix::fs::MetadataExt,
    path::Path,
    process::{self, Output},
};

#[derive(Serialize)]
pub struct BranchInfo {
    pub points_at: String,
    pub updated_at: String,
}

pub fn get_commit_hash_and_time(repo_path: &String, branch_name: &String) -> Result<BranchInfo> {
    let Output {
        status,
        stdout,
        stderr,
    } = process::Command::new("git")
        .arg("log")
        .arg("-1")
        // .arg("--format=%H$%ct")
        .arg("--format=%H$%cI")
        .arg(format!("refs/heads/{}", branch_name))
        .current_dir(repo_path)
        .output()?;

    if status.success() == false {
        let stderr = String::from_utf8(stderr)?;
        return Err(stderr.into());
    }

    let stdout = String::from_utf8(stdout)?;
    let stdout = stdout.replace("\n", "");

    let splitted: Vec<&str> = stdout.split('$').collect();

    let points_at = (match splitted.get(0) {
        Some(value) => value,
        None => {
            return Err("Cannot found branch pointer.".into());
        }
    })
    .to_string();

    let updated_at = (match splitted.get(1) {
        Some(value) => value,
        None => {
            return Err("Cannot found last updated time.".into());
        }
    })
    .to_string();

    Ok(BranchInfo {
        points_at,
        updated_at,
    })
}

pub fn is_valid_branch(path: &Path) -> Result<bool> {
    let file_metadata = std::fs::metadata(path)?;

    if file_metadata.size() == 41 {
        return Ok(true);
    }

    Ok(false)
}

pub struct FolderElements {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}

pub fn get_folder_elements(folder_path: &Path) -> Result<FolderElements> {
    // merge list_files and list_folders functions if they are not used in this function.
    Ok(FolderElements {
        files: list_files(folder_path)?,
        folders: list_folders(folder_path)?,
    })
}

fn list_folders(folder_path: &Path) -> Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();

    for entry in read_dir(folder_path)? {
        let path = entry?.path();

        if path.is_dir() {
            let folder_name = match path.file_name() {
                Some(os_str) => match os_str.to_str() {
                    Some(str) => Some(str.to_string()),
                    _ => None,
                },
                _ => None,
            };

            match folder_name {
                Some(file_name) => {
                    files.push(file_name);
                }
                _ => {}
            }
        }
    }

    Ok(files)
}

fn list_files(folder_path: &Path) -> Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();

    for entry in read_dir(folder_path)? {
        let path = entry?.path();

        if path.is_file() {
            let file_name = match path.file_name() {
                Some(os_str) => match os_str.to_str() {
                    Some(str) => Some(str.to_string()),
                    _ => None,
                },
                _ => None,
            };

            match file_name {
                Some(file_name) => {
                    files.push(file_name);
                }
                _ => {}
            }
        }
    }

    Ok(files)
}
