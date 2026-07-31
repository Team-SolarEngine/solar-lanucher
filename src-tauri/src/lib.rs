mod apps;
mod json;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_current_ver(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            json::get_keys,
            json::add_key,
            json::update_key,
            json::delete_key,
            apps::start_app,
            apps::open_folder,
            apps::get_file_content,
            get_current_ver,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
