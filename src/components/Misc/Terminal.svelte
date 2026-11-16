<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { getContext, onMount, tick } from "svelte";
    import Convert from 'ansi-to-html';

    const ansiConverter = new Convert();
    let logContainer: HTMLElement;
    let terminalLogs = $state(`Solar Launcher Logs
    Hi! This is a very experimental logs page.
    Stuff may be buggy, and I genuinely don't know fix them.
    Best you're gonna get is this. Thanks a lot! <3 <br/><br/>`);

    const callError: any = getContext('errorReport');
    let { terminalMetadata = $bindable() } = $props()

    async function setupLogListener() {
        try {
            await listen("terminal-output", (event) => {
                const rawline = event.payload.message;
                const coloredHtmlLine = ansiConverter.toHtml(rawline);
                terminalLogs += coloredHtmlLine + "<br>";
                
                tick().then(() => {
                    if (logContainer) {
                        logContainer.scrollTop = logContainer.scrollHeight;
                    }
                });
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

    $effect(() => {
        setupLogListener()
    })
</script>

<div class="overlay" class:active={terminalMetadata.open}></div>
<dialog style="width: 48rem !important;" class:active={terminalMetadata.open}>
    <!-- Bind logContainer to the element with overflow-y: auto -->
    <pre class="terminal" bind:this={logContainer}><code>{@html terminalLogs}</code></pre>
    
    <div style="margin-top: 8px;" class="right-align" onclick={() => terminalMetadata.open = false}>
        <button><i>close</i> Close</button>
    </div>
</dialog>

<style>
    .terminal {
        height: 32rem;
        overflow-y: auto;
        border-radius: 8px;
    }
</style>