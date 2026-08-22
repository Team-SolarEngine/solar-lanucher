<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import CardApp from "../../CardApp.svelte";
    import { useSnackbarError, type Snackbar, pickFile } from "$lib/interface";

    let { modalNew = $bindable(), onAppAdded = () => {}, prefill = {} } = $props()

    let appName = $state("")
    let appIconURL = $state("")
    let appPath = $state("")
    let appWorkingDirectory = $state("")
    let appDescription = $state("")
    let bannerURL = $state("")

    let submitted = $state(false)
    let showExplorer = $state(false)
    let folderContents: Array<{ path: string; is_folder: boolean }> = $state([]);
    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    })

    function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    $effect(() => {
        if (modalNew && prefill && Object.keys(prefill).length > 0) {
            appName = prefill.name ?? "";
            appIconURL = prefill.iconUrl ?? "";
            appPath = prefill.executeCommand ?? "";
            appWorkingDirectory = prefill.workingDirectory ?? "";
            appDescription = prefill.description ?? "";
            bannerURL = prefill.bannerUrl ?? "";
        }
    })

    function close() {
        /*
         * This function closes the popup and clears all the form fields.
         */
        modalNew = false;
        submitted = false;
        showExplorer = false;
        setTimeout(() => {
            appName = ""
            appIconURL = ""
            appPath = ""
            appWorkingDirectory = ""
            appDescription = ""
            bannerURL = ""
        }, 500)
    }

    async function addApp() {
        /*
         * This function adds a new app by sending the form values
         * to the backend, then closes the popup and refreshes the list.
         */
        submitted = true;
        if (!appName || !appPath || !appWorkingDirectory) return useComponentSnackbarError(`Please fill in all fields.`);

        try {
            await invoke("add_key", {
                collection: "apps",
                value: {
                    name: appName,
                    icon_url: appIconURL,
                    execute_command: appPath.replace("C:\\fakepath\\", "./"),
                    working_directory: appWorkingDirectory,
                    description: appDescription,
                    banner_url: bannerURL,
                }
            });
            modalNew = false;
            submitted = false;
            showExplorer = false;
            setTimeout(() => {
                appName = ""
                appIconURL = ""
                appPath = "".replace("C:\\fakepath\\", "./")
                appWorkingDirectory = ""
                appDescription = ""
                bannerURL = ""
            }, 500);
            onAppAdded();
        } catch (e) {
            useComponentSnackbarError(`Failed to add app: ${e}`);
        }
    }

    async function showFolderContents() {
        /*
         * This function lists the contents of the current working
         * directory and stores them so the explorer can show them.
         */
        if (!appWorkingDirectory) return;
        folderContents = [];
        try {
            folderContents = await invoke("list_folder", { workingDirectory: appWorkingDirectory, showFoldersOnly: false });
        } catch (e) {
            useComponentSnackbarError(`Failed to list contents: ${e}`)
        }
    }

    function getItemName(path: string) {
        /*
         * This function extracts the name of a file or folder
         * by grabbing everything after the last slash or backslash.
         *
         * Arguments:
         *    path: string -> the full path
         *
         * Returns:
         *    string -> just the name at the end of the path
         */
        const match = path.match(/[^/\\]+$/);
        return match ? match[0] : path;
    }

    function toggleExplorer() {
        /*
         * This function opens or closes the folder explorer,
         * loading the current folder's contents when it opens.
         */
        showExplorer = !showExplorer;
        if (showExplorer) showFolderContents();
    }
</script>

<div class="overlay" class:active={modalNew} onclick={close}></div>
<dialog class="right" class:active={modalNew}>
  <h5>Add a New FNF Instance</h5>
  <span><b>Pro tip</b>; Click on the clip icon for a file explorer!</span>

  <div class="field label border" class:invalid={submitted && !appName}>
    <input type="text" bind:value={appName}>
    <label>FNF Mod/Engine name <span style="color: red;">*</span></label>
    <output>The FNF mod/engine name to display in the launcher.</output>
  </div>

  <div class="field label prefix border">
    <a onclick={async () => appIconURL = await pickFile(["png", "gif", "jpeg"], "Icon")}> <i>attach_file</i> </a>
    <input type="text" bind:value={appIconURL}>
    <label>Icon Path</label>
    <output>Any icon your heart desires. Make sure it's 1:1. It can be URL or Path.</output>
  </div>

  <div class="field label border" class:invalid={submitted && !appPath}>
    <input type="text" bind:value={appPath}>
    <label>Execute Command <span style="color: red;">*</span></label>
    <output>The command to execute when launching the app. eg; <code>.\Funkin.exe</code></output>
  </div>

  <div class="field label prefix border" class:invalid={submitted && !appWorkingDirectory}>
    <a onclick={async () => appWorkingDirectory = await pickFile([], "Folder", true)}> <i>attach_file</i> </a>
    <input type="text" bind:value={appWorkingDirectory}>
    <label>Working Directory <span style="color: red;">*</span></label>
    <output>The working directory for the app. eg; <code>D:\Games\FNF\Funkin</code></output>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appDescription}>
    <label>Description</label>
    <output>The description of the app to display in the launcher. eg; <code>Base game FNF V-Slice</code></output>
  </div>

  <div class="field label prefix border">
    <a onclick={async () => bannerURL = await pickFile(["png", "gif", "jpeg"], "Folder")}> <i>attach_file</i> </a>
    <input type="text" bind:value={bannerURL}>
    <label>Banner URL</label>
    <output>The banner image URL or Path for the app. Aspect ratio doesn't matter. But we reccomend 16:9.</output>
  </div>

  <CardApp
      name={appName || "App Name"}
      iconUrl={appIconURL || "https://placehold.co/128x128"}
      executeCommand=""
      workingDirectory={appWorkingDirectory || "/"}
      description={appDescription || ""}
      isPreview={true}
  />

  <nav class="right-align no-space">
    <button class="primary link" onclick={addApp}>Add App</button>
    <button class="transparent link" onclick={toggleExplorer}>Explorer</button>
    <button class="transparent link" onclick={close}>Close</button>
  </nav>
</dialog>

<article class:active={showExplorer} class="_explorer scroll" style="max-width: 600px; position: absolute; top: 0; bottom: 0; left: 0; z-index: 999; margin-bottom: 12px; margin-left: 18px;">
    <h6>Folder Contents</h6>
    {#if folderContents.length === 0}
        <span>No folders found here.</span>
    {:else}
        <div>
            {#each folderContents as folder}
                <div style="margin-top: 10px;">
                    <i>{folder.is_folder ? "folder" : "description"}</i>
                    <span style="overflow-wrap: break-word; word-break: break-word; display: inline-block;">{getItemName(folder.path)}</span>
                </div>
                <hr class="small"/>
            {/each}
        </div>
    {/if}
</article>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>

<style>
    ._explorer {
        transform: translateX(-620px);
        transition: transform 200ms ease-out;

        &.active {
            transform: translateX(0px);
        }
    }
</style>