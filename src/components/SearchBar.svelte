<script lang="ts">
    import cn from 'classnames';
    import { search } from '../utils/StackOverflowUtil';
    import { results } from '../stores/StackOverflowStore';

    let query = '';

    async function onUpdate(newQuery: string) {
        if (isNullOrWhitespace(newQuery)) {
            console.log('WHAT');
            results.set([]);
            return;
        }

        results.set(await search(newQuery));
    }

    function isNullOrWhitespace(input) {
        return !input || !input.trim();
    }

    $: onUpdate(query);
</script>

<input
    class={cn(
        !isNullOrWhitespace(query) &&
            query.length > 0 &&
            'border-b-2 border-b-gray-600 mb-4',
        'w-full px-2 pt-1 pb-2 bg-transparent'
    )}
    placeholder="What do you need?"
    bind:value={query}
/>

{#if !isNullOrWhitespace(query) && query.length > 0 && $results.length <= 0}
    <p class="text-center">No Results.</p>
{/if}
