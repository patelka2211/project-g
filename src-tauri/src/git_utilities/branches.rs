use std::{io::Error, path::Path};

use helpers::{get_folder_elements, is_valid_branch, FolderElements};

mod helpers;

struct Branch {
    name: String,
    points_at: String,
    updated_at: String,
}

fn collect_branches(root_path: &Path, parent_path: &Option<String>) -> Result<Vec<String>, Error> {
    let mut branches: Vec<String> = Vec::new();

    let parent_path = match parent_path {
        Some(path) => path,
        None => &String::from(""),
    };

    let FolderElements { files, folders }: FolderElements =
        get_folder_elements(root_path.join(parent_path).as_path())?;

    for file in files {
        let absolute_file_path = Path::new(root_path);

        let validity = is_valid_branch(absolute_file_path.join(parent_path).join(&file).as_path());

        match validity {
            Ok(validity) => {
                if validity == true {
                    branches.push(format!("{}{}", parent_path, file));
                }
            }
            Err(_) => {}
        }
    }

    for folder in folders {
        let recursive_branches =
            collect_branches(root_path, &Some(format!("{}{}/", parent_path, folder)))?;

        branches.extend(recursive_branches);
    }

    Ok(branches)
}

#[tauri::command]
pub fn local_branches(repo_path: String) -> String {
    let heads = format!("{}/.git/refs/heads", repo_path);
    let heads = Path::new(&heads);
    let branches = collect_branches(&heads, &None);

    let null = String::from("null");

    match branches {
        Ok(files) => serde_json::to_string(&files).unwrap_or(null),
        Err(_) => null,
    }
}
