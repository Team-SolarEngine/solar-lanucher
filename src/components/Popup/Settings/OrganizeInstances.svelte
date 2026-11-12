<script lang="ts">
    import { loadApps, type AppData } from "$lib/apps";
    import { getWord } from "$lib/localization";
    import { invoke } from "@tauri-apps/api/core";
    import CardApp from "../../CardApp.svelte";

    let { currentlyOpen = $bindable() } = $props();

    let apps = $state<AppData[]>([]);
    let focused_one = $state("");
    let draggedIndex = $state<number | null>(null);

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

    function startDrag(event: DragEvent, index: number) {
        draggedIndex = index;

        event.dataTransfer?.setData("text/plain", String(index));
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = "move";
        }
    }

    function allowDrop(event: DragEvent) {
        event.preventDefault();
    }

    async function dropOn(targetIndex: number) {
        if (draggedIndex === null) return;

        let currentIndex = draggedIndex;
        draggedIndex = null;

        while (currentIndex !== targetIndex) {
            const nextIndex = currentIndex + (targetIndex > currentIndex ? 1 : -1);
            await moveInstance(currentIndex, nextIndex);
            currentIndex = nextIndex;
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
        if (currentlyOpen) {
            void loadInstances();
        }
    });
</script>

<div class="overlay" class:active={currentlyOpen} onclick={() => currentlyOpen = false}></div>
<dialog class="left" class:active={currentlyOpen} style="max-width: 700px;">
    {#if apps.length > 0}
        {#each apps as app, i}
            <article
                draggable="true"
                ondragstart={(event) => startDrag(event, i)}
                ondragover={allowDrop}
                ondrop={() => dropOn(i)}
                style="display: flex; gap: 8px; cursor: grab !important;"
                class:_focused_one={app.name === focused_one}
            >
                <div
                    style="flex: 1;"
                    ondragstart={(event) => startDrag(event, i)}
                >
                    <CardApp
                        name={app.name}
                        iconUrl={app.icon_url}
                        isPreview={true}
                    />
                </div>


                <div style="display: flex; justify-content: center; flex-direction: column;">
                    <span
                        style="cursor: grab;"
                    >
                        <i>drag_indicator</i>
                    </span>
                </div>
            </article>
        {/each}
    {:else}
        <span>{#await getWord("global.not_found") then word}{word.replace("{type}", "Instances")}{/await}</span>
    {/if}
</dialog>

<style>
    ._focused_one::before {
        content: '';
        position: absolute;
        width: 100%;
        height: 100%;
        transform: translateX(-15px) translateY(-15px);
        background-color: rgba(255, 255, 255, 0.05);
        box-shadow: 0 0 8px 4px rgba(255, 255, 255, 0.05);
        border-radius: 8px;
    }
</style>