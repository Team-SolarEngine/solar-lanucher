import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

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
