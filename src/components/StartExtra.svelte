<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let {
        executeCommand,
        workingDirectory,
        onDeleted = () => {},
        onEdit = () => {},
        index = -1,
        stretch = false,
    } = $props();

    let extraFunctionalities = $derived([
        { name: "Open in Terminal", icon: "terminal", action: () => startApp(true) },
        { name: "Edit", icon: "edit", action: () => onEdit(index) },
        { name: "Open Folder", icon: "folder", action: openFolder, extra: "right-round" },
        { name: "Delete", icon: "delete", action: deleteApp },
    ])

    async function startApp(openTerminal = false) {
        try {
            await invoke("start_app", {
                workingDir: workingDirectory,
                commandExec: executeCommand,
                openTerminal,
            });
        } catch (e) {
            console.error("Failed to start app:", e);
        }
    }

    async function openFolder() {
        try {
            await invoke("open_folder", { path: workingDirectory });
        } catch (e) {
            console.error("Failed to open folder:", e);
        }
    }

    async function deleteApp() {
        try {
            await invoke("delete_key", { collection: "apps", key: index });
            onDeleted();
        } catch (e) {
            console.error("Failed to delete app:", e);
        }
    }
</script>

<nav class="group split">
    <button class="border left-round primary" onclick={() => startApp()}>
      <i>play_arrow</i>
      <span>Start</span>
    </button>
    {#if !stretch}
        <div>
            <button class="border right-round square">
            <i>keyboard_arrow_down</i>
            </button>
            <menu class="no-wrap">
                {#each extraFunctionalities as functionality}
                <li onclick={functionality.action}>
                    <i>{functionality.icon}</i> {functionality.name}
                </li>
                {/each}
            </menu>
        </div>
    {:else}
        {#each extraFunctionalities as functionality}
            {#if functionality.name != "Delete" && functionality.name != "Edit"}
                <button class="border no-round {functionality.extra}" onclick={functionality.action}>
                    <i>{functionality.icon}</i>
                    <span>{functionality.name}</span>
                </button>
            {/if}
        {/each}
    {/if}
</nav>