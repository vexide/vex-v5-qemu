<script lang="ts">
    export let title: string;
    export let barebones = false;
    export let open = true;

    import { ChevronRight } from "svelte-feathers";
</script>

<details bind:open>
    <summary>
        <header class="node-header">
            <div class="node-header-left">
                <slot name="handle" />
                <slot name="icon" />
                <strong>{title}</strong>
            </div>
            <div class="node-header-right">
                <div class="caret">
                    <ChevronRight />
                </div>
            </div>
        </header>
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
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        height: 36px;
        border-radius: 12px;
        background: var(--background-tertiary);
        color: var(--foreground-primary);
        border: none;
    }

    .node-header > :global(svg) {
        color: var(--accent-primary);
    }

    .node-header-left {
        display: flex;
        gap: 8px;
        padding-inline: 16px;
    }

    .node-header-right {
        display: flex;
        align-items: end;
        gap: 8px;
        padding-inline: 4px;
    }

    details[open] .node-header-right {
        padding-inline: 8px;
    }

    .caret {
        transition: transform 0.2s;
    }
    details[open] .caret {
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
