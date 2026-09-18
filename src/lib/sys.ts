import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

export async function sendNotif(title: string, body: string) {
  /*
   * This function sends a system notification, requesting
   * notification permission first if it hasn't been granted yet.
   *
   * Arguments:
   *    title: string -> the title of the notification
   *    body: string  -> the body text of the notification
   */
  let permissionGranted = await isPermissionGranted();

  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === "granted";
  }

  if (permissionGranted) {
    sendNotification({ title, body });
  }
}

export function imageSrc(path: string): string {
  if (!path) return "";
  if (path.startsWith("http://") || path.startsWith("https://")) return path;
  if (path.startsWith("/") || /^[A-Z]:[\\]/.test(path)) return convertFileSrc(path);
  return path;
}

export async function getOS(): Promise<string> {
  /*
   * This function gets the OS of the user
   * via the get_OS in the rust code.
   */
  try {
    return await invoke<string>("get_os");
  } catch (e) {
    return "undefined"
  }
}