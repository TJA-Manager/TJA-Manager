<script>
  import { Tabs } from '@skeletonlabs/skeleton-svelte';
  import { Toaster, createToaster } from '@skeletonlabs/skeleton-svelte';

  // Images
  import iconUrl from '$lib/images/icon.png'

  // Lucide icons
  import { AppWindow } from 'lucide-svelte';
  import { List } from 'lucide-svelte';
  import { ScrollText } from 'lucide-svelte';

  // Tab state
  let group = $state('about');
  
  // 
  const getChangelog = async () => {
    const owner = 'BeaniCraft';
    const repo = 'TJA-Manager';
    const changelogPath = 'CHANGELOG.md';

    fetch(`https://api.github.com/repos/${owner}/${repo}/contents/${changelogPath}`, {
        headers: {'Accept': 'application/vnd.github.v3.raw'}
    })

    .then(response => {
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.text();
    }) 

    .then(changelogContent => {
      console.log(changelogContent);

      const changelog = document.getElementById("changelog");
      
      if (changelog) {
        changelog.innerText = changelogContent;
        toaster.success({
          title: 'Changelog has been yoinked!',
          duration: 5000
        
        });
      } else {
        console.warn('changelog element not found');
      }
    }) 

    .catch(error => {console.error('Error fetching changelog!', error);});
  }

  // Toaster
  const toaster = createToaster({placement: 'bottom-end'});
</script>

<Toaster {toaster}></Toaster>

<!-- Content -->
<Tabs value={group} onValueChange={(e) => (group = e.value)} listJustify="justify-center">
  {#snippet list()}
    <Tabs.Control value="about">{#snippet lead()}<AppWindow/>{/snippet} About TJAMGR</Tabs.Control>
    <Tabs.Control value="changelog">{#snippet lead()}<List/>{/snippet} Changelog</Tabs.Control>
    <Tabs.Control value="credits-licenses">{#snippet lead()}<ScrollText/>{/snippet} Credits/Licenses</Tabs.Control>
  {/snippet}
  
  {#snippet content()}
    <Tabs.Panel value="about">
      <div class="flex card preset-outlined-surface-500 p-4">
        <img src={iconUrl} alt="TJAMGR icon">
                  
        <div class="relative w-full">
          <h1 class="head1"><b>TJAMGR:</b> A hassle free tool to manage TJAs.</h1>
          <hr>
          <p>Current version: [placeholder]</p>

          <button class="btn btn-sm preset-filled-primary-500" onclick={() => {toaster.info({title: 'This is a toast!'});}}>Toast</button>

          <p class="absolute bottom-0"><b>BeaniCraft <span class="text-primary-400">|</span> 2025</b></p>
        </div>
      </div>
    </Tabs.Panel>
    
    <Tabs.Panel value="changelog">
      <div class="flex card preset-outlined-surface-500 p-4">
        <div class="relative w-full">
          <h1 class="head1"><b>Changelog</b></h1>
          <hr>
          <button type="button" class="btn btn-sm preset-filled-primary-500" onclick={getChangelog}>Refresh Changelog</button>

          <p id="changelog">Not yoinked</p>
        </div>
      </div>
    </Tabs.Panel>
          
    <Tabs.Panel value="credits-licenses">
      <div class="flex card preset-outlined-surface-500 p-4">
        <div class="relative w-full">
          <h1 class="head1"><b>Credits/Licenses</b></h1>
          
          <hr>
          
          <h2 class="head2">Software Credits</h2>
          
          <hr>
          
          <h2 class="head2">Contributor Credits</h2>
          
          <hr>

        </div>
      </div>
    </Tabs.Panel>
  {/snippet}
</Tabs>

<style>

</style> 