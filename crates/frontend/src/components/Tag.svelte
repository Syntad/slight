<script lang="ts">
    import cn from 'classnames';

    let clazz: string = '';
    export { clazz as class };
</script>

<span
    on:click
    on:keydown
    class={cn(
        clazz,
        'bg-gray-700 uppercase font-semibold rounded-sm px-2 text-xs whitespace-nowrap'
    )}
>
    <slot />
</span>
