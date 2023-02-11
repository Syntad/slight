<script lang="ts">
    import { appWindow, LogicalSize } from '@tauri-apps/api/window';
    import { onMount, afterUpdate } from 'svelte';
    import Result from '../components/Result.svelte';

    import { results } from '../stores/StackOverflowStore';

    let container: Element = null;

    onMount(() => {
        container = document.getElementById('container');
    });

    afterUpdate(async () => {
        if (container == null) return;

        await appWindow.setSize(
            new LogicalSize(600, container.clientHeight + 2)
        );
    });
</script>

<div class="flex flex-col gap-2 max-h-64 overflow-x-hidden overflow-scroll">
    {#each $results as result}
        <Result bind:item={result} />
    {/each}
</div>
