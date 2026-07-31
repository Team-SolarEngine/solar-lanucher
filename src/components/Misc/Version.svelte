<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { onMount } from "svelte";

    let version: string;
    let latest: string;

    async function getVersion() {
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
    {#if version == latest}
        <span>Your version is; {version}, <span style="color: green">which is up to date!</span></span>
    {:else if version != latest}
        <span>Your version is; {version}, <span style="color: red">which is not up to date...</span></span>
    {:else}
        <span>Loading version...</span>
    {/if}
    <button onclick={() => openUrl("https://github.com/Team-SolarEngine/solar-lanucher/releases/latest")}>Open releases</button>
</div>