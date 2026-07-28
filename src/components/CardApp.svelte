<script lang="ts">
    import StartExtra from "./StartExtra.svelte";

    let {
      name,
      iconUrl,
      executeCommand,
      workingDirectory,
      isPreview,
      description,
      index = -1,
      onDeleted = () => {},
      onEdit = () => {},
    } = $props()
</script>

<article style="height: fit-content; z-index: inherit;">
    <div class="row">
        <img src={iconUrl || "https://placehold.co/128x128"} alt={name} class="large square"/>
        <div>
            <h5>{name}</h5>
            <span class="_desc">{description}</span>
        </div>
    </div>

    {#if !isPreview}
        <StartExtra
            executeCommand={executeCommand}
            workingDirectory={workingDirectory}
            onDeleted={onDeleted}
            onEdit={onEdit}
            index={index}
        />
    {/if}
</article>

<style>
    ._desc {
        font-size: 0.8rem;
        opacity: 50%;
    }
</style>
