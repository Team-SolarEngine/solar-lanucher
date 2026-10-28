<script lang="ts">
    import { loadApps, type AppData } from "$lib/apps";
    import { getWord } from "$lib/localization";
    import { invoke } from "@tauri-apps/api/core";
    import CardApp from "../../CardApp.svelte";

    let { currentlyOpen = $bindable() } = $props();

    let apps = $state<AppData[]>([]);
    let focused_one = $state("");

    async function loadInstances() {
        /*
         * This function loads the list of apps from the backend
         * so the CardApp components have all the data they need.
         */
        try {
            apps = await loadApps();
        } catch (e) {
            console.error("Failed to load apps:", e);
        }
    }

    async function moveInstance(a: number, b: number) {
        /*
         * Swaps two instances locally, then persists both
         * indices with the existing update_key command.
         */
        const first = apps[a];
        const second = apps[b];
    
        apps[a] = second;
        apps[b] = first;

        focused_one = first.name;

        // I genuinely have no clue how this works
        // but if it works.. it works I guess?
        await invoke("update_key", { collection: "apps", key: a, value: apps[a] });
        await invoke("update_key", { collection: "apps", key: b, value: apps[b] });
    }

    $effect(() => {
        loadInstances();
    });
</script>

<div class="overlay" class:active={currentlyOpen} onclick={() => currentlyOpen = false}></div>
<dialog class="left" class:active={currentlyOpen} style="max-width: 700px;">
    {#if apps.length > 0}
        {#each apps as app, i}
            <article style="display: flex; gap: 8px;" class:_focused_one={app.name == focused_one}>
                <div style="flex: 1;">
                    <CardApp
                        name={app.name}
                        iconUrl={app.icon_url}
                        isPreview={true}
                    />
                </div>
                <div style="display: flex; justify-content: center; flex-direction: column;">
                    {#if i > 0}
                        <button onclick={() => moveInstance(i, i - 1)} class="transparent circle">
                            <i>arrow_upward</i>
                        </button>
                    {/if}
                    {#if i !== apps.length - 1 && apps.length > 1}
                        <button onclick={() => moveInstance(i, i + 1)} class="transparent circle">
                            <i>arrow_downward</i>
                        </button>
                    {/if}
                </div>
            </article>
        {/each}
    {:else}
        <span>{#await getWord("global.not_found") then word}{word.replace("{type}", "Instances")}{/await}</span>
    {/if}
</dialog>

<style>
    ._focused_one {
        background-color: rgb(31, 30, 33);
        box-shadow: 0 0 8px 4px rgb(31, 30, 33);
    }
</style>