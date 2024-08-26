<script lang="ts" context="module">
    export interface DragData {
        nodeType: string;
        x: number;
        y: number;
        valid: boolean;
    }
</script>

<script lang="ts">
    import { Clock, Hash, Menu, PlusCircle } from "svelte-feathers";

    import { Button, DraggableDevice } from "~/lib/components";
    import { drag } from "~/lib/actions";

    import {
        ADI,
        Motor,
        Controller,
        DistanceSensor,
        OpticalSensor,
        RotationSensor,
        Magnet,
        GenericSerial,
        VisionSensor,
        AIVisionSensor,
        Potentiometer,
        GPSSensor,
        LineTracker,
        LightSensor,
    } from "~/lib/icons";
    import { useSvelteFlow, type Node, type NodeTypes } from "@xyflow/svelte";
    import { createEventDispatcher } from "svelte";

    const { screenToFlowPosition, flowToScreenPosition } = useSvelteFlow();

    const dispatch = createEventDispatcher<{ nodeDrag: DragData }>();

    let width = 260;
    let scrollEdgeTop = true;
    let scrollEdgeBottom = false;
    let widthBeforeCollapse = width;
    let sidebar: HTMLElement | undefined;
    let collapsed = false;

    const MAX_WIDTH = 400;
    const COLLAPSE_BREAKPOINT = 185;

    function handleDragStart(event: MouseEvent, nodeType: string) {
        dispatch("nodeDrag", {
            nodeType,
            x: event.clientX,
            y: event.clientY,
            valid: false,
        });
    }

    function toggleCollapse() {
        if (!collapsed) {
            widthBeforeCollapse = width;
            collapsed = true;
        } else {
            width = widthBeforeCollapse;
            collapsed = false;
        }
    }

    function adjustWidth(newWidth: number) {
        if (newWidth < COLLAPSE_BREAKPOINT) {
            widthBeforeCollapse = width;
            collapsed = true;
        } else {
            width = Math.min(newWidth, MAX_WIDTH);
            collapsed = false;
        }
    }

    function handleSplitterKeyDown({ key }: KeyboardEvent) {
        if (key == "ArrowLeft") {
            adjustWidth(width - 8);
        } else if (key == "ArrowRight") {
            adjustWidth(collapsed ? COLLAPSE_BREAKPOINT : width + 8);
        }
    }

    function handleScroll(event: Event) {
        const target = event.target as HTMLElement;
        scrollEdgeTop = target.scrollTop < 24;
        scrollEdgeBottom =
            target.scrollHeight - target.scrollTop - target.clientHeight < 24;
    }

    const DATA_NODES = [
        {
            name: "Value",
            icon: Hash,
            node: "value",
        },
        {
            name: "Math",
            icon: PlusCircle,
            node: "math",
        },
        {
            name: "Time",
            icon: Clock,
            node: "time",
        },
    ];

    const SMART_DEVICES = [
        {
            name: "Motor",
            icon: Motor,
            node: "motor",
        },
        {
            name: "Controller",
            icon: Controller,
            node: "controller",
        },
        {
            name: "Rotation Sensor",
            icon: RotationSensor,
            node: "rotation",
        },
        {
            name: "Distance Sensor",
            icon: DistanceSensor,
            node: "distance",
        },
        {
            name: "GPS Sensor",
            icon: GPSSensor,
            node: "gps",
        },
        {
            name: "Optical Sensor",
            icon: OpticalSensor,
            node: "optical",
        },
        {
            name: "Vision Sensor",
            icon: VisionSensor,
            node: "vision",
        },
        {
            name: "AI Vision Sensor",
            icon: AIVisionSensor,
            node: "ai_vision",
        },
        {
            name: "Serial Port",
            icon: GenericSerial,
            node: "serial",
        },
        {
            name: "ADI Expander",
            icon: ADI,
            node: "adi",
        },
        {
            name: "Electromagnet",
            icon: Magnet,
            node: "electromagnet",
        },
    ];

    const ADI_DEVICES = [
        {
            name: "Potentiometer",
            icon: Potentiometer,
            node: "potentiometer",
        },
        {
            name: "Line Tracker",
            icon: LineTracker,
            node: "line_tracker",
        },
        {
            name: "Light Sensor",
            icon: LightSensor,
            node: "light_sensor",
        },
    ];
