<script lang="ts">
    import { onDestroy, onMount, SvelteComponent } from "svelte";

    import { listen, TauriEvent, type UnlistenFn } from "@tauri-apps/api/event";
    import { trace, info, error, attachConsole } from "@tauri-apps/plugin-log";
    import { open } from "@tauri-apps/plugin-dialog";

    import { spawnQemu, killQemu } from "~/lib/invoke";
    import type { DragEnterPayload } from "./lib/payload";

    import Display from "~/lib/Display.svelte";
    import Button from "~/lib/Button.svelte";
    import ControlsHeader from "~/lib/ControlsHeader.svelte";
    import SerialMonitor from "~/lib/SerialMonitor.svelte";
    import Uploader from "~/lib/Uploader.svelte";
    import DevicesSidebar from "~/lib/DevicesSidebar.svelte";
    import Dialog from "~/lib/Dialog.svelte";

    import PauseIcon from "svelte-feathers/Pause.svelte";
    import PlayIcon from "svelte-feathers/Play.svelte";
    import RefreshCwIcon from "svelte-feathers/RefreshCw.svelte";
    import SettingsIcon from "svelte-feathers/Settings.svelte";
    import PowerIcon from "svelte-feathers/Power.svelte";

    import path from "path-browserify-esm";

    class Session {
        binary: string;
        paused: boolean = false;
        running: boolean = false;

        constructor(binary: string) {
            this.binary = binary;
        }

        async start() {
            if (!this.running) {
                this.running = true;
                this.paused = false;
                spawnQemu({
                    gdb: false,
                    qemu: "qemu-system-arm",
                    kernel: "../../kernel/target/armv7a-none-eabi/debug/kernel",
                    binary: this.binary,
                    qemu_args: [],
                });
            }
        }

        async stop() {
            if (this.running) {
                this.running = false;
                this.paused = false;
                killQemu();
            }
        }

        async reset() {
            if (this.running) {
                display?.clear();
                monitor?.clear();
                killQemu();
                spawnQemu({
                    gdb: false,
                    qemu: "qemu-system-arm",
                    // temporary
                    kernel: "../../kernel/target/armv7a-none-eabi/debug/kernel",
                    binary: this.binary,
                    qemu_args: [],
                });
            }
        }
    }

    let settingsDialogOpen = false;
    let deviceDialogOpen = false;

    let monitor: SvelteComponent | undefined;
    let display: SvelteComponent | undefined;
    let session: Session | undefined;

    let detachConsole: UnlistenFn | undefined;
    let unlistenUserSerial: UnlistenFn | undefined;
    let unlistenDragEnter: UnlistenFn | undefined;

    const decoder = new TextDecoder("UTF-8");

    onMount(async () => {
        detachConsole = await attachConsole();
        unlistenUserSerial = await listen<number[]>("user_serial", (event) => {
            monitor?.write(decoder.decode(new Uint8Array(event.payload)));
        });
        unlistenDragEnter = await listen<DragEnterPayload>(
            TauriEvent.DRAG_ENTER,
            async (event) => {
                session = new Session(event.payload.paths[0]);
                session.start();
            },
        );
    });

    onDestroy(() => {
        session?.stop();
        detachConsole?.();
        unlistenUserSerial?.();
        unlistenDragEnter?.();
    });

    async function handleUpload() {
        const file = await open({
            title: "Select program file",
            filters: [{ name: "", extensions: ["bin"] }],
        });

        if (file) {
            session = new Session(file.path);
            session.start();
        }
    }
</script>

<main class="split-view">
    <DevicesSidebar />
    <div class="app-left">
        <ControlsHeader>
            <svelte:fragment slot="left">
                <Button
                    small
                    title={(session?.paused ? "Unpause" : "Pause") +
                        " execution"}
                    disabled={!session?.running}
                >
                    <svelte:component
                        this={session?.paused ? PlayIcon : PauseIcon}
                        size="16"
                    />
                </Button>
                <Button
                    small
                    title="Reset program"
                    disabled={!session?.running}
                    on:click={() => session?.reset()}
                >
                    <RefreshCwIcon size="16" />
                </Button>
                <Button
                    small
                    title="Unload program"
                    disabled={!session?.running}
                    on:click={() => {
                        session?.stop();
                        session = undefined;
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
            {#if session?.running}
                <Display
                    programName={path.parse(session.binary).name}
                    bind:this={display}
                />
            {:else}
                <Uploader on:upload={handleUpload} />
            {/if}
        </section>
        {#if session?.running}
            <SerialMonitor bind:this={monitor} />
        {/if}
    </div>

    <Dialog bind:open={settingsDialogOpen}>
        <svelte:fragment slot="header">Settings</svelte:fragment>

        todo

        <svelte:fragment slot="footer">
            <Button variant="accent" on:click={() => settingsDialogOpen = false}>Done</Button>
        </svelte:fragment>
    </Dialog>
</main>

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
