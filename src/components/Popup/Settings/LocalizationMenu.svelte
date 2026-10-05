<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import { getUserLanguage } from "$lib/localization";
    import { onMount } from "svelte";

    let { currentlyOpen = $bindable() } = $props();

    let localization = $state("");

    let allLocalization = $state([
      { name: "English", code: "en", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/a/a5/Flag_of_the_United_Kingdom_%281-2%29.svg/250px-Flag_of_the_United_Kingdom_%281-2%29.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "Daveberry" },
    ]);

    let snackbar = $state<Snackbar>({
        snackbarError: false,
        snackbarTime: 0,
        givenError: "",
    }); function useComponentSnackbarError(message: string) { useSnackbarError(message, snackbar); }

    async function saveLanguage(value: any) {
        /*
         * This function saves the language string setting to the backend.
         *
         * Arguments:
         *    value: string -> the new value of the language
         */

        console.log("Saving language:", value);
        await invoke("update_key", {
            collection: "settings",
            key: "currentLanguage",
            value: value,
        });
    }

    onMount(async () => {
        localization = await getUserLanguage()
    });
</script>

<div class="overlay" class:active={currentlyOpen} onclick={() => currentlyOpen = false}></div>
<dialog class="left" class:active={currentlyOpen}>
    <h3>Pick a language!</h3>
    <span>Can't find your language? You can contribute for your language for free!</span>
    {#each allLocalization as locale}
        <article
            style="display: flex; align-items: center; gap: 1rem; cursor: pointer;"
            class:selected={localization === locale.code}
            onclick={() => saveLanguage(locale.code)}
        >
            <img src={locale.flag} alt={locale.name} width="100px"/>
            <div style="display: flex; flex-direction: column; ">
                <span style="font-size: 1.4rem">{locale.name}</span>
                <span>Translated by <b>{locale.translator}</b></span>
            </div>
        </article>
    {/each}

    <div class="right-align no-space" style="margin-top: 12px;">
        <button onclick={() => currentlyOpen = false} class="transparent">
            Close
        </button>
    </div>
</dialog>

<style>
    .selected { border-left: 3px solid var(--primary); }
</style>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>