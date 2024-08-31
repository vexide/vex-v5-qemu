<script lang="ts">
    export let title: string;
    export let barebones = false;
    export let open = true;

    import { ChevronRight } from "svelte-feathers";

    let className = "";
    export { className as class };
</script>

<details class="node-base {className}" bind:open>
    <summary class="node-header">
        <slot name="handle" />
        <slot name="icon" />
        <strong>{title}</strong>
        <ChevronRight size="16" class="caret" />
    </summary>

    <div class="node-body" class:barebones>
        <slot />
    </div>
</details>

<style>
    /* We use a fake version of this because webkit sucks */
    summary::-webkit-details-marker {
        display: none;
    }

    .node-header {
        position: relative;
        display: flex;
        align-items: center;
        gap: 8px;
        width: 100%;
        height: 36px;
        border-radius: 12px;
        padding-inline: 16px 12px;
        background: var(--background-tertiary);
        color: var(--foreground-primary);
        border: none;
    }

    .node-header strong {
        flex: 1 1 auto;
    }

    .node-header > :global(svg) {
        flex: 0 0 auto;
        color: var(--accent-primary);
    }

    .node-header > :global(.caret) {
        transition: 150ms ease transform;
        color: var(--foreground-secondary);
    }

    .node-base[open] :global(.caret) {
        transform: rotate(90deg);
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
