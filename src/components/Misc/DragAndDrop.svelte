<script lang="ts">
    import { getCurrentWebview } from "@tauri-apps/api/webview";
    import { getContext, onMount } from "svelte";
    import CardApp from "../CardApp.svelte";
    import { pickFile } from "$lib/interface";
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import { getWord } from "$lib/localization";
    import { loadApps, type AppData } from "$lib/apps";
    import { loadSetting } from "$lib/settings";

    let hoveringDrag = $state(false);
    let pathToCopy = $state();
    let pathToPaste = $state();
    let favouritePath = $state();
    let openDialogForInstances = $state(false);
    const localInstance: any = getContext("localInstance")

    let apps = $state<AppData[]>([]);

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    }); function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    async function loadInstances() {
        try {
            apps = await loadApps();
        } catch (e) {
            console.error("Failed to load apps:", e);
        }
    }

    async function copyFolderToDest(hardCoded: boolean = false,) {
        /*
         * This function calls the backend code to paste the
         * directory to be copied and to the destination to paste.
         * Once everything is done, it'll close the dialog, and
         * clear pathToCopy and pathToPaste.
         */
        try {
            await invoke("paste_to_dir", { toCopy: pathToCopy, destToPaste: pathToPaste })
            openDialogForInstances = false;
            const lastSlash = pathToCopy.lastIndexOf("/");
            const filePath = pathToCopy.slice(lastSlash + 1);

            if (hardCoded) {
                localInstance({
                    open: true,
                    appWorkingDirectory: `${pathToPaste}/${filePath}`,
                })
            }

            pathToCopy = "";
            pathToPaste = "";
        } catch (e) {
            useComponentSnackbarError(`Failed to copy mod: ${e}`)
        }
    }

    onMount(() => {
        let unlisten: () => void = () => {};

        getCurrentWebview().onDragDropEvent((event) => {
            if (event.payload.type === "enter") {
                hoveringDrag = true;
                console.log("Entering with files:", event.payload.paths);
            } else if (event.payload.type === "over") {
                if (!hoveringDrag) {
                    hoveringDrag = true
                    console.log("Hovering files");
                }
            } else if (event.payload.type === "drop") {
                console.log("Dropping files");
                pathToCopy = event.payload.paths[0];
                openDialogForInstances = true;
                loadInstances();
                hoveringDrag = false;
            } else {
                if (hoveringDrag) {
                    hoveringDrag = false
                    console.log("Cancelled drop");
                }
            }
        }).then((unlistenFn) => {
            unlisten = unlistenFn;
        });

        return () => {
            unlisten();
        };
    })

    $effect(() => {
        const loadedFavouritePath = async () => {
            favouritePath = await loadSetting("favouritePath") as any;
        }; loadedFavouritePath();
    })
</script>

<div class="_overlay" class:active={hoveringDrag}>
    <div class="_child">
        <i class="extra">download</i>
        <span>{#await getWord("drag_and_drop.overlay.text") then word}{@html word}{/await}</span>
    </div>
</div>

<div class="overlay" class:active={openDialogForInstances} onclick={() => openDialogForInstances = false}></div>
<dialog class="right" class:active={openDialogForInstances} style="max-width: 600px;">
    <h6>{#await getWord("drag_and_drop.title") then word}{@html word}{/await}</h6>
    <span>{pathToCopy}</span>

    <hr class="medium"/>

    <h6>{#await getWord("drag_and_drop.path_to_paste.title") then word}{@html word}{/await}</h6>
    <span>{#await getWord("drag_and_drop.path_to_paste.description") then word}{@html word}{/await}</span>

    <div class="field label prefix border">
        <a onclick={async () => pathToPaste = await pickFile([""], "Folder", true)}> <i>attach_file</i> </a>
        <input type="text" bind:value={pathToPaste}>
        <label>{#await getWord("drag_and_drop.path_to_paste.field.title") then word}{@html word}{/await}</label>
    </div>

    <div class="no-space">
        <button style="margin-top: 8px;" onclick={() => copyFolderToDest(true)}>
            <i>check</i>
            {#await getWord("global.confirm") then word}{@html word}{/await}
        </button>

        <button class="transparent" onclick={async () => pathToPaste = favouritePath} style="margin-top: 4px;">
            <i>content_copy</i>
            {#await getWord("global.use_favourite_path") then word}{@html word}{/await}
        </button>
    </div>

    <hr class="medium"/>

    <h6>{#await getWord("drag_and_drop.instances.title") then word}{@html word}{/await}</h6>
    <span>{#await getWord("drag_and_drop.instances.description") then word}{@html word}{/await}</span>

    {#if apps.length > 0}
        {#each apps as app}
            <section onclick={() => { pathToPaste = app.working_directory + "/mods"; copyFolderToDest(); }}>
                <CardApp
                    name={app.name}
                    iconUrl={app.icon_url}
                    description={app.description}
                    isPreview={true}
                />
            </section>
        {/each}
    {:else}
        <span>{#await getWord("global.no_instances_found") then word}{@html word}{/await}</span>
    {/if}
</dialog>

<style>
    ._overlay {
        position: fixed;
        inset: 0;
        opacity: 0%;
        border: 4px dashed var(--primary);
        background: color-mix(in srgb, var(--primary) 20%, transparent);
        z-index: 999;
        pointer-events: none;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: opacity 100ms ease-in-out;

        ._child {
            display: flex;
            flex-direction: column;
            align-items: center;
            font-size: 2rem;
            gap: 0.5rem;
        }
    }

    ._overlay.active {
        opacity: 100%;
    }
</style>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>
