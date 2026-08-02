import { convertFileSrc } from "@tauri-apps/api/core";

export function imageSrc(path: string): string {
    if (!path) return "";
    if (path.startsWith("http://") || path.startsWith("https://")) return path;
    if (path.startsWith("/") || /^[A-Z]:[\\]/.test(path)) return convertFileSrc(path);
    return path;
}

export default imageSrc;