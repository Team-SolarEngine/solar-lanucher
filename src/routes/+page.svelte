<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onOpenUrl, getCurrent } from "@tauri-apps/plugin-deep-link";
    import imageSrc from "../lib/imageSrc";
    import { onMount } from "svelte";
    import "beercss";
    import "material-dynamic-colors";

    import Sidebar from '../components/Sidebar.svelte';
    import PromptForNew from '../components/Popup/PromptForNew.svelte';
    import EditApp from '../components/Popup/EditApp.svelte';
    import MainContent from "../components/MainContent.svelte";

    import Local from "../components/Popup/AddNew/Local.svelte";
    import Download from "../components/Popup/AddNew/Download.svelte";
    import GameBananaMod from "../components/Popup/GameBananaMod.svelte";

    type AppData = {name: string, icon_url: string, execute_command: string, working_directory: string, description: string, banner_url: string};

    let apps = $state<AppData[]>([]);

    let promptForNew = $state(false);
    let modalNew = $state(false);
    let modalDownload = $state(false);

    let modalEdit = $state(false);
    let editingApp = $state<AppData | null>(null);
    let editIndex = $state(-1);
    let selectedIndex = $state(-1);

    let showPet = $state(false);
    let petIconUrl = $state("images/sussy.png");

    let modalGameBanana = $state(false);
    let gameBananaModId = $state(0);
    let newAppPrefill = $state({});

    function handleModDownloaded(mod: any) {
        /*
         * This function opens the AddNew popup pre-filled with
         * the downloaded mod's info so the user can finish adding it.
         *
         * Arguments:
         *    mod: object -> the name, icon, banner, description and path
         */
        newAppPrefill = mod;
        modalNew = true;
    }

    async function handleDeepLink(url: string) {
        /*
         * This function handles a solar-launch:// deep link.
         * It expects links like solar-launch://mods/<id>,
         * then opens the GameBanana mod popup with that id.
         *
         * Arguments:
         *    url: string -> the full deep link url
         */
        try {
            const parsed = new URL(url);
            const path = parsed.pathname.split("/").filter(Boolean);
            if (parsed.protocol === "solar-launch:" && parsed.host === "mods") {
                gameBananaModId = Number(path[0]);
                modalGameBanana = true;
            }
        } catch (e) {
            console.error("Failed to handle deep link:", e);
        }
    }

    async function loadSettings() {
        /*
         * This function loads the settings from the backend.
         * It updates the pet and its icon based on what's stored.
         */
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        showPet = data?.addPet === true;
        petIconUrl = data?.petIconUrl || "images/sussy.png";
    }

    async function loadApps() {
        /*
         * This function loads the list of apps from the backend.
         * It stores them so the sidebar and main content can render them.
         */
        try {
            apps = await invoke("get_keys", { collection: "apps" });
        } catch (e) {
            console.error("Failed to load apps:", e);
        }
    }

    function handleEdit(index: number) {
        /*
         * This function prepares the app to be edited.
         * It copies the chosen app into a fresh object,
         * remembers its index, and opens the edit popup.
         *
         * Arguments:
         *    index: number -> the index of the app to edit
         */
        editingApp = { ...apps[index] };
        editIndex = index;
        modalEdit = true;
    }

    onMount(() => {
        loadApps();

        // listen for deep links while the app is running
        onOpenUrl((urls) => {
            for (const url of urls) handleDeepLink(url);
        });

        // if the app was opened via a deep link, handle it too
        getCurrent().then((urls) => {
            if (urls) for (const url of urls) handleDeepLink(url);
        });

        // to be honest, I don't really know how to refresh
        // every time it's been changed in ../components/Popup/Settings.svelte
        // so this is our workaround
        setInterval(loadSettings, 1000);
    });
</script>

<main class="main">
    <div class="_sidebarSection">
        <Sidebar {apps} onOpenAdd={() => promptForNew = true} onEdit={handleEdit} onDelete={loadApps} onSelect={(i) => selectedIndex = i} />
    </div>

    <div class="_mainContent">
        {#if selectedIndex >= 0 && apps[selectedIndex]}
            <MainContent
                name={apps[selectedIndex].name}
                logoUrl={apps[selectedIndex].icon_url}
                bannerUrl={apps[selectedIndex].banner_url}
                description={apps[selectedIndex].description}
                workingDirectory={apps[selectedIndex].working_directory}
                executeCommand={apps[selectedIndex].execute_command}
            />
        {:else}
            <MainContent />
        {/if}
    </div>

    {#if showPet}
        <div class="_funnylol">
            <img width="128" src={imageSrc(petIconUrl)} alt="imposter!!"/>
        </div>
    {/if}
</main>

<PromptForNew bind:promptForNew bind:modalNew bind:modalDownload />
<EditApp bind:modalEdit bind:editingApp {editIndex} onAppEdited={loadApps} />

<Local bind:modalNew onAppAdded={loadApps} prefill={newAppPrefill}/>
<Download bind:modalDownload />
<GameBananaMod bind:modalGameBanana modId={gameBananaModId} onDownloaded={handleModDownloaded} />

<style>
    .main {
        display: flex;
        flex-direction: row;
        height: 100vh;
    }

    ._mainContent {
        flex: 1;
        height: 100dvh;
        overflow-y: auto;
    }

    ._funnylol {
        position: absolute;
        scale: 1;
        bottom: 0;
        right: 0;
        transform: translate(-40px, -40px);
        transition: opacity 0.5s;
        img { animation: 1s linear spin infinite; }
        &:hover { opacity: 0.5 }
    }

    @keyframes spin {
        from {
            rotate: 0deg;
        } to {
            rotate: 360deg;
        }
    }
</style>
