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

export async function cuteify(funny: string, lang: string)
{
  console.log(funny)
  console.log(lang)
  if (lang == "cuteMode")
  {
    funny = funny.replaceAll('l', 'w').replaceAll("L", 'W').replaceAll('r', 'w').replaceAll('R', 'W')
  }

  console.log(funny)
  return funny
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
    const language = await getUserLanguage();
    const localization = await getLocalization(language);

    if (localization[word] != "") {
      return cuteify(localization[word], language);
    }

    const fallbackLocalization = await getLocalization("en");
    return cuteify(fallbackLocalization[word], language);
  } catch (error) {
    console.log("Error getting word:", error);
    return "Error: " + error;
  }
}

export async function getLocalization(language: string) {
  if (language == "cuteMode")
  {
    language = "en"
  }
  try {
    const response = await fetch(`/localization/${language}.json`);
    const data = await response.json();

    return data;
  } catch (error) {
    console.log("Error getting localization:", error);
    return "Error: " + error;
  }
}