<script lang="ts">
    import { Menu } from "svelte-feathers";
    import { dndType } from "~/lib/stores";
    import Button from "~/lib/Button.svelte";
    import drag from "~/lib/drag";
    import ADI from "./icons/ADI.svelte";
    import Motor from "./icons/Motor.svelte";
    import Controller from "./icons/Controller.svelte";
    import DistanceSensor from "./icons/DistanceSensor.svelte";
    import OpticalSensor from "./icons/OpticalSensor.svelte";
    import RotationSensor from "./icons/RotationSensor.svelte";
    import Magnet from "./icons/Magnet.svelte";
    import GenericSerial from "./icons/GenericSerial.svelte";
    import VisionSensor from "./icons/VisionSensor.svelte";
    import AiVisionSensor from "./icons/AIVisionSensor.svelte";
    import Potentiometer from "./icons/Potentiometer.svelte";
    import Gps from "./icons/GPS.svelte";
    import LineTracker from "./icons/LineTracker.svelte";


    let width = 260;
    let widthBeforeCollapse = width;
    let sidebar: HTMLElement | undefined;
    let collapsed = false;

    const MAX_WIDTH = 400;
    const COLLAPSE_BREAKPOINT = 185;

    function handleDragStart(event: DragEvent, nodeType: string) {
        if (!event.dataTransfer) {
            return;
        }

        $dndType = nodeType;

        event.dataTransfer.effectAllowed = "move";
        // Workaround for a webkit bug.
        // see: https://github.com/tauri-apps/tauri/issues/6695
        event.dataTransfer.setData(
            "text/plain",
            (event.target as HTMLElement).id,
        );
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
</script>

<aside
    class="sidebar"
    class:collapsed
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
    <ul class="device-picker">
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <Motor />
                <span class="device-label">Motor (11w)</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <Motor />
                <span class="device-label">Motor (5.5w)</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <Controller />
                <span class="device-label">Controller</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <RotationSensor />
                <span class="device-label">
                    Rotation Sensor
                </span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <Potentiometer />
                <span class="device-label">
                    ADI Potentiometer
                </span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <DistanceSensor />
                <span class="device-label">Distance Sensor</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <Gps size="200px"/>
                <span class="device-label">GPS Sensor</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <OpticalSensor />
                <span class="device-label">Optical Sensor</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <VisionSensor />
                <span class="device-label">Vision Sensor</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <AiVisionSensor />
                <span class="device-label">AI Vision Sensor</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <LineTracker />
                <span class="device-label">ADI Line Tracker</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <GenericSerial />
                <span class="device-label">Serial Port</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <ADI />
                <span class="device-label">ADI Expander</span>
            </Button>
        </li>
        <li>
            <Button
                small
                draggable
                class="device"
                on:dragstart={(event) => handleDragStart(event, "input")}
            >
                <Magnet />
                <span class="device-label">Electromagnet</span>
            </Button>
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
        max-width: 400px;
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

    .device-picker {
        padding: 8px;
        margin: 0;
        display: flex;
        flex-direction: column;
        list-style-type: none;
        gap: 8px;
    }

    .device-picker :global(.device) {
        width: 100%;
        gap: 8px;
        justify-content: flex-start;
        letter-spacing: normal;
        text-transform: none;
        font-size: 14px;
        background-color: var(--background-tertiary);
        color: var(--foreground-primary);
    }

    .device-picker :global(.device svg) {
        color: var(--foreground-secondary);
        width: 16px;
        height: auto;
    }

    .sidebar.collapsed header h1,
    .sidebar.collapsed .device-label {
        display: none;
    }

    .sidebar.collapsed header {
        padding: 0 8px;
    }

    .sidebar.collapsed {
        width: fit-content !important;
    }
</style>
