use crate::{
    error::Result,
    fs::{get_folder_elements, FolderElements},
};
use std::{os::unix::fs::MetadataExt, path::Path};

pub fn is_valid_branch(path: &Path) -> Result<bool> {
    let file_metadata = std::fs::metadata(path)?;

    if file_metadata.size() == 41 {
        return Ok(true);
    }

    Ok(false)
}

pub fn collect_branches(root_path: &Path, parent_path: &Option<String>) -> Result<Vec<String>> {
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
