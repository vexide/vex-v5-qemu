<script lang="ts">
    export let value: any = "";

    $: value = Math.max(min ?? -Infinity, Math.min(max ?? Infinity, value));

    export let max: number | undefined = undefined;
    export let min: number | undefined = undefined;
</script>

<input bind:value max={max} min={min} class="number-input nodrag" type="number" {...$$restProps} />

<style>
    .number-input {
        font-family: inherit;
        font-weight: 600;
        background: var(--background-secondary);
        border: 1px solid var(--interactive-primary);
        color: var(--foreground-secondary);
        padding-block: 2px;
        padding-inline: 4px 0;
        border-radius: 4px;
        font-size: 14px;
        width: 64px;
    }

    .number-input:focus {
        outline: none;
        border-color: var(--accent-faded);
        box-shadow: 0 0 0 2px var(--accent-faded);
    }

    .number-input[disabled] {
        color: var(--foreground-tertiary);
        border-color: var(--background-tertiary);
    }
</style>
