<script lang="ts">
    import { Button } from "~/lib/components";

    export let name: string;
</script>

<Button small draggable class="draggable-device" title="{name} (double-click or drag)" on:dragstart>
    <slot name="icon" />
    <span class="device-name">{name}</span>
</Button>

<style>
    :global(.button.small.draggable-device) {
        width: 100%;
        gap: 8px;
        justify-content: flex-start;
        letter-spacing: normal;
        text-transform: none;
        font-size: 14px;
        background-color: var(--background-tertiary);
        color: var(--foreground-primary);
    }

    :global(.draggable-device > svg) {
        color: var(--foreground-secondary);
        width: 16px;
        height: auto;
    }
</style>
