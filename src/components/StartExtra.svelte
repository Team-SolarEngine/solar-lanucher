<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let {
        executeCommand,
        workingDirectory,
        onDeleted = () => {},
        onEdit = () => {},
        index = -1,
    } = $props();

    let extraFunctionalities = $derived([
        { name: "Open in Terminal", icon: "terminal", action: () => startApp(true) },
        { name: "Edit", icon: "edit", action: () => onEdit(index) },
        { name: "Delete", icon: "delete", action: deleteApp },
        { name: "Open Folder", icon: "folder", action: openFolder },
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
            await invoke("delete_app", { index });
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
</nav>