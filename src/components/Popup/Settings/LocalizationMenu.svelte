<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { useSnackbarError, type Snackbar } from "$lib/interface";
    import { getUserLanguage, getWord, getLocalization } from "$lib/localization";
    import { onMount } from "svelte";

    let { currentlyOpen = $bindable() } = $props();
    let requireRestart = $state(false);

    let localization = $state("");

    let allLocalization = $state([
      { name: "English", code: "en", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/a/a5/Flag_of_the_United_Kingdom_%281-2%29.svg/250px-Flag_of_the_United_Kingdom_%281-2%29.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "Daveberry" },
      { name: "Malay", code: "my", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/6/66/Flag_of_Malaysia.svg/250px-Flag_of_Malaysia.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "Daveberry" },
      { name: "Turkish", code: "tr", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/b/b4/Flag_of_Turkey.svg/330px-Flag_of_Turkey.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "BaranMuzu" },
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
        requireRestart = true;
        await invoke("update_key", {
            collection: "settings",
            key: "currentLanguage",
            value: value,
        });
    }

    async function totalLocalizationWord(code: string) {
      try {
        const localizationWords = await getLocalization(code);
        let count = 0;
        let totalCount = 0;

        for (const key in localizationWords) {
          if (!key.startsWith('            //-- '))
            totalCount++;
          if (localizationWords[key] !== "")
            count++;
        }

        return [totalCount, count];
      } catch (error) {
        console.log("Error getting total localization words:", error);
      }
    }

    onMount(async () => {
        localization = await getUserLanguage()
    });
</script>

<div class="overlay" class:active={currentlyOpen} onclick={() => currentlyOpen = false}></div>
<dialog class="left" class:active={currentlyOpen}>
    <h3>{#await getWord("settings.localization.manage_submenu.title") then word}{@html word}{/await}</h3>
    <span>{#await getWord("settings.localization.manage_submenu.description") then word}{@html word}{/await}</span>
    {#each allLocalization as locale}
        <article
            style="display: flex; align-items: center; gap: 1rem; cursor: pointer;"
            class:selected={localization === locale.code}
            onclick={() => saveLanguage(locale.code)}
        >
            <img src={locale.flag} alt={locale.name} width="100px"/>
            <div style="display: flex; flex-direction: column; ">
                <span style="font-size: 1.4rem;">{locale.name}
                    <span style="opacity: 0.5; font-size: 0.85rem;">
                        {#await totalLocalizationWord(locale.code) then result}
                            {result[1]}/{result[0]}
                        {/await}
                    </span>
                </span>
                <span>{#await getWord("settings.localization.manage_submenu.translated_by") then word}{@html word}{/await}<b>{locale.translator}</b></span>
            </div>
        </article>
    {/each}

    <div class="right-align no-space" style="margin-top: 12px;">
        <button onclick={() => currentlyOpen = false} class="transparent">
            {#await getWord("global.close") then word}{@html word}{/await}
        </button>
    </div>
</dialog>

<div class="overlay" class:active={requireRestart} onclick={() => requireRestart = false}></div>
<dialog class:active={requireRestart}>
    <h3>{#await getWord("settings.localization.manage_submenu.restart.title") then word}{@html word}{/await}</h3>
    <span>{#await getWord("settings.localization.manage_submenu.restart.description") then word}{@html word}{/await}</span>

    <div class="right-align no-space">
        <button onclick={() => window.location.reload()}>
            {#await getWord("global.confirm") then word}{@html word}{/await}
        </button>
        <button onclick={() => requireRestart = false} class="transparent">
            {#await getWord("global.close") then word}{@html word}{/await}
        </button>
    </div>
</dialog>

<style>
    .selected { border-left: 3px solid var(--primary); }
</style>

<div class="snackbar error" class:active={snackbar.snackbarError}>{snackbar.givenError}</div>