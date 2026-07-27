use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct App {
    pub name: String,
    pub icon_url: String,
    pub execute_command: String,
    pub working_directory: String,
}

const APP_ID: &str = "net.solarengine.solar-launcher";

fn get_apps_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Could not determine config directory");
    let app_dir = config_dir.join(APP_ID);
    fs::create_dir_all(&app_dir).ok();
    app_dir.join("apps.json")
}

fn read_apps() -> Vec<App> {
    let path = get_apps_path();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_default()
}

fn write_apps(apps: &[App]) {
    let path = get_apps_path();
    let json = serde_json::to_string_pretty(apps).unwrap();
    fs::write(&path, json).expect("Failed to write apps.json");
}

#[tauri::command]
pub fn get_apps() -> Vec<App> {
    read_apps()
}

#[tauri::command]
pub fn add_app(app: App) -> Vec<App> {
    let mut apps = read_apps();
    apps.push(app);
    write_apps(&apps);
    apps
}

#[tauri::command]
pub fn delete_app(index: usize) -> Vec<App> {
    let mut apps = read_apps();
    if index < apps.len() {
        apps.remove(index);
        write_apps(&apps);
    }
    apps
}

#[tauri::command]
pub fn update_app(index: usize, app: App) -> Vec<App> {
    let mut apps = read_apps();
    if index < apps.len() {
        apps[index] = app;
        write_apps(&apps);
    }
    apps
}
