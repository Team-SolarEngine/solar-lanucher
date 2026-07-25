<script lang="ts">
    import CardApp from "../CardApp.svelte";

    let { modalNew = $bindable() } = $props()

    let appName = $state("")
    let appIconURL = $state("")
    let appPath = $state("")
    let appWorkingDirectory = $state("")

    function clearInputs(reason: string) {
        setTimeout(() => {
            appName = ""
            appIconURL = ""
            appPath = ""
            appWorkingDirectory = ""
        }, 500)

        if (reason === "close") {
            modalNew = false
        }
    }
    
    function addApp() {
        modalNew = false;
        console.log("App added", { appName, appIconURL, appPath, appWorkingDirectory })
        clearInputs("add")
    }
</script>
 
<div class="overlay" class:active={modalNew} onclick={() => modalNew = false}></div>
<dialog class="right" class:active={modalNew}>
  <h5>Add a New App</h5>
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
      workingDirectory={appWorkingDirectory || "/home/dvs/app/appshortsies/"}
      isPreview={true}
  />

  <nav class="right-align no-space">
    <button class="transparent link" onclick={() => clearInputs("close")}>Close</button>
    <button class="primary link" onclick={addApp}>Add App</button>
  </nav>
</dialog>