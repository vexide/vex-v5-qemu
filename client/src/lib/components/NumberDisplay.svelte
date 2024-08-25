<script lang="ts">
    export let value: number;
    export let decimals: number = 2;
</script>

<div class="number-display">
    {value.toFixed(decimals)}
</div>

<style>
    .number-display {
        width: 64px;
        border: 1px solid var(--interactive-primary);
        padding-block: 2px;
        padding-inline: 4px 0;
        border-radius: 4px;
        overflow-x: hidden;
    }
</style>