</script>

<aside
    class="sidebar"
    class:collapsed
    class:edge-top={scrollEdgeTop}
    class:edge-bottom={scrollEdgeBottom}
    style:width="{width}px"
    bind:this={sidebar}
>
    <header>
        <h1>Devices</h1>
        <Button
            title="{collapsed ? 'Expand' : 'Collapse'} sidebar"
            on:click={toggleCollapse}
            small
        >
            <Menu size="16" />
        </Button>
    </header>
    <div class="scroller" on:scroll={handleScroll}>
        <ul class="device-category">
            {#each SMART_DEVICES as device}
                <li>
                    <DraggableDevice
                        name={device.name}
                        on:mousedown={(e) => handleDragStart(e, device.node)}
                    >
                        <svelte:component
                            this={device.icon}
                            slot="icon"
                            size="16"
                        />
                    </DraggableDevice>
                </li>
            {/each}
        </ul>
        <hr />
        <ul class="device-category">
            {#each ADI_DEVICES as device}
                <li>
                    <DraggableDevice
                        name={device.name}
                        on:mousedown={(e) => handleDragStart(e, device.node)}
                    >
                        <svelte:component
                            this={device.icon}
                            slot="icon"
                            size="16"
                        />
                    </DraggableDevice>
                </li>
            {/each}
        </ul>
        <hr />
        <ul class="device-category">
            {#each DATA_NODES as device}
                <li>
                    <DraggableDevice
                        name={device.name}
                        on:mousedown={(e) => handleDragStart(e, device.node)}
                    >
                        <svelte:component
                            this={device.icon}
                            slot="icon"
                            size="16"
                        />
                    </DraggableDevice>
                </li>
            {/each}
        </ul>
    </div>
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
        display: flex;
        flex-direction: column;
        flex: 0 0 auto;
        width: 260px;
        max-width: 400px;
        min-height: 0;
        height: 100%;
        background-color: var(--background-secondary);
        border-right: 1px solid var(--interactive-primary);
    }

    .sidebar header {
        flex: 0 0 auto;
        display: flex;
        align-items: center;
        justify-content: space-between;
        position: relative;
        height: 46px;
        padding-inline: 16px 8px;
        border-bottom: 1px solid var(--interactive-primary);
    }

    .sidebar header h1 {
        margin: 0;
        font-size: 16px;
        font-weight: 600;
        font-family: var(--font-family-text);
    }

    .sidebar .scroller {
        --scrollbar-color: var(--background-secondary);
        position: relative;
        padding: 8px;
        overflow: auto;
        flex: 1 1 auto;
    }

    .sidebar .scroller::-webkit-scrollbar {
        display: none;
    }

    .sidebar::before,
    .sidebar::after {
        content: "";
        z-index: 1;
        pointer-events: none;
        position: absolute;
        left: 0;
        width: 100%;
        height: 24px;
        opacity: 1;
        transition: 150ms ease;
        background: var(--background-secondary);
    }

    .sidebar::before {
        top: 46px;
        mask: linear-gradient(black, transparent);
        -webkit-mask: linear-gradient(black, transparent);
    }

    .sidebar::after {
        bottom: 0;
        mask: linear-gradient(transparent, black);
        -webkit-mask: linear-gradient(transparent, black);
    }

    .sidebar.edge-top::before,
    .sidebar.edge-bottom::after {
        opacity: 0;
    }

    .device-category {
        margin: 0;
        display: flex;
        flex-direction: column;
        list-style-type: none;
        gap: 8px;
    }

    .sidebar hr {
        margin-block: 16px;
        border: none;
        border-top: 1px solid var(--interactive-primary);
    }

    .device-category :global(.device) {
        width: 100%;
        gap: 8px;
        justify-content: flex-start;
        letter-spacing: normal;
        text-transform: none;
        font-size: 14px;
        background-color: var(--background-tertiary);
        color: var(--foreground-primary);
    }

    .device-category {
        font-size: 16px;
        margin: 0;
        padding: 0;
    }

    .sidebar.collapsed header h1,
    .sidebar.collapsed :global(.device-name) {
        display: none;
    }

    .sidebar.collapsed header {
        padding: 0 8px;
    }

    .sidebar.collapsed {
        width: fit-content !important;
    }
</style>
