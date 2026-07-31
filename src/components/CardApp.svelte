<script lang="ts">
    import StartExtra from "./StartExtra.svelte";
    import imageSrc from "../lib/imageSrc";

    let {
      name,
      iconUrl,
      executeCommand,
      workingDirectory,
      isPreview,
      description,
      bannerUrl = "",
      index = -1,
      isLast = false,
      onDeleted = () => {},
      onEdit = () => {},
      onSelect = () => {},
    } = $props()
</script>

<article style="height: fit-content; cursor: pointer;" onclick={() => onSelect(index)}>
    <div class="row" style="white-space: normal; gap: 0.5rem;">
        <img src={imageSrc(iconUrl) || "https://placehold.co/128x128"} alt={name} class="large square"/>
        <div style="min-width: 0; flex: 1; overflow-wrap: break-word;">
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
            isLast={isLast}
        />
    {/if}
</article>

<style>
    ._desc {
        font-size: 0.8rem;
        opacity: 50%;
    }
</style>
