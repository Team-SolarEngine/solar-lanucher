<script lang="ts">
    import { getOS } from "$lib/sys";
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";
    import { getWord } from "$lib/localization";

    let { returnVersionOnly = false } = $props()

    let version = $state("");
    let latest = $state("");
    let os = $state("");

    async function getLatestVersion() {
        /*
         * This function gets the current version from
         * the latest version from the GitHub releases page.
         *
         * Returns:
         *    Promise -> an object with the current and latest versions
         */
        const repo = await fetch("https://api.github.com/repos/Team-SolarEngine/solar-lanucher/tags");
        const tags = await repo.json();
        const curLatest = tags[0].name;
        return curLatest;
    }

    async function getLocalVersion() {
        /*
         * This functions get the current version from
         * the backend.
         */
        const curVersion = await invoke<string>("get_current_ver");
        return curVersion
    }

    onMount(async () => {
        version = await getLocalVersion();
        latest = await getLatestVersion();
        os = await getOS();
    })
</script>

{#if returnVersionOnly}
    <span>
        {#if version && latest && version != latest}
            <span style="color: rgba(255, 200, 200, 1);">{version}</span>
        {:else}
            <span>{version}</span>
        {/if}
    </span>
{:else}
    <div style="display: flex; flex-direction: column; gap: 0.5rem">
        <span>
            {#await getWord("settings.version.section1") then word}{@html word}{/await}
            {#if version && latest && version == latest}
                {os}-{version}, <span style="color: green">{#await getWord("settings.version.section2.up_to_date") then word}{@html word}{/await}</span>
            {:else if version && latest && version != latest}
                {os}-{version}, <span style="color: red">{#await getWord("settings.version.section2.out_of_date") then word}{@html word}{/await}</span>
            {:else}
                {version}! ... {#await getWord("settings.version.section2.limited") then word}{@html word}{/await}
            {/if}
        </span>
        <button onclick={() => openUrl("https://github.com/Team-SolarEngine/solar-lanucher/releases/latest")}>{#await getWord("settings.version.button") then word}{@html word}{/await}</button>
    </div>
{/if}