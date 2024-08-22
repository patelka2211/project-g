use super::utilities;

#[tauri::command]
/// returns list of **LOCAL** branches with its metadata.
///```ts
/// // returns
/// Array<{
///     name: string,
///     upstream?: string,
///     isHead: boolean,
///     commitTime: number
/// }>
/// ```
pub fn get_local_branches(
    repo_path: String,
) -> core::result::Result<Vec<utilities::Branch>, String> {
    match utilities::get_local_branches(repo_path) {
        Ok(branches) => Ok(branches),
        Err(error) => return Err(error.to_string()),
    }
}
