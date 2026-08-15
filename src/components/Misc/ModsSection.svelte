<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import imageSrc from "$lib/imageSrc";

    let {
        workingDirectory,
        modsFolder = "mods",
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

            const enabledFolders: Array<string> = await invoke("list_mods", { workingDirectory, modsFolder });
            for (const modFolder of enabledFolders) {
                mods.push({ ...(await readMod(modFolder)), enabled: true });
            }

            const disabledFolders: Array<string> = await invoke("list_mods", { workingDirectory, modsFolder: "disabled-mods" });
            for (const modFolder of disabledFolders) {
                mods.push({ ...(await readMod(modFolder)), enabled: false });
            }
        } catch (error) {
            useComponentSnackbarError(`Failed to list mods: ${error}`);
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

    $effect(() => {
        listMods();
    });
</script>

<article>
    <h5>Mods</h5>
    {#if mods.length > 0}
        {#each mods as mod}
            <article style={mod.enabled ? "" : "opacity: 0.5;"}>
                <div class="row">
                    {#if mod.icon}
                        <img src={imageSrc(mod.icon)} class="large" />
                    {/if}
                    <div>
                        <h6>{mod.name}</h6>
                        <span>{mod.description}</span>
                    </div>
                    <div class="max"></div>

                    <div class="no-space row">
                        <button class="transparent circle" onclick={() => trashMod(mod.folder)}><i>delete</i></button>
                        <button class="transparent circle" onclick={() => openFolder(mod.folder)}><i>folder</i></button>
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
</article>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>