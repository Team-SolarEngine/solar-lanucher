<script lang="ts">
    let {
        bannerUrl = "",
        logoUrl = "",
        description = "",
        name = "",
    } = $props();
</script>

{#if name}
    <main style="height: 100dvh; width: 100%;">
        <img src={bannerUrl} alt={name} style="width: 100%; height: 200px; object-fit: cover; border-radius: 20px;" />
        <div style="padding: 1rem;">
            <div class="row">
                <img src={logoUrl} alt={name} style="width: 128px; height: 128px; border-radius: 20px;" />
                <div>
                    <h2>{name}</h2>
                    <span>{description}</span>
                </div>
            </div>
        </div>
    </main>
{:else}
    <div style="text-align: center; height: 100dvh; width: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center;">
        <img src="images/Solar Icon.png" alt="Solar Launcher" style="width: 128px; height: 128px;" />
        <h3>Solar Launcher</h3>
        <span>Your new, lightweight FNF launcher. All in one place.</span>
    </div>
{/if}