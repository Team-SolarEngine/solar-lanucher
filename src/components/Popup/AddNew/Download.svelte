<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "../../../lib/interface";
    import { pickFile } from "../../../lib/interface";
    import { sendNotif } from "../../../lib/sys";

    let { modalDownload = $bindable(), onDownloaded = () => {} } = $props();

    let modalDownloading = $state(false);
    let pathToDownload = $state("");
    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    });

    let engines = $state<Array<{
      name: string;
      imageUrl: string;
      releases: Array<{
        tag: string;
        author: string;
        avatarUrl: string;
        downloads: Array<{
          name: string;
          url: string;
          uploadedAt: string
          uploader: Array<{
            login: string,
            avatarUrl: string,
          }>
        }>
      }>
    }>>([]);

    const FNF_Engines = [
        { name: "Solar Engine", url: "Team-SolarEngine/Solar-Engine-Archive", imageUrl: "https://github.com/Team-SolarEngine/Solar-Engine-Archive/raw/main/assets/exclude/images/universe.png" },
        { name: "Codename Engine", url: "CodenameCrew/CodenameEngine", imageUrl: "https://avatars.githubusercontent.com/u/122549339?s=200&v=4" },
        { name: "Psych Engine", url: "ShadowMario/FNF-PsychEngine", imageUrl: "https://shadowmario.github.io/psychengine.lua/assets/icon.ico" },
        { name: "Funkin", url: "FunkinCrew/Funkin", imageUrl: "https://avatars.githubusercontent.com/u/117059284?s=200&v=4" }
    ];

    function useComponentSnackbarError(message: string) {
        useSnackbarError(message, snackbar);
    }

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

    async function getAllEngines() {
        /*
         * This function fetches the latest releases of every known engine
         * from GitHub and stores them so the list can be rendered.
         * 
         * Additionally, if you have additional repositories configured,
         * they will be added to the list. They are re-loaded every time
         * this function runs, so new repos show up without a restart.
         */
        engines = [];

        const token = await loadSetting("githubToken");

        const settingsData = await invoke("get_keys", { collection: "settings" }) as any;
        const additionalRepos = Array.isArray(settingsData?.additionalRepos) ? settingsData.additionalRepos : [];
        const allEngines = [...FNF_Engines, ...additionalRepos];

        for (const { name, url, imageUrl } of allEngines) {
            try {
                const headers: Record<string, string> = {};
                if (token) headers["Authorization"] = `Bearer ${token}`;

                const response = await fetch(`https://api.github.com/repos/${url}/releases?per_page=100`, { headers });

                const releases = await response.json();
                if (!releases?.length) continue;

                engines.push({
                    name,
                    imageUrl,
                    releases: releases.map((release: any) => ({
                        name: release.name,
                        tag: release.tag_name,
                        author: release.author?.login ?? "unknown",
                        avatarUrl: release.author?.avatar_url ?? "",
                        downloads: (release.assets ?? []).map((asset: any) => ({
                            name: asset.name,
                            url: asset.browser_download_url,
                            uploadedAt: asset.created_at,
                            uploader: ({
                                login: asset.uploader?.login,
                                avatarUrl: asset.uploader?.avatar_url,
                            })
                        })),
                    })),
                });
            } catch (e) {
                useComponentSnackbarError(`Failed to fetch ${name}: ${e}`);
            }
        }
    }

    async function handleDownload(url: string, passthrough: Array<{ name: string, iconUrl: string, description: string, tag: string }>) {
        /*
         * This function handles downloading a file from a given URL and saving it to the specified path.
         * If pathToDownload is not set, it displays an error message and returns early.
         * Otherwise, it hides the modal, sets the downloading state, and invokes the download_to_custom_dir backend function to save the file.
         * 
         * Arguments:
         *     url: string -> the URL of the file to download
         *     passthrough: Array<{
         *         name: string,
         *         iconUrl: string,
         *         description: string
         *     }> -> an array of objects containing the file's name, icon URL, and description
         */

        if (!pathToDownload) return useComponentSnackbarError("Missing input for download path.");

        modalDownload = false;
        modalDownloading = true;
        const finalDownloadPath = `${pathToDownload}/${passthrough[0].name.slice(0, -4)}-${formatDate()}_${passthrough[0].tag}`;
        /* should be just like; /home/daveberrys/FNF/Codename.Engine-linux.zip-2026-08-16_07-49-00_1.1.0-rc1
         *                      ^--------------------------------------------------------------------------^
         *                                                  THIS IS AN EXAMPLE.
         */

        try {
            await invoke<string>("download_to_custom_dir", { url, filePath: finalDownloadPath });
            modalDownloading = false;
            await sendNotif(
                "Engine Downloading",
                "We're done downloading! We just need you to input the executable file, and you should be all set!"
            );

            onDownloaded({
                name: `${passthrough[0].name.slice(0, -4)} - ${passthrough[0].tag}`,
                iconUrl: passthrough[0].iconUrl,
                bannerUrl: "",
                description: passthrough[0].description,
                workingDirectory: finalDownloadPath,
            });
        } catch (e) {
            useComponentSnackbarError(`Failed to download mod: ${e}`);
            modalDownload = false;
        }
    }

    function formatDate() {
        /* 
         * This function formats the date and time using your
         * local time zone. This is needed in case of a accidental
         * folder confliction when the mods have same names.
         * 
         * Return:
         *    String -> The formatted date. eg; 2026-08-16_07-49-00
         * 
         */

        const now = new Date();
        const year = now.getFullYear();
        const month = String(now.getMonth() + 1).padStart(2, '0');
        const day = String(now.getDate()).padStart(2, '0');
        const hours = String(now.getHours()).padStart(2, '0');
        const minutes = String(now.getMinutes()).padStart(2, '0');
        const seconds = String(now.getSeconds()).padStart(2, '0');
        
        return `${year}-${month}-${day}_${hours}-${minutes}-${seconds}`;
    }

    $effect(() => {
        if (modalDownload) getAllEngines();
    });
