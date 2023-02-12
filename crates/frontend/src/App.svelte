<script lang="ts">
    import Footer from './components/Footer.svelte';
    import SearchBar from './components/SearchBar.svelte';
    import SearchResults from './components/SearchResults.svelte';
</script>

<main id="container" class="rounded-lg bg-gray-900/80 border border-gray-600">
    <div class="p-4">
        <SearchBar />
        <SearchResults />
    </div>

    <Footer />
</main>
