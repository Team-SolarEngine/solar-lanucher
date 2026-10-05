import { invoke } from "@tauri-apps/api/core";

export let localization: Record<string, string> = {};

export async function getUserLanguage() {
  try {
    const data = await invoke("get_keys", { collection: "settings" }) as any;
    return data?.["currentLanguage"];
  } catch (error) {
    console.log("Error getting language:", error);
    return "Error: " + error;
  }
}

export async function getWord(word: string) {
  /*
   * Returns the localized word for the given key,
   * falling back to English if not found.
   * 
   * Arguments:
   *   word -> The key of the word to retrieve.
   * 
   * Returns:
   *   The localized word, or the English fallback if not found.
   */
  try {
    let localization = await getLocalization(false);

    if (localization[word] != "") {
      return localization[word];
    }

    let fallbackLocalization = await getLocalization(true);
    return fallbackLocalization[word];
  } catch (error) {
    console.log("Error getting word:", error);
    return "Error: " + error;
  }
}

async function getLocalization(fallback: boolean) {
  try {
    const language = await getUserLanguage();

    const response = await fetch(`/localization/${fallback ? "en" : language}.json`);
    const data = await response.json();

    return data;
  } catch (error) {
    console.log("Error getting localization:", error);
    return "Error: " + error;
  }
}