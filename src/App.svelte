<script lang="ts">
    import { appWindow, LogicalSize } from '@tauri-apps/api/window';
    import { afterUpdate } from 'svelte';
    import Result from './components/Result.svelte';
    import SearchBar from './components/SearchBar.svelte';
    import { results } from './stores/StackOverflowStore';

    const onBlur = async () => {
        await appWindow.hide();
    };

    afterUpdate(async () => {
        const height = document.getElementById('container').clientHeight;

        await appWindow.setSize(new LogicalSize(500, height + 2));
    });
</script>

<svelte:window on:blur={onBlur} />

<main
    id="container"
    class="h-fit rounded-lg bg-black/40 border border-gray-600 overflow-hidden"
>
    <div class="p-4">
        <SearchBar />

        <div class="flex flex-col gap-2 max-h-64 overflow-scroll">
            {#each $results as result}
                <Result bind:item={result} />
            {/each}
        </div>
    </div>

    <div
        class="bg-gray-700/40 border-t border-gray-700 px-4 w-full rounded-b-lg"
    >
        Slight
    </div>
</main>
