use std::fs;

#[tauri::command]
pub fn list_mods(working_directory: &str, mods_folder: &str) -> Vec<String> {
    /*
     * This function lists all the mod folders inside the engine's mods folder.
     * Each mod is expected to be a folder that contains a pack.json.
     *
     * Arguments:
     *    working_directory: string -> the path to the engine's folder
     *    mods_folder: string -> the name of the mods folder for this engine
     *                           (Psych Engine uses "mods", others may differ)
     *
     * Returns:
     *    Vec<String> -> a list of full paths to each mod folder
     */
    let mut mods = Vec::new();
    let mods_path = format!("{working_directory}/{mods_folder}");

    if let Ok(entries) = fs::read_dir(&mods_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                mods.push(path.to_str().unwrap().to_string());
            }
        }
    }

    mods
}

#[tauri::command]
pub fn toggle_mod(
    mod_folder: String,
    working_directory: String,
    mods_folder: String,
    enable: bool,
) -> Result<String, String> {
    /*
     * This function toggles a mod between enabled and disabled.
     * When disabling, the mod moves into the disabled-mods folder.
     * When enabling, the mod moves back into the engine's mods folder.
     * The disabled-mods folder is created if it does not exist.
     *
     * Arguments:
     *    mod_folder: string -> the full path to the mod folder to move
     *    working_directory: string -> the path to the engine's folder
     *    mods_folder: string -> the name of the mods folder for this engine
     *    enable: bool -> true moves the mod into mods, false moves it to disabled-mods
     *
     * Returns:
     *    Result<String, String> -> a message or an error message
     */
    let disabled_folder = format!("{working_directory}/disabled-mods");
    fs::create_dir_all(&disabled_folder)
        .map_err(|e| format!("Failed to create disabled-mods folder: {e}"))?;

    let folder_name = mod_folder.rsplit(['/', '\\']).next().unwrap_or(&mod_folder);
    let mods_path = format!("{working_directory}/{mods_folder}");

    let destination = if enable {
        format!("{mods_path}/{folder_name}")
    } else {
        format!("{disabled_folder}/{folder_name}")
    };

    fs::rename(&mod_folder, &destination)
        .map_err(|e| format!("Failed to move mod: {e}"))?;

    if enable {
        Ok(format!("Enabled mod: {folder_name}"))
    } else {
        Ok(format!("Disabled mod: {folder_name}"))
    }
}

#[tauri::command]
pub fn trash_folder(mod_folder: String) -> Result<String, String> {
    /*
     * This functions deletes the mod given from the
     * arguemnt.
     * 
     * Arguments:
     *     mod_folder: String -> The mod to trash.
     */

    fs::remove_dir_all(&mod_folder)
        .map_err(|e| format!("Failed to delete mod: {e}"))?;

    Ok(format!("Trashed mod: {mod_folder}"))
}