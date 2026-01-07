<script lang="ts">
    import { PenLine } from 'lucide-svelte';

    import logoUrl from "$lib/images/logo.png";
    
    let currentSimPath = $state("C:\\Users\\BeaniCraft\\Desktop\\testing");

    let { navValue = $bindable() } = $props();
</script>

<main>
    <img src={logoUrl} alt="TJAMGR Logo" class="m-auto">
    <hr>
    <article class="card preset-filled-surface-100-900">
        <p>Welcome to TJA Manager!</p>
        <hr>
        <p class="truncate">Currently downloading to: {currentSimPath}</p>
        <button class="anchor" onclick={() => navValue = "settings"}>(Don't want to download there?)</button>
    </article>
</main>

<style>
</style>