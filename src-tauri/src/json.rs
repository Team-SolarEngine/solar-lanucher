use serde_json::Value;
use std::fs;
use std::path::PathBuf;

const APP_ID: &str = "net.solarengine.solar-launcher";

fn get_data_path() -> PathBuf {
    /*
     * This function returns the path to the data.json file.
     * It creates the app config directory if it does not exist yet.
     *
     * Returns:
     *    PathBuf -> the path to the data.json file
     */
    let config_dir = dirs::config_dir().expect("Could not determine config directory");
    let app_dir = config_dir.join(APP_ID);
    fs::create_dir_all(&app_dir).ok();
    app_dir.join("data.json")
}

fn read_all() -> Value {
    /*
     * This function reads all the data from the data.json file.
     * It returns an empty object if the file does not exist or cannot be parsed.
     *
     * Returns:
     *    Value -> the full data as a JSON value
     */
    let path = get_data_path();
    if !path.exists() {
        return Value::Object(serde_json::Map::new());
    }
    let data = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
    serde_json::from_str(&data).unwrap_or(Value::Object(serde_json::Map::new()))
}

fn write_all(data: &Value) {
    /*
     * This function writes all the given data to the data.json file.
     *
     * Arguments:
     *    data: Value -> the JSON data to write to the file
     */
    let path = get_data_path();
    let json = serde_json::to_string_pretty(data).unwrap();
    fs::write(&path, json).expect("Failed to write data.json");
}

#[tauri::command]
pub fn get_keys(collection: String) -> Value {
    /*
     * This function returns the value of a collection from the stored data.
     *
     * Arguments:
     *    collection: string -> the name of the collection to fetch
     *
     * Returns:
     *    Value -> the collection value, or null if it does not exist
     */
    let data = read_all();
    data.get(&collection).cloned().unwrap_or(Value::Null)
}

#[tauri::command]
pub fn add_key(collection: String, value: Value) -> Value {
    /*
     * This function adds a new value to a collection in the stored data.
     * If the collection is an array, the value is pushed to the end.
     * If the collection does not exist, it creates a new array with the value.
     *
     * Arguments:
     *    collection: string -> the name of the collection to add to
     *    value: Value -> the value to add to the collection
     *
     * Returns:
     *    Value -> the updated collection value
     */
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
    /*
     * This function updates a value inside a collection in the stored data.
     * For arrays, the key is the index of the element to update.
     * For objects, the key is the name of the field to update.
     *
     * Arguments:
     *    collection: string -> the name of the collection to update
     *    key: Value -> the index or field name of the value to update
     *    value: Value -> the new value to set
     *
     * Returns:
     *    Value -> the updated collection value
     */
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
    /*
     * This function removes a value from a collection in the stored data.
     * For arrays, the key is the index of the element to remove.
     * For objects, the key is the name of the field to remove.
     *
     * Arguments:
     *    collection: string -> the name of the collection to remove from
     *    key: Value -> the index or field name of the value to remove
     *
     * Returns:
     *    Value -> the updated collection value
     */
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
