<script lang="ts">
    import Settings from "./Popup/Settings.svelte"
    import CardApp from "./CardApp.svelte";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { getWord } from "$lib/localization";
    import { loadApps } from "$lib/apps";

    let {
        onOpenAdd = () => {},
        apps = [],
        onEdit = () => {},
        onDelete = () => {},
        onSelect = () => {},
    }: {
        onOpenAdd: () => void;
        apps: any[];
        onEdit: (index: number) => void;
        onDelete: () => void;
        onSelect: (index: number) => void;
    } = $props()

    let filterSearch = $state("");
    let modalSettings = $state(false)
    let sidebarOpen = $state(true)
    const listsOfOptions = [
        {name: "Settings", icon: "settings", action: () => {openModal("settings")}},
        {name: "Github", icon: "commit", action: () => openUrl("https://github.com/Team-SolarEngine/solar-lanucher")},
    ]

    function openModal(modal: string) {
        /*
         * This function opens a modal based on the given name.
         *
         * Arguments:
         *    modal: string -> the name of the modal to open
         */
        if (modal === "settings") {
            modalSettings = true
        }
    }

    async function loadInstances() {
        /*
         * This function fetches loadApps from Tauri.
         * If next is identical to app, then don't do a change.
         */
        const next = await loadApps();
        if (JSON.stringify(next) !== JSON.stringify(apps)) {
            apps = next;
        }
    }

    let filteredItems = $derived(
        apps
            .map((app, originalIndex) => ({ app, originalIndex }))
            .filter(({ app }) =>
                app.name.toLowerCase().includes(filterSearch.trim().toLowerCase())
            )
    );

    $effect(() => {
        loadInstances();
        const interval = setInterval(loadInstances, 1000);
        return () => clearInterval(interval);
    });
</script>

<article style="height: 100%; overflow-y: hidden; min-width: 25rem; max-width: 25rem; transition: all 0.3s ease-in-out; overflow-x: hidden;" class:_sidebarClose={!sidebarOpen}>
    <nav style="position: sticky; top: 0; z-index: 1">
        <div>
            <button class="transparent circle" onclick={() => sidebarOpen = !sidebarOpen}>
                {#if sidebarOpen}
                    <i>left_panel_close</i>
                {:else}
                    <i>left_panel_open</i>
                {/if}
            </button>
            <span class="tooltip right" class:_sidebarElementClose={!sidebarOpen}>
                {#await getWord("sidebar.button.collapse_sidebar.tooltip") then word}{@html word}{/await}
            </span>
        </div>
        <div class="max"></div>
        <div class:_sidebarElementClose={!sidebarOpen} class="row" style="transition: all 0.3s ease-in-out;">
            <div class="no-space">
                {#each listsOfOptions as op}
                    <button onclick={op.action} class="transparent circle">
                        <i>{op.icon}</i>
                    </button>
                {/each}
            </div>
    
            <div>
                <button onclick={onOpenAdd} oncontextmenu={(event) => {
                  event.preventDefault();
                  openUrl("https://solarengine.net/shares");
                }}>
                    <i>add</i>
                    <span class="tooltip left">
                        {#await getWord("sidebar.button.add.tooltip") then word}{@html word}{/await}
                    </span>
                </button>
            </div>
        </div>
    </nav>

    <div class="field label border">
        <input type="text" bind:value={filterSearch}>
        <label>{#await getWord("instance_filter.title") then word}{@html word}{/await}</label>
    </div>

    <div class="scroll" style="z-index: 0; height: 100%; transition: all 0.3s ease-in-out;" class:_sidebarElementClose={!sidebarOpen}>
        {#if apps.length === 0}
            <span style="display: flex; align-items: center; justify-content: center; height: 100%;">{#await getWord("global.not_found") then word}{@html word.replace("{type}", "Instances")}{/await}</span>
        {:else if filteredItems.length === 0}
            <span style="display: flex; align-items: center; justify-content: center; height: 100%;">{#await getWord("global.not_found") then word}{@html word.replace("{type}", "Filtered search")}{/await}</span>
        {:else}
            {#each filteredItems as { app, originalIndex }, position (originalIndex)}
                <CardApp
                    name={app.name}
                    iconUrl={app.icon_url}
                    executeCommand={app.execute_command}
                    workingDirectory={app.working_directory}
                    description={app.description}
                    isPreview={false}
                    index={originalIndex}
                    isLast={position === filteredItems.length - 1 && filteredItems.length > 1}
                    onDeleted={onDelete}
                    bannerUrl={app.banner_url}
                    onEdit={() => onEdit(originalIndex)}
                    onSelect={() => onSelect(originalIndex)}
                />
            {/each}
            <div style="min-height: 120px; width: 100%;"></div>
        {/if}
    </div>

    <!-- what a fucking mess lmao -->
    <div class:_show={!sidebarOpen} style="
            position: absolute;
            bottom: 47.5%; left: 0;
            writing-mode: vertical-rl;
            text-orientation: mixed;
            transform: rotate(180deg) translateX(70%);
            transition: all 0.3s ease-in-out;
        ">
        <span>ouu shi 👀</span>
    </div>
</article>

<Settings bind:modalSettings />

<style>
    ._sidebarClose {
        min-width: 4.5rem !important;
        max-width: 4.5rem !important;
    }

    ._sidebarElementClose {
        opacity: 0;
        transform: translateX(140%);
    }

    ._show { transform: rotate(180deg) translateX(-90%) !important; }
</style>