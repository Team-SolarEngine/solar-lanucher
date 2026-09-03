mod apps;
mod json;
mod downloading;
mod mods_togglers;
mod misc;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_current_ver(app: tauri::AppHandle) -> String {
    /*
     * This function returns the current version of the application.
     *
     * Returns:
     *    string -> the current version of the application
     */
    app.package_info().version.to_string()
}

fn fix_linux_handler_desktop_files(app: &tauri::AppHandle) {
    /*
     * This function patches the deep-link handler desktop files on Linux.
     *
     * The deep-link plugin writes Exec="<path>" %u with quotation marks,
     * but xdg-open keeps the quotes when it reads the Exec line,
     * so it fails to find the command and deep links die silently.
     * See https://gitlab.freedesktop.org/xdg/xdg-utils/-/issues/151
     *
     * Arguments:
     *    app: AppHandle -> the tauri app handle, used to find the data dir
     */
    #[cfg(target_os = "linux")]
    {
        let applications_dir = app.path().data_dir().unwrap().join("applications");
        if let Ok(entries) = std::fs::read_dir(&applications_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.ends_with("-handler.desktop") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        // strip both the leading and trailing quote from Exec="..."
                        let fixed = content
                            .replace("Exec=\"", "Exec=")
                            .replace("\" %u", " %u");
                        if fixed != content {
                            let _ = std::fs::write(&path, fixed);
                        }
                    }
                }
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            /*
             * This callback runs when a second instance of the app is opened.
             * The single-instance plugin, with its deep-link feature enabled,
             * already forwards any deep link argument to the running instance
             * before this callback fires.
             */
            println!("single instance triggered with: {argv:?} for {}", app.package_info().name);
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            /*
             * This setup registers the solar-launch deep link scheme
             * and fixes the generated handler desktop file on Linux.
             */
            #[cfg(any(target_os = "linux", windows))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                let deep_link = app.deep_link();
                // register may fail if xdg-mime is missing, so just ignore errors
                let _ = deep_link.register_all();
                fix_linux_handler_desktop_files(app.handle());
            }
            Ok(())
        })
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
            downloading::download_to_custom_dir,
            mods_togglers::list_folder,
            mods_togglers::toggle_mod,
            mods_togglers::trash_folder,
            misc::paste_to_dir,
            misc::get_os,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
