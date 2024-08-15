<script lang="ts">
    import {
        getContext,
        onDestroy,
        onMount,
        setContext,
        SvelteComponent,
    } from "svelte";
    import { writable, type Writable } from "svelte/store";

    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { trace, info, error, attachConsole } from "@tauri-apps/plugin-log";
    import { open } from "@tauri-apps/plugin-dialog";

    import {
        SvelteFlowProvider,
        type Edge,
        type Node,
    } from "@xyflow/svelte";

    import Button from "~/lib/Button.svelte";
    import ControlsHeader from "~/lib/ControlsHeader.svelte";
    import SerialMonitor from "~/lib/SerialMonitor.svelte";
    import Uploader from "~/lib/Uploader.svelte";
    import DevicesSidebar from "~/lib/DevicesSidebar.svelte";
    import Dialog from "~/lib/Dialog.svelte";
    import Session, { session } from "~/lib/session";

    import PauseIcon from "svelte-feathers/Pause.svelte";
    import PlayIcon from "svelte-feathers/Play.svelte";
    import RefreshCwIcon from "svelte-feathers/RefreshCw.svelte";
    import SettingsIcon from "svelte-feathers/Settings.svelte";
    import PowerIcon from "svelte-feathers/Power.svelte";

    import "@xyflow/svelte/dist/style.css";
    import "~/styles/flow.css";
    import BrainNode from "./lib/nodes/BrainNode.svelte";
    import AdiNode from "./lib/nodes/AdiNode.svelte";
    import BatteryNode from "./lib/nodes/BatteryNode.svelte";
    import Flow from "./lib/Flow.svelte";

    let settingsDialogOpen = false;

    let monitor: SvelteComponent | undefined;

    let detachConsole: UnlistenFn | undefined;
    let unlistenUserSerial: UnlistenFn | undefined;

    const decoder = new TextDecoder("UTF-8");

    const dndType = writable<string | null>(null);
    const nodeTypes = {
        brain: BrainNode,
        adi: AdiNode,
        battery: BatteryNode,
    };
    const nodes = writable<Node[]>([
        {
            id: "brain",
            type: "brain",
            data: {},
            position: { x: 0, y: 0 },
        },
        {
            id: "battery",
            type: "battery",
            data: {
                capacity: 0,
                temperature: 0,
                current: 0,
                voltage: 0,
            },
            position: { x: 590, y: 225 },
        },
        {
            id: "onboard_adi",
            type: "adi",
            data: {
                onboard: true,
            },
            position: { x: -100, y: 0 },
        },
    ]);
    const edges = writable<Edge<any>[]>([
        {
            id: "battery_connection",
            type: "default",
            source: "battery",
            target: "brain",
            sourceHandle: "connector",
            targetHandle: "battery_port",
            deletable: false,
        },
        {
            id: "onboard_adi_connection",
            type: "default",
            source: "onboard_adi",
            target: "brain",
            sourceHandle: "connector",
            targetHandle: "onboard_adi_port",
            deletable: false,
        },
    ]);

    setContext("dnd", dndType);

    onMount(async () => {
        detachConsole = await attachConsole();
        unlistenUserSerial = await listen<number[]>("user_serial", (event) => {
            monitor?.write(decoder.decode(new Uint8Array(event.payload)));
        });
    });

    onDestroy(() => {
        $session?.stop();
        detachConsole?.();
        unlistenUserSerial?.();
    });

    async function handleUpload() {
        const file = await open({
            title: "Select program file",
            filters: [{ name: "", extensions: ["bin"] }],
        });

        if (file) {
            $session = new Session(file.path);
            $session.start();
        }
    }
</script>

<SvelteFlowProvider>
    <main class="split-view">
        <DevicesSidebar />
        <div class="app-left">
            <ControlsHeader>
                <svelte:fragment slot="left">
                    <Button
                        small
                        title={($session?.paused ? "Unpause" : "Pause") +
                            " execution"}
                        disabled={!$session?.running}
                    >
                        <svelte:component
                            this={$session?.paused ? PlayIcon : PauseIcon}
                            size="16"
                        />
                    </Button>
                    <Button
                        small
                        title="Reset program"
                        disabled={!$session?.running}
                        on:click={() => $session?.reset()}
                    >
                        <RefreshCwIcon size="16" />
                    </Button>
                    <Button
                        small
                        title="Unload program"
                        disabled={!$session?.running}
                        on:click={() => {
                            $session?.stop();
                            $session = null;
                        }}
                    >
                        <PowerIcon size="16" />
                    </Button>
                </svelte:fragment>
                <Button
                    small
                    slot="right"
                    title="Open settings"
                    on:click={() => {
                        settingsDialogOpen = true;
                    }}
                >
                    <SettingsIcon size="16" />
                </Button>
            </ControlsHeader>
            <section class="display-view">
                {#if $session?.running}
                    <Flow
                        {nodeTypes}
                        {nodes}
                        {edges}
                    />
                {:else}
                    <Uploader on:upload={handleUpload} />
                {/if}
            </section>
            {#if $session?.running}
                <SerialMonitor bind:this={monitor} />
            {/if}
        </div>

        <Dialog bind:open={settingsDialogOpen}>
            <svelte:fragment slot="header">Settings</svelte:fragment>

            todo

            <svelte:fragment slot="footer">
                <Button
                    variant="accent"
                    on:click={() => (settingsDialogOpen = false)}>Done</Button
                >
            </svelte:fragment>
        </Dialog>
    </main>
</SvelteFlowProvider>

<style>
    .split-view {
        display: flex;
        width: 100%;
        height: 100%;
    }

    .app-left {
        flex: 1 1 auto;
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow: auto;
        background: var(--background-primary);
    }

    .display-view {
        flex: 1 1 auto;
        overflow: auto;
        display: flex;
        align-items: safe center;
        justify-content: center;
        position: relative;
        background-image: linear-gradient(
                var(--background-secondary) 1px,
                transparent 1px
            ),
            linear-gradient(
                90deg,
                var(--background-secondary) 1px,
                transparent 1px
            );
        background-position: -1px -1px;
        background-size: 20px 20px;
    }
</style>
