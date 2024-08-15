<script lang="ts">
    import { getContext } from "svelte";
    import drag from "./drag";
    import type { Writable } from "svelte/store";
    import { Menu } from "svelte-feathers";
    import Button from "./Button.svelte";

    let width = 260;
    let widthBeforeCollapsed = width;
    let sidebar: HTMLElement | undefined;
    let collapsed = false;

    const dndType = getContext("dnd") as Writable<string | null>;

    function handleDragStart(event: DragEvent, nodeType: string) {
        if (!event.dataTransfer) {
            return;
        }

        $dndType = nodeType;

        event.dataTransfer.effectAllowed = 'move';
        // Workaround for a webkit bug.
        // see: https://github.com/tauri-apps/tauri/issues/6695
        event.dataTransfer.setData("text/plain", (event.target as HTMLElement).id);
    }

    function toggleCollapse() {
        if (!collapsed) {
            widthBeforeCollapsed = width;
            collapsed = true;
        } else {
            width = widthBeforeCollapsed;
            collapsed = false;
        }
    }

    function adjustWidth(newWidth: number) {
        if (newWidth < 150) {
            collapsed = true;
        } else {
            collapsed = false;
            width = newWidth;
        }
    }

    function handleSplitterKeyDown({ key }: KeyboardEvent) {
        if (key == "ArrowLeft") {
            adjustWidth(width - 8);
        } else if (key == "ArrowRight") {
            adjustWidth(width + 8);
        }
    }
</script>

<aside class="sidebar" class:collapsed style:width="{width}px" bind:this={sidebar}>
    <header>
        <h1>Devices</h1>
        <Button title="{collapsed ? "Expand" : "Collapse"} sidebar" on:click={toggleCollapse} small>
            <Menu size="16" />
        </Button>
    </header>
    <ul class="device-picker">
        <li>
            <button
                class="device"
                on:dragstart={(event) => handleDragStart(event, 'input')}
                title="Motor (double-click or drag to add)"
                draggable={true}
            >
                <svg width="29" height="24" viewBox="0 0 29 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <rect x="6" y="1" width="22" height="22" rx="2" stroke="currentColor" stroke-width="2"/>
                    <mask id="path-2-inside-1_0_1" fill="white">
                    <path d="M0 12C0 11.4477 0.447715 11 1 11H7V16H1C0.447715 16 0 15.5523 0 15V12Z"/>
                </mask>
                <path d="M0 12C0 11.4477 0.447715 11 1 11H7V16H1C0.447715 16 0 15.5523 0 15V12Z" stroke="currentColor" stroke-width="4" mask="url(#path-2-inside-1_0_1)"/>
                    <mask id="path-3-inside-2_0_1" fill="white">
                        <path d="M0 16C0 15.4477 0.447715 15 1 15H7V20H1C0.447715 20 0 19.5523 0 19V16Z"/>
                    </mask>
                    <path d="M0 16C0 15.4477 0.447715 15 1 15H7V20H1C0.447715 20 0 19.5523 0 19V16Z" stroke="currentColor" stroke-width="4" mask="url(#path-3-inside-2_0_1)"/>
                    <rect x="11" y="2" width="2" height="20" fill="currentColor"/>
                    <path d="M0 6.5C0 6.22386 0.223858 6 0.5 6H5V8H0.5C0.223858 8 0 7.77614 0 7.5V6.5Z" fill="var(--accent-primary)"/>
                </svg>
                <span class="device-label">Motor</span>
            </button>
        </li>
    </ul>
    <!--
        svelte-ignore a11y-no-noninteractive-element-interactions a11y-no-noninteractive-tabindex
        (resize splitters are a special usecase, since they don't have a dedicated ARIA role, see https://github.com/w3c/aria/issues/1348)
    -->
    <div
        class="splitter"
        role="separator"
        tabindex="0"
        on:keydown={handleSplitterKeyDown}
        on:dblclick={toggleCollapse}
        use:drag={(event) => {
            if (!sidebar) return;
            adjustWidth(event.clientX - sidebar.getBoundingClientRect().left);
        }}
    ></div>
</aside>

<style>
    .splitter {
        position: absolute;
        top: 0;
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
        outline: none;
    }

    .splitter:hover,
    .splitter:focus-visible {
        background-color: var(--accent-faded);
    }

    .sidebar {
        position: relative;
        flex: 0 0 auto;
        width: 260px;
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

    .device-picker {
        padding: 8px;
        margin: 0;
        display: flex;
        flex-direction: column;
        list-style-type: none;
    }

    .device {
		display: flex;
		align-items: center;
		vertical-align: middle;
		box-sizing: border-box;
		text-align: center;
		font-family: inherit;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 600;
		padding-inline: 16px;
		padding-block: 10px;
		gap: 12px;
		border: none;
		outline: none;
		cursor: pointer;
		transition: 150ms ease;
        width: 100%;
        color: var(--foreground-primary);
        background-color: var(--interactive-primary);
    }

    .device svg {
        color: var(--foreground-secondary);
        width: 16px;
        height: 16px;
    }

    .device:hover {
        background-color: var(--interactive-secondary);
    }

    .device:active {
        background-color: var(--interactive-tertiary);
    }

    .device:focus {
        box-shadow: 0 0 0 3px var(--accent-faded);
    }

    .sidebar.collapsed header h1,
    .sidebar.collapsed .device-label {
        display: none;
    }

    .sidebar.collapsed header {
        padding: 0 10px;
    }

    .sidebar.collapsed {
        width: fit-content !important;
    }
</style>
