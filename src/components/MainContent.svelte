<script lang="ts">
    import StartExtra from "./StartExtra.svelte";
    import { marked } from "marked";
    import { invoke } from "@tauri-apps/api/core";

    let {
        bannerUrl = "",
        logoUrl = "",
        description = "",
        name = "",
        executeCommand,
        workingDirectory,
    } = $props();

    let readme = $state("");
    let changelog = $state("");
    async function loadMarkdowns() {
        try {
            readme = await invoke("get_markdown", { path: `${workingDirectory}/README.md` });
            changelog = await invoke("get_markdown", { path: `${workingDirectory}/Changelog.md` });
        } catch (e) {
            console.error("Failed to load markdowns:", e);
            readme = "";
            changelog = "";
        }
    }

    $effect(() => {
        loadMarkdowns();
    });
</script>

{#if name}
    <main style="height: 100dvh; width: 100%;">
        <img src={bannerUrl} alt={name} style="width: 100%; height: 200px; object-fit: cover; border-radius: 20px;" />
        <div style="padding: 1rem;">
            <div class="row">
                <img src={logoUrl} alt={name} style="width: 128px; height: 128px; border-radius: 20px;" />
                <div style="flex: 1; min-width: 0;">
                    <h2>{name}</h2>
                    <span style="overflow-wrap: break-word; word-break: break-word; display: inline-block;">
                        {description}
                    </span>
                </div>
            </div>
        </div>

        <div style="display: flex; justify-content: center;">
            <StartExtra
                workingDirectory={workingDirectory}
                executeCommand={executeCommand}
                stretch={true}
            />
        </div>

        {#if readme && changelog}
            <div style="display: flex; gap: 0.5rem;">
                <span style="display: none;">fixes the first child being taller yeah yeah whatever</span>
                {#if readme}
                    <article style="flex: 1;">
                        <h6 style="font-weight: bold;">README.md</h6>
                        <hr class="medium" />
                        {@html marked(readme)}
                    </article>
                {/if}

                {#if changelog}
                    <article style="flex: 1;">
                        <h6 style="font-weight: bold;">Changelog.md</h6>
                        <hr class="medium" />
                        {@html marked(changelog)}
                    </article>
                {/if}
            </div>
        {/if}
    </main>
{:else}
    <div style="text-align: center; height: 100dvh; width: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <img src="images/Solar Icon.png" alt="Solar Launcher" style="width: 128px; height: 128px;" />
        <h3>Solar Launcher</h3>
        <span>Your new, lightweight FNF launcher. All in one place.</span>
    </div>
{/if}