<script lang="ts">
    export let value: any = "";
</script>

<input bind:value class="text-input" type="text" {...$$restProps} />

<style>
    .text-input {
        font-family: inherit;
        font-weight: 600;
        background: var(--background-secondary);
        border: 1px solid var(--interactive-primary);
        color: var(--foreground-secondary);
        padding-block: 2px;
        padding-inline: 4px 0;
        border-radius: 4px;
        font-size: 14px;
    }

    .text-input:focus {
        outline: none;
        border-color: var(--accent-faded);
        box-shadow: 0 0 0 2px var(--accent-faded);
    }
</style>
