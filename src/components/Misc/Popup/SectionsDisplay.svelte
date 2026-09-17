<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import CardApp from "../../../components/CardApp.svelte";
    import { imageSrc } from "$lib/sys";

    let {
        workingDirectory,
        modsFolder = "mods",
        moduleOpen = false,
        onclose = () => {},
    } = $props()

    let mods = $state<Array<{
        folder: string;
        name: string;
        description: string;
        color: string;
        icon: string;
        enabled: boolean;
    }>>([]);

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    }); function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    let instancesOfInstnaces = $state([]);
    let instancesDialog = $state(false);
    let selectedModFolder = $state("");

    async function loadSetting(key: string) {
        /*
         * This function loads a single setting value from the backend.
         *
         * Arguments:
         *    key: string -> the name of the setting to load
         *
         * Returns:
         *    Promise -> the value of the setting, or null
         */
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        return data?.[key];
    }

    async function openFolder(workingDirectory: string) {
        /*
         * This function tells the backend to open the app's
         * working directory in the system file explorer.
         * Inherited from @Settings.svelte
         */
        try {
            await invoke("open_folder", { path: workingDirectory });
        } catch (e) {
            useComponentSnackbarError(`Failed to open folder: ${e}`);
        }
    }

    async function listMods() {
        /*
         * This function lists all the mods from both the mods folder
         * and the disabled-mods folder, combined into a single list.
         * Each mod keeps an enabled flag so it can be toggled.
         *
         * Returns:
         *     mods: Array -> An array of mod objects.
         */

        try {
            mods = [];

            const enabledFolderFinal = `${workingDirectory}/${modsFolder}`
            const enabledFolders: Array<{ path: string; is_folder: boolean }> = await invoke("list_folder", { workingDirectory: enabledFolderFinal, showFoldersOnly: true });
            for (const modFolder of enabledFolders) {
                mods.push({ ...(await readMod(modFolder.path)), enabled: true });
            }

            let disabledFolderFinal = `${workingDirectory}/disabled-${modsFolder}`;
            const disabledFolders: Array<{ path: string; is_folder: boolean }> = await invoke("list_folder", { workingDirectory: disabledFolderFinal, showFoldersOnly: true });
            for (const modFolder of disabledFolders) {
                mods.push({ ...(await readMod(modFolder.path)), enabled: false });
            }
        } catch (error) {
            useComponentSnackbarError(`Failed to list ${modsFolder}: ${error}`);
        }
    }

    async function readMod(modFolder: string) {
        /*
         * This function reads a single mod folder and returns a mod object.
         * It checks for pack.json in the folder to figure out
         * which engine the mod belongs to:
         * if it exists, the mod is handled by Psych Engine.
         * if not, the mod is just listed by its folder name.
         */
        if (await hasFileFromEngine(modFolder, "pack.json")) {
            const content = await invoke("get_file_content", { path: `${modFolder}/pack.json` }) as string;
            const pack = JSON.parse(content);
            return {
                folder: modFolder,
                name: pack.name ?? modFolder,
                description: pack.description ?? "",
                color: pack.color ?? "",
                icon: `${modFolder}/pack.png`,
            };
        } else {
            return {
                folder: modFolder,
                name: modFolder.split(/[/\\]/).pop() ?? modFolder,
                description: "",
                color: "",
                icon: "",
            };
        }
    }

    async function hasFileFromEngine(modFolder: string, fileName: string): Promise<boolean> {
        /*
         * This function checks if a mod folder has a pack.json file,
         * which means it is a Psych Engine mod.
         */
        try {
            await invoke("get_file_content", { path: `${modFolder}/${fileName}` });
            return true;
        } catch {
            return false;
        }
    }

    async function toggleMod(mod: { folder: string }, enable: boolean) {
        /*
         * This function moves a mod between the mods folder and the
         * disabled-mods folder, then refreshes both lists.
         */
        try {
            await invoke("toggle_mod", {
                modFolder: mod.folder,
                workingDirectory,
                modsFolder,
                enable,
                typeOf: modsFolder,
            });
            await listMods();
        } catch (error) {
            useComponentSnackbarError(`Failed to toggle mod: ${error}`);
        }
    }

    async function trashMod(modFolder: string) {
        /*
         * This functions calls the Tauri backend to
         * trash the folder given from the arguments.
         * 
         * Arguments:
         *    modFolder: string -> The folder you want to delete
         */

        try {
            await invoke("trash_folder", { modFolder });
            await listMods();
        } catch(e) {
            useComponentSnackbarError(`Failed to trash folder: ${e}`)
        }
    }

    async function openEditor(workingDirectory: string) {
        try {
            let editorCMD = await loadSetting("codeEditor")
            let command = `${editorCMD} "${workingDirectory}"`
            console.log(command)

            await invoke("run_command", { command, createTerminalWindow: false, workingDir: "." })
        } catch(e) {
            useComponentSnackbarError(`Failed to open editor: ${e}`)
        }
    }

    async function transferToInstance(modFolder: string, destToPaste: string) {
        /*
         * Transfers a mod to the specified instance
         * 
         * Arguments:
         *    modFolder: string -> The folder you want to transfer
         *    destToPaste: string -> The destination folder to paste the mod into
         */
        try {
            await invoke("paste_to_dir", { toCopy: modFolder, destToPaste})
            await trashMod(modFolder)
            instancesDialog = false;
        } catch(e) {
            useComponentSnackbarError(`Failed to transfer mod folder: ${e}`)
            console.error(e)
        }
    }

    function openInstancesDialog(modFolder: string) {
        /*
         * Opens the instnaces dialog and sets a valid variable for
         * `selectedModFolder`. This is mainly for passthrough.
         * 
         * Arguments:
         *    modFolder: string -> The folder you want to transfer
         */
        selectedModFolder = modFolder;
        instancesDialog = true;
    }

    async function loadApps() {
        /*
         * This function loads the list of apps from the backend
         * so the CardApp components have all the data they need.
         */
        try {
            instancesOfInstnaces = await invoke("get_keys", { collection: "apps" });
        } catch (e) {
            console.error("Failed to load apps:", e);
        }
    }

    $effect(() => {
        listMods();
        loadApps();
    });
