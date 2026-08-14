use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;
use reqwest::Client;
use zip::read::ZipArchive;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use futures_util::StreamExt;

#[tauri::command]
pub async fn download_to_custom_dir(
    app: AppHandle,
    url: String,
    file_path: String,
) -> Result<String, String> {
    /*
     * Downloads a file to a custom directory specified by the code.
     * 
     * Arguments:
     *     url(str): The URL of the file to download.
     *     file_path(str): The path to save the downloaded file.
     * 
     * Returns:
     *     str: The path to the downloaded file.
     */

    println!("Starting download for {}", url);

    println!("Resolve the base directory for the downloads");
    let base = app
        .path()
        .resolve("solar_lanucher_downloads", BaseDirectory::Home)
        .map_err(|e| format!("Failed to resolve base dir: {e}"))?;

    println!("Create the base directory if it doesn't exist");
    tokio::fs::create_dir_all(&base)
        .await
        .map_err(|e| format!("Failed to create directory: {e}"))?;

    println!("Self explainatory.");
    let zip_path = base.join("mod_download.zip");

    println!("Download the file from the URL");
    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    println!("Check if the response is successful");
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    println!("Read the response body");
    let mut stream = response.bytes_stream();

    println!("Save the file to the target path");
    let mut file = File::create(&zip_path)
        .map_err(|e| format!("Failed to create file: {e}"))?;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Failed to read response: {e}"))?;
        file.write_all(&chunk)
            .map_err(|e| format!("Failed to write file: {e}"))?;
    }

    println!("Unzip the file");
    unzip_file(
        zip_path.to_str().ok_or("Invalid zip path")?,
        &file_path,
    ).map_err(|e| format!("Unzip failed: {e}"))?;

    println!("Delete the zip");
    std::fs::remove_file(zip_path).map_err(|e| format!("Failed to delete zip: {e}"))?;

    println!("Return the path to the downloaded file");
    Ok(file_path)
}

fn unzip_file(zip_path: &str, target_dir: &str) -> zip::result::ZipResult<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    std::fs::create_dir_all(target_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(target_dir).join(file.mangled_name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
