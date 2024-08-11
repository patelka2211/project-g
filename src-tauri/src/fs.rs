use crate::error::Result;
use std::{fs::read_dir, path::Path};

pub struct FolderElements {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}

pub fn get_folder_elements(folder_path: &Path) -> Result<FolderElements> {
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
