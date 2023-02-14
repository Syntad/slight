<script lang="ts">
    import { unescapeHTML } from '@/utils/stackflowUtils';
    import type { Item } from '@/types/stackflowTypes';
    import { open } from '@tauri-apps/api/shell';
    import Tag from './Tag.svelte';

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

    const handleError = (ev) =>
        (ev.target.src =
            'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7');
</script>

<div class="flex rounded-lg p-4 hover:bg-gray-700">
    <div
        class="relative mr-4 h-10 w-10 shrink-0 overflow-hidden rounded-full border-2 border-transparent hover:border-gray-300">
        <svg
            class="absolute -left-0.5 h-10 w-10 rounded-full bg-gray-600 text-gray-400"
            fill="currentColor"
            viewBox="0 0 20 20"
            xmlns="http://www.w3.org/2000/svg"
            ><path
                fill-rule="evenodd"
                d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"
                clip-rule="evenodd"></path
            ></svg>

        <img
            class="absolute h-10 w-10 cursor-pointer rounded-full"
            src="{item.owner.profile_image}"
            alt=""
            on:error="{handleError}"
            on:click="{openUser}"
            on:keypress="{openUser}" />
    </div>

    <div
        class="flex w-full items-center justify-between border-b border-gray-600 pb-2">
        <div class="max-w-sm">
            <p
                class="cursor-pointer truncate hover:underline"
                on:click="{() => open(item.link)}"
                on:keypress="{() => open(item.link)}">
                {unescapeHTML(item.title)}
            </p>

            <div class="flex gap-2 overflow-scroll">
                {#each item.tags as tag}
                    <Tag
                        class="cursor-pointer hover:bg-gray-500"
                        on:click="{() => open(TAG_REF + tag)}"
                        on:keydown="{() => open(TAG_REF + tag)}">{tag}</Tag>
                {/each}
            </div>
        </div>

        {#if item.accepted_answer_id !== undefined}
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="h-10 w-10 cursor-pointer rounded-full p-1 text-green-400 transition-all duration-150 hover:bg-gray-600"
                on:click="{openLink}"
                on:keypress="{openLink}">
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                ></path>
            </svg>
        {/if}
    </div>
</div>
