<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Version from "../Misc/Version.svelte";
    import AdditionalRepos from "./Settings/AdditionalRepos.svelte";

    let { modalSettings = $bindable() } = $props()
    let settings = $state({} as any)

    let toggleAddRepos = $state(false);

    const settingFields = [
        { title: "Add Pet", key: "addPet", desc: "Have sussy amogus on the bottom right!\nKeeps you company.", type: "toggle", default: false },
        { title: "Pet Icon URL", key: "petIconUrl", desc: "Don't like sussy amogus? Use a URL or Path to a custom icon!", type: "text", default: "" },
        { title: "Github Token", key: "githubToken", desc: "Tired of rate limits? Create your own token for GitHub and use it!", type: "text", default: "", hidden: true },
        { title: "Compact Mode", key: "compactMode", desc: "Too spaced out and too much content? Turn this on!", type: "toggle", default: "", hidden: true },
        // { title: "Path To Downloaded", key: "pathToDownloaded", desc: "When using the download options, files will be saved to this path.", type: "text", default: "" },
        { title: "Additional Engine Repositories", key: "additionalRepos", desc: "Tired of the current engine selection? You can add more!", type: "array", default: [] },
        { title: "Favourite Path", key: "favouritePath", desc: "Have a path that you use frequently? Set it here! We only have one slot, but we plan to add more in the future.", type: "text", default: "" },
    ]

    async function loadSetting(key: string) {
        /*
         * This function loads a single setting value from the backend.
         *
         * Arguments:
         *    key: string -> the name of the setting to load
         *
         * Returns:
         *    Promise -> the value of the setting, or null
         */
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        return data?.[key];
    }

    async function saveSetting(key: string, value: any) {
        /*
         * This function saves a single setting value to the backend.
         *
         * Arguments:
         *    key: string -> the name of the setting to save
         *    value: string or boolean -> the new value of the setting
         */

        console.log("Saving setting:", key, value);
        await invoke("update_key", {
            collection: "settings",
            key: key,
            value: value,
        });
    }

    $effect(() => {
        if (modalSettings) {
            for (const field of settingFields) {
                loadSetting(field.key).then(v => {
                    if (field.type === "toggle") settings[field.key] = v === true || v === "true";
                    else if (field.type === "array") settings[field.key] = Array.isArray(v) ? v : (field.default as any[]);
                    else settings[field.key] = v ?? field.default;
                });
            }
        }
    })
</script>

<div class="overlay" class:active={modalSettings} onclick={() => modalSettings = false}></div>
<dialog class="right" class:active={modalSettings} style="max-width: 500px;">
    <h5>Settings</h5>

    {#each settingFields as field}
        {#if field.type === "toggle"}
            <div class="field middle-align">
                <nav>
                    <div class="max">
                        <h6>{field.title}</h6>
                        {#if field.desc}
                            <div>{@html field.desc.replace(/\n/g, "<br/>")}</div>
                        {/if}
                    </div>
                    <label class="switch">
                        <input type="checkbox" checked={settings[field.key]} onchange={() => { settings[field.key] = !settings[field.key]; saveSetting(field.key, settings[field.key]); }}>
                        <span></span>
                    </label>
                </nav>
            </div>
        {:else if field.type === "text"}
            <div class="field label border">
                <input type={field.hidden ? "password" : "text"} bind:value={settings[field.key]} onchange={() => saveSetting(field.key, settings[field.key])}>
                <label>{field.title}</label>
                {#if field.desc}
                    <output>{field.desc}</output>
                {/if}
            </div>
        {:else if field.type === "array"}
            <div class="field label border">
                <h6>{field.title}</h6>
                {#if field.desc}
                    <div>{field.desc}</div>
                {/if}

                <button onclick={() => toggleAddRepos = true}>
                    Manage
                </button>
            </div>
        {/if}

        {#if field !== settingFields[settingFields.length - 1]}
            <hr class="medium"/>
        {/if}
    {/each}

    <hr class="medium"/>
    <Version />

    <nav class="right-align no-space">
        <button class="transparent link" onclick={() => modalSettings = false}>Close</button>
    </nav>
</dialog>

<AdditionalRepos bind:currentlyOpen={toggleAddRepos} />
