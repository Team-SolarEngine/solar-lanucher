export function listenerSanitizer(str: string) {
    return str.replaceAll(/[^a-zA-Z0-9\-_/:]/g, "-")
}
