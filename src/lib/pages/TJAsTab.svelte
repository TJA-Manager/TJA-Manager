<script>
  import { Accordion } from '@skeletonlabs/skeleton-svelte';
  import { Tabs } from '@skeletonlabs/skeleton-svelte';
  
  // Lucide icons
  import { RefreshCw } from 'lucide-svelte';
  import { Download } from 'lucide-svelte';
  import { Image } from 'lucide-svelte';
  import { Clapperboard } from 'lucide-svelte';
  import { FileType } from 'lucide-svelte';

  import { ThumbsUp } from 'lucide-svelte';
  
  import { Star } from 'lucide-svelte';
  import { Plus } from 'lucide-svelte';
  import { Minus } from 'lucide-svelte';

  import { CirclePlus } from 'lucide-svelte';
  import { CircleChevronLeft } from 'lucide-svelte';
  import { Bomb } from 'lucide-svelte';

  import { Zap } from 'lucide-svelte';
  import { TriangleAlert } from 'lucide-svelte';
  import { MessageCircleWarning } from 'lucide-svelte';
  import { Ban } from 'lucide-svelte';
</script>

<!-- Content -->
<div class="p-3"> 
  <div> 
    <div class="items-center justify-center">
      <!--bro what did you think would be here this is a prototype

    <button type="button" class="btn btn-sm preset-filled-primary-500">do something idk</button>

      <AudioPlayer/>-->
    </div>

  
    <Accordion collapsible>
        {#snippet iconOpen()}<Minus/>{/snippet}
        {#snippet iconClosed()}<Plus/>{/snippet}
      <Accordion.Item value="test" classes="card preset-outlined-surface-500">
        {#snippet lead()}Badge guide{/snippet}
        {#snippet control()}{/snippet}
        {#snippet panel()}
          <h2 class="head2">Difficulties:</h2><br>

          <span class="badge preset-filled bg-blue-400 text-black">Easy</span>
          <span class="badge preset-filled bg-emerald-500 text-black">Normal</span>
          <span class="badge preset-filled bg-amber-400 text-black">Hard</span>
          <span class="badge preset-filled bg-red-400 text-black">Oni</span>
          <span class="badge preset-filled bg-purple-400 black">Edit</span>
          <br><br>
          <span class="badge preset-filled bg-blue-400 text-black"><Star size={18}/>[number]</span>
          <span class="badge preset-filled bg-emerald-500 text-black"><Star size={18}/>[number]</span>
          <span class="badge preset-filled bg-amber-400 text-black"><Star size={18}/>[number]</span>
          <span class="badge preset-filled bg-red-400 text-black"><Star size={18}/>[number]</span>
          <span class="badge preset-filled bg-purple-400 black"><Star size={18}/>[number]</span>
          <br><br>
          <span class="badge preset-filled bg-blue-400 text-black"><Star size={18}/>??</span>
          <span class="badge preset-filled bg-emerald-500 text-black"><Star size={18}/>??</span>
          <span class="badge preset-filled bg-amber-400 text-black"><Star size={18}/>??</span>
          <span class="badge preset-filled bg-red-400 text-black"><Star size={18}/>??</span>
          <span class="badge preset-filled bg-purple-400 black"><Star size={18}/>??</span>
          
          <br><br><h2 class="head2">Side Modes:</h2><br>
          
          <span class="badge preset-filled bg-teal-500 text-black">Dan</span>
          <span class="badge preset-filled bg-orange-500 text-black">Tower</span>
          <br><br>
          <span class="badge preset-filled bg-teal-500 text-black"><Star size={18}/>[number]</span>
          <span class="badge preset-filled bg-orange-500 text-black"><Star size={18}/>[number]</span>
          <br><br>
          <span class="badge preset-filled bg-teal-500 text-black"><Star size={18}/>??</span>
          <span class="badge preset-filled bg-orange-500 text-black"><Star size={18}/>??</span>
          
          <br><br><h2 class="head2">Warnings:</h2><br>
          
          <span class="badge preset-filled bg-red-400 text-black"><MessageCircleWarning/>Explicit Lyrics</span>
          <span class="badge preset-filled bg-red-400 text-black"><TriangleAlert/>Disturbing Content</span>
          <span class="badge preset-filled bg-yellow-400 text-black"><Zap/>Flashing Lights</span>

          <br><br><h2 class="head2">Files:</h2><br>

          <span class="badge preset-filled bg-white-400 text-black"><Image/>Album Cover</span>
          <span class="badge preset-filled bg-white-400 text-black"><Clapperboard/>Video</span>
          <span class="badge preset-filled bg-white-400 text-black"><FileType/>VTT Lyrics</span>

          <br><br><h2 class="head2">Gimmicks:</h2><br>

          <span class="badge preset-filled bg-white-400 text-black"><CirclePlus/>JPOS Scroll</span>
          <span class="badge preset-filled bg-white-400 text-black"><CircleChevronLeft/>HBScroll</span>
          <span class="badge preset-filled bg-white-400 text-black"><Bomb/>Bomb Notes</span>
          <span class="badge preset-filled bg-white-400 text-black"><Bomb/>Fuserolls</span>
          
        
          <br><br><h2 class="head2">Compatibility:</h2><br>

          <span class="badge preset-filled bg-green-400 text-black"><ThumbsUp/>Compatible with all simulators</span>
          <span class="badge preset-filled bg-yellow-400 text-black"><TriangleAlert/>Made for OpenTaiko</span>
          <span class="badge preset-filled bg-red-400 text-black"><Ban/>Only compatible with OpenTaiko</span>
        {/snippet}
      </Accordion.Item>
    </Accordion>
  
  </div>
</div>

<style>

</style> 