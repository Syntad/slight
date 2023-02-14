<script>
    import Tag from './Tag.svelte';
</script>

<div
    class="relative flex h-8 px-4 items-center justify-between rounded-b-lg bg-gray-800/80 border-t border-gray-700"
>
    <div class="flex justify-center items-center px-4">
        <img src="/imgs/logo.svg" class="w-3 h-full opacity-40" alt="" />
    </div>

    <!-- <btn
        class="flex justify-center items-center border-l px-2 border-gray-700 text-sm h-full hover:bg-gray-700 cursor-pointer rounded-br-lg"
    >
        Quick Actions
    </btn> -->

    <Tag class="bg-orange-300/20">STACK</Tag>
    <!-- <Tag class="bg-green-300/20">GPT</Tag> -->
</div>
