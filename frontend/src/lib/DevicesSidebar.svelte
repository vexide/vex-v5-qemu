<script>
    import { createEventDispatcher } from "svelte";
    import Button from "./Button.svelte";

    import PlusIcon from "svelte-feathers/Plus.svelte";

    const dispatch = createEventDispatcher();
</script>

<aside class="sidebar">
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
    <div class="no-devices">No devices.</div>
</aside>


<style>
    .no-devices {
        font-size: 14px;
        color: var(--foreground-secondary);
        padding: 16px;
    }

    .sidebar {
        flex: 0 0 auto;
        width: 260px;
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
