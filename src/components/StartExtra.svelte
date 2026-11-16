<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import { getWord } from "$lib/localization";
    import { getContext, onMount } from "svelte";
    import {
      addInstanceState,
      getInstancePids,
      instanceIsRunning,
      removeInstanceState
    } from "$lib/instance.svelte";

    const terminalDialog: any = getContext('terminal')
    let {
        executeCommand = "",
        workingDirectory = "",
        onDeleted = () => {},
        onEdit = () => {},
        index = -1,
        isLast = false,
        stretch = false,
        firstTimePopup,
    } = $props();

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    })

    function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    let isRunning = $state(false);
    let checking = $state(false);
    let disposed = $state(false);

    let deleteInstancePopup = $state(false);
    let pid = $state(0);

    const check = async () => {
        if (checking) return;
        checking = true;

        try {
            const pids = getInstancePids(workingDirectory);

            const results = await Promise.all(
                pids.map(async (pid) => {
                    const running = await invoke("pid_exists", { pid });
                    return { pid, running };
                })
            );

            if (disposed) return;

            for (const result of results) {
                if (!result.running) {
                    removeInstanceState(result.pid);
                }
            }

            isRunning = instanceIsRunning(workingDirectory);
        } catch (e) {
            console.error("Failed to check app processes:", e);
            if (!disposed) {
                isRunning = instanceIsRunning(workingDirectory);
            }
        } finally {
            checking = false;
        }
    };

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
            console.log(`currrent pid: ${pid}`)
            if (instanceIsRunning(workingDirectory)) {
                const pids = getInstancePids(workingDirectory);

                for (const instancePid of pids) {
                    if (instancePid <= 1) continue;
            
                    await invoke("kill_app", { pid: instancePid });
                    removeInstanceState(instancePid);
                }

                isRunning = instanceIsRunning(workingDirectory);
                return;
            }

            pid = await invoke("start_app", {
                workingDir: workingDirectory,
                commandExec: executeCommand,
                openTerminal,
            });
            addInstanceState(pid, workingDirectory)
            if (openTerminal) terminalDialog({ open: true, pid })
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

    onMount(() => {
        void check();
        const timer = setInterval(() => void check(), 1000);

        return () => {
            disposed = true;
            clearInterval(timer);
        };
    });
</script>

<nav class="group split">
    <button class="border left-round primary" class:tertiary={isRunning} onclick={() => startApp()}>
      <i>
          {#if isRunning}
              stop_circle
          {:else}
              play_arrow
          {/if}
      </i>
      <span>
            {#if !firstTimePopup}
                {#if isRunning}
                    {#await getWord("start_extra.button.stop") then word}{@html word}{/await}
                {:else}
                    {#await getWord("start_extra.button.start") then word}{@html word}{/await}
                {/if}
            {:else}
                Start
            {/if}
      </span>
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
            {#if !firstTimePopup}
                <menu class="no-wrap" class:top={isLast}>
                    {#each extraFunctionalities as functionality}
                        {#if functionality.name != "Delete"}
                            <li onclick={functionality.action}>
                                <i>{functionality.icon}</i> {#await getWord(functionality.key) then word}{@html word}{/await}
                            </li>
                        {/if}
                    {/each}
    
                    <hr class="small" />
    
                    {#each deleteTypes as deleteType}
                        <li onclick={deleteType.action} class="tertiary-text">
                            <i>delete</i> {#await getWord(deleteType.key) then word}{@html word}{/await}
                        </li>
                    {/each}
                </menu>
            {/if}
        </div>
    {:else}
        {#each extraFunctionalities as functionality}
            <!-- {#if functionality.name != "Delete" && functionality.name != "Edit"} -->
                <button class="border no-round {functionality.extra}" onclick={functionality.action}>
                    <i>{functionality.icon}</i>
                    <span>{#await getWord(functionality.key) then word}{@html word}{/await}</span>
                </button>
            <!-- {/if} -->
        {/each}
    {/if}
</nav>

<div class="overlay" class:active={deleteInstancePopup} onclick={() => deleteInstancePopup = false}></div>
<dialog class:active={deleteInstancePopup} style="overflow: visible !important;">
    <h3>{#await getWord("delete_instance.title") then word}{@html word}{/await}</h3>
    <span>{#await getWord("delete_instance.description") then word}{@html word}{/await}</span>

    <div class="row right-align">
        <button class="border no-round tertiary-text" onclick={() => deleteApp()}>
            <i>delete</i>
            <span>{#await getWord("delete_instance.button.delete_shortcut") then word}{@html word}{/await}</span>
            <span class="tooltip bottom">{#await getWord("delete_instance.button.delete_shortcut.tooltip") then word}{@html word}{/await}</span>
        </button>
        <button class="border no-round tertiary-text" onclick={() => deleteInstance()}>
            <i>delete</i>
            <span>{#await getWord("delete_instance.button.delete_instance") then word}{@html word}{/await}</span>
            <span class="tooltip bottom">{#await getWord("delete_instance.button.delete_instance.tooltip") then word}{@html word}{/await}</span>
        </button>
    </div>
</dialog>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>