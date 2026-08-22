<script lang="ts">
    import StartExtra from "./StartExtra.svelte";
    import { marked } from "marked";
    import { invoke } from "@tauri-apps/api/core";
    import imageSrc from "../lib/imageSrc";
    import ModsSection from "./Misc/ModsSection.svelte";

    let {
        bannerUrl = "",
        logoUrl = "",
        description = "",
        name = "",
        executeCommand = "",
        workingDirectory = "",
        index = -1,
        onEdit = () => {},
        onDelete = () => {},
    } = $props();

    let oneOfTwo = $state(true);
    let readme = $state("");
    let changelog = $state("");
    let splash = $state("");

    async function loadMarkdowns() {
        /*
         * This function loads the README and Changelog markdown
         * from the app's working directory so they can be shown.
         */
        try { readme = await invoke("get_file_content", { path: `${workingDirectory}/README.md` }); }
        catch { readme = ""; }
        try { changelog = await invoke("get_file_content", { path: `${workingDirectory}/Changelog.md` }); }
        catch { changelog = ""; }
    }

    async function getSplashes() {
        try {
            const response = await fetch("./misc/splash.txt");
            const text = await response.text();

            const splashes = text
                .split(/\r?\n/)
                .map((line) => line.trim())
                .filter(Boolean);
            splash = splashes[Math.floor(Math.random() * splashes.length)];
        } catch (e) {
            splash = "Well, something went wrong I guess.";
        }
    }

    $effect(() => {
        loadMarkdowns();
        getSplashes();
    });
</script>

{#if name}
    <main style="width: 100%; overflow-y: auto;">
        {#if bannerUrl}
            <img src={imageSrc(bannerUrl)} alt={name} style="width: 100%; height: 200px; object-fit: cover; border-radius: 20px;" />
        {:else if !bannerUrl && !readme && !changelog}
            <div style="height: 35%;"></div>
        {/if}
        <div style="padding: 1rem;">
            <div class="row">
                <img src={imageSrc(logoUrl)} alt={name} style="width: 128px; height: 128px; border-radius: 20px;" />
                <div style="flex: 1; min-width: 0;">
                    <h2>{name}</h2>
                    <span style="overflow-wrap: break-word; word-break: break-word; display: inline-block;">
                        {description}
                    </span>
                </div>
            </div>
        </div>

        <div style="display: flex; margin-left: 30px;">
            <StartExtra
                workingDirectory={workingDirectory}
                executeCommand={executeCommand}
                stretch={true}
                index={index}
                onEdit={onEdit}
                onDeleted={onDelete}
            />
        </div>

        <ModsSection workingDirectory={workingDirectory} />

        {#if readme || changelog}
            <!-- <div style="display: flex; gap: 0.5rem;">
                <span style="display: none;">fixes the first child being taller yeah yeah whatever</span>
                {#if readme}
                    <article style="flex: 1; height: 25rem; overflow-y: auto;">
                        <div>
                            <h6 style="font-weight: bold;">README.md</h6>
                            <hr class="medium" />
                        </div>
                        {@html marked(readme)}
                    </article>
                {/if}

                {#if changelog}
                    <article style="flex: 1; height: 25rem; overflow-y: auto;">
                        <div>
                            <h6 style="font-weight: bold;">Changelog.md</h6>
                            <hr class="medium" />
                        </div>
                        {@html marked(changelog)}
                    </article>
                {/if}
            </div> -->

            <div style="margin-top: 8px;">
                <div class="tabs">
                    <a class:active={oneOfTwo} onclick={() => oneOfTwo = true}>
                        <i>book</i>
                        <span>README</span>
                    </a>
                    <a class:active={!oneOfTwo} onclick={() => oneOfTwo = false}>
                        <i>change_circle</i>
                        <span>Changelog</span>
                    </a>
                </div>
                <div class="page padding" class:active={oneOfTwo}>
                    {#if readme} {@html marked(readme)} {:else} <p style="opacity: 0.5;">No README.md found.</p> {/if}
                </div>
                <div class="page padding" class:active={!oneOfTwo}>
                    {#if changelog} {@html marked(changelog)} {:else} <p style="opacity: 0.5;">No Changelog.md found.</p> {/if}
                </div>
            </div>
        {/if}

        <div style="margin-bottom: 8px;"></div>
    </main>
{:else}
    <div style="text-align: center; height: 100dvh; width: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <!--
            so for some reason, splashText only renders when
            it's a nested div. probably because of the
            overwhelming syles lmao!!
        -->
        <div>
            <img src="images/Solar Icon.png" alt="Solar Launcher" style="width: 128px; height: 128px;" />
            <h3>Solar Launcher</h3>
            <span>Your new, lightweight FNF launcher. All in one place.</span>
            <span class="_splashText">{splash}</span>
        </div>
    </div>
{/if}

<style>
    ._splashText {
        position: absolute;
        right: -6rem;
        bottom: -1rem;
        rotate: -4deg;
        color: yellow;
        animation: scaleInOut 1.5s ease-in-out infinite alternate;
        text-align: center;
    }

    @keyframes scaleInOut {
        0% { transform: scale(0.85); }
        50% { transform: scale(1); }
        100% { transform: scale(0.85); }
    }
</style>