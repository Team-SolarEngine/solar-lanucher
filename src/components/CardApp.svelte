<script lang="ts">
    import StartExtra from "./StartExtra.svelte";
    import { imageSrc } from "$lib/sys";
    import { invoke } from "@tauri-apps/api/core";

    let compactMode = $state(false)

    async function loadSetting(key: string) {
        /*
         * This function loads a single setting value from the backend.
         *
         * Arguments:
         *    key: string -> the name of the setting to load
         *
         * Returns:
         *    Promise -> the value of the setting, or null
         */
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        return data?.[key];
    }

    let {
      name = "",
      iconUrl = "",
      executeCommand = "",
      workingDirectory = "",
      isPreview = false,
      description = "",
      bannerUrl = "",
      index = -1,
      isLast = false,
      onDeleted = () => {},
      onEdit = () => {},
      onSelect = () => {},
    } = $props()

    $effect(() => {
        const refreshCompactMode = async () => {
            compactMode = await loadSetting("compactMode") === true;
        }; refreshCompactMode();

        // reload the setting periodically so it picks up
        // changes made in the settings popup
        const interval = setInterval(refreshCompactMode, 1000);
        return () => clearInterval(interval);
    })
</script>

<article style="height: fit-content; cursor: pointer;" onclick={() => onSelect(index)}>
    {#if !compactMode}
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
    {:else}
        <div class="row" style="white-space: normal; gap: 0.5rem;">
            <img src={imageSrc(iconUrl) || "https://placehold.co/128x128"} alt={name} class="small square"/>
            <div style="min-width: 0; flex: 1; overflow-wrap: break-word;">
                <h6>{name}</h6>
            </div>
        </div>
    {/if}
</article>

<style>
    ._desc {
        font-size: 0.8rem;
        opacity: 50%;
    }
</style>
