<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import "beercss";
    import "material-dynamic-colors";

    import Topbar from '../components/Topbar.svelte';
    import CardApp from '../components/CardApp.svelte';
    import AddNew from '../components/Popup/AddNew.svelte';
    import EditApp from '../components/Popup/EditApp.svelte';

    type AppData = {name: string, icon_url: string, execute_command: string, working_directory: string};

    let apps = $state<AppData[]>([]);
    let modalNew = $state(false);
    let modalEdit = $state(false);
    let editingApp = $state<AppData | null>(null);
    let editIndex = $state(-1);

    async function loadApps() {
        try {
            apps = await invoke("get_apps");
        } catch (e) {
            console.error("Failed to load apps:", e);
        }
    }

    function openEdit(index: number) {
        editingApp = { ...apps[index] };
        editIndex = index;
        modalEdit = true;
    }

    onMount(() => {
        loadApps();
    });
</script>

<main class="main">
    <div class="_topbarSection">
        <Topbar onOpenAdd={() => modalNew = true} />
    </div>

    <div class="_programsCard">
        <div class="_programsCardChild">
            <div style="display: none;">hi this just to fix the first-child fucking up! thanks for understanding.</div>
            {#each apps as app, i}
                <CardApp
                    name={app.name}
                    iconUrl={app.icon_url}
                    executeCommand={app.execute_command}
                    workingDirectory={app.working_directory}
                    isPreview={false}
                    index={i}
                    onDeleted={loadApps}
                    onEdit={openEdit}
                />
            {/each}
        </div>
    </div>
</main>

<AddNew bind:modalNew onAppAdded={loadApps} />
<EditApp bind:modalEdit bind:editingApp {editIndex} onAppEdited={loadApps} />

<style>
    .main {
        display: flex;
        flex-direction: column;
        height: 100vh;
    }

    ._programsCard {
        height: 100dvh;
        overflow-y: auto;
        ._programsCardChild {
            display: flex;
            flex-wrap: wrap;
            gap: 5px;
            justify-content: center;
        }
    }
</style>
