<script lang="ts">
    import Result from '@/components/Result.svelte';

    import { results } from '@/stores/stackflowStore';
</script>

<div
    id="r-container"
    class="flex max-h-64 flex-col gap-2 overflow-scroll overflow-x-hidden">
    {#each $results as result}
        <Result bind:item="{result}" />
    {/each}
</div>
