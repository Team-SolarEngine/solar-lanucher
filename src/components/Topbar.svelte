<script lang="ts">
    import Settings from "./Popup/Settings.svelte"
    import AddNew from "./Popup/AddNew.svelte"
    
    const listsOfOptions = [
        {name: "Settings", icon: "settings", action: () => {openModal("settings")}},
    ]

    let modalSettings = false
    let modalNew = false

    function openModal(modal: string) {
        if (modal === "settings") {
            modalSettings = true
        } else if (modal === "new") {
            modalNew = true
        }
    }
</script>

<footer>
    <nav>
        <span>Appshortsies - Your home to start apps deep.</span>
        <div class="max"></div>
        <div>
            <button class="transparent small">
                <i>more_horiz</i>
            </button>
            <menu class="left no-wrap">
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
            <button onclick={() => {openModal("new")}}>
                <i>add</i>
            </button>
        </div>
    </nav>
</footer>

<Settings bind:modalSettings />
<AddNew bind:modalNew />