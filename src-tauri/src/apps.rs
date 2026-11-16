use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tauri::{AppHandle, Emitter};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Clone, serde::Serialize)]
struct LogPayload {
    message: String,
}

#[tauri::command]
pub async fn start_app(
    app: AppHandle,
    working_dir: String,
    command_exec: String,
    _open_terminal: bool
) -> Result<u32, String> {
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
    let pid = run_command(app, command_exec, _open_terminal, working_dir).await?;

    #[cfg(unix)]
    {
        let trust_exec = async |cmd: &str, dir: &str| -> Result<String, String> {
            let _ = run_command(app.clone(), format!("chmod +x '{}'", cmd), false, dir.to_string()).await;
            Ok("Trusted executable.".to_string())
        };

        trust_exec(&command_exec, &working_dir).await?;
    }

    #[cfg(unix)]
    let pid = run_command(app, command_exec, _open_terminal, working_dir.to_string()).await?;

    Ok(pid)
}

#[tauri::command]
pub async fn open_folder(
    app: AppHandle,
    path: String
) -> Result<String, String> {
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
    run_command(app, format!("xdg-open \"{}\"", path), false, ".".to_string())
        .await
        .map_err(|e| format!("{}", e))?;

    #[cfg(target_os = "macos")]
    run_command(app, format!("open \"{}\"", path), false, ".".to_string())
        .await
        .map_err(|e| format!("{}", e))?;

    #[cfg(target_os = "windows")]
    run_command(app, format!("explorer {}", path.replace('/', "\\")), false, ".".to_string())
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
pub async fn pure_run_command(command: String, args: Vec<String>) -> Result<String, String> {
    /*
     * This function spawns a command directly instead of wrapping it
     * in a cmd/sh shell.
     *
     * Arguments:
     *    command: string       -> the executable to run
     *    args: array of strings -> the arguments to pass to the executable
     *
     * Returns:
     *    Result<String, String> -> a success message or an error message
     */
    let child = {
        #[cfg(unix)]
        if is_running_in_flatpak() {
            /*
             * Inside Flatpak, run through flatpak-spawn --host so the
             * command executes on the real system with the user's
             * libraries instead of the sandbox.
             */
            let mut cmd = Command::new("flatpak-spawn");
            cmd.arg("--host");
            cmd.arg(&command);
            cmd.args(&args);
            cmd.spawn()
        } else {
            let mut cmd = Command::new(&command);
            cmd.args(&args);
            cmd.spawn()
        }

        #[cfg(windows)]
        {
            let mut cmd = Command::new(&command);
            cmd.creation_flags(CREATE_NO_WINDOW);
            cmd.args(&args);
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

#[tauri::command]
pub async fn run_command(
    app: AppHandle,
    command: String,
    _open_terminal: bool,
    working_dir: String
) -> Result<u32, String> {
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
     let mut child = {
        #[cfg(windows)]
        {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &command.replace('/', "\\")]);
            cmd.creation_flags(CREATE_NO_WINDOW);
            cmd.current_dir(working_dir);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.spawn()
        }

        #[cfg(unix)]
        {
            if is_running_in_flatpak() {
                /*
                 * Inside Flatpak, run through flatpak-spawn --host so the
                 * command executes on the real system with the user's shell
                 * and libraries (e.g. libvlc) instead of the sandbox.
                 */
                let host_command = format!("cd '{working_dir}' && {command}");
                let mut cmd = Command::new("flatpak-spawn");
                cmd.args(["--host", "sh", "-c", &host_command]);
                cmd.stdout(Stdio::piped());
                cmd.stderr(Stdio::piped());
                cmd.spawn()
            } else {
                let mut cmd = Command::new("sh");
                cmd.args(["-c", &command]);
                cmd.current_dir(working_dir);
                cmd.stdout(Stdio::piped());
                cmd.stderr(Stdio::piped());
                cmd.spawn()
            }
        }
    }
    .map_err(|e| e.to_string())?;

    let pid = child.id();
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let app_clone = app.clone();

    tokio::spawn(async move {
        let _ = tokio::task::spawn_blocking(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(text) = line {
                    let _ = app_clone.emit("terminal-output", LogPayload { message: text });
                }
            }
            let _ = child.wait();
        }).await;
    });

    Ok(pid)
}

#[tauri::command]
pub async fn kill_app(pid: u32) -> Result<String, String> {
    #[cfg(unix)]
    {
        // On Linux/Unix, use the `kill` command or nix signal
        let status = std::process::Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(format!("Process {} terminated.", pid))
        } else {
            Err(format!("Failed to terminate process {}", pid))
        }
    }

    #[cfg(windows)]
    {
        // On Windows, use `taskkill`
        let status = std::process::Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            Ok(format!("Process {} terminated.", pid))
        } else {
            Err(format!("Failed to terminate process {}", pid))
        }
    }
}

#[tauri::command]
pub fn is_running_in_flatpak() -> bool {
    /*
     * Returns true when the app is running inside a Flatpak sandbox.
     * The /.flatpak-info file only exists inside the sandbox.
     *
     * Returns:
     *    bool -> true if sandboxed by Flatpak
     */
    std::path::Path::new("/.flatpak-info").exists()
}