import { invoke } from "@tauri-apps/api/core";

export type AppData = {
    name: string;
    icon_url: string;
    execute_command: string;
    working_directory: string;
    description: string;
    banner_url: string;
};

export async function loadApps(): Promise<AppData[]> {
  /*
   * This function loads the list of apps from the backend
   * so the CardApp components have all the data they need.
   */
  try {
    return await invoke<AppData[]>('get_keys', { collection: 'apps' });
  } catch (e) {
    console.error('Failed to load apps:', e);
    return [];
  }
}