</script>

<div class="overlay" class:active={moduleOpen} onclick={onclose}></div>
<dialog class="left" class:active={moduleOpen} style="max-width: 50rem">
    <div class="row">
        <!-- sourcery shit what the FUCK -->
        <h5>{modsFolder.charAt(0).toUpperCase() + modsFolder.slice(1)}</h5>
        <div class="row no-space">
            <button class="transparent circle" onclick={() => openFolder(`${workingDirectory}/${modsFolder}`)}><i>folder</i></button>
            <button class="transparent circle" onclick={() => listMods()}><i>refresh</i></button>
        </div>
        <div class="max"></div>
        <button class="transparent circle" onclick={() => onclose()}><i>close</i></button>
    </div>
    {#if mods.length > 0}
        {#each mods as mod}
            <article style={mod.enabled ? "" : "opacity: 0.5;"}>
                <div class="row">
                    {#if mod.icon}
                        <img src={imageSrc(mod.icon)} class="large" />
                    {/if}
                    <div style="flex: 1; min-width: 0;">
                        <h6>{mod.name}</h6>
                        {#if mod.description}
                            <span style="overflow-wrap: break-word; word-break: break-word; display: inline-block;">
                                {mod.description}
                            </span>
                        {/if}
                    </div>

                    <div class="no-space row" style="margin-left: 12px;">
                        <button class="transparent circle" onclick={() => trashMod(mod.folder)}><i>delete</i></button>
                        <button class="transparent circle" onclick={() => openFolder(mod.folder)}><i>folder</i></button>
                        <button class="transparent circle" onclick={() => openEditor(mod.folder)}><i>code</i></button>
                        <button class="transparent circle" onclick={() => openInstancesDialog(mod.folder)}><i>compare_arrows</i></button>
                        <label class="checkbox large">
                            <input type="checkbox" checked={mod.enabled} onchange={() => toggleMod(mod, !mod.enabled)} />
                            <span></span>
                        </label>
                    </div>
                </div>
            </article>
        {/each}
    {:else}
        <span>Empty...</span>
    {/if}
</dialog>

<div class="overlay" class:active={instancesDialog} onclick={() => instancesDialog = false}></div>
<dialog class:active={instancesDialog} class="right" style="max-width: 750px">
    <h4>Select an instance to transfer!</h4>
    <article class="tertiary">
        <h3>Please note;</h3>
        <span>
            This will transfer the <span class="code">{selectedModFolder}</span> folder to the selected instance.
            If you're transferring to <span class="code">mods</span>, you will have to transfer it yourself.
        </span>
    </article>

    <hr class="medium" />

    {#if instancesOfInstnaces.length > 0}
        {#each instancesOfInstnaces as app}
            <section onclick={() => transferToInstance(selectedModFolder, app.working_directory + "/" + modsFolder)}>
                <CardApp
                    name={app.name}
                    iconUrl={app.icon_url}
                    description={app.description}
                    isPreview={true}
                />
            </section>
        {/each}
    {:else}
        <span>No instances found.</span>
    {/if}
</dialog>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>

<style>
    .code {
        background-color: rgba(0, 0, 0, 0.25);
        border-radius: 4px;
        padding: 0 0.25rem
    }
</style>