<script lang="ts">
    import { unescapeHTML } from '../utils/StackOverflowUtil';
    import type { Item } from '../types/StackOverflowTypes';
    import { open } from '@tauri-apps/api/shell';

    const ANSWER_REF = 'https://stackoverflow.com/a/';

    export let item: Item;

    function openLink() {
        open(ANSWER_REF + item.accepted_answer_id);
    }

    const handleError = (ev) => (ev.target.src = '/imgs/avatar.png');
</script>

<div
    class="flex rounded-lg p-4 cursor-pointer hover:bg-gray-800"
    on:click={() => openLink()}
    on:keypress={() => openLink()}
>
    <img
        class="w-10 h-10 rounded-full mr-4"
        src={item.owner.profile_image}
        alt={item.owner.display_name}
        on:error={handleError}
    />

    <div class="w-full border-b border-gray-600">
        <p class="truncate max-w-sm">{unescapeHTML(item.title)}</p>

        <div class="flex gap-2 mb-2 overflow-scroll">
            {#each item.tags as tag}
                <span
                    class="bg-gray-700 uppercase font-semibold rounded-sm px-2 text-xs whitespace-nowrap"
                    >{tag}</span
                >
            {/each}
        </div>
    </div>
</div>
