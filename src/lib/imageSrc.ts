import { convertFileSrc } from "@tauri-apps/api/core";

export function imageSrc(path: string): string {
    if (!path) return "";
    if (path.startsWith("http://") || path.startsWith("https://")) return path;
    return convertFileSrc(path);
}

export default imageSrc;