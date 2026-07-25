<script lang="ts">
    let { modalSettings = $bindable() } = $props()
</script>

<div class="overlay" class:active={modalSettings} onclick={() => modalSettings = false}></div>
<dialog class="right" class:active={modalSettings}>
  <h5>Settings</h5>
  <span>Nothing yet... maybe next time?</span>

  <nav class="right-align no-space">
    <button class="transparent link" onclick={() => modalSettings = false}>Close</button>
  </nav>
</dialog>