<script lang="ts">
    import { Slider } from '@skeletonlabs/skeleton-svelte';
    import { Play, Pause, Volume1, Volume2, VolumeX } from 'lucide-svelte';
    
    let { audioUrl } = $props();

    let paused = $state(true);

    let time = $state([0]);
    let duration = $state(0);
    let durationRounded = $derived(Math.trunc(duration));

    let volume = $state([1]);
    let savedvolume = $state(0);

    function handleMute() {
        if (volume[0] != 0) {
            savedvolume = volume[0];
            volume[0] = 0;
        } else if (volume[0] === 0) {
            volume[0] = savedvolume;
        } 
    }

    function format(time: number) {
		if (isNaN(time)) return '...';

		const minutes = Math.floor(time / 60);
		const seconds = Math.floor(time % 60);

		return `${minutes}:${seconds < 10 ? `0${seconds}` : seconds}`;
	}

    function convertRangeToPercentage(value: number) {
        let percentage = value * 100;

        if (value < 0) value = 0;
        if (value > 1) value = 1;

        percentage = Math.trunc(percentage);

        return percentage;
    }
</script>

<audio
	src={audioUrl}
	bind:currentTime={time[0]}
	bind:duration={duration}

    bind:volume={volume[0]}
	bind:paused

    onended={() => time[0] = 0}
></audio>

<main class="py-2">
    <section class="flex">
        <button type="button" class="btn-icon size-6 mr-2 rounded-full bg-primary-500" title="Play/pause" onclick={() => paused = !paused}>
            {#if paused === true}
                <Play/>
            {:else}
                <Pause/>
            {/if}
        </button>

        <Slider value={time} defaultValue={[0]} onValueChange={(e) => (time = e.value)} max={duration || 0} step={0.1}>
            <Slider.Label class="mb-[-3px] opacity-50">
                <small>
                    <span title="Current time">{format(time[0])}</span> 
                    | 
                    <span title="Duration">{duration ? format(duration) : '...'}</span> 
                    | 
                    <span>duration: {duration} / durationRounded: {durationRounded}</span>
                </small>
            </Slider.Label>

            <Slider.Control>
                <Slider.Track class="bg-primary-950">
                    <Slider.Range class="bg-primary-500"/>
                </Slider.Track>

                <Slider.Thumb index={0} class="rounded-t-lg w-1 mt-3">
                    <Slider.HiddenInput/>
                </Slider.Thumb>
            </Slider.Control>
        </Slider>

        <button type="button" class="btn-icon size-6 mx-2 rounded-full bg-primary-500" title="Mute/Unmute" onclick={handleMute}>
            {#if volume[0] === 0}
                <VolumeX/>
            {:else if volume[0] <= 0.5}
                <Volume1/>
            {:else if volume[0] <= 1}
                <Volume2/>
            {/if}
        </button>

        <Slider class="w-[300px]" value={volume} defaultValue={[100]} onValueChange={(e) => (volume = e.value)} max={1} step={0.01}>
            <Slider.Label class="mb-[-3px] opacity-50"><small>Volume: {convertRangeToPercentage(volume[0])}</small></Slider.Label>

            <Slider.Control>
                <Slider.Track class="bg-primary-950">
                    <Slider.Range class="bg-primary-500"/>
                </Slider.Track>

                <Slider.Thumb index={0} class="rounded-t-lg w-1 mt-3">
                    <Slider.HiddenInput/>
                </Slider.Thumb>
            </Slider.Control>
        </Slider>
    </section>
</main>

<style>
</style>