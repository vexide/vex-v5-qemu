<script lang="ts">
    export let title: string;
    export let barebones = false;
</script>

<header class="node-header">
    <slot name="handle" />
    <slot name="icon" />
    <strong>{title}</strong>
</header>

<div class="node-body" class:barebones>
    <slot />
</div>

<style>
    .node-header {
        position: relative;
        display: flex;
        align-items: center;
        gap: 8px;
        width: 100%;
        height: 36px;
        padding-inline: 16px;
        border-radius: 12px 12px 0 0;
        background: var(--background-tertiary);
        color: var(--foreground-primary);
        border: none;
    }

    .node-header > :global(svg) {
        color: var(--accent-primary);
    }

    .node-body {
        padding: 16px;
        border-radius: 0 0 12px 12px;
        display: flex;
        flex-direction: column;
        gap: 8px;
        background-color: var(--background-secondary);
    }

    .node-body.barebones {
        background-color: transparent;
    }
</style>
