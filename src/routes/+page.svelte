<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import "beercss";
    import "material-dynamic-colors";

    import Sidebar from '../components/Sidebar.svelte';
    import AddNew from '../components/Popup/AddNew.svelte';
    import EditApp from '../components/Popup/EditApp.svelte';
    import MainContent from "../components/MainContent.svelte";

    type AppData = {name: string, icon_url: string, execute_command: string, working_directory: string, description: string, banner_url: string};

    let apps = $state<AppData[]>([]);
    let modalNew = $state(false);
    let modalEdit = $state(false);
    let editingApp = $state<AppData | null>(null);
    let editIndex = $state(-1);
    let selectedIndex = $state(-1);

    async function loadApps() {
        try {
            apps = await invoke("get_keys", { collection: "apps" });
        } catch (e) {
            console.error("Failed to load apps:", e);
        }
    }

    function handleEdit(index: number) {
        editingApp = { ...apps[index] };
        editIndex = index;
        modalEdit = true;
    }

    onMount(() => {
        loadApps();
    });
</script>

<main class="main">
    <div class="_sidebarSection">
        <Sidebar {apps} onOpenAdd={() => modalNew = true} onEdit={handleEdit} onDelete={loadApps} onSelect={(i) => selectedIndex = i} />
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

    <div class="_funnylol">
        <img class="" src="images/sussy.png" alt="imposter!!"/>
    </div>
</main>

<AddNew bind:modalNew onAppAdded={loadApps} />
<EditApp bind:modalEdit bind:editingApp {editIndex} onAppEdited={loadApps} />

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
        scale: 0.1;
        bottom: 0;
        right: 0;
        transform: translate(400%, 420%);
        img {
            animation: 1s linear spin infinite;
        }
    }

    @keyframes spin {
        from {
            rotate: 0deg;
        } to {
            rotate: 360deg;
        }
    }
</style>
