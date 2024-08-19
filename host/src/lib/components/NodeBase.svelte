<script lang="ts">
    export let title: string;
    export let barebones = false;
    export let open = true;

    import {ChevronDown} from "svelte-feathers";
</script>

<details {open}>
    <summary>
        <header class="node-header">
            <slot name="handle" />
            <slot name="icon" />
            <strong>{title}</strong>
        </header>
    </summary>

    <div class="node-body" class:barebones>
        <slot />
    </div>
</details>

<style>
    details {
        background-color: transparent;
    }
    details > summary  {
        display: flex;
        flex-direction: row-reverse;
        align-items: center;
        list-style: none;
        padding-inline-end: 8px;
    }

    .node-header {
        position: relative;
        display: flex;
        align-items: center;
        gap: 8px;
        width: 100%;
        height: 36px;
        padding-inline: 16px;
        border-radius: 12px 12px 0px 12px;
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
