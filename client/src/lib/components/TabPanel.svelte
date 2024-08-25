<script context="module" lang="ts">
    export interface TabCtx {
        id: string;
        label: string;
    }
</script>

<script lang="ts">
    import { getContext, onDestroy, onMount } from "svelte";
    import type { Writable } from "svelte/store";

    const uuid = getContext<string>("uuid");
    const tabs = getContext<Writable<TabCtx[]>>("tabs");
    const selectedId = getContext<Writable<string | undefined>>("selectedId");

    export let label: string;
    export let id: string;

    let prevLabel: string | undefined = label;
    let prevId: string | undefined = id;

    $: updateLabelCtx(label);
    $: updateIdCtx(id);

    function updateLabelCtx(label: string) {
        if (!prevLabel || prevId) return;

        const currentTab = $tabs.find((t) => t.id === prevId);
        if (currentTab) currentTab.label = label;

        prevLabel = label;
    }

    function updateIdCtx(id: string) {
        if (!prevId) return;

        const currentTab = $tabs.find((t) => t.id === id);
        if (currentTab) currentTab.id = id;

        prevId = id;
    }

    onMount(() => {
        $tabs.push({ id, label });
        $tabs = $tabs;
    });

    onDestroy(() => {
        $tabs = $tabs.filter((t) => t.id === id);
    });
</script>

{#if $selectedId === id}
    <div
        class="tab-panel"
        aria-labelledby="tab-{uuid}-{id}"
        id="tabpanel-{uuid}-{id}"
        role="tabpanel"
    >
        <slot />
    </div>
{/if}

<style>
    .tab-panel {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }
</style>
