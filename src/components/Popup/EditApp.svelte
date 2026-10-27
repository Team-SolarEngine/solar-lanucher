<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import CardApp from "../CardApp.svelte";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import { getWord } from "$lib/localization";
    import { getOS } from "$lib/sys";

    let { modalEdit = $bindable(), editingApp = $bindable(), editIndex = -1, onAppEdited = () => {} } = $props()

    let appName = $state("")
    let appIconURL = $state("")
    let appPath = $state("")
    let appWorkingDirectory = $state("")
    let appDescription = $state("")
    let bannerURL = $state("")

    let submitted = $state(false)
    let showExplorer = $state(false)
    let folderContents: Array<{ path: string; is_folder: boolean }> = $state([])

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    }); function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    $effect(() => {
        if (editingApp) {
            appName = editingApp.name
            appIconURL = editingApp.icon_url
            appPath = editingApp.execute_command
            appWorkingDirectory = editingApp.working_directory
            appDescription = editingApp.description
            bannerURL = editingApp.banner_url
        }
    })

    function close() {
        /*
         * This function closes the edit popup and clears the edited app.
         */
        modalEdit = false;
        editingApp = null;
        submitted = false;
        showExplorer = false;
    }

    async function saveApp() {
        /*
         * This function saves the edited app by sending
         * the form values to the backend, then closes the popup.
         */
        submitted = true;
        showExplorer = false;
        if (!appName || !appPath || !appWorkingDirectory) return useComponentSnackbarError(`Please fill in all fields.`);

        try {
            await invoke("update_key", {
                collection: "apps",
                key: editIndex,
                value: {
                    name: appName,
                    icon_url: appIconURL,
                    execute_command: appPath,
                    description: appDescription,
                    banner_url: bannerURL,
                    working_directory: appWorkingDirectory,
                }
            });
            modalEdit = false;
            editingApp = null;
            submitted = false;
            onAppEdited();
        } catch (e) {
            useComponentSnackbarError(`Failed to update app: ${e}`);
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

    async function setPathForOS(path: string, is_file: boolean) {
        /*
         * This function sets a path for the specific os for
         * the execute command. This is especially helpful for
         * first comers or people who doesn't know what they're doing!
         * 
         * If is_file is false, then it'll return nothing.
         */

        if (!is_file) return

        if (await getOS() == "windows")
            return appPath = `.\\${path}`
        else
          if (path.includes(".exe"))
              return appPath = `wine ./${path}`
          return appPath = `./${path}`
    }
</script>

<div class="overlay" class:active={modalEdit} onclick={close}></div>
<dialog class="right" class:active={modalEdit}>
  <h5>{#await getWord("edit.title") then word}{word.replace("{instance}", appName)}{/await}</h5>
  <span></span>

  <div class="field label border" class:invalid={submitted && !appName}>
    <input type="text" bind:value={appName}>
    <label>{#await getWord("local_or_edit.field.mod_or_engine_name.title") then word}{@html word}{/await} <span style="color: red;">*</span></label>
    <output>{#await getWord("local_or_edit.field.mod_or_engine_name.description") then word}{@html word}{/await}</output>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appIconURL}>
    <label>{#await getWord("local_or_edit.field.icon_path.title") then word}{@html word}{/await}</label>
    <output>{#await getWord("local_or_edit.field.icon_path.description") then word}{@html word}{/await}</output>
  </div>

  <div class="field label border" class:invalid={submitted && !appPath}>
    <input type="text" bind:value={appPath}>
    <label>{#await getWord("local_or_edit.field.execute_command.title") then word}{@html word}{/await} <span style="color: red;">*</span></label>
    <output>{#await getWord("local_or_edit.field.execute_command.description") then word}{@html word}{/await}</output>
  </div>

  <div class="field label border" class:invalid={submitted && !appWorkingDirectory}>
    <input type="text" bind:value={appWorkingDirectory}>
    <label>{#await getWord("local_or_edit.field.working_directory.title") then word}{@html word}{/await} <span style="color: red;">*</span></label>
    <output>{#await getWord("local_or_edit.field.working_directory.description") then word}{@html word}{/await}</output>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appDescription}>
    <label>{#await getWord("local_or_edit.field.description.title") then word}{@html word}{/await}</label>
    <output>{#await getWord("local_or_edit.field.description.description") then word}{@html word}{/await}</output>
  </div>

  <div class="field label border">
    <input type="text" bind:value={bannerURL}>
    <label>{#await getWord("local_or_edit.field.banner_url.title") then word}{@html word}{/await}</label>
    <output>{#await getWord("local_or_edit.field.banner_url.description") then word}{@html word}{/await}</output>
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
    <button class="transparent link" onclick={close}>{#await getWord("global.cancel") then word}{@html word}{/await}</button>
    <button class="transparent link" onclick={toggleExplorer}>{#await getWord("global.explorer") then word}{@html word}{/await}</button>
    <button class="primary link" onclick={saveApp}>{#await getWord("global.save") then word}{@html word}{/await}</button>
  </nav>
</dialog>

<article class:active={showExplorer} class="_explorer scroll" style="max-width: 600px; position: absolute; top: 0; bottom: 0; left: 0; z-index: 999; margin-bottom: 12px; margin-left: 18px;">
    <h6>{#await getWord("global.explorer") then word}{@html word}{/await}</h6>
    <span>{#await getWord("local_or_edit.submenu.explorer.description") then word}{@html word}{/await}</span> <br/>
    <article style="background-color: rgb(33, 32, 35);">
        {#if folderContents.length === 0}
            <span>{#await getWord("global.not_found") then word}{@html word.replace("{type}", "Folder contents")}{/await}</span>
        {:else}
            {#each folderContents as folder}
                <div style="padding: 5px;" onclick={() => setPathForOS(getItemName(folder.path), !folder.is_folder)} class:_selectable={!folder.is_folder}>
                    <i>{folder.is_folder ? "folder" : "description"}</i>
                    <span style="overflow-wrap: break-word; word-break: break-word; display: inline-block;">{getItemName(folder.path)}</span>
                </div>
            {/each}
        {/if}
    </article>
</article>

<div class="snackbar error" class:active={snackbar.snackbarError || submitted && !appName}>{snackbar.givenError}</div>

<style>
    ._explorer {
        transform: translateX(-620px);
        transition: transform 200ms ease-out;

        &.active {
            transform: translateX(0px);
        }
    }

    ._selectable {
        transition: all 0.1s ease-in-out;
        &:hover {
            background-color: rgb(48, 47, 50);
            border-radius: 4px;
            cursor: pointer;
        }
    }
</style>
