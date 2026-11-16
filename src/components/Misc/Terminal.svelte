<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { getContext, onMount, tick, untrack } from "svelte";
    import Convert from 'ansi-to-html';
    import { invoke } from "@tauri-apps/api/core";
    import { getWord } from "$lib/localization";

    const ansiConverter = new Convert();
    let logContainer: HTMLElement;
    let terminalLogs = $state('');

    const callError: any = getContext('errorReport');
    let { terminalMetadata = $bindable() } = $props()

    function printLog(str: string) {
        terminalLogs += str + "<br>";
        tick().then(() => {
            if (logContainer) {
                logContainer.scrollTop = logContainer.scrollHeight;
            }
        });
    }

    async function setupLogListener() {
        try {
            await listen(terminalMetadata.sanitized, (event) => {
                const rawline = event.payload.message;
                const coloredHtmlLine = ansiConverter.toHtml(rawline);
                printLog(coloredHtmlLine)
            })
        } catch(e) {
            callError({
                open: true,
                action: 'Launching the game in terminal',
                supposed_to_happen: `Launching ${terminalMetadata.appName} instance in terminal`,
                why_it_failed: 'Read error?',
                logs: e,
                extra_info: `...`
            })
        }
    }

    async function closeApp(pid: number) {
        try {
            await invoke("kill_app", { pid })
            printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Sucessfully terminated ${pid}`)
        } catch(e) {
            printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Unsucessfully terminated ${pid}. What happened: ${e}`)
        }
    }

    $effect(() => {
        if (terminalMetadata.open) {
            setupLogListener()
            untrack(() => {
                terminalLogs = "";

                printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Hi! This is a very experimental logs page.`);
                printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Stuff may be buggy, and I genuinely don't know how to fix it.`);
                printLog(`<span style="color: #A730FF;">[ SOLAR LAUNCHER ]</span> Best you're gonna get is this. Thanks a lot! <3`);
            });
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

        code {
            background-color: transparent;
            user-select: text !important;
            -webkit-user-select: text !important;
        }
    }
</style>