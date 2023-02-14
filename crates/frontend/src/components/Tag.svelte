<script lang="ts">
    import cn from 'classnames';

    let clazz: string = '';
    export { clazz as class };
</script>

<span
    on:click
    on:keydown
    class="{cn(
        clazz,
        'whitespace-nowrap rounded-sm bg-gray-700 px-2 text-xs font-semibold uppercase'
    )}">
    <slot />
</span>
