<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog';
    import { invoke } from '@tauri-apps/api/core';
    import { Folders, LoaderCircle, Settings, PenLine, AppWindow } from 'lucide-svelte';

    let { navValue, tileState = $bindable() } = $props();

    let loading = $state(false);

    let repoLink = $state("");
    let clonePath = $state("C:\\Users\\BeaniCraft\\Desktop\\testing");
    let consoleText = $state("...");

    let cloneInputDisabled = $state(false);
    let cloneButtonDisabled = $derived.by(() => {
		if (!repoLink.includes("https://") || cloneInputDisabled === true) {
            return true;
        } else {
            return false;
        }
	});

    $effect(() => {
        if (navValue === "clone-repo" && tileState != "loading") {
            tileState = "null"
        }
    });

    export function wait(ms: number) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
    
    async function cloneRepo() {
        loading = cloneInputDisabled = true;
        tileState = "loading"
        consoleText = "..."

        try {
            const msg = await invoke("clone_repo", {url: repoLink, path: clonePath}) as string;
            consoleText = msg;
            tileState = "success"
        } 
        catch (err: any) {
            consoleText = err;
            tileState = "error"
        }

        loading = cloneInputDisabled = false;

        if (tileState != "error") {
            await wait(10000);
            tileState = "null"
        }
    }

    async function changeClonePath() {
        const selectedDirectory = await open({directory: true})
        
        if (selectedDirectory === null) {
            console.log("Folder selection canceled.");
        } else {
            console.log("Selected folder:", selectedDirectory);
            
            clonePath = selectedDirectory;
        }
    }
</script>

{#if navValue === "clone-repo"}
    <main class="grid place-items-center h-full">   
        <article class="card preset-filled-surface-100-900 w-[600px] place-self-center">
            <div class="mx-auto flex gap-1.5">
                <input class="w-full" placeholder="Enter repository link." bind:value={repoLink} disabled={cloneInputDisabled}/>
                <button type="button" class="btn btn-action" onclick={cloneRepo} disabled={cloneButtonDisabled}>
                    {#if loading === true}
                        <LoaderCircle class="animate-spin"/>
                    {:else}
                        <Folders/>
                    {/if}

                    <span>Clone</span>
                </button>
            </div>

            <section class="card mx-0! preset-filled-surface-200-800">
                <h6 class="flex items-center gap-1"><Settings/> Settings</h6>
                <hr>                
                <p class="truncate">Currently cloning to: {clonePath}</p>
                <button type="button" class="btn btn-action mt-1" onclick={changeClonePath} disabled={cloneInputDisabled}><PenLine/> <span>Change path</span></button>
            </section>

            <section class="card mx-0! mb-0! preset-filled-surface-200-800">
                <h6 class="flex items-center gap-1"><AppWindow/> Console</h6>
                <hr>                
                <pre class="w-full">{consoleText}</pre>
            </section>
        </article>
    </main>
{/if}

<style>
</style>