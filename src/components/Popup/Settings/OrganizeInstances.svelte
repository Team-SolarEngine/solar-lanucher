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
        /*
         * This function starts the drag of the current element to
         * other element. It first makes a value to remember the draggedIndex (global),
         * then it would set the data, then tell the browser that this is a movable
         * item.
         */
        draggedIndex = index;

        event.dataTransfer?.setData("text/plain", String(index));
        if (event.dataTransfer) {
            event.dataTransfer.effectAllowed = "move";
        }
    }

    async function dropOn(targetIndex: number) {
        /*
         * This function makes it actually drop above the instance
         * you want to put it below it.
         * 
         * If the dragged index null, then it'll return nothing.
         * Then, it gets the currentIndex by making variable and
         * then makes draggedIndex to null.
         * 
         * When it hits while, it'll make a const variable of
         * nextIndex and it'll do simple math to do currentIndex - or + of
         * the current index, depending if it's big or small through true/false.
         * Then, it would move it, and then update currentIndex.
         * 
         * The while will keep looping until it's exactly the targetted index
         */
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
                ondragover={(e) => e.preventDefault()}
                ondrop={() => dropOn(i)}
                class="dragger"
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
    .dragger {
        display: flex;
        gap: 8px;
        cursor: grab !important;
        user-select: auto !important;
        -webkit-user-select: auto !important;
    }

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