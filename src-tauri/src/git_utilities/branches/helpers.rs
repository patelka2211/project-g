use std::{
    fs::read_dir,
    io::{self, Error},
    os::unix::{fs::MetadataExt, process::ExitStatusExt},
    path::Path,
    process::{self, Output},
};

// fn recursive_files(parent_path: &Path, folder_name: String) -> Result<Vec<String>, Error> {
//     let files = list_files(parent_path.join(&folder_name).as_path())?;

//     let mut files_recursive: Vec<String> = Vec::new();

//     for file in files {
//         files_recursive.push(format!("{}/{}", folder_name, file));
//     }

//     Ok(files_recursive)
// }

fn get_commit_hash_and_time(repo_path: &String, branch_name: &String) -> Result<(), Error> {
    let result = process::Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--format=%H$%cI")
        .arg(format!("refs/heads/{}", branch_name))
        .current_dir(repo_path)
        .output();

    // match result {
    //     Ok(output) => {
    //         let Output {
    //             status,
    //             stdout,
    //             stderr,
    //         } = output;

    //         if status.success() {
    //             let output = match String::from_utf8(stdout) {
    //                 Ok(it) => it,
    //                 Err(err) => return Err(err),
    //             };
    //         }
    //         Ok(())
    //     }
    //     Err(error) => Err(error),
    // }
    Ok(())
}

pub fn is_valid_branch(path: &Path) -> Result<bool, Error> {
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

pub fn get_folder_elements(folder_path: &Path) -> Result<FolderElements, Error> {
    // merge list_files and list_folders functions if they are not used in this function.
    Ok(FolderElements {
        files: list_files(folder_path)?,
        folders: list_folders(folder_path)?,
    })
}

fn list_folders(folder_path: &Path) -> Result<Vec<String>, Error> {
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

fn list_files(folder_path: &Path) -> Result<Vec<String>, Error> {
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
