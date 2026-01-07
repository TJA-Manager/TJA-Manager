<script lang="ts">
    import { Download, HardDriveDownload, Info, Trash2, XIcon } from 'lucide-svelte';
	import { Dialog, Portal } from '@skeletonlabs/skeleton-svelte';
    import SongInfo from './SongInfo.svelte';

    let { songTitle } = $props();

    let moreInfo = $state(false);

    const animation = 'transition transition-discrete opacity-0 translate-y-[100px] starting:data-[state=open]:opacity-0 starting:data-[state=open]:translate-y-[100px] data-[state=open]:opacity-100 data-[state=open]:translate-y-0';
</script>

<main class="card preset-filled-surface-100-900 box-border max-w-content h-fit">
    <h6 class="truncate">{songTitle}</h6>
    <hr>
    <section class="gap-1">
        <button class="btn btn-download" title="Download"><Download/> Download</button>
        <Dialog closeOnInteractOutside={false}>
            <Dialog.Trigger class="btn btn-ghost" onclick={() => moreInfo = !moreInfo}><Info/> Info</Dialog.Trigger>
            <Portal>
                <Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
                <Dialog.Positioner class="fixed inset-0 z-50 flex justify-center size-screen p-[92px]">
                    <Dialog.Content class="card preset-filled-surface-100-900 bg-surface-100-900 w-screen overflow-y-auto {animation}">
                        <header class="flex justify-between items-center">
                            <Dialog.Title><h1>Song Info</h1></Dialog.Title>
                            <Dialog.CloseTrigger class="btn-icon btn-ghost" onclick={() => moreInfo = !moreInfo}><XIcon/></Dialog.CloseTrigger>
                        </header>
                        
                        <Dialog.Description>
                            <hr>
                            {#if moreInfo === true}
                                <SongInfo
                                    songTitle="「DISCARDED_DISRUPTION」"
                                    songSubtitle="BeaniCraft"
                                    songCharter="Komi,BeaniCraft"

                                    songTjaUrl=""
                                    songAudioUrl="https://github.com/OpenTaiko/OpenTaiko-Soundtrack/raw/refs/heads/main/05%20OpenTaiko%20Chapter%20V/220%20-%20Discarded%20Disruption/DDv2.ogg"
                                    songImageUrl="https://raw.githubusercontent.com/OpenTaiko/OpenTaiko-Soundtrack/refs/heads/main/05%20OpenTaiko%20Chapter%20V/220%20-%20Discarded%20Disruption/DDv2.png"

                                    easyRating={5}
                                    normalRating={7}
                                    hardRating={8}
                                    oniRating={10.3}
                                    editRating={11.8}
                                />
                            {/if}
                        </Dialog.Description>
                    </Dialog.Content>
                </Dialog.Positioner>
            </Portal>
        </Dialog>
    </section>
</main>

<!-- <button class="btn" title="Info" onclick={() => moreInfo = !moreInfo}></button> -->
<!-- class="truncate"  -->

<style>
</style>