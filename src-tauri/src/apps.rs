use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[tauri::command]
pub fn start_app(working_dir: String, command_exec: String, _open_terminal: bool) -> Result<String, String> {
    /*
     * This function starts an app by running a command in the given directory.
     * It opens a terminal emulator if requested, otherwise it runs in the background.
     *
     * Arguments:
     *    working_dir: string -> the directory to run the command in
     *    command_exec: string -> the command to execute
     *    _open_terminal: bool -> whether to open a terminal emulator or not
     *
     * Returns:
     *    Result<String, String> -> a success message or an error message
     */
    #[cfg(windows)]
    let mut cmd = if _open_terminal {
        let mut c = Command::new("cmd");
        c.args(["/C", &command_exec.replace("/", "\\").to_string()]);
        c
    } else {
        let mut c = Command::new("cmd");
        c.args(["/C", &command_exec.replace("/", "\\").to_string()]);
        c.creation_flags(CREATE_NO_WINDOW);
        c
    };

    #[cfg(unix)]
    let mut cmd = if !_open_terminal {
        let mut c = Command::new("sh");
        c.args(["-c", &command_exec]);
        c
    } else {
        for &emulator in &["alacritty", "gnome-terminal", "xfce4-terminal", "terminator", "foot", "konsole", "kitty"] {
            let mut c = Command::new(emulator);
            c.args(["-e", &command_exec]).current_dir(&working_dir);
            if c.spawn().is_ok() {
                return Ok("App started".to_string());
            }
        }
        return Err("No terminal emulator found".to_string());
    };

    cmd.current_dir(&working_dir);
    cmd.spawn()
        .map_err(|e| format!("Failed to start app: {}", e))?;

    Ok("App started".to_string())
}

#[tauri::command]
pub fn open_folder(path: String) -> Result<String, String> {
    /*
     * This function opens a folder in the file explorer of the current OS.
     *
     * Arguments:
     *    path: string -> the path to the folder to open
     *
     * Returns:
     *    Result<String, String> -> a success message or an error message
     */
    #[cfg(target_os = "linux")]
    let mut cmd = {
        let mut c = Command::new("xdg-open");
        c.arg(&path);
        c
    };

    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = Command::new("open");
        c.arg(&path);
        c
    };

    #[cfg(target_os = "windows")]
    let mut cmd = {

        let mut c = Command::new("explorer");
        c.arg(path.replace("/", "\\").to_string());
        c
    };

    cmd.spawn()
        .map_err(|e| format!("Failed to open folder: {}", e))?;

    Ok("Folder opened".to_string())
}

#[tauri::command]
pub fn get_file_content(path: String) -> Result<String, String> {
    /*
     * This function reads the content of a file and returns it as a string.
     *
     * Arguments:
     *    path: string -> the path to the file to read
     *
     * Returns:
     *    Result<String, String> -> the file content or an error message
     */
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;
    Ok(content)
}