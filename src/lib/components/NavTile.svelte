<script lang="ts">
    import { LoaderCircle, Check, CircleX } from 'lucide-svelte';
  
    let { content, tileId, tileSize, tileState = $bindable(), navValue = $bindable() } = $props();
</script>

<!--  -->
<button 
    class:selected={navValue === tileId} 
    class:nav={tileSize === "full"}
    class:nav-small={tileSize === "small"}
    onclick={() => navValue = tileId}
>
    {#if tileSize === "full" && navValue != tileId}
        {#if tileState === "error"}
            <CircleX class="absolute top-1.5 right-1.5 size-4"/>
        {:else if tileState === "success"}
            <Check class="absolute top-1.5 right-1.5 size-4"/>
        {:else if tileState === "loading"}
            <LoaderCircle class="animate-spin absolute top-1.5 right-1.5 size-4"/>
        {/if}
    {/if}

    {@render content()}
</button>

<style>
    @reference "../../app.css";

    .nav {@apply btn w-[76px] gap-0 mb-1 relative flex flex-col hover:preset-tonal-surface text-wrap;}
    .nav-small {@apply btn w-9 gap-0 flex flex-col hover:preset-tonal-surface;}
    .selected {@apply preset-filled-primary-500 hover:preset-tonal-primary;}
</style>