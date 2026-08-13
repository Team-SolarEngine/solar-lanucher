<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { openUrl } from "@tauri-apps/plugin-opener";

    let { modalDownload = $bindable() } = $props();

    let engines = $state<Array<{ name: string; imageUrl: string; releases: Array<{ tag: string; author: string; avatarUrl: string; downloads: Array<{ name: string; url: string; uploadedAt: string }> }> }>>([]);

    const FNF_Engines = [
        { name: "Solar Engine", url: "Team-SolarEngine/Solar-Engine-Archive", imageUrl: "https://github.com/Team-SolarEngine/Solar-Engine-Archive/raw/main/assets/exclude/images/universe.png" },
        { name: "Codename Engine", url: "CodenameCrew/CodenameEngine", imageUrl: "https://avatars.githubusercontent.com/u/122549339?s=200&v=4" },
        { name: "Psych Engine", url: "ShadowMario/FNF-PsychEngine", imageUrl: "https://shadowmario.github.io/psychengine.lua/assets/icon.ico" },
        { name: "Funkin", url: "FunkinCrew/Funkin", imageUrl: "https://avatars.githubusercontent.com/u/117059284?s=200&v=4" }
    ];

    async function loadSetting(key: string) {
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        return data?.[key];
    }

    async function getAllEngines() {
        engines = [];

        const token = await loadSetting("githubToken");
        for (const { name, url, imageUrl } of FNF_Engines) {
            try {
                const headers: Record<string, string> = {};
                if (token) headers["Authorization"] = `Bearer ${token}`;

                const response = await fetch(`https://api.github.com/repos/${url}/releases?per_page=100`, { headers });

                const releases = await response.json();
                if (!releases?.length) continue;

                engines.push({
                    name,
                    imageUrl,
                    releases: releases.map((release: any) => ({
                        name: release.name,
                        tag: release.tag_name,
                        author: release.author?.login ?? "unknown",
                        avatarUrl: release.author?.avatar_url ?? "",
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

    <hr class="medium" />

    <div style="margin-top: 5px;">
        <b style="font-size: 1.1rem">NOTE</b>; This will open a browser and download the engine from GitHub. <br/>
        You will have to manually put it in your preferred folder and extract it. <br/>
        I'm sorry, I can't find another way, I'm exhausted. <br/>
        <span style="font-size: 0.6rem; opacity: 0.8">- daveberry.</span>
    </div>

    <hr class="medium" />

    {#if engines.length === 0}
        <p>Loading...</p>
    {:else}
        {#each engines as engine}
            <article>
                <details>
                    <summary style="margin: 10px 0; display: flex; justify-content: space-between; align-items: center; ">
                        <div style="display: flex; gap: 10px; align-items: center;">
                            <img style="width: 64px; height: 64px; border-radius: 5px;" src={engine.imageUrl} alt={engine.name}/>
                            <span style="font-size: 2rem;">{engine.name}</span>
                        </div>
                        <i>arrow_drop_down</i>
                    </summary>
                    <div style="display: flex; flex-direction: column; gap: 10px">
                        {#each engine.releases as release}
                            <details>
                                <summary>
                                    <article style="display: flex; justify-content: space-between; align-items: center; ">
                                        <div style="display: flex; gap: 10px; align-items: center;">
                                            <img style="width: 50px; height: 50px; border-radius: 5px;" src={release.avatarUrl} alt={engine.name}/>
                                            <div>
                                                <h6 style="font-weight: bold">{release.tag}</h6>
                                                Published by - {release.author}
                                            </div>
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