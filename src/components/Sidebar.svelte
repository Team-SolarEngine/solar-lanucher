<script lang="ts">
    import Settings from "./Popup/Settings.svelte"
    import CardApp from "./CardApp.svelte";

    let { onOpenAdd = () => {} }: { onOpenAdd: () => void } = $props()

    const listsOfOptions = [
        {name: "Settings", icon: "settings", action: () => {openModal("settings")}},
    ]

    let modalSettings = $state(false)

    function openModal(modal: string) {
        if (modal === "settings") {
            modalSettings = true
        }
    }
</script>

<article style="height: 100%; overflow: hidden;">
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
            <button onclick={onOpenAdd}>
                <i>add</i>
            </button>
        </div>
    </nav>

    <div style="z-index: 0; overflow-y: auto; height: 100dvh">
        <CardApp
            name="asaasdasdasdasdd"
            iconUrl="asd"
            executeCommand="aasdasdasdsd"
            workingDirectory="aasdasdassd"
            isPreview={false}
            index={0}
            onDeleted={() => {}}
            onEdit={() => {}}
        />
    </div>
</article>

<Settings bind:modalSettings />
