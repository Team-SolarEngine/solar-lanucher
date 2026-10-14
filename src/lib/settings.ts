import { invoke } from "@tauri-apps/api/core";

export async function loadSetting(key: string) {
    /*
     * This function loads a single setting value from the backend.
     *
     * Arguments:
     *    key: string -> the name of the setting to load
     *
     * Returns:
     *    Promise -> the value of the setting, or null
     */
    const data = await invoke("get_keys", { collection: "settings" }) as any;
    return data?.[key];
}

export async function saveSetting(key: string, value: any) {
    /*
     * This function saves a single setting value to the backend.
     *
     * Arguments:
     *    key: string -> the name of the setting to save
     *    value: string or boolean -> the new value of the setting
     */

    console.log("Saving setting:", key, value);
    await invoke("update_key", {
        collection: "settings",
        key: key,
        value: value,
    });
}