<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { attachConsole } from "@tauri-apps/plugin-log";
    import { open } from "@tauri-apps/plugin-dialog";

    import { SvelteFlowProvider } from "@xyflow/svelte";

    import Session from "~/lib/session";
    import { terminal, session, nodes, edges } from "~/lib/stores";
    import { Button, Dialog } from "~/lib/components";
    import { Toolbar, Uploader, Sidebar, Flow, Terminal } from "~/lib/layout";
    import {
        BrainNode,
        AdiNode,
        BatteryNode,
        GpsNode,
        DistanceNode,
        ValueNode,
    } from "~/lib/nodes";

    import { Pause, Play, RefreshCw, Settings, Power } from "svelte-feathers";

    import "@xyflow/svelte/dist/style.css";
    import "~/styles/flow.css";

    let settingsDialogOpen = false;

    let detachConsole: UnlistenFn | undefined;
    let unlistenUserSerial: UnlistenFn | undefined;

    const decoder = new TextDecoder("UTF-8");
    const nodeTypes = {
        brain: BrainNode,
        adi: AdiNode,
        battery: BatteryNode,
        gps: GpsNode,
        distance: DistanceNode,
        value: ValueNode,
    };

    onMount(async () => {
        detachConsole = await attachConsole();
        unlistenUserSerial = await listen<number[]>("user_serial", (event) => {
            $terminal?.write(decoder.decode(new Uint8Array(event.payload)));
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

    function handleWindowKeyDown({ key, ctrlKey, metaKey }: KeyboardEvent) {
        const ctrlOrMeta = ctrlKey || metaKey;

        if (ctrlOrMeta && key == "r") {
            $session?.reset();
        } else if (ctrlOrMeta && key == ",") {
            settingsDialogOpen = !settingsDialogOpen;
        }
    }
</script>

<svelte:window on:keydown={handleWindowKeyDown} />

<SvelteFlowProvider>
    <main class="split-view">
        <Sidebar />
        <div class="app-left">
            <Toolbar>
                <svelte:fragment slot="left">
                    <Button
                        small
                        title={($session?.paused ? "Unpause" : "Pause") +
                            " execution"}
                        disabled={!$session?.running}
                    >
                        <svelte:component
                            this={$session?.paused ? Play : Pause}
                            size="16"
                        />
                    </Button>
                    <Button
                        small
                        title="Reset program"
                        disabled={!$session?.running}
                        on:click={() => $session?.reset()}
                    >
                        <RefreshCw size="16" />
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
                        <Power size="16" />
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
                    <Settings size="16" />
                </Button>
            </Toolbar>
            <section class="display-view">
                {#if $session?.running}
                    <Flow {nodeTypes} {nodes} {edges} />
                {:else}
                    <Uploader on:upload={handleUpload} />
                {/if}
            </section>
            {#if $session?.running}
                <Terminal />
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
