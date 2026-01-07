<script lang="ts">
    import { Navigation } from '@skeletonlabs/skeleton-svelte';
    import { House, Download, Folders, FilePenLine, Wrench, Settings, Info } from 'lucide-svelte';

    import NavTile from '$lib/components/NavTile.svelte';

    import HomeTab from '$lib/pages/HomeTab.svelte';
    import ChartingToolsTab from '$lib/pages/ChartingToolsTab.svelte';
    import CloneRepoTab from '$lib/pages/CloneRepoTab.svelte';
    import UITestingTab from '$lib/pages/UITestingTab.svelte';
    import SettingsTab from '$lib/pages/SettingsTab.svelte';
    import InfoTab from '$lib/pages/InfoTab.svelte';

    let debugMode = $state(true);
    let downloadSongsState = $state("null");
    let cloneRepoState = $state("null");
    let themeLoading = $state(false);

    let navValue = $state("home");
</script>

<main class="w-full flex">
    <aside class="h-screen sticky top-0">
        <Navigation layout="rail" class="w-[84px] p-1">
            <Navigation.Header>
                <NavTile bind:navValue={navValue} tileId="home" tileSize="full">
                    {#snippet content()}<House/> <span>Home</span>{/snippet}
                </NavTile>
                <NavTile bind:navValue={navValue} bind:tileState={cloneRepoState} tileId="charting-tools" tileSize="full">
                    {#snippet content()}<FilePenLine/> <span>Charting Tools</span>{/snippet}
                </NavTile>
                <hr>
                <NavTile bind:navValue={navValue} bind:tileState={downloadSongsState} tileId="download-songs" tileSize="full">
                    {#snippet content()}<Download/> <span>Download Songs</span>{/snippet}
                </NavTile>
                <NavTile bind:navValue={navValue} bind:tileState={cloneRepoState} tileId="clone-repo" tileSize="full">
                    {#snippet content()}<Folders/> <span>Clone Repo</span>{/snippet}
                </NavTile>                
                {#if debugMode === true}
                    <hr>
                    <NavTile bind:navValue={navValue} tileId="ui-testing" tileSize="full">
                        {#snippet content()}<Wrench/> <span>UI Testing</span>{/snippet}
                    </NavTile>
                {/if}
            </Navigation.Header>

            <Navigation.Content>
                <Navigation.Menu>
                    <!-- Keep this empty to add space between the navrail's header and footer -->
                </Navigation.Menu>
            </Navigation.Content>

            <Navigation.Footer>
                <section class="grid grid-cols-2 gap-1">
                    <NavTile bind:navValue={navValue} tileId="settings" tileSize="small">
                        {#snippet content()}<Settings/>{/snippet}
                    </NavTile>
                    <NavTile bind:navValue={navValue} tileId="info" tileSize="small">
                        {#snippet content()}<Info/>{/snippet}
                    </NavTile>
                </section>
            </Navigation.Footer>
        </Navigation>
    </aside>

    <article class="grow p-2.5">
        {#if navValue === "home"}
            <HomeTab bind:navValue={navValue}/>
        {/if}

        {#if navValue === "charting-tools"}
            <ChartingToolsTab/>
        {/if}

        {#if navValue === "download-songs"}
            VALUE: {navValue}
        {/if}

        <CloneRepoTab navValue={navValue} bind:tileState={cloneRepoState}/>

        {#if navValue === "ui-testing"}
            <UITestingTab/>
        {/if}

        {#if navValue === "settings"}
            <SettingsTab/>
        {/if}

        {#if navValue === "info"}
            <InfoTab/>
        {/if}
    </article>
</main>

<style>
</style>