</script>

<div class="overlay" class:active={modalDownload} onclick={() => modalDownload = false}></div>
<dialog class="right" class:active={modalDownload}>
    <h5>Download an engine!</h5>
    <div>Whether that's Solar, Codename, Psych or Vanilla Funkin, we support it!</div>

    <hr class="medium" />

    <div class="border field prefix label">
        <a onclick={async () => pathToDownload = await pickFile([""], "Folder", true)}> <i>attach_file</i> </a>
        <input type="text" bind:value={pathToDownload} />
        <label>Path to download <span style="color: red;">*</span></label>
        <output> A path to download the Engine. Example; <code>C:\Games\FNF\</code> </output>
    </div>

    <hr class="medium" />

    {#if engines.length === 0}
        <p>Loading...</p>
    {:else}
        {#each engines as engine}
            <article>
                <details>
                    <summary style="margin: 10px 0; display: flex; justify-content: space-between; align-items: center; ">
                        <div style="display: flex; gap: 10px; align-items: center;">
                            {#if engine.imageUrl}<img style="width: 64px; height: 64px; border-radius: 5px;" src={engine.imageUrl} alt={engine.name}/>{/if}
                            <span style="font-size: 2rem;">{engine.name}</span>
                        </div>
                        <i>arrow_drop_down</i>
                    </summary>
                    <div style="display: flex; flex-direction: column; gap: 10px">
                        {#each engine.releases as release}
                            <details>
                                <summary>
                                    <article style="display: flex; justify-content: space-between; align-items: center; ">
                                        <div style="display: flex; gap: 10px; align-items: center;">
                                            <img style="width: 50px; height: 50px; border-radius: 5px;" src={release.avatarUrl} alt={engine.name}/>
                                            <div>
                                                <h6 style="font-weight: bold">{release.tag}</h6>
                                                Published by - {release.author}
                                            </div>
                                        </div>
                                        <i>arrow_drop_down</i>
                                    </article>
                                </summary>
                                <div style="margin-left: 20px; margin-top: 10px;">
                                    {#if release.downloads.length === 0}
                                        <p>No download assets on this release.</p>
                                    {:else}
                                        {#each release.downloads as download}
                                            <article onclick={() => handleDownload(download.url, [
                                              {
                                                name: download.name,
                                                iconUrl: engine.imageUrl,
                                                description: "Sickass engine called " + engine.name,
                                                tag: release.tag,
                                              }
                                            ])} style="cursor: pointer; display: flex; gap: 10px; align-items: center;">
                                                <!-- <img style="width: 35px; height: 35px; border-radius: 5px;" src={download.uploader.avatarUrl} alt={download.uploader.login}/> -->
                                                <div>
                                                    <div>
                                                        <i>download</i>
                                                        <span><b>{download.name}</b></span>
                                                    </div>
                                                    <span>Uploaded by; <b>{download.uploader.login}</b> @ {new Date(download.uploadedAt).toLocaleString()}</span>
                                                </div>
                                            </article>
                                        {/each}
                                    {/if}
                                </div>
                            </details>
                        {/each}
                    </div>
                </details>
            </article>
        {/each}
    {/if}

    <!--
    this basically prevents it where it would keep on moving the characters
    for the description and the note.
    -->
    <hr style="margin-right: 600px"/>
</dialog>

<div class="overlay" class:active={modalDownloading}></div>
<dialog class:active={modalDownloading} style="width: 600px;">
    <h5>Please wait while we do the magic...</h5>
    <span>For you to wait, why don't you watch YouTube? Massive time killer by the way.</span>
    <span>This may take a long time depending where you live or your connection!</span>
    <progress class="wavy indeterminate" value="100" max="100"></progress>
</dialog>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>