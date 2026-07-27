<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import CardApp from "../CardApp.svelte";

    let { modalNew = $bindable(), onAppAdded = () => {} } = $props()

    let appName = $state("")
    let appIconURL = $state("")
    let appPath = $state("")
    let appWorkingDirectory = $state("")
    let appDescription = $state("")
    let bannerURL = $state("")

    let submitted = $state(false)

    function close() {
        modalNew = false;
        submitted = false;
        setTimeout(() => {
            appName = ""
            appIconURL = ""
            appPath = ""
            appWorkingDirectory = ""
            appDescription = ""
            bannerURL = ""
        }, 500)
    }

    async function addApp() {
        submitted = true;
        if (!appName || !appPath || !appWorkingDirectory) return;

        try {
            await invoke("add_app", {
                app: {
                    name: appName,
                    icon_url: appIconURL,
                    execute_command: appPath,
                    working_directory: appWorkingDirectory,
                    description: appDescription,
                    banner_url: bannerURL,
                }
            });
            modalNew = false;
            submitted = false;
            setTimeout(() => {
                appName = ""
                appIconURL = ""
                appPath = ""
                appWorkingDirectory = ""
                appDescription = ""
                bannerURL = ""
            }, 500);
            onAppAdded();
        } catch (e) {
            console.error("Failed to add app:", e);
        }
    }
</script>

<div class="overlay" class:active={modalNew} onclick={() => modalNew = false}></div>
<dialog class="right" class:active={modalNew}>
  <h5>Add a New FNF Instance</h5>
  <span></span>

  <div class="field label border" class:invalid={submitted && !appName}>
    <input type="text" bind:value={appName}>
    <label>FNF Mod/Engine name <span style="color: red;">*</span></label>
    <output>The FNF mod/engine name to display in the launcher.</output>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appIconURL}>
    <label>Icon URL</label>
    <output>Any icon your heart desires. Make sure it's 1:1.</output>
  </div>

  <div class="field label border" class:invalid={submitted && !appPath}>
    <input type="text" bind:value={appPath}>
    <label>Execute Command <span style="color: red;">*</span></label>
    <output>The command to execute when launching the app. eg; <code>.\Funkin.exe</code></output>
  </div>

  <div class="field label border" class:invalid={submitted && !appWorkingDirectory}>
    <input type="text" bind:value={appWorkingDirectory}>
    <label>Working Directory <span style="color: red;">*</span></label>
    <output>The working directory for the app. eg; <code>D:\Games\FNF\Funkin\</code></output>
  </div>

  <div class="field label border">
    <input type="text" bind:value={appDescription}>
    <label>Description</label>
    <output>The description of the app to display in the launcher. eg; <code>Base game FNF V-Slice</code></output>
  </div>

  <div class="field label border">
    <input type="text" bind:value={bannerURL}>
    <label>Banner URL</label>
    <output>The banner image URL for the app. Aspect ratio doesn't matter. But we reccomend 16:9</output>
  </div>

  <CardApp
      name={appName || "App Name"}
      iconUrl={appIconURL || "https://placehold.co/128x128"}
      executeCommand=""
      workingDirectory={appWorkingDirectory || "/"}
      isPreview={true}
  />

  <nav class="right-align no-space">
    <button class="transparent link" onclick={close}>Close</button>
    <button class="primary link" onclick={addApp}>Add App</button>
  </nav>
</dialog>
