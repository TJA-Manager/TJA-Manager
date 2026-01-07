<script lang="ts">
    import { onMount } from 'svelte';
    import { open } from '@tauri-apps/plugin-dialog';
    import { LazyStore } from '@tauri-apps/plugin-store';
    import { FolderOpen, Palette, Settings, PenLine } from 'lucide-svelte';

    const store = new LazyStore("user_settings.json");

    let value = $state("");
    let currentSimPath = $state("") as string | unknown;
    let themeId = $state("") as string | unknown;
    let themeMode = $state("") as string | unknown;

    // "themeMode": "dark",
    // "themeId": "tjamgr",

    async function setDefaultSettings() {
        await store.set("currentSimPath", "path");
        await store.set("themeId", "tjamgr");
        await store.set("themeMode", "dark");
    }

    async function getSetting() {
        const currentSimPathValue = await store.get<string>("currentSimPath");
        const themeIdValue = await store.get<string>("themeId");
        const themeModeValue = await store.get<string>("themeMode");
        
        currentSimPath = currentSimPathValue
        themeId = themeIdValue
        themeMode = themeModeValue
    }

    async function changeClonePath() {
        const selectedDirectory = await open({directory: true})
        
        if (selectedDirectory === null) {
            console.log("Folder selection canceled.");
        } else {
            console.log("Selected folder:", selectedDirectory);

            await store.set("currentSimPath", selectedDirectory);
            currentSimPath = selectedDirectory;
        }
    }

    onMount(async () => {
        getSetting();
    });
</script>

<main>
    <button class="btn btn-action" onclick={getSetting}>getSetting</button>
    <input class="w-full" placeholder="Enter repository link." bind:value={value}/>

    <article class="card preset-filled-surface-100-900">
        <section class="card mx-0! preset-filled-surface-200-800">
                <h6 class="flex items-center gap-1"><FolderOpen/> Simulator Path</h6>
                <hr>
                Currently cloning to:
                <pre class="w-full my-1">{currentSimPath}</pre>
                <button type="button" class="btn btn-action mt-1" onclick={changeClonePath}><PenLine/> <span>Change path</span></button>
            </section>

            <section class="card mx-0! mb-0! preset-filled-surface-200-800">
                <h6 class="flex items-center gap-1 opacity-50"><Palette/> <span>Theme Settings <small>(Coming soon!)</small></span></h6>
                <hr>
                themeId: {themeId}
                themeMode: {themeMode}
            </section>
    </article>
</main>

<style>

</style>