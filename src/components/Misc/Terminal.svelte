<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { getContext, onMount, tick, untrack } from "svelte";
    import Convert from 'ansi-to-html';
    import { invoke } from "@tauri-apps/api/core";
    import { getWord } from "$lib/localization";
    import { getOS } from "$lib/sys";

    const ansiConverter = new Convert();
    let logContainer: HTMLElement;
    let terminalLogs = $state('');

    const callError: any = getContext('errorReport');
    let { terminalMetadata = $bindable() } = $props()

    async function printLog(str: string, gameLogs: boolean = false) {
        /*
         * This prints the logs to `terminalLogs`, which is a mutable
         * variable and can be changed any time, and the terminal div
         * will be automatically scrolled to the bottom.
         */

        // for some reason, windows already does `\n`, so this code
        // prevents <br> being added to the terminal logs.
        if (gameLogs && await getOS() == "windows") terminalLogs += str
            else terminalLogs += str + "<br>";
        tick().then(() => {
            if (logContainer) {
                logContainer.scrollTop = logContainer.scrollHeight;
            }
        });
    }

    function setupLogListener() {
        /*
         * This function sets up the log listener from the
         * given UUID. It also queues logs and waits 50ms
         * to send them to the DOM. This way; It prevents
         * lag, and memory/listen leaks.
         */
        let stopped = false;
        let unlisten: (() => void) | undefined;
        let pending: string[] = [];

        const timer = setInterval(() => {
            if (!pending.length) return;
            const sendLogs = pending.splice(0).join("");
            printLog(sendLogs, true)
        }, 50);

        void listen(terminalMetadata.sanitized, (event) => {
            pending.push(ansiConverter.toHtml(event.payload.message));
        }).then((stopListening) => {
            if (stopped) stopListening();
            else unlisten = stopListening;
        }).catch((e) => {
            callError({
                open: true,
                action: 'Launching the game in terminal',
                supposed_to_happen: `Launching ${terminalMetadata.appName} instance in terminal`,
                why_it_failed: 'Read error?',
                logs: e,
                extra_info: `...`
            })
        });

        return () => {
            stopped = true;
            unlisten?.();
            clearInterval(timer);
            pending = [];
        };
    }

    async function closeApp(pid: number) {
        /*
         * This function just closes the app and prints that it
         * has sucessfully closed or failed. Nothing special.
         */
        try {
            await invoke("kill_app", { pid })
            printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Sucessfully terminated ${pid}`)
        } catch(e) {
            printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Unsucessfully terminated ${pid}. What happened: ${e}`)
        }
    }

    $effect(() => {
        if (terminalMetadata.open) {
            const cleanup = setupLogListener();
    
            untrack(() => {
                terminalLogs = "";
    
                printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Hi! This is a very experimental logs page.`);
                printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Stuff may be buggy, and I genuinely don't know how to fix it.`);
                printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Best you're gonna get is this. Thanks a lot! <3`);
            });
    
            return cleanup;
        }
    });
</script>

<div class="overlay" class:active={terminalMetadata.open}></div>
<dialog style="width: 48rem !important;" class:active={terminalMetadata.open}>
    <div style="display: flex; gap: 0.4rem; align-items: center;">
        <img style="width: 64px; border-radius: 8px;" src={terminalMetadata.appIconURL} alt={terminalMetadata.appName} />
        <div style="display: flex; flex-direction: column;">
            <span style="font-size: 1.4rem; font-weight: bold;">{terminalMetadata.appName}</span>
            <span style="opacity: 0.5;">Listening: {terminalMetadata.sanitized}</span>
        </div>
    </div>

    <!-- Bind logContainer to the element with overflow-y: auto -->
    <pre class="terminal" bind:this={logContainer}><code>{@html terminalLogs}</code></pre>

    <div style="margin-top: 8px;" class="right-align">
        <button class="transparent" onclick={() => closeApp(terminalMetadata.pid)}>
            <i>stop_circle</i>
            {#await getWord("start_extra.button.stop") then word}{@html word}{/await}
        </button>
        <button onclick={() => terminalMetadata.open = false}>
            <i>close</i>
            {#await getWord("global.close") then word}{@html word}{/await}
        </button>
    </div>
</dialog>

<style>
    .terminal {
        height: 32rem;
        overflow-y: auto;
        border-radius: 8px;
        background-color: rgba(0, 0, 0, 0.25);
        overflow-x: hidden;

        code {
            background-color: transparent;
            user-select: text !important;
            -webkit-user-select: text !important;
            white-space: pre-wrap;
            overflow-wrap: anywhere;
            overflow-x: hidden;
        }
    }
</style>