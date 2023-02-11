<script lang="ts">
    import { unescapeHTML } from '../utils/StackOverflowUtil';
    import type { Item } from '../types/StackOverflowTypes';
    import { open } from '@tauri-apps/api/shell';

    const ANSWER_REF = 'https://stackoverflow.com/a/';
    const TAG_REF = 'https://stackoverflow.com/questions/tagged/';
    const USER_REF = 'https://stackoverflow.com/users/';

    export let item: Item;

    function openLink() {
        open(ANSWER_REF + item.accepted_answer_id);
    }

    function openUser() {
        open(USER_REF + item.owner.user_id);
    }

    const handleError = (ev) => (ev.target.src = '/imgs/avatar.png');
</script>

<div class="flex rounded-lg p-4 hover:bg-gray-800 max-w-full">
    <img
        class="w-10 h-10 rounded-full mr-4 border-2 border-transparent hover:border-gray-600 cursor-pointer"
        src={item.owner.profile_image}
        alt={item.owner.display_name}
        on:error={handleError}
        on:click={openUser}
        on:keypress={openUser}
    />

    <div
        class="flex items-center justify-between pb-2 w-full border-b border-gray-600"
    >
        <div class="max-w-sm">
            <p
                class="truncate hover:underline cursor-pointer"
                on:click={() => open(item.link)}
                on:keypress={() => open(item.link)}
            >
                {unescapeHTML(item.title)}
            </p>

            <div class="flex gap-2 overflow-scroll">
                {#each item.tags as tag}
                    <span
                        class="bg-gray-700 hover:bg-gray-500 cursor-pointer uppercase font-semibold rounded-sm px-2 text-xs whitespace-nowrap"
                        on:click={() => open(TAG_REF + tag)}
                        on:keydown={() => open(TAG_REF + tag)}>{tag}</span
                    >
                {/each}
            </div>
        </div>

        {#if item.accepted_answer_id != undefined}
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-10 h-10 text-green-400 cursor-pointer p-1 hover:bg-gray-600 rounded-full transition-all duration-150"
                on:click={openLink}
                on:keypress={openLink}
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
            </svg>
        {/if}
    </div>
</div>
