<script>
    let { header, content, footer, imgUrl } = $props();
</script>

<section class="card preset-filled-surface-200-800 p-0! flex flex-col divide-surface-300-700 divide-y overflow-hidden">
    <header>
        <img src={imgUrl} class="w-full aspect-20/9" alt="Tool card banner" />
    </header>

    <article class="p-2.5 grow">
        <h4>{@render header()}</h4>
        <hr>
        <p>{@render content()}</p>
    </article>

    <footer class="mt-auto p-2.5">
        {@render footer()}
    </footer>
</section>

<style>
</style>