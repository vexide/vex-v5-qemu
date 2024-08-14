<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import Button from "./Button.svelte";

    import drag from "./drag";

    import PlusIcon from "svelte-feathers/Plus.svelte";

    const dispatch = createEventDispatcher();

    let width = "260px";
    let sidebar: HTMLElement | undefined;
</script>

<aside class="sidebar" style:width bind:this={sidebar}>
    <div
        class="splitter"
        use:drag={(event) => {
            if (!sidebar) return;

            const sidebarRect = sidebar.getBoundingClientRect();
            width = (event.clientX - sidebarRect.left) + "px";
        }}
    ></div>
    <header>
        <h1>Devices</h1>
        <Button
            variant="accent"
            title="Add a device"
            small
            on:click={() => dispatch("add")}
        >
            <PlusIcon size="16" />
            Add
        </Button>
    </header>
    <div
        style="font-size: 14px; color: var(--foreground-secondary); padding: 16px;"
        >No devices.</div
    >
</aside>

<style>
    .splitter {
        position: absolute;
        right: -3px;
        width: 4px;
        height: 100%;
        z-index: 1;
        transition: 0.2s ease;
        cursor: ew-resize;
        box-sizing: content-box;
        background-clip: content-box;
        border-left: 1px solid transparent;
        border-right: 1px solid transparent;
    }

    .splitter:hover {
        background-color: var(--accent-faded);
    }

    .sidebar {
        position: relative;
        flex: 0 0 auto;
        width: 260px;
        min-width: 185px;
        max-width: 40%;
        min-height: 0;
        height: 100%;
        background-color: var(--background-secondary);
        border-right: 1px solid var(--interactive-primary);
    }

    .sidebar header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        position: relative;
        height: 48px;
        padding-inline: 16px 10px;
        border-bottom: 1px solid var(--interactive-primary);
    }

    .sidebar header h1 {
        margin: 0;
        font-size: 16px;
        font-weight: 600;
        font-family: var(--font-family-text);
    }
</style>
