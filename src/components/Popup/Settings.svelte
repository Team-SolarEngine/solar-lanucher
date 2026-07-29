<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let { modalSettings = $bindable() } = $props()

    let addPet = $state(false)
    let petIconUrl = $state("")

    async function loadSetting(key: string) {
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        return data?.[key] ?? "";
    }

    async function saveSetting(key: string, value: string | boolean) {
        await invoke("update_key", {
            collection: "settings",
            key: key,
            value: value,
        });
    }

    $effect(() => {
        if (modalSettings) {
            loadSetting("addPet").then(v => addPet = v === true || v === "true");
            loadSetting("petIconUrl").then(v => petIconUrl = v);
        }
    })
</script>

<div class="overlay" class:active={modalSettings} onclick={() => modalSettings = false}></div>
<dialog class="right" class:active={modalSettings}>
    <h5>Settings</h5>

    <div class="field middle-align">
        <nav>
            <div class="max">
                <h6>Funny little pet</h6>
                <div>Have sussy amogus on the bottom right!<br/>Keeps you company.</div>
            </div>
            <label class="switch">
                <input type="checkbox" checked={addPet} onchange={() => { addPet = !addPet; saveSetting("addPet", addPet); }}>
                <span></span>
            </label>
        </nav>
    </div>

    <hr class="medium"/>

    <div class="field label border">
        <input type="text" bind:value={petIconUrl} onchange={() => saveSetting("petIconUrl", petIconUrl)}>
        <label>Pet icon URL</label>
        <output>Don't like sussy amogus? Use a URL to a custom icon!</output>
    </div>

    <nav class="right-align no-space">
        <button class="transparent link" onclick={() => modalSettings = false}>Close</button>
    </nav>
</dialog>
