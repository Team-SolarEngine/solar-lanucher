<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { openUrl } from '@tauri-apps/plugin-opener';
    import { getContext } from 'svelte';
    import Version from './Version.svelte';

    const appWindow = getCurrentWindow();
    let checkMaximized = $state(false);

    const downloadEnginePopup: any = getContext('downloadEngine')
    const localInstancePopup: any = getContext('localInstance')

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


    /*
     * == TODO LIST ==
     * - if there's no function and there's only action, make that a only button
     */
    const functionsToCall = $derived([
        { name: "Open Dialog", functions: [
            { name: "Add new", icon: "add", action: () => localInstancePopup({open: true}) },
            { name: "Download Engine", icon: "download", action: () => downloadEnginePopup({open: true}) },
        ]},
        { name: "Extras", functions: [
            { name: "Reload App", icon: "refresh", action: () => window.location.reload() },
            { name: "Information", icon: "info", action: () => openUrl("https://github.com/Team-SolarEngine/solar-lanucher/blob/main/README.md") }, //temp for now
        ]},
        { name: "Contributing", functions: [
            { name: "Open Github Repository", icon: "commit", action: () => openUrl("https://github.com/Team-SolarEngine/solar-lanucher") },
            { name: "Open Github Issues", icon: "bug_report", action: () => openUrl("https://github.com/Team-SolarEngine/solar-lanucher/issues/new/choose") },
        ]},
        { name: "Websites", functions: [
            { name: "GameBanana", icon: "more", action: () => openUrl("https://gamebanana.com/tools/23770") },
            { name: "Discord", icon: "groups", action: () => openUrl("https://discord.gg/RaHmP5fgyA") },
            { name: "FNF Solar Team", icon: "web", action: () => openUrl("https://solarengine.net") },
        ]},
    ])
</script>

<article class="systb" data-tauri-drag-region>
    <div class="hitbox">
        <div class="appName">
            <img
                src="/images/Solar Icon.png"
                alt="Solar Launcher"
                width="32"
            />
            <span style="font-size: 1.2rem"><b>Solar Launcher <Version returnVersionOnly={true}/></b></span>
            <span style="margin-left: 4px;">- Yet another launcher for FNF.</span>
        </div>

        <div class="functions">
            {#each functionsToCall as ftc}
                <div>
                    <button class="transparent small">
                        {ftc.name}
                        <i>arrow_drop_down</i>
                    </button>
                    <menu class="no-wrap">
                        {#each ftc.functions as actions}
                            <li onclick={actions.action}>
                                <i>{actions.icon}</i>
                                <span>{actions.name}</span>
                            </li>
                        {/each}
                    </menu>
                </div>
            {/each}
        </div>
    </div>

    <div class="control">
        {#each sysTopbarAction as sta}
            <button class="transparent" onclick={sta.action}><i>{sta.icon}</i></button>
        {/each}
    </div>
</article>

<style>
    .systb {
        cursor: grab;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        border-radius: 0;
        z-index: 999;
        height: 2.5rem;

        &.styled {
            background-color: transparent;
            border: 2px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
            backdrop-filter: blur(8px);
            border-radius: 1rem;
        }
    }

    .control {
        display: flex;
        position: fixed;
        top: 0;
        right: 0;
    }

     .hitbox {
         display: flex;
         flex-direction: column;

         .appName {
             display: flex;
             flex-direction: row;
             align-items: center;
             position: fixed;
             transition: all 200ms ease-in-out;
             transform: translateY(2.5px);
             top: 0;
             left: 0;
         }

         .functions {
             top: 0;
             left: 0;
             display: flex;
             flex-direction: row;
             align-items: center;
             position: fixed;
             opacity: 0;
             transition: all 200ms ease-in-out;
             transform: translateY(20px);
         }

         &:hover {
            .appName {
                transform: translateY(-20px);
                opacity: 0
            }

            .functions {
                transform: translateY(4px);
                opacity: 1;
            }
         }
     }
</style>