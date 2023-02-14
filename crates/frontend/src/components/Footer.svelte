<script>
    import Tag from './Tag.svelte';
</script>

<div
    class="relative flex h-8 items-center justify-between rounded-b-lg border-t border-gray-700 bg-gray-800/80 px-4">
    <div class="flex items-center justify-center px-4">
        <img src="/imgs/logo.svg" class="h-full w-3 opacity-40" alt="" />
    </div>

    <!-- <btn
        class="flex justify-center items-center border-l px-2 border-gray-700 text-sm h-full hover:bg-gray-700 cursor-pointer rounded-br-lg"
    >
        Quick Actions
    </btn> -->

    <Tag
        class="min-w-sm cursor-pointer border border-orange-400/20 bg-orange-300/20 text-orange-400"
        >STACK</Tag>
    <!-- <Tag class="bg-green-300/20">GPT</Tag> -->
</div>
