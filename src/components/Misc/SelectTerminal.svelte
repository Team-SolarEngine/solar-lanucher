<script lang="ts">
    import { instanceState } from "$lib/instance.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { getContext } from "svelte";

    let { thisVar = $bindable() } = $props()
    const terminalDialog: any = getContext('terminal')

    async function closeApp(pid: number) {
        try {
            await invoke("kill_app", { pid })
        } catch(e) {
            console.error("Error Killing app: " + e)
        }
    }

    function openTerminalPopup(pid: number, sanitized: string, appName: string, appIconURL: string) {
        thisVar.open = false;
        terminalDialog({
            open: true,
            pid,
            sanitized,
            appName,
            appIconURL
        })
    }
</script>

<div class="overlay" class:active={thisVar.open} onclick={() => thisVar.open = false}></div>
<dialog class:active={thisVar.open} style="width: 48rem;">
    {#each instanceState.instances as is}
        <article style="display: flex; align-items: center; justify-content: space-between;">
            <div style="display: flex; align-items: center; gap: 0.4rem;">
                <img src={is.icon} alt={is.name} style="width: 85px; object-fit: cover; border-radius: 8px;"/>
                <div style="display: flex; flex-direction: column; gap: 0.2rem;">
                    <span style="font-size: 1.4rem; font-weight: bold;">{is.name}</span>
                    <span style="opacity: 0.5;">Listening: {is.sanitized} <br/> PID: {is.pid}</span>
                </div>
            </div>
            <div style="display: flex; flex-direction: column; gap: 0.2rem;">
                <button class="tertiary" onclick={() => closeApp(is.pid)}>
                    <i>stop_circle</i>
                </button>
                <button onclick={() => openTerminalPopup(is.pid, is.sanitized, is.name, is.icon)}>
                    <i>terminal</i>
                </button>
            </div>
        </article>
    {:else}
        <span>No current active terminals here. Try opening one in the background.</span>
    {/each}
</dialog>