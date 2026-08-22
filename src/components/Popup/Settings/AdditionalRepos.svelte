<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";

    let { currentlyOpen = $bindable() } = $props();

    type RepoEntry = {
        name: string;
        url: string;
        imageUrl: string;
    };

    let repos = $state<RepoEntry[]>([]);
    let newRepo = $state<RepoEntry>({ name: "", url: "", imageUrl: "" });
    let isAdding = $state(false);
    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    }); function useComponentSnackbarError(message: string) { useSnackbarError(message, snackbar); }

    async function loadRepos() {
        /*
         * This function loads the additional repos array from the backend settings.
         * It defaults to an empty array if nothing is stored yet.
         */
        const data = await invoke("get_keys", { collection: "settings" }) as any;
        repos = Array.isArray(data?.additionalRepos) ? data.additionalRepos : [];
    }

    async function saveRepos() {
        /*
         * This function saves the current repos array to the backend settings.
         */
        try {
            await invoke("update_key", {
                collection: "settings",
                key: "additionalRepos",
                value: repos,
            });
        } catch (error) {
            useComponentSnackbarError("Failed to save repos: " + error);
        }
    }

    async function addRepo() {
        /*
         * This function adds a new repo to the array if name and URL are provided.
         * It then saves the updated array and resets the form.
         */
        if (!newRepo.name || !newRepo.url) {
            useComponentSnackbarError("Name and URL are required.");
            return;
        }

        repos = [...repos, { ...newRepo }];
        await saveRepos();
        newRepo = { name: "", url: "", imageUrl: "" };
        isAdding = false;
    }

    async function removeRepo(index: number) {
        /*
         * This function removes a repo from the array by its index.
         *
         * Arguments:
         *    index: number -> the index of the repo to remove
         */
        repos = repos.filter((_, i) => i !== index);
        await saveRepos();
    }

    $effect(() => {
        /*
         * When the dialog opens, load the repos from the backend.
         */
        if (currentlyOpen) loadRepos();
    });
</script>

<div class="overlay" class:active={currentlyOpen} onclick={() => currentlyOpen = false}></div>
<dialog class:active={currentlyOpen} class="left">
    <h5>Additional Repositories</h5>
    <div>Add custom engine repositories to download from.</div>

    {#if repos.length === 0}
        <p>No additional repositories added yet.</p>
    {:else}
        {#each repos as repo, index}
            <article>
                <nav>
                    <div class="max row">
                        {#if repo.imageUrl}<img src={repo.imageUrl} width="128" style="object-fit: contain;" />{/if}
                        <div>
                            <h6>{repo.name}</h6>
                            <div style="font-size: 0.85rem; opacity: 0.7;">{repo.url}</div>
                        </div>
                    </div>
                    <button class="transparent circle" onclick={() => removeRepo(index)}>
                        <i>delete</i>
                    </button>
                </nav>
            </article>
        {/each}
    {/if}

    <nav class="right-align no-space">
        <button onclick={() => isAdding = true}>
            <i>add</i>
            Add Repository
        </button>
        <button class="transparent link" onclick={() => currentlyOpen = false}>Close</button>
    </nav>
</dialog>

<div class="overlay" class:active={isAdding} onclick={() => isAdding = false}></div>
<dialog class:active={isAdding}>
    <h5>Whatcha' thinkin?</h5>
    <div class="field label border">
        <input type="text" bind:value={newRepo.name} />
        <label>Name <span style="color: red;">*</span></label>
        <output>The name of the repository shown in the UI. eg; <code>Solar Engine</code></output>
    </div>
    <div class="field label border">
        <input type="text" bind:value={newRepo.url} />
        <label>GitHub URL (owner/repo) <span style="color: red;">*</span></label>
        <output>The URL of the repository shown in the UI. eg; <code>Team-SolarEngine/Solar-Engine-Archived</code></output>
    </div>
    <div class="field label border">
        <input type="text" bind:value={newRepo.imageUrl} />
        <label>Icon</label>
        <output>The icon of the repository shown in the UI. eg; <code>https://...</code></output>
    </div>
    <nav class="right-align no-space">
        <button class="transparent" onclick={() => isAdding = false}>Cancel</button>
        <button onclick={() => { addRepo(); isAdding = false; }}>Save</button>
    </nav>
</dialog>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>