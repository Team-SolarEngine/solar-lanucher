import { invoke } from "@tauri-apps/api/core";

export async function getUserLanguage() {
  try {
    const data = await invoke("get_keys", { collection: "settings" }) as any;
    return data?.["currentLanguage"];
  } catch (error) {
    console.log("Error getting language:", error);
    return "Error: " + error;
  }
}

export async function getLocalization() {
  try {
    const language = await getUserLanguage();

    const response = await fetch(`/localization/${language}.json`);
    const data = await response.json();

    return data;
  } catch (error) {
    console.log("Error getting localization:", error);
    return "Error: " + error;
  }
}