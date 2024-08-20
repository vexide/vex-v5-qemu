<script lang="ts">
    import type { TabCtx } from "./TabPanel.svelte";
    import { setContext } from "svelte";
    import { writable, type Writable } from "svelte/store";

    export let selected: string | undefined = undefined;

    let tabsElement: HTMLDivElement | undefined;

    const uuid = crypto.randomUUID();
    const tabs = writable<TabCtx[]>([]);
    const selectedId = writable<string | undefined>(selected);

    setContext("uuid", uuid);
    setContext("tabs", tabs);
    setContext("selectedId", selectedId);

    $: selected, $selectedId = selected;

    function handleKeyDown(event: KeyboardEvent) {
        const key = event.key;
        const selectedTab = $tabs.find(t => t.id === $selectedId);
        const selectedIndex = selectedTab ? $tabs.indexOf(selectedTab) : 0;

        let newIndex: number = 0;

        if (key === "ArrowLeft") {
            newIndex = selectedIndex === 0 ? $tabs.length - 1 : selectedIndex - 1;
        } else if (key === "ArrowRight") {
            newIndex = selectedIndex === $tabs.length - 1 ? 0 : selectedIndex + 1;
        } else if (key === "Home") {
            newIndex = 0;
        } else {
            newIndex = $tabs.length - 1;
        }

        (tabsElement?.children[newIndex] as HTMLElement | undefined)?.focus();
        selected = $tabs[newIndex]?.id;
    }
</script>

<div class="tabs" role="tablist" bind:this={tabsElement} {...$$restProps}>
    {#each $tabs as tab}
        {@const isSelected = (tab.id === selected)}

        <button
            class="tab"
            class:selected={isSelected}
            tabindex={isSelected ? -1 : 1}
            id="tabs-{uuid}-{tab.id}"
            role="tab"
            aria-controls="tabpanel-{uuid}-{tab.id}"
            aria-selected={isSelected}
            on:click={() => (selected = tab.id)}
            on:keydown={handleKeyDown}
        >
            {tab.label}
        </button>
    {/each}
</div>

<slot />

<style>
    .tabs {
        display: flex;
        gap: 2px;
        padding: 2px;
        background-color: var(--interactive-primary);
        border-radius: 6px;
        width: 100%;
    }

    .tab {
        position: relative;
        color: var(--foreground-secondary);
        background-color: transparent;
        border: none;
        border-radius: 4px;
        text-align: center;
        font-family: inherit;
        font-size: 14px;
        padding-inline: 16px;
        padding-block: 2px;
        font-weight: 600;
        flex: 1;
        cursor: pointer;
    }

    .tab:hover {
        background-color: var(--interactive-secondary);
    }

    .tab:focus-visible {
        z-index: 1;
        outline: 2px solid var(--accent-faded);
        outline-offset: 2px;
    }

    .tab.selected {
        cursor: default;
        background-color: var(--accent-primary);
        color: var(--background-primary);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }
</style>
