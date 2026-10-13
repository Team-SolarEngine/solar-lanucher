use tauri::AppHandle;
use reqwest::Client;
use futures_util::StreamExt;
use std::fs;
use std::fs::File;
use std::io::Write;
use super::json::APP_ID;

const BINARIES_URL: &str = "https://github.com/Team-SolarEngine/solar-lanucher/tree/main/src-tauri/binaries";

#[tauri::command]
pub async fn download_7z(_app: AppHandle) -> Result<String, String> {
    /*
     * Downloads the 7-Zip binary for the current operating system.
     *
     * Returns:
     *     String -> the path of the downloaded 7-Zip executable
     */

    let os = std::env::consts::OS;
    let mut files = Vec::new();
    let executable;

    if os == "windows" {
        files.push("7za.dll");
        files.push("7z.dll");
        files.push("7za.exe");
        files.push("7z.exe");
        executable = "7z.exe";
    } else if os == "linux" {
        files.push("7za-linux");
        executable = "7za-linux";
    } else if os == "macos" {
        files.push("7za-mac");
        executable = "7za-mac";
    } else {
        return Err(format!("Unsupported operating system: {os}"));
    }

    let config_dir = dirs::config_dir().ok_or("Could not determine config directory")?;
    let app_dir = config_dir.join(APP_ID);
    fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create directory: {e}"))?;

    let client = Client::new();
    for file_name in files {
        let url = format!("{BINARIES_URL}/{file_name}");
        let target = app_dir.join(file_name);

        println!("Downloading {url}");
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()));
        }

        let mut file = File::create(&target).map_err(|e| format!("Failed to create file: {e}"))?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("Failed to read response: {e}"))?;
            file.write_all(&chunk).map_err(|e| format!("Failed to write file: {e}"))?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&target, fs::Permissions::from_mode(0o755))
                .map_err(|e| format!("Failed to set permissions: {e}"))?;
        }
    }

    // Hand back where the executable ended up.
    Ok(app_dir.join(executable).to_string_lossy().to_string())
}
