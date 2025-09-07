<script>
  import { open } from '@tauri-apps/plugin-dialog';
  import { Navigation } from '@skeletonlabs/skeleton-svelte';
  import { Tooltip } from '@skeletonlabs/skeleton-svelte';

  // Pages
  import TJAsTab from '$lib/pages/TJAsTab.svelte';
  import AboutTab from '$lib/pages/AboutTab.svelte';

  // Images
  import logoUrl from '$lib/images/logo.png';

  // Lucide icons
  import { House } from 'lucide-svelte';
  import { Globe } from 'lucide-svelte';
  import { Info } from 'lucide-svelte';

  import { WifiOff } from 'lucide-svelte';
  
  import { FolderSearch } from 'lucide-svelte';
  import { RefreshCw } from 'lucide-svelte';
    
  // Navigation/tooltip state
  let value = $state('home');
  let infoHome = $state(false);
  let infoTJA = $state(false);
  let infoAbout = $state(false);

  let currentSimPath = $state("");

  const getSimPath = async () => {
    const selectedDirectory = await open({
      directory: true
    })
    
    if (selectedDirectory === null) {
      // User canceled the dialog
      console.log('Folder selection canceled.');
    } else {
      // User selected a folder
      console.log('Selected folder:', selectedDirectory);
      currentSimPath = selectedDirectory;
      // You can now use the 'selectedDirectory' path for your application's logic
    }

    selectedDirectory
  }
</script>

<div class="grid w-full">
  <!-- Navigation -->
  <div class="fixed h-screen">
    <Navigation.Rail width="w-[72px]" {value} onValueChange={(newValue) => (value = newValue)}>
      {#snippet header()}
        <Tooltip
          open={infoHome}
          onOpenChange={(e) => (infoHome = e.open)}
          positioning={{ placement: 'top' }}
          triggerBase="underline"
          contentBase="card preset-filled p-4"
          openDelay={200}
          arrow
        >
        {#snippet trigger()}<Navigation.Tile id="home"><House class="mx-auto"/><p>Home</p></Navigation.Tile>{/snippet}
        {#snippet content()}This is a tooltip.{/snippet}
      </Tooltip>
        
        
        
        <!--<Navigation.Tile id="home"><House class="mx-auto"/><p>Home</p></Navigation.Tile>-->
        <Navigation.Tile id="tjas"><Globe class="mx-auto"/><p>Browse TJAs</p></Navigation.Tile>
      {/snippet}
      
      {#snippet tiles()}
      <!-- Don't know why this breaks if this is deleted, but okay -->
      {/snippet}

      {#snippet footer()}
        <Navigation.Tile id="about"><Info class="mx-auto"/><p>About</p></Navigation.Tile>
      {/snippet}
    </Navigation.Rail>
  </div>
  
  <!-- Content -->
  <div class="py-[10px] pl-[82px] pr-[10px] z-0"> 
    {#if value === "home"}
      <div> 
        <img src={logoUrl} alt="TJAMGR logo" class="mx-auto mb-3">
        
        <hr>
        
        <div class="card p-4 preset-outlined-surface-500">
          <h1 class="head1">Welcome to TJA Manager!</h1>
          <hr>
          <p><b>Current songs folder path:</b> {currentSimPath}</p>
          <hr class="w-1/2">
          <button type="button" class="btn btn-sm preset-filled-primary-500" onclick={getSimPath}><FolderSearch/>Get songs folder path</button>  
        </div>
      </div>
    {/if}
      
    {#if value === "tjas"}
      <TJAsTab/>
    {/if}

    {#if value === "about"}
      <AboutTab/>
    {/if}
  </div>
</div>

<style>

</style> 