<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";

    let { modalDownload = $bindable() } = $props();

    let engines = $state<Array<{ name: string; releases: Array<{ tag: string; author: string; downloads: Array<{ name: string; url: string; uploadedAt: string }> }> }>>([]);

    const FNF_Engines = [
        { name: "Solar Engine", url: "Team-SolarEngine/Solar-Engine-Archive" },
        { name: "Codename Engine", url: "CodenameCrew/CodenameEngine" },
        { name: "Psych Engine", url: "ShadowMario/FNF-PsychEngine" },
        { name: "Funkin", url: "FunkinCrew/Funkin" }
    ];

    async function loadSetting(key: string) {
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        return data?.[key];
    }

    async function getAllEngines() {
        engines = [];

        const token = await loadSetting("githubToken");
        for (const { name, url } of FNF_Engines) {
            try {
                const headers: Record<string, string> = {};
                if (token) headers["Authorization"] = `Bearer ${token}`;

                const response = await fetch(`https://api.github.com/repos/${url}/releases?per_page=100`, { headers });

                const releases = await response.json();
                if (!releases?.length) continue;

                engines.push({
                    name,
                    releases: releases.map((release: any) => ({
                        name: release.name,
                        tag: release.tag_name,
                        author: release.author?.login ?? "unknown",
                        downloads: (release.assets ?? []).map((asset: any) => ({
                            name: asset.name,
                            url: asset.browser_download_url,
                            uploadedAt: asset.created_at,
                        })),
                    })),
                });
            } catch (e) {
                console.error(`Failed to fetch ${name}:`, e);
            }
        }
    }

    $effect(() => {
        if (modalDownload) getAllEngines();
    });
</script>

<div class="overlay" class:active={modalDownload} onclick={() => modalDownload = false}></div>
<dialog class="right" class:active={modalDownload}>
    <h5>Download an engine!</h5>
    <div>Whether that's Solar, Codename, Psych or Vanilla Funkin, we support it!</div>

    {#if engines.length === 0}
        <p>Loading...</p>
    {:else}
        {#each engines as engine}
            <article>
                <details>
                    <summary style="margin: 10px 0; display: flex; justify-content: space-between; align-items: center; ">
                        <h5>{engine.name}</h5>
                        <i>arrow_drop_down</i>
                    </summary>
                    <div style="display: flex; flex-direction: column; gap: 10px">
                        {#each engine.releases as release}
                            <details>
                                <summary>
                                    <article style="display: flex; justify-content: space-between; align-items: center; ">
                                        <div>
                                            <h6 style="font-weight: bold">{release.tag}</h6>
                                            Published by - {release.author}
                                        </div>
                                        <i>arrow_drop_down</i>
                                    </article>
                                </summary>
                                <div style="margin-left: 20px; margin-top: 10px;">
                                    {#if release.downloads.length === 0}
                                        <p>No download assets on this release.</p>
                                    {:else}
                                        {#each release.downloads as download}
                                            <article onclick={() => openUrl(download.url)} style="cursor: pointer">
                                                <i>download</i> <span><b>{download.name}</b> - {new Date(download.uploadedAt).toLocaleString()}</span>
                                            </article>
                                        {/each}
                                    {/if}
                                </div>
                            </details>
                        {/each}
                    </div>
                </details>
            </article>
        {/each}
    {/if}
</dialog>