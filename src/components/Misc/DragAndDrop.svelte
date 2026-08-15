<script lang="ts">
    import { getCurrentWebview } from "@tauri-apps/api/webview";
    import { onMount } from "svelte";
    import CardApp from "../CardApp.svelte";
    import { pickFile } from "$lib/interface";
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";

    let hoveringDrag = $state(false);
    let pathToCopy = $state();
    let pathToPaste = $state();
    let openDialogForInstances = $state(false);

    type AppData = {
        name: string;
        icon_url: string;
        execute_command: string;
        working_directory: string;
        description: string;
        banner_url: string;
    };

    let apps = $state<AppData[]>([]);

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    }); function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    async function loadApps() {
        /*
         * This function loads the list of apps from the backend
         * so the CardApp components have all the data they need.
         */
        try {
            apps = await invoke("get_keys", { collection: "apps" });
        } catch (e) {
            console.error("Failed to load apps:", e);
        }
    }

    async function copyFolderToDest() {
        /*
         * This function calls the backend code to paste the
         * directory to be copied and to the destination to paste.
         * Once everything is done, it'll close the dialog, and
         * clear pathToCopy and pathToPaste.
         */
        try {
            await invoke("paste_to_dir", { toCopy: pathToCopy, destToPaste: pathToPaste })
            openDialogForInstances = false;
            pathToCopy = "";
            pathToPaste = "";
        } catch (e) {
            useComponentSnackbarError(`Failed to copy mod: ${e}`)
        }
    }

    onMount(() => {
        loadApps();

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
</script>

<div class="_overlay" class:active={hoveringDrag}>
    <div class="_child">
        <i class="extra">download</i>
        <span>Drag that folder in!</span>
    </div>
</div>

<div class="overlay" class:active={openDialogForInstances} onclick={() => openDialogForInstances = false}></div>
<dialog class="right" class:active={openDialogForInstances}>
    <h6>The folder you are trying to paste is</h6>
    <span>{pathToCopy}</span>

    <hr class="medium"/>

    <h6>Importing a mod that's not engine modded?</h6>
    <span>Put your path where you wanna put it here!</span>

    <div class="field label prefix border">
        <a onclick={async () => pathToPaste = await pickFile([""], "Folder", true)}> <i>attach_file</i> </a>
        <input type="text" bind:value={pathToPaste}>
        <label>Path To Paste</label>
    </div>

    <button style="margin-top: 8px;" onclick={() => copyFolderToDest()}>
        <i>check</i>
        Confirm
    </button>

    <hr class="medium"/>

    <h6>Importing a mod that's engine modded?</h6>
    <span>Select one of these instances!</span>

    {#if apps.length > 0}
        {#each apps as app}
            <section onclick={() => { pathToPaste = app.working_directory + "/mods"; copyFolderToDest(); }}>
                <CardApp
                    name={app.name}
                    iconUrl={app.icon_url}
                    description={app.description}
                    isPreview={false}
                />
            </section>
        {/each}
    {:else}
        <span>No instances found.</span>
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