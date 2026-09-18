use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[tauri::command]
pub async fn start_app(working_dir: String, command_exec: String, _open_terminal: bool) -> Result<String, String> {
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
    let _ = run_command(command_exec, _open_terminal, working_dir).await;

    #[cfg(unix)]
    async fn trust_exec(command_exec: &str, working_dir: &str) -> Result<String, String> {
        let _ = run_command(format!("chmod +x '{}'", command_exec), false, working_dir.to_string()).await;
        Ok("Trusted executable.".to_string())
    }

    #[cfg(unix)]
    let _ = trust_exec(&command_exec, &working_dir).await?;

    #[cfg(unix)]
    if !_open_terminal {
        let _ = run_command(command_exec, false, working_dir.to_string()).await;
    } else {
        for &emulator in &["alacritty", "gnome-terminal", "xfce4-terminal", "terminator", "foot", "konsole", "kitty"] {
            let _ = run_command(format!("{emulator} -e {command_exec}"), false, working_dir.to_string()).await;
        }
        return Err("No terminal emulator found".to_string());
    };

    Ok("App started".to_string())
}

#[tauri::command]
pub async fn open_folder(path: String) -> Result<String, String> {
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
    run_command(format!("xdg-open '{}'", path), false, ".".to_string())
        .await
        .map_err(|e| format!("{}", e))?;

    #[cfg(target_os = "macos")]
    run_command(format!("open '{}'", path), false, ".".to_string())
        .await
        .map_err(|e| format!("{}", e))?;

    #[cfg(target_os = "windows")]
    run_command(format!("explorer '{}'", path.replace('/', "\\")), false, ".".to_string())
        .await
        .map_err(|e| format!("{} - If you frequently see this, ignore it.", e))?;

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

#[tauri::command]
pub async fn run_command(command: String, _create_terminal_window: bool, working_dir: String) -> Result<String, String> {
    /*
     * This function runs a command in the specified working directory and returns the output.
     * Note:
     *   We can't use `cmd.spawn()` or `cmd.status()` because they make the app unresponsive.
     *   So, we use `tokio::task::spawn_blocking` to run the command in a blocking thread.
     *
     * Arguments:
     *    command: string -> the command to run
     *    create_terminal_window: bool -> whether to create a terminal window for the command
     *    working_dir: string -> the working directory to run the command in
     *
     * Returns:
     *    Result<String, String> -> the command output or an error message
     */
     let child = {
        #[cfg(windows)]
        {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &command.replace('/', "\\")]);
            cmd.creation_flags(if _create_terminal_window { 0 } else { CREATE_NO_WINDOW });
            cmd.current_dir(working_dir);
            cmd.spawn()
        }

        #[cfg(unix)]
        {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &command]);
            cmd.current_dir(working_dir);
            cmd.spawn()
        }
    }
    .map_err(|e| e.to_string())?;

    let result = tokio::task::spawn_blocking(move || {
        child.wait_with_output()
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    if result.status.success() {
        Ok(String::from_utf8_lossy(&result.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&result.stderr).to_string())
    }
}