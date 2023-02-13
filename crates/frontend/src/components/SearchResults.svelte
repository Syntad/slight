<script lang="ts">
    import Result from '../components/Result.svelte';

    import { results } from '../stores/StackOverflowStore';
</script>

<div
    id="r-container"
    class="flex flex-col gap-2 max-h-64 overflow-x-hidden overflow-scroll"
>
    {#each $results as result}
        <Result bind:item={result} />
    {/each}
</div>
