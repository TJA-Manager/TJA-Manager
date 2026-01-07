<script lang="ts">
    import { Accordion, Portal, Tooltip } from '@skeletonlabs/skeleton-svelte';
    import { Star, Minus, Plus, Info, Download } from 'lucide-svelte';

    import AudioPlayer from "./AudioPlayer.svelte";

    let { 
        songType = "taiko", // Can be the following: "taiko" (default), "dan", or "tower".
        songTitle, songSubtitle, songCharter = "",
        songTjaUrl, songAudioUrl, songImageUrl = "/src/lib/images/album-default.png",
        easyRating = 0, normalRating = 0, hardRating = 0, oniRating = 0, editRating = 0
    } = $props();

    let isInfoOpen = $state(false);
</script>

<!-- w-fit min-w-[600px] max-w-[600px] -->

<main>
    <section class="flex gap-2">
        <img src={songImageUrl} alt="preimage" class="rounded-container aspect-square size-[90px] preset-filled-surface-50-950">

        <div class="grow min-w-0">
            <h3 class="truncate">{songTitle}</h3>
            <small class="align-top dark-text truncate">{songSubtitle}</small>
            <br>
            <div class="flex gap-1">
                {#if easyRating != 0}
                    <span class="badge border-blue-500" title="Easy"><Star size={16} class="fill-surface-950 dark:fill-surface-50"/> {easyRating}</span>
                {/if}
                
                {#if normalRating != 0}
                    <span class="badge border-green-500" title="Normal"><Star size={16} class="fill-surface-950 dark:fill-surface-50"/> {normalRating}</span>
                {/if}
                
                {#if hardRating != 0}
                    <span class="badge border-amber-500" title="Hard"><Star size={16} class="fill-surface-950 dark:fill-surface-50"/> {hardRating}</span>
                {/if}

                {#if oniRating != 0}
                    <span class="badge border-red-500" title="Oni/Extreme"><Star size={16} class="fill-surface-950 dark:fill-surface-50"/> {oniRating}</span>
                {/if}
                
                {#if editRating != 0}
                    <span class="badge border-violet-500" title="Ura/Edit"><Star size={16} class="fill-surface-950 dark:fill-surface-50"/> {editRating}</span>
                {/if} 
            </div>
        </div>

        <div class="content-end self-start gap-1">
            <button class="btn btn-download" title="Download"><Download/> Download</button>
        </div>
    </section>

    <AudioPlayer audioUrl={songAudioUrl}/>

    <section>
        
    </section>
</main>

<style>
</style>