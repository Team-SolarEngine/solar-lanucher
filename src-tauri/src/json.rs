use serde_json::Value;
use std::fs;
use std::path::PathBuf;

const APP_ID: &str = "net.solarengine.solar-launcher";

fn get_data_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Could not determine config directory");
    let app_dir = config_dir.join(APP_ID);
    fs::create_dir_all(&app_dir).ok();
    app_dir.join("data.json")
}

fn read_all() -> Value {
    let path = get_data_path();
    if !path.exists() {
        return Value::Object(serde_json::Map::new());
    }
    let data = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&data).unwrap_or(Value::Object(serde_json::Map::new()))
}

fn write_all(data: &Value) {
    let path = get_data_path();
    let json = serde_json::to_string_pretty(data).unwrap();
    fs::write(&path, json).expect("Failed to write data.json");
}

#[tauri::command]
pub fn get_keys(collection: String) -> Value {
    let data = read_all();
    data.get(&collection).cloned().unwrap_or(Value::Null)
}

#[tauri::command]
pub fn add_key(collection: String, value: Value) -> Value {
    let mut data = read_all();
    let entry = data.get_mut(&collection);
    match entry {
        Some(Value::Array(arr)) => {
            arr.push(value);
        }
        _ => {
            data.as_object_mut()
                .unwrap()
                .insert(collection.clone(), Value::Array(vec![value]));
        }
    }
    write_all(&data);
    data.get(&collection).cloned().unwrap_or(Value::Null)
}

#[tauri::command]
pub fn update_key(collection: String, key: Value, value: Value) -> Value {
    let mut data = read_all();
    match data.get_mut(&collection) {
        Some(Value::Array(arr)) => {
            if let Some(idx) = key.as_u64() {
                let i = idx as usize;
                if i < arr.len() {
                    arr[i] = value;
                }
            }
        }
        Some(Value::Object(obj)) => {
            if let Some(k) = key.as_str() {
                obj.insert(k.to_string(), value);
            }
        }
        _ => {
            if let Some(k) = key.as_str() {
                let mut map = serde_json::Map::new();
                map.insert(k.to_string(), value);
                data.as_object_mut()
                    .unwrap()
                    .insert(collection.clone(), Value::Object(map));
            }
        }
    }
    write_all(&data);
    data.get(&collection).cloned().unwrap_or(Value::Null)
}

#[tauri::command]
pub fn delete_key(collection: String, key: Value) -> Value {
    let mut data = read_all();
    match data.get_mut(&collection) {
        Some(Value::Array(arr)) => {
            if let Some(idx) = key.as_u64() {
                let i = idx as usize;
                if i < arr.len() {
                    arr.remove(i);
                }
            }
        }
        Some(Value::Object(obj)) => {
            if let Some(k) = key.as_str() {
                obj.remove(k);
            }
        }
        _ => {}
    }
    write_all(&data);
    data.get(&collection).cloned().unwrap_or(Value::Null)
}
