<script>
    import "beercss";
    import "material-dynamic-colors";

    import Topbar from '../components/Topbar.svelte';
    import CardApp from '../components/CardApp.svelte';
</script>

<main class="main">
    <div class="_topbarSection">
        <Topbar />
    </div>

    <div class="_programsCard">
        <div class="_programsCardChild">
            <div style="display: none;">hi this just to fix the first-child fucking up! thanks for understanding.</div>
            <CardApp
                name="HostRN"
                iconUrl="https://github.com/daveberrys/HostRN/raw/main/frontend/public/favicon.ico"
                executeCommand="puiouiopuiop"
                workingDirectory="asdfasdfasdf"
                isPreview={false}
            />
            <CardApp
                name="VLC.UTILS"
                iconUrl="https://github.com/PinpointTools/VLC.UTILS/raw/main/icon/VLC.UTILS.png"
                executeCommand="puiouiopuiop"
                workingDirectory="asdfasdfasdf"
                isPreview={false}
            />
            <CardApp
                name="Send Your Files"
                iconUrl="https://github.com/PinpointTools/SendYourFiles-RW/raw/main/public/icons/192/syf.png"
                executeCommand="pouiuiopuiop"
                workingDirectory="asdfasdfasf"
                isPreview={false}
            />
            <CardApp
                name="GUH"
                iconUrl="https://github.com/daveberrys/guh/raw/main/.github/readme/Guh.svg"
                executeCommand="piouuioppoi"
                workingDirectory="asdfasdfasdf"
                isPreview={false}
            />
        </div>
    </div>
</main>

<style>
    .main {
        display: flex;
        flex-direction: column;
        height: 100vh;
    }

    ._programsCard {
        height: 100dvh;
        overflow-y: auto;
        ._programsCardChild {
            display: flex;
            flex-wrap: wrap;
            gap: 5px;
            justify-content: center;
        }
    }
</style>