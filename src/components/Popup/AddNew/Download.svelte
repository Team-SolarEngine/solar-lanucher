<script lang="ts">
    export let modalDownload: boolean;
</script>

<div class="overlay" class:active={modalDownload} onclick={() => modalDownload = false}></div>
<dialog class="right" class:active={modalDownload}>
  <h5>Download an engine!</h5>
  <div>Whether that's Solar, Codename, Psych or vslice, we support it!</div>

  <span>TODO: this</span>
</dialog>