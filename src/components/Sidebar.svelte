<script lang="ts">
    import Settings from "./Popup/Settings.svelte"
    import CardApp from "./CardApp.svelte";
    import { openUrl } from "@tauri-apps/plugin-opener";

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
                Collapse sidebar
            </span>
        </div>
        <div class="max"></div>
        <div class:_sidebarElementClose={!sidebarOpen} class="row" style="transition: all 0.3s ease-in-out;">
            <div>
                <button class="transparent small">
                    <i>more_horiz</i>
                </button>
                <menu class="no-wrap">
                    {#each listsOfOptions as op}
                        <li>
                            <section onclick={op.action} class="transparent">
                                <i>{op.icon}</i>
                                <span>{op.name}</span>
                            </section>
                        </li>
                    {/each}
                </menu>
            </div>
    
            <div>
                <button onclick={onOpenAdd} oncontextmenu={() => openUrl("https://solarengine.net/shares")}>
                    <i>add</i>
                    <span class="tooltip left">
                        Left click to add a new app<br>
                        Right click to open the Solar Engine share page
                    </span>
                </button>
            </div>
        </div>
    </nav>

    <div class="scroll" style="z-index: 0; height: 100%; transition: all 0.3s ease-in-out;" class:_sidebarElementClose={!sidebarOpen}>
        {#if apps.length > 0}
            {#each apps as app, i}
                <CardApp
                    name={app.name}
                    iconUrl={app.icon_url}
                    executeCommand={app.execute_command}
                    workingDirectory={app.working_directory}
                    description={app.description}
                    isPreview={false}
                    index={i}
                    isLast={i === apps.length - 1 && apps.length > 1}
                    onDeleted={onDelete}
                    bannerUrl={app.banner_url}
                    onEdit={() => onEdit(i)}
                    onSelect={onSelect}
                />
            {/each}
            <div style="min-height: 40px; width: 100%;"></div>
        {:else}
            <span style="display: flex; align-items: center; justify-content: center; height: 100%;">No instances found. Maybe try adding one?</span>
        {/if}
    </div>

    <!-- what a fucking mess lmao -->
    <div class:_show={!sidebarOpen} style="
            position: absolute;
            bottom: 40%; left: 0;
            writing-mode: vertical-rl;
            text-orientation: mixed;
            transform: rotate(180deg) translateX(70%);
            transition: all 0.3s ease-in-out;
        ">
        <span>A launcher that's honest.</span>
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