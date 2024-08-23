<script lang="ts">
    import {
        type NodeProps,
        type Node,
        Handle,
        Position,
    } from "@xyflow/svelte";
    import { path } from "@tauri-apps/api";
    import { open } from "@tauri-apps/plugin-dialog";

    import { derived, type Readable } from "svelte/store";

    import { session } from "~/lib/stores";
    import { SmartPortHandle } from "~/lib/handles";
    import { Button } from "~/lib/components";

    import Display from "../layout/Display.svelte";
    import { Power, Upload } from "svelte-feathers";
    import Session from "../session";

    type $$Props = NodeProps<Node<NodeData>>;

    type NodeData = {};

    export let data: NodeData;
    data; // so svelte isnt annoying about unused exports
    export let id: $$Props["id"];

    const programName: Readable<string> = derived(session, ($session, set) => {
        if ($session) {
            path.basename($session.binary || "", ".bin").then((name) =>
                set(name),
            );
        }

        return () => {
            set = () => {};
        };
    });

    async function handleButtonClick(event: MouseEvent) {
        if ($session?.running) {
            $session.stop();
            $session = null;
        } else {
            const file = await open({
                title: "Select program file",
                filters: [{ name: "", extensions: ["bin"] }],
            });

            if (file) {
                $session = new Session(file.path);
                $session.start();
            }
        }
    }
</script>

<div class="ports ports-top">
    {#each { length: 10 } as _, n}
        <div class="smart-port">
            <span class="smart-port-label">{n + 1}</span>
            <SmartPortHandle
                id={`${n + 1}`}
                parentNode={id}
                type="target"
                position={Position.Top}
            />
            <svg
                class="smart-port-indicator"
                width="37"
                height="35"
                viewBox="0 0 37 35"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    d="M4.99994 1L32 1C34.2091 1 36 2.79086 36 5L36 24H31V29H26V34H11V29H5.99999L6 24H0.999939V5C0.999939 2.79086 2.7908 1 4.99994 1Z"
                    stroke="var(--interactive-primary)"
                    fill="var(--background-primary)"
                />
            </svg>
        </div>
        {#if n == 4}
            <div class="ports-spacer"></div>
        {/if}
    {/each}
</div>
<div class="housing-inner">
    <Display programName={$programName} />
    <Button class="brain-button" on:click={handleButtonClick}>
        {#if $session?.running}
            <Power size="24" />
        {:else}
            <Upload size="24" />
        {/if}
    </Button>
</div>
<div class="housing-inner-focus-outline"></div>
<div class="ports ports-bottom">
    {#each { length: 10 } as _, n}
        <div class="smart-port">
            <span class="smart-port-label">{n + 11}</span>
            <SmartPortHandle
                id={`${n + 11}`}
                parentNode={id}
                type="target"
                position={Position.Bottom}
            />
            <svg
                width="37"
                height="35"
                viewBox="0 0 37 35"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    d="M32.0001 34H5.00002C2.79088 34 1.00002 32.2091 1.00002 30L1 11H6.00002V6H11V1H26V6H31L31 11H36.0001V30C36.0001 32.2091 34.2092 34 32.0001 34Z"
                    stroke="var(--interactive-primary)"
                    fill="var(--background-primary)"
                />
            </svg>
        </div>
        {#if n == 4}
            <div class="ports-spacer"></div>
        {/if}
    {/each}
</div>
<div class="port-21">
    <span class="smart-port-label">21</span>
    <SmartPortHandle
        id="21"
        parentNode={id}
        type="target"
        position={Position.Right}
    />
</div>

<Handle
    id="battery_port"
    class="battery-port"
    style="top: 66.66%;"
    type="target"
    isConnectable={false}
    position={Position.Right}
/>
<SmartPortHandle
    id="onboard_adi"
    parentNode={id}
    type="target"
    position={Position.Left}
/>

<style>
    :global(.svelte-flow__node-brain .display) {
        border-radius: 4px;
        border: 1px solid var(--interactive-primary);
    }

    :global(.svelte-flow__node.svelte-flow__node-brain) {
        width: 658px;
        height: 450px;
        border-radius: 60px;
        background-color: var(--background-secondary);
        border: 1px solid var(--interactive-primary);
    }

    :global(.svelte-flow__node-brain .battery-port) {
        height: 24px;
        transform: translateX(100%);
        border-radius: 0 4px 4px 0;
        width: 6px;
        border-left: none;
        border-color: inherit;
        transition: 150ms ease;
        z-index: -1;
        background-color: var(--background-primary);
    }

    :global(.svelte-flow__node-brain.selected .battery-port),
    :global(.svelte-flow__node-brain:focus-visible .battery-port) {
        box-shadow: 0 0 0 2px var(--accent-faded), 0 0 0 2px var(--background-secondary);
    }

    .housing-inner {
        background-color: var(--background-tertiary);
        border: 1px solid var(--interactive-primary);
        border-radius: 24px;
        width: 658px;
        height: 355px;
        position: absolute;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding-inline: 60px 18px;
        transition: 150ms ease;
    }

    .housing-inner :global(.button) {
        padding: 0;
        width: 64px;
        height: 64px;
        border-radius: 6px;
    }

    .smart-port {
        position: relative;
        display: flex;
        justify-content: center;
    }

    .smart-port :global(.smart-port-handle) {
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        transform: none;
        opacity: 0;
    }

    .smart-port svg {
        display: block;
    }

    .port-21 {
        right: 0;
        top: 100px;
        position: absolute;
        display: flex;
        align-items: center;
    }

    .port-21 .smart-port-label {
        right: 12px;
    }

    .smart-port-label {
        position: absolute;
        border: 1.5px solid currentColor;
        border-radius: 4px;
        width: 22px;
        height: 22px;
        text-align: center;
        line-height: 20px;
        font-size: 12px;
        color: var(--foreground-tertiary);
    }

    .ports-top .smart-port-label {
        bottom: -36px;
    }

    .ports-bottom .smart-port-label {
        top: -36px;
    }

    .ports {
        z-index: 1;
        position: absolute;
        left: 72px;
        width: 430px;
        display: flex;
        justify-content: space-evenly;
    }

    .ports-top {
        top: 4px;
    }

    .ports-bottom {
        bottom: 4px;
    }

    .ports-spacer {
        width: 20px;
    }

    .ports > :global(.svelte-flow__handle) {
        position: relative;
        transform: none;
        top: 0;
        left: 0;
    }
</style>
