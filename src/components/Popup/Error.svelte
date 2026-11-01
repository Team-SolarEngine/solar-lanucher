<!--
thought I'd make a simple code on how to call this popup

```
import { getContext } = 'svelte';

const callError = getContext('errorReport');

function pretendFail() {
    try {
        // code
    } catch(e) {
        callError({
            open: true,
            action: 'Download engine',
            supposed_to_happen: 'Download engine in {path}',
            why_it_failed: 'Network error',
            logs: '', // optional
            extra_info: 'Path used: {path}'
        })
    }
}

```

-->

<script lang="ts">
    let { errorDat = $bindable() } = $props();

    async function template_err_msg() {
        let msg = `# SL ERROR - <@&1354116717990903890>
**Action**: ${errorDat.action}
**What was supposed to happen:**: ${errorDat.supposed_to_happen}
**Why it failed**: ${errorDat.why_it_failed}`

        if (errorDat.logs)
            msg = msg + `\n\n## Logs\n${errorDat.logs}`
 
        msg = msg + `\n\n## Extra Information\n${errorDat.extra_info}`
        await navigator.clipboard.writeText(msg);
    }
</script>

<div class="overlay" class:active={errorDat.open}></div>
<dialog class:active={errorDat.open}>
    <h3>Uh oh! You've encountered a error!</h3>
    <span>Looks like you hit a road block. And we, as the developers, want to help you!</span> <br/>

    <article>
        <h4>Here's what happened.</h4>
        <b>You tried to</b>: {errorDat.action} <br/>
        <b>What was supposed to happen</b>: {errorDat.supposed_to_happen} <br/>
        <b>Why it failed</b>: {errorDat.why_it_failed} <br/>

        {#if errorDat.logs}
            <h5>Logs</h5>
            <pre><code>{errorDat.logs}</code></pre>
        {/if}

        <h5>Any extra info?</h5>
        <div class="field border">
            <textarea bind:value={errorDat.extra_info} placeholder="I tried to ... but then ... and now ... and I want to fix this error!"></textarea>
        </div>
    </article>

    <div class="right-align no-space">
        <button onclick={template_err_msg}>Copy for Discord</button>
        <button class="transparent">Continue on</button>
    </div>
</dialog>
