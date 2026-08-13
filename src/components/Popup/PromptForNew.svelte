<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";

    export let promptForNew: boolean;
    export let modalNew: boolean;
    export let modalDownload: boolean;

    const options = [
      { name: "Local", icon: "desktop_windows", type: "local", primary: true },
      { name: "Download Engines", icon: "download", type: "download" },
      { name: "GameBanana", icon: "globe", type: "https://gamebanana.com/games/8694" }
    ]

    function close(type: string = "") {
        /*
         * This function closes the dialog and, depending on the type,
         * opens the matching popup or a browser page.
         *
         * Arguments:
         *    type: string -> the type of the chosen option
         */
        promptForNew = false;

        if (type === "local") {
            modalNew = true;
        } else if (type === "download") {
            modalDownload = true;
        } else if (type.includes("https://")) {
            openUrl(type);
        }
    }
</script>

<div class="overlay" class:active={promptForNew} onclick={() => close("")}></div>
<dialog class:active={promptForNew} style="width: 500px;">
  <h5>Adding a new instance for FNF...</h5>
  <div>
      You can either download a new instance...<br/>
      or add in your already-downloaded ones!
  </div>
  <nav class="no-space center-align" style="display: flex; flex-wrap: wrap;">
    {#each options as option}
        <button onclick={() => close(option.type)} class:transparent={!option.primary}><i>{option.icon}</i>{option.name}</button>
    {/each}
  </nav>
</dialog>