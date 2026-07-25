<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import CardApp from "../CardApp.svelte";

    let { modalEdit = $bindable(), editingApp = $bindable(), editIndex = -1, onAppEdited = () => {} } = $props()

    let appName = $state("")
    let appIconURL = $state("")
    let appPath = $state("")
    let appWorkingDirectory = $state("")

    $effect(() => {
        if (editingApp) {
            appName = editingApp.name
            appIconURL = editingApp.icon_url
            appPath = editingApp.execute_command
            appWorkingDirectory = editingApp.working_directory
        }
    })

    function close() {
        modalEdit = false;
        editingApp = null;
    }

    async function saveApp() {
        try {
            await invoke("update_app", {
                index: editIndex,
                app: {
                    name: appName,
                    icon_url: appIconURL,
                    execute_command: appPath,
                    working_directory: appWorkingDirectory,
                }
            });
            modalEdit = false;
            editingApp = null;
            onAppEdited();
        } catch (e) {
            console.error("Failed to update app:", e);
        }
    }
</script>

<div class="overlay" class:active={modalEdit} onclick={close}></div>
<dialog class="right" class:active={modalEdit}>
  <h5>Edit App</h5>
  <span></span>

  <div class="field label border">
    <input type="text" bind:value={appName}>
    <label>App Name</label>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appIconURL}>
    <label>Icon URL</label>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appPath}>
    <label>Execute Command</label>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appWorkingDirectory}>
    <label>Working Directory</label>
  </div>

  <CardApp
      name={appName || "App Name"}
      iconUrl={appIconURL || "https://placehold.co/128x128"}
      executeCommand=""
      workingDirectory={appWorkingDirectory || "/"}
      isPreview={true}
  />

  <nav class="right-align no-space">
    <button class="transparent link" onclick={close}>Cancel</button>
    <button class="primary link" onclick={saveApp}>Save</button>
  </nav>
</dialog>
