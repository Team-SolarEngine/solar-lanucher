<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";

    let version: string;
    let latest: string;

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
    })
</script>

<div style="display: flex; flex-direction: column; gap: 0.5rem">
    {#if version && latest && version == latest}
        <span>Your version is; {version}, <span style="color: green">which is up to date!</span></span>
    {:else if version && latest && version != latest}
        <span>Your version is; {version}, <span style="color: red">which is not up to date...</span></span>
    {:else}
        <span>It either could be loading, or you've hit github's rate limit!</span>
    {/if}
    <button onclick={() => openUrl("https://github.com/Team-SolarEngine/solar-lanucher/releases/latest")}>Open releases</button>
</div>