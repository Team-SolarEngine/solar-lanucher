<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import Version from "../Misc/Version.svelte";
    import AdditionalRepos from "./Settings/AdditionalRepos.svelte";
    import LocalizationMenu from "./Settings/LocalizationMenu.svelte";
    import { getWord } from "$lib/localization";
    import { useSnackbarError, type Snackbar } from "$lib/interface";

    let { modalSettings = $bindable() } = $props()
    let settings = $state({} as any)

    let toggleAddRepos = $state(false);
    let toggleLocalizationMenu = $state(false);
    let modalForDoneSevenZip = $state(false);

    const settingFields = [
        { title: "settings.add_pet.title", key: "addPet", desc: "settings.add_pet.description", type: "toggle", default: false },
        { title: "settings.pet_icon_url.title", key: "petIconUrl", desc: "settings.pet_icon_url.description", type: "text", default: "" },
        { title: "settings.github_token.title", key: "githubToken", desc: "settings.github_token.description", type: "text", default: "", hidden: true },
        { title: "settings.compact_mode.title", key: "compactMode", desc: "settings.compact_mode.description", type: "toggle", default: "", hidden: true },
        // { title: "Path To Downloaded", key: "pathToDownloaded", desc: "When using the download options, files will be saved to this path.", type: "text", default: "" },
        { title: "settings.additional_engine_repositories.title", key: "additionalRepos", desc: "settings.additional_engine_repositories.description", buttonText: "settings.global.manage", type: "menu", default: "", call: () => toggleAddRepos = true },
        { title: "settings.favourite_path.title", key: "favouritePath", desc: "settings.favourite_path.description", type: "text", default: "" },
        { title: "settings.code_editor.title", key: "codeEditor", desc: "settings.code_editor.description", type: "text", default: "" },
        { title: "settings.localization.title", key: "currentLanguage", desc: "settings.localization.description", buttonText: "settings.global.manage", type: "menu", default: "en", call: () => toggleLocalizationMenu = true},
        { title: "settings.download_7zip.title", key: "", desc: "settings.download_7zip.description", buttonText: "global.download", type: "menu", default: "en", call: () => download8Zip()},
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

    async function download8Zip() {
        try {
            await invoke("download_7z");
            modalForDoneSevenZip = true;
            setInterval(() => modalForDoneSevenZip = false, 5000)
        } catch(e) {
            useComponentSnackbarError("Could not download 7zip: " + e)
        }
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

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    }); function useComponentSnackbarError(message: string) { useSnackbarError(message, snackbar); }
</script>

<div class="overlay" class:active={modalSettings} onclick={() => modalSettings = false}></div>
<dialog class="right" class:active={modalSettings} style="max-width: 500px;">
    <h5>{#await getWord("settings.global.Settings") then word}{@html word}{/await}</h5>

    {#each settingFields as field}
        {#if field.type === "toggle"}
            <div class="field middle-align">
                <nav>
                    <div class="max">
                        <h6>{#await getWord(field.title) then word}{@html word}{/await}</h6>
                        {#if field.desc}
                            <div>{#await getWord(field.desc) then word}{@html word.replace(/\n/g, "<br/>")}{/await}</div>
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
                <label>{#await getWord(field.title) then word}{@html word}{/await}</label>
                {#if field.desc}
                    <output>{#await getWord(field.desc) then word}{@html word}{/await}</output>
                {/if}
            </div>
        {:else if field.type === "menu"}
            <div class="field label border">
                <h6>{#await getWord(field.title) then word}{@html word}{/await}</h6>
                {#if field.desc}
                    <div>{#await getWord(field.desc) then word}{@html word}{/await}</div>
                {/if}

                <button onclick={field.call}>
                    {#await getWord(field.buttonText ?? "settings.global.manage") then word}{@html word}{/await}
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
        <button class="transparent link" onclick={() => modalSettings = false}>{#await getWord("global.close") then word}{@html word}{/await}</button>
    </nav>
</dialog>

<AdditionalRepos bind:currentlyOpen={toggleAddRepos} />
<LocalizationMenu bind:currentlyOpen={toggleLocalizationMenu} />

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>

<div class="snackbar primary" class:active={modalForDoneSevenZip}>
    7zip is now downloaded! You can now download files that are compressed with .rar, .7z, .zip, and etc!
</div>