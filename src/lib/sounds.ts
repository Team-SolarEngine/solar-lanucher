import { loadSetting } from "./settings";

export async function playSound(file: string, vol: number = 0.25) {
    if (await loadSetting("disableSounds")) return;
    try {
        const sound = new Audio(`/sounds/${file}`);
        sound.currentTime = 0;
        sound.volume = vol;
        sound.play().catch(() => {});
    } catch (e) {
        console.error(e)
    }
}