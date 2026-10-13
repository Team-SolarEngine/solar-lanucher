<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { getWord } from "$lib/localization";

    export let promptForNew: boolean;
    export let modalNew: boolean;
    export let modalDownload: boolean;

    const options = [
      { word: "prompt_for_new.button.local", icon: "desktop_windows", type: "local", primary: true },
      { word: "prompt_for_new.button.download_engines", icon: "download", type: "download" },
      { name: "GameBanana", icon: "globe", type: "https://gamebanana.com/games/8694" }
    ]

    function close(type: string = "") {
        /*
         * This function closes the dialog and, depending on the type,
         * opens the matching popup or a browser page.
         *
         * Arguments:
         *    type: string -> the type of the chosen option
         */
        promptForNew = false;

        if (type === "local") {
            modalNew = true;
        } else if (type === "download") {
            modalDownload = true;
        } else if (type.includes("https://")) {
            openUrl(type);
        }
    }
</script>

<div class="overlay" class:active={promptForNew} onclick={() => close("")}></div>
<dialog class:active={promptForNew} style="width: 500px;">
  <h5>{#await getWord("prompt_for_new.title") then word}{@html word}{/await}</h5>
  <div>
      {#await getWord("prompt_for_new.description") then word}{@html word}{/await}
  </div>
  <nav class="no-space center-align" style="display: flex; flex-wrap: wrap;">
    {#each options as option}
        <button onclick={() => close(option.type)} class:transparent={!option.primary}><i>{option.icon}</i>{#if option.word}{#await getWord(option.word) then word}{@html word}{/await}{:else}{option.name}{/if}</button>
    {/each}
  </nav>
</dialog>