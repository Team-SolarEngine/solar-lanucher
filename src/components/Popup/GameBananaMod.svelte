<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "../../lib/interface";
    import { pickFile } from "../../lib/interface";
    import { sendNotif } from "../../lib/sys";
    import CardApp from "../CardApp.svelte";

    let { modalGameBanana = $bindable(), modId = 0, onDownloaded = () => {} } = $props();
    type AppData = {
        name: string;
        icon_url: string;
        execute_command: string;
        working_directory: string;
        description: string;
        banner_url: string;
    };

    let name = $state("");
    let description = $state("");
    let bannerUrl = $state("");
    let images = $state<string[]>([]);
    let downloads = $state<Array<{ name: string; url: string; size: string }>>([]);
    let loading = $state(false);
    let downloadPath = $state("");
    let modalDownload = $state(false);
    let apps = $state<AppData[]>([]);

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    })

    function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

    $effect(() => {
        if (modalGameBanana) loadMod();
    });

    async function loadMod() {
        /*
         * This function fetches the mod's info from the GameBanana API
         * and fills the popup with its name, description, banner,
         * images, and download links.
         */
        loading = true;
        name = "";
        description = "";
        bannerUrl = "";
        images = [];
        downloads = [];

        try {
            const response = await fetch(`https://gamebanana.com/apiv11/Mod/${modId}/ProfilePage`);
            const mod = await response.json();

            name = mod._sName || "Unknown mod";
            description = mod._sText || "";

            const media = mod._aPreviewMedia?._aImages || [];
            const urls = media
                .map((img: any) => img._sBaseUrl + "/" + img._sFile)
                .filter(Boolean);
            bannerUrl = urls[0] || "";
            images = urls.slice(1);

            downloads = (mod._aFiles || []).map((file: any) => ({
                name: file._sFile,
                url: file._sDownloadUrl,
                size: formatBytes(file._nFilesize),
            }));
        } catch (e) {
            useComponentSnackbarError(`Failed to fetch mod: ${e}`);
        }

        loading = false;
    }

    function formatBytes(bytes: number) {
        /*
         * This function turns a byte count into a readable size string.
         *
         * Arguments:
         *    bytes: number -> the size in bytes
         *
         * Returns:
         *    string -> the size in a readable format
         */
        if (!bytes) return "";
        const units = ["B", "KB", "MB", "GB"];
        let i = 0;
        let value = bytes;
        while (value >= 1024 && i < units.length - 1) {
            value /= 1024;
            i++;
        }
        return `${value.toFixed(1)} ${units[i]}`;
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

    async function openSelected(url: string) {
        /*
         * This function opens the modal for the loading state
         * for downloading, and downloads everything in the background
         * with the Rust backend.
         * 
         * If the path is a mods/addons folder, it's a mod for an engine,
         * not a standalone instance, so skip the AddNew dialog.
         * 
         * Arguments:
         *     url: string -> the url pass through to download the mod
         * 
         */
        if (!downloadPath) return useComponentSnackbarError("Please input a download path.");

        modalGameBanana = false;
        modalDownload = true;
        const finalDownloadPath = downloadPath + "/" + name;

        try {
            await invoke<string>("download_to_custom_dir", { url, filePath: finalDownloadPath });
            modalDownload = false;

            if (isModForEngine(downloadPath)) {
                await sendNotif(
                    "GameBanana Mod",
                    "We're done downloading!"
                );
            } else {
                await sendNotif(
                    "GameBanana Mod",
                    "We're done downloading! We just need you to input the executable file, and you should be all set!"
                );
            }

            // if the path is a mods/addons folder, it's a mod for an engine,
            // not a standalone instance, so skip the AddNew dialog
            if (isModForEngine(downloadPath)) return;

            openFolder(finalDownloadPath);
            onDownloaded({
                name,
                iconUrl: bannerUrl,
                bannerUrl,
                description,
                workingDirectory: finalDownloadPath,
            });
        } catch (e) {
            useComponentSnackbarError(`Failed to download mod: ${e}`);
            modalDownload = false;
        }
    }

    function isModForEngine(path: string) {
        /*
         * This function checks whether the given path is a mods or addons
         * folder, meaning the download is a mod for an engine
         * rather than a standalone instance.
         *
         * Arguments:
         *    path: string -> the path to check
         *
         * Returns:
         *    boolean -> true if the path is a mods or addons folder
         */
        const normalized = path.replace(/\\/g, "/").toLowerCase();
        return /\/mods\/?$/.test(normalized) || /\/addons\/?$/.test(normalized);
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

    $effect(() => {
        loadApps();
    })
</script>

<div class="overlay" class:active={modalGameBanana} onclick={() => modalGameBanana = false}></div>
<dialog class="right" class:active={modalGameBanana}>
    {#if loading}
        <p>Loading mod...</p>
    {:else if name}
        {#if downloads.length > 0}
            <h6 style="margin-bottom: 10px;">Downloads for <b>{name}</b></h6>
            <img src={bannerUrl} alt={name} style="width: 100%; height: 200px; object-fit: cover;" class="round" />
            <div style="display: flex; flex-direction: column; gap: 0.5rem; margin-top: 10px;">
                {#each downloads as download}
                    <article onclick={() => openSelected(download.url)} style="cursor: pointer; display: flex; justify-content: space-between; align-items: center; gap: 0.5rem;">
                        <div style="min-width: 0; overflow-wrap: break-word;">
                            <i>download</i>
                            <span><b>{download.name}</b></span>
                        </div>
                        <span>{download.size}</span>
                    </article>
                {/each}
            </div>

            <hr class="medium"/>

            <h6>Importing a mod that's not engine modded?</h6>
            <span>Put your path where you wanna put it here!</span>

            <div class="border field prefix label">
                <a onclick={async () => downloadPath = await pickFile([""], "Folder", true)}> <i>attach_file</i> </a>
                <input type="text" bind:value={downloadPath} />
                <label>Path to download <span style="color: red;">*</span></label>
                <output> A path to download the mod. Example; <code>C:\Games\FNF\</code> </output>
            </div>

            <hr class="medium"/>
        
            <h6>Importing a mod that's engine modded?</h6>
            <span>Select one of these instances!</span>
        
            {#if apps.length > 0}
                {#each apps as app}
                    <section onclick={() => downloadPath = app.working_directory + "/mods"}>
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

        {:else}
            <p>No download assets on this mod.</p>
        {/if}
    {:else}
        <p>Could not find that mod.</p>
    {/if}
</dialog>

<div class="overlay" class:active={modalDownload}></div>
<dialog class:active={modalDownload} style="width: 600px;">
    <h5>Please wait while we do the magic...</h5>
    <span>For you to wait, why don't you watch YouTube? Massive time killer by the way.</span>
    <span>This may take a long time depending where you live or your connection!</span>
    <progress class="wavy indeterminate" value="100" max="100"></progress>
</dialog>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>