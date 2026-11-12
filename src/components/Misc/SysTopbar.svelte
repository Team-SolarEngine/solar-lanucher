<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    const appWindow = getCurrentWindow();
    let checkMaximized = $state(false);

    async function toggleMaximize() {
        if (await appWindow.isMaximized()) {
            checkMaximized = true;
            await appWindow.unmaximize();
        } else {
            checkMaximized = false;
            await appWindow.maximize();
        }
    }

    const sysTopbarAction = $derived([
        { icon: "minimize", action: () => appWindow.minimize() },
        { icon: checkMaximized ? "open_in_full" : "close_fullscreen", action: () => toggleMaximize() },
        { icon: "close", action: () => appWindow.close() }
    ])
</script>

<article class="systb" data-tauri-drag-region>
    {#each sysTopbarAction as sta}
        <button class="transparent" onclick={sta.action}><i>{sta.icon}</i></button>
    {/each}
</article>

<style>
    .systb {
        position: absolute;
        right: 0;
        top: 0;
        padding: 8px;
        margin: 8px;
        cursor: grab;
        padding-left: 6rem;

        &.styled {
            background-color: transparent;
            border: 2px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
            backdrop-filter: blur(8px);
            border-radius: 1rem;
        }
    }
</style>