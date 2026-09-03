use std::fs;

#[tauri::command]
pub fn paste_to_dir(to_copy: &str, dest_to_paste: &str) -> Result<String, String> {
    /*
     * This function copies a folder from one place to another.
     * Folders are copied recursively, so all their contents come along.
     *
     * Arguments:
     *    to_copy: string -> the path of the folder to copy
     *    dest_to_paste: string -> the destination folder path
     *
     * Returns:
     *    Result<String, String> -> a success message or an error message
     */
    let source = fs::metadata(to_copy)
        .map_err(|e| format!("Failed to read source path: {e}"))?;

    if !source.is_dir() {
        return Err(format!("The source path is not a folder: {to_copy}"));
    }

    let folder_name = std::path::Path::new(to_copy)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("mod");

    let final_destination = std::path::Path::new(dest_to_paste).join(folder_name);
    let final_destination = final_destination.to_str().unwrap_or_default();

    copy_folder_recursive(to_copy, final_destination)?;

    Ok(format!("Sucessfully copied {to_copy} to {final_destination}"))
}

fn copy_folder_recursive(source: &str, destination: &str) -> Result<(), String> {
    /*
     * This function copies a folder and everything inside it.
     *
     * Arguments:
     *    source: string -> the folder to copy
     *    destination: string -> where to copy the folder to
     *
     * Returns:
     *    Result<(), String> -> nothing, or an error message
     */
    fs::create_dir_all(destination)
        .map_err(|e| format!("Failed to create destination folder: {e}"))?;

    let entries = fs::read_dir(source)
        .map_err(|e| format!("Failed to read source folder: {e}"))?;

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let new_path = std::path::Path::new(destination).join(entry.file_name());

        if entry_path.is_dir() {
            copy_folder_recursive(
                &entry_path.to_str().unwrap_or_default(),
                &new_path.to_str().unwrap_or_default(),
            )?;
        } else {
            fs::copy(&entry_path, &new_path)
                .map_err(|e| format!("Failed to copy file: {e}"))?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_os() -> String {
    std::env::consts::OS.to_string()
}
