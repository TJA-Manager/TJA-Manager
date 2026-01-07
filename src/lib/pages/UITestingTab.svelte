<script lang="ts">
    import SongInfo from "$lib/components/SongInfo.svelte";
    import SongCard from "$lib/components/SongCard.svelte";
</script>

<main>
    <h1>SongInfo Testing</h1>
    <hr>
    <section class="grid grid-cols-3 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5">
        <SongCard songTitle="Title 1"/>
        <SongCard songTitle="Title 2"/>
        <SongCard songTitle="Title 3"/>
        <SongCard songTitle="Title 1"/>
        <SongCard songTitle="Title 2"/>
        <SongCard songTitle="Title 3"/>
    </section>
    <h1>Buttons</h1>
    <hr>
    <p>
        <button id="clone-button" type="button" class="btn btn-action my-2">Action</button>
        <button id="clone-button" type="button" class="btn btn-download my-2">Download</button>
        <button id="clone-button" type="button" class="btn btn-repeat my-2">Repeat</button>
        <button id="clone-button" type="button" class="btn btn-destructive my-2">Destructive/Error</button>
        <button id="clone-button" type="button" class="btn btn-ghost my-2">Ghost</button>
    </p>
    

    <!-- 
        <SongInfo
            songTitle="[title]"
            songSubtitle="[subtitle]"
            songCharter="[charter]"

            songTjaUrl=""
            songAudioUrl="https://ese.tjadataba.se/BeaniCraft/BeaniCrafts-Charts/raw/branch/main/BC0%20Beani%27s%20Charts/06%20Synthesizer%20V%20Music/Spoken%20For/Spoken%20For.ogg"
            songImageUrl="/src/lib/images/album-default.png"

            easyRating={0}
            normalRating={0}
            hardRating={0}
            oniRating={9}
            editRating={0}
        />
    -->
</main>

<style>
</style>