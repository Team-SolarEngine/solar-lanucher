<script lang="ts">
    import SectionDisplay from "./Popup/SectionsDisplay.svelte";

    let {
        workingDirectory,
    } = $props();

    let modsOpen = $state(false);
    let addonsOpen = $state(false);
    let contentOpen = $state(false);

    const buttons = [
        { name: "Mods", class: "left-round", onclick: () => modsOpen = true },
        { name: "Addons", class: "no-round border transparent", onclick: () => addonsOpen = true },
        { name: "Content", class: "right-round border transparent", onclick: () => contentOpen = true },
    ];
</script>

<nav class="group split">
    <div style="margin-left: 35px;"></div>
    {#each buttons as button}
        <button class={button.class} on:click={button.onclick}>
            {button.name}
        </button>
    {/each}
</nav>

<SectionDisplay {workingDirectory} modsFolder="mods" moduleOpen={modsOpen} onclose={() => modsOpen = false}/>
<SectionDisplay {workingDirectory} modsFolder="addons" moduleOpen={addonsOpen} onclose={() => addonsOpen = false}/>
<SectionDisplay {workingDirectory} modsFolder="contents" moduleOpen={contentOpen} onclose={() => contentOpen = false}/>
