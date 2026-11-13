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

    const functionsToCall = $derived([
        { name: "Open Dialog", functions: [
            { name: "Add new", icon: "add", action: () => console.log("hi") },
        ]}
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
            <span style="font-size: 1.2rem"><b>Solar Launcher</b></span>
            <span style="margin-left: 4px;">- Yet another launcher for FNF.</span>
        </div>

        <div class="functions">
            {#each functionsToCall as ftc}
                <div>
                    <button class="transparent">
                        {ftc.name}
                        <i>arrow_drop_down</i>
                    </button>
                    <menu>
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

    <div>
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
        margin-bottom: 8px;
        border-radius: 0;
        z-index: 999;

        &.styled {
            background-color: transparent;
            border: 2px solid rgba(255, 255, 255, 0.1);
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.5);
            backdrop-filter: blur(8px);
            border-radius: 1rem;
        }
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
         }

         .functions {
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
                transform: translateY(0);
                opacity: 1;
            }
         }
     }
</style>