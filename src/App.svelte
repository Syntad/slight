<script lang="ts">
    import SearchBar from './components/SearchBar.svelte';
    import SearchResults from './components/SearchResults.svelte';
</script>

<main id="container" class="rounded-lg bg-black/40 border border-gray-600">
    <!-- svelte-ignore empty-block -->

    <div class="p-4">
        <SearchBar />
        <SearchResults />
    </div>

    <div
        class="bg-gray-700/40 border-t border-gray-700 px-4 w-full rounded-b-lg"
    >
        Slight
    </div>
</main>
