<script>
    let {
      name,
      iconUrl,
      executeCommand,
      workingDirectory,
      isPreview,
    } = $props()

    let extraFunctionalities = [
      // { name: "", icon: "", action: () => {console.log("")} },
      { name: "Run as Admin", icon: "shield", action: () => {console.log("Run as admin")} },
      { name: "Edit", icon: "edit", action: () => {console.log("Edit")} },
      { name: "Delete", icon: "delete", action: () => {console.log("Delete")} },
      { name: "Open Folder", icon: "folder", action: () => {console.log("Open folder")} },
    ]
</script>

<article style="height: fit-content;">
    <div class="row">
        <img src={iconUrl} alt={name} class="large square"/>
        <div>
            <h5>{name}</h5>
            <span class="_desc">{workingDirectory}</span>
        </div>
    </div>

    {#if !isPreview}
    <nav class="group split">
        <button class="border left-round primary">
          <i>play_arrow</i>
          <span>Start</span>
        </button>
        <div>
            <button class="border right-round square">
              <i>keyboard_arrow_down</i>
            </button>
            <menu class="left no-wrap">
                {#each extraFunctionalities as functionality}
                  <li on:click={functionality.action}>
                      <i>{functionality.icon}</i> {functionality.name}
                  </li>
                {/each}
            </menu>
        </div>
    </nav>
    {/if}
</article>

<style>
    ._desc {
        font-size: 0.8rem;
        opacity: 50%;
    }
</style>