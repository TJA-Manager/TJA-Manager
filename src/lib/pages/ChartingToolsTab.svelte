<script lang="ts">
    import { openUrl } from '@tauri-apps/plugin-opener';
    import { Popover, Portal } from '@skeletonlabs/skeleton-svelte';
    import { Download, ExternalLink } from 'lucide-svelte';

    import ToolCard from '$lib/components/ToolCard.svelte';
    import peepoDrumKitUrl from "$lib/images/peepodrumkit.png";
    import subtitleEditUrl from "$lib/images/subtitleedit.png";
    import optkUtilitiesUrl from "$lib/images/optkutilities.png"
    import tjaToolsUrl from "$lib/images/tjatools.png";
    import arrowVortexUrl from "$lib/images/arrowvortex.png";
   
</script>

<main>
    <article class="card preset-filled-surface-100-900 grid grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4">
        <ToolCard imgUrl={peepoDrumKitUrl}>
            {#snippet header()}PeepoDrumKit{/snippet}
            {#snippet content()}<p>The recommended tool to make taiko charts.</p>{/snippet}
            {#snippet footer()}
            <Popover positioning={{placement: 'top-start'}}>
                <Popover.Trigger class="btn btn-download"><Download/> Download</Popover.Trigger>
                <Portal>
                    <Popover.Positioner>
                        <Popover.Content class="card m-0! max-w-md p-4 bg-surface-500">
                            <Popover.Description>
                                <p>What version?</p>
                                <hr>
                                <button class="anchor" onclick={() => openUrl("https://drive.google.com/uc?export=download&id=1TQuvKo1tBZrXZIMlUMJ3-1vU1jfsxI2H")}>Official Latest (by samyuu)</button>
                                <br><button class="anchor" onclick={() => openUrl("https://github.com/0auBSQ/PeepoDrumKit/releases/latest")}>Komi/0auBSQ Fork</button> <span class="opacity-50">(Recommended)</span>
                                <br><button class="anchor" onclick={() => openUrl("https://github.com/IepIweidieng/PeepoDrumKit/releases/latest")}>IID/IepIweidieng Fork</button>
                            </Popover.Description>
                            <Popover.Arrow class="[--arrow-size:--spacing(3)] [--arrow-background:var(--color-surface-500)]">
                                <Popover.ArrowTip />
                            </Popover.Arrow>
                        </Popover.Content>
                    </Popover.Positioner>
                </Portal>
            </Popover>

            <Popover positioning={{placement: 'top'}}>
                <Popover.Trigger class="btn btn-action"><ExternalLink/> Go to GitHub repo</Popover.Trigger>
                <Portal>
                    <Popover.Positioner>
                        <Popover.Content class="card m-0! max-w-md p-4 bg-surface-500">
                            <Popover.Description>
                                <p>What version?</p>
                                <hr>
                                <button class="anchor" onclick={() => openUrl("https://github.com/samyuu/PeepoDrumKit")}>Official Latest (by samyuu)</button>
                                <br><button class="anchor" onclick={() => openUrl("https://github.com/0auBSQ/PeepoDrumKit")}>Komi/0auBSQ Fork</button>
                                <br><button class="anchor" onclick={() => openUrl("https://github.com/IepIweidieng/PeepoDrumKit")}>IID/IepIweidieng Fork</button>
                            </Popover.Description>
                            <Popover.Arrow class="[--arrow-size:--spacing(3)] [--arrow-background:var(--color-surface-500)]">
                                <Popover.ArrowTip />
                            </Popover.Arrow>
                        </Popover.Content>
                    </Popover.Positioner>
                </Portal>
            </Popover>
            {/snippet}
        </ToolCard>

        <ToolCard imgUrl={subtitleEditUrl}>
            {#snippet header()}Subtitle Edit{/snippet}
            {#snippet content()}<p>The recommended tool to make .VTT files for taiko charts.</p>{/snippet}
            {#snippet footer()}
                <button type="button" class="btn btn-download" onclick={() => openUrl("https://github.com/SubtitleEdit/subtitleedit/releases/latest")}><Download/> Download</button>
                <button type="button" class="btn btn-action" onclick={() => openUrl("https://github.com/SubtitleEdit/subtitleedit")}><ExternalLink/> Go to GitHub repo</button>
                {/snippet}
        </ToolCard>

        <ToolCard imgUrl={optkUtilitiesUrl}>
            {#snippet header()}OpenTaiko Utilities{/snippet}
            {#snippet content()}<p>Scripts meant to help with various tasks relating to charting.</p>{/snippet}
            {#snippet footer()}
                <button type="button" class="btn btn-download" onclick={() => openUrl("https://github.com/OpenTaiko/OpenTaiko-Utils/archive/refs/heads/main.zip")}><Download/> Download</button>
                <button type="button" class="btn btn-action" onclick={() => openUrl("https://github.com/OpenTaiko/OpenTaiko-Utils/")}><ExternalLink/> Go to GitHub repo</button>
            {/snippet}
        </ToolCard>

        <ToolCard imgUrl={tjaToolsUrl}>
            {#snippet header()}TJA Tools <small class="opacity-50">(WHMHammer's fork)</small>{/snippet}
            {#snippet content()}<p>Visualize, and get stats about your taiko charts.</p>{/snippet}
            {#snippet footer()}
                <button type="button" class="btn btn-action" onclick={() => openUrl("https://whmhammer.github.io/tja-tools/")}><ExternalLink/> Open in browser</button>
                <button type="button" class="btn btn-action" onclick={() => openUrl("https://github.com/WHMHammer/tja-tools")}><ExternalLink/> Go to GitHub repo</button>
            {/snippet}
        </ToolCard>

        <ToolCard imgUrl={arrowVortexUrl}>
            {#snippet header()}Arrow Vortex{/snippet}
            {#snippet content()}<p>The recommended tool to sync your taiko charts hassle free.</p>{/snippet}
            {#snippet footer()}
                <button type="button" class="btn btn-download" onclick={() => openUrl("https://github.com/uvcat7/ArrowVortex/releases/latest")}><Download/> Download</button>
                <button type="button" class="btn btn-action" onclick={() => openUrl("https://github.com/uvcat7/ArrowVortex")}><ExternalLink/> Go to GitHub repo</button>
            {/snippet}
        </ToolCard>
    </article>
</main>

<style>
</style>