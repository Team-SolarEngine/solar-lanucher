<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import { getWord } from "$lib/localization";

    let {
        executeCommand,
        workingDirectory,
        onDeleted = () => {},
        onEdit = () => {},
        index = -1,
        isLast = false,
        stretch = false,
    } = $props();

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    })

    function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    let deleteInstancePopup = $state(false);
    let extraFunctionalities = $derived([
        { name: "Open in Terminal", key: "start_extra.button.submenu.open_in_terminal", icon: "terminal", action: () => startApp(true) },
        { name: "Edit", key: "start_extra.button.submenu.edit", icon: "edit", action: () => onEdit(index) },
        { name: "Open Folder", key: "start_extra.button.submenu.open_folder", icon: "folder", action: openFolder },
        { name: "Delete", key: "global.delete", icon: "delete", action: () => deleteInstancePopup = true, extra: "right-round tertiary-text" },
    ])

    let deleteTypes = $derived([
        { name: "Delete Shortcut", key: "start_extra.button.submenu.delete_shortcut", action: () => deleteApp() },
        { name: "Delete Instance", key: "start_extra.button.submenu.delete_instance", action: () => deleteInstance() },
    ])

    async function startApp(openTerminal = false) {
        /*
         * This function starts the app by telling the backend to run
         * its execute command in its working directory.
         *
         * Arguments:
         *    openTerminal: boolean -> whether to open a terminal or run hidden
         */
        try {
            await invoke("start_app", {
                workingDir: workingDirectory,
                commandExec: executeCommand,
                openTerminal,
            });
        } catch (e) {
            useComponentSnackbarError(`Failed to start app: ${e}`);
        }
    }

    async function openFolder() {
        /*
         * This function tells the backend to open the app's
         * working directory in the system file explorer.
         */
        try {
            await invoke("open_folder", { path: workingDirectory });
        } catch (e) {
            useComponentSnackbarError(`Failed to open folder: ${e}`);
        }
    }

    async function deleteApp() {
        /*
         * This function removes the app from the collection
         * and tells the parent to refresh the app list.
         */
        try {
            await invoke("delete_key", { collection: "apps", key: index });
            onDeleted();
        } catch (e) {
            useComponentSnackbarError(`Failed to delete app: ${e}`);
        }
    }

    async function deleteInstance() {
        /*
         * This function deletes the instance from the working directory
         * and removes it from the collection.
         */
        try {
            await invoke("trash_folder", { modFolder: workingDirectory });
            await deleteApp();
        } catch (e) {
            useComponentSnackbarError(`Failed to delete instance: ${e}`);
        }
    }
</script>

<nav class="group split">
    <button class="border left-round primary" onclick={() => startApp()}>
      <i>play_arrow</i>
      <span>{#await getWord("start_extra.button.start") then word}{word}{/await}</span>
    </button>
    {#if !stretch}
        <div>
            <button class="border right-round square">
                {#if isLast}
                    <i>keyboard_arrow_up</i>
                {:else}
                    <i>keyboard_arrow_down</i>
                {/if}
            </button>
            <menu class="no-wrap" class:top={isLast}>
                {#each extraFunctionalities as functionality}
                    {#if functionality.name != "Delete"}
                        <li onclick={functionality.action}>
                            <i>{functionality.icon}</i> {#await getWord(functionality.key) then word}{word}{/await}
                        </li>
                    {/if}
                {/each}

                <hr class="small" />

                {#each deleteTypes as deleteType}
                    <li onclick={deleteType.action} class="tertiary-text">
                        <i>delete</i> {#await getWord(deleteType.key) then word}{word}{/await}
                    </li>
                {/each}
            </menu>
        </div>
    {:else}
        {#each extraFunctionalities as functionality}
            <!-- {#if functionality.name != "Delete" && functionality.name != "Edit"} -->
                <button class="border no-round {functionality.extra}" onclick={functionality.action}>
                    <i>{functionality.icon}</i>
                    <span>{#await getWord(functionality.key) then word}{word}{/await}</span>
                </button>
            <!-- {/if} -->
        {/each}
    {/if}
</nav>

<div class="overlay" class:active={deleteInstancePopup} onclick={() => deleteInstancePopup = false}></div>
<dialog class:active={deleteInstancePopup} style="overflow: visible !important;">
    <h3>{#await getWord("delete_instance.title") then word}{word}{/await}</h3>
    <span>{#await getWord("delete_instance.description") then word}{word}{/await}</span>

    <div class="row right-align">
        <button class="border no-round tertiary-text" onclick={() => deleteApp()}>
            <i>delete</i>
            <span>{#await getWord("delete_instance.button.delete_shortcut") then word}{word}{/await}</span>
            <span class="tooltip bottom">{#await getWord("delete_instance.button.delete_shortcut.tooltip") then word}{word}{/await}</span>
        </button>
        <button class="border no-round tertiary-text" onclick={() => deleteInstance()}>
            <i>delete</i>
            <span>{#await getWord("delete_instance.button.delete_instance") then word}{word}{/await}</span>
            <span class="tooltip bottom">{#await getWord("delete_instance.button.delete_instance.tooltip") then word}{word}{/await}</span>
        </button>
    </div>
</dialog>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>