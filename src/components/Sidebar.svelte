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
    const listsOfOptions = [
        {name: "Settings", icon: "settings", action: () => {openModal("settings")}},
        {name: "Github", icon: "commit", action: () => openUrl("https://github.com/Team-SolarEngine/solar-lanucher")},
    ]

    function openModal(modal: string) {
        if (modal === "settings") {
            modalSettings = true
        }
    }
</script>

<article style="height: 100%; overflow-y: hidden; min-width: 25rem; max-width: 25rem;">
    <nav style="position: sticky; top: 0; z-index: 1">
        <span>Solar Launcher</span>
        <div class="max"></div>
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
    </nav>

    <div class="scroll" style="z-index: 0; height: 100%">
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
</article>

<Settings bind:modalSettings />
