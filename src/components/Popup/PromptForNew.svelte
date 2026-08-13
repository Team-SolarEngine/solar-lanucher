<script lang="ts">
    export let promptForNew: boolean;
    export let modalNew: boolean;
    export let modalDownload: boolean;

    function close(type: string) {
        promptForNew = false;

        if (type === "local") {
            modalNew = true;
        } else if (type === "download") {
            modalDownload = true;
        }
    }
</script>

<div class="overlay" class:active={promptForNew} onclick={() => close("")}></div>
<dialog class:active={promptForNew}>
  <h5>Adding a new instance for FNF...</h5>
  <div>
      You can either download a new instance...<br/>
      or add in your already-downloaded ones!
  </div>
  <nav class="right-align no-space">
    <button onclick={() => close("local")}>Local</button>
    <button onclick={() => close("download")} class="transparent">Download</button>
  </nav>
</dialog>