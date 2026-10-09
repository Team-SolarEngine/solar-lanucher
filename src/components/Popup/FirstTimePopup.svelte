<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import CardApp from "../CardApp.svelte";

    let allLocalization = $state([
      { name: "English", code: "en", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/a/a5/Flag_of_the_United_Kingdom_%281-2%29.svg/250px-Flag_of_the_United_Kingdom_%281-2%29.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "Daveberry" },
      { name: "Malay", code: "my", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/6/66/Flag_of_Malaysia.svg/250px-Flag_of_Malaysia.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "Daveberry" },
      { name: "Turkish", code: "tr", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/b/b4/Flag_of_Turkey.svg/330px-Flag_of_Turkey.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "BaranMuzu" },
      { name: "Belgium", code: "nl-BE", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/6/65/Flag_of_Belgium.svg/250px-Flag_of_Belgium.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail", translator: "VideoBot" },
      {name: "LOLcat", code: "lolcat", flag: "https://thumb.wikimedia.org/wikipedia/commons/thumb/6/68/Orange_tabby_cat_sitting_on_fallen_leaves-Hisashi-01A.jpg/960px-Orange_tabby_cat_sitting_on_fallen_leaves-Hisashi-01A.jpg?utm_source=en.wikipedia.org&utm_campaign=imageinfo&utm_content=thumbnail", translator: "CharGoldenYT"}
    ]);

    async function saveSetting(key: string, value: any) {
        /*
         * This function saves a single setting value to the backend.
         *
         * Arguments:
         *    key: string -> the name of the setting to save
         *    value: string or boolean -> the new value of the setting
         */

        console.log("Saving setting:", key, value);
        await invoke("update_key", {
            collection: "settings",
            key: key,
            value: value,
        });
    }

    async function translationSet(code: string) {
        saveSetting("currentLanguage", code);
        localization = code;
    }

    function proceed() {
        if (localization == "") {
            buttonMessage = "Please select a language."; buttonMessageBool = true;
            setInterval(() => { buttonMessage = "Confirm"; buttonMessageBool = false; }, 2000)
            return
        }

        window.location.reload()
    }

    let buttonMessageBool = $state(false);
    let buttonMessage = $state("Confirm");

    let compactMode = $state(false);
    let localization = $state("");
    let { modalForFirstTime = $bindable() } = $props()
</script>

<div class="overlay" class:active={modalForFirstTime}></div>
<dialog class:active={modalForFirstTime}>
    <h2>Welcome to Solar Launcher!</h2>
    <span>Looks like this is your first time! Let's customize your settings a bit!</span>

    <hr class="medium" />

    <div class="field middle-align">
        <nav>
            <div class="max">
                <h6>Compact Mode</h6>
                  <div>Too spaced out and too much content? Turn this on!</div>
            </div>
            <label class="switch">
                <input type="checkbox" checked={compactMode} onchange={() => { compactMode = !compactMode; saveSetting("compactMode", compactMode); }}>
                <span></span>
            </label>
            <div class="tooltip max bottom" style="width: 25rem; box-shadow: none; background-color: var(--background);">
                <CardApp
                    name="Psych Engine"
                    iconUrl="https://shadowmario.github.io/psychengine.lua/assets/icon.ico"
                    executeCommand=""
                    workingDirectory=""
                    description="<b>Psych Engine</b> Hell Yeah"
                    isPreview={false}
                    firstTimePopup={true}
                />
                <CardApp
                    name="Codename Engine"
                    iconUrl="https://avatars.githubusercontent.com/u/122549339?s=200&v=4"
                    executeCommand=""
                    workingDirectory=""
                    description="Ouu shi 👀"
                    isPreview={false}
                    firstTimePopup={true}
                />
            </div>
        </nav>
    </div>

    <hr class="medium" />

    <h3>Select your language</h3>
    <span>Don't speak English? Pick one of these!</span>
    {#each allLocalization as locale}
        <article
            style="display: flex; align-items: center; gap: 1rem; cursor: pointer;"
            class:selected={localization === locale.code}
            class="localization"
            onclick={() => translationSet(locale.code)}
        >
            <img src={locale.flag} alt={locale.name} width="100px" height="55px" style="object-fit: cover;"/>
            <div style="display: flex; flex-direction: column;">
                <span style="font-size: 1.4rem;">{locale.name}</span>
                <span>Translated by; <b>{locale.translator}</b></span>
            </div>
        </article>
    {/each}

    <hr class="medium" />

    <h3>Ready?</h3>
    <span>Click on the button below and you should be done!</span>
    
    <div style="margin-top: 8px;">
        <button onclick={() => proceed()} class="{buttonMessageBool ? "error" : "primary"}">
            <i>{buttonMessageBool ? "close" : "check"}</i>
            {buttonMessage}
        </button>
    </div>
</dialog>

<style>
    .localization {
        transition: border-left 100ms ease-in-out;
        border-left: 0 solid transparent;
        &.selected {
            border-left: 3px solid var(--primary);
        }
    }
</style>
