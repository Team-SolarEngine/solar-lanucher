<script lang="ts">
    import { getOS } from "$lib/sys";
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";
    import { getWord } from "$lib/localization";

    let version: string;
    let latest: string;
    let os: string;

    async function getVersion() {
        /*
         * This function gets the current version from the backend
         * and the latest version from the GitHub releases page.
         *
         * Returns:
         *    Promise -> an object with the current and latest versions
         */
        const curVersion = await invoke<string>("get_current_ver");
        const repo = await fetch("https://api.github.com/repos/Team-SolarEngine/solar-lanucher/tags");
        const tags = await repo.json();
        const curLatest = tags[0].name;
        return { curVersion, curLatest };
    }

    onMount(async () => {
        const { curVersion, curLatest } = await getVersion();
        version = curVersion;
        latest = curLatest;
        os = await getOS();
    })
</script>

<div style="display: flex; flex-direction: column; gap: 0.5rem">
    {#if version && latest && version == latest}
        <span>{#await getWord("settings.version.section1") then word}{@html word}{/await} {os}-{version}, <span style="color: green">{#await getWord("settings.version.section2.up_to_date") then word}{@html word}{/await}</span></span>
    {:else if version && latest && version != latest}
        <span>{#await getWord("settings.version.section1") then word}{@html word}{/await} {os}-{version}, <span style="color: red">{#await getWord("settings.version.section2.out_of_date") then word}{@html word}{/await}</span></span>
    {:else}
        <span>{#await getWord("settings.version.section2.limited") then word}{@html word}{/await}</span>
    {/if}
    <button onclick={() => openUrl("https://github.com/Team-SolarEngine/solar-lanucher/releases/latest")}>{#await getWord("settings.version.button") then word}{@html word}{/await}</button>
</div>