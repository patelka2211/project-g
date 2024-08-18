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
pub fn get_local_branches(repo_path: String) -> core::result::Result<String, String> {
    let branches = match utilities::get_local_branches(repo_path) {
        Ok(branches) => branches,
        Err(error) => return Err(error.to_string()),
    };

    let output = match serde_json::to_string(&branches) {
        Ok(output) => output,
        Err(error) => return Err(error.to_string()),
    };

    Ok(output)
}
