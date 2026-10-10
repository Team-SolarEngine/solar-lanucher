use tauri::AppHandle;

#[tauri::command]
pub async fn download_7z(app: AppHandle) {
    println!("hi");
}