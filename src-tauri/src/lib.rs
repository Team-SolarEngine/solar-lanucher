mod apps;
mod json;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            json::get_apps,
            json::add_app,
            json::delete_app,
            json::update_app,
            apps::start_app,
            apps::open_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
