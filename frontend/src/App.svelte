<script lang="ts">
    import { onDestroy, onMount, SvelteComponent } from "svelte";

    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { resolveResource } from "@tauri-apps/api/path";
    import { trace, info, error, attachConsole } from "@tauri-apps/plugin-log";

    import { spawnQemu, killQemu } from "~/lib/invoke";

    import PauseIcon from "svelte-feathers/Pause.svelte";
    import PlayIcon from "svelte-feathers/Play.svelte";
    import RefreshCwIcon from "svelte-feathers/RefreshCw.svelte";
    import SettingsIcon from "svelte-feathers/Settings.svelte";
    import PowerIcon from "svelte-feathers/Power.svelte";

    import Screen from "~/lib/Screen.svelte";
    import Button from "~/lib/Button.svelte";
    import ControlsHeader from "~/lib/ControlsHeader.svelte";
    import SerialMonitor from "~/lib/SerialMonitor.svelte";
    import Uploader from "~/lib/Uploader.svelte";
    import DevicesSidebar from "~/lib/DevicesSidebar.svelte";

    let running = false;
    let paused = false;
    let detachConsole: UnlistenFn | undefined;
    let unlistenUserSerial: UnlistenFn | undefined;
    let monitor: SvelteComponent | undefined;

    const decoder = new TextDecoder("UTF-8");

    onMount(async () => {
        detachConsole = await attachConsole();
        unlistenUserSerial = await listen<number[]>("user_serial", (event) => {
            console.log("Mounted");
            monitor?.write(decoder.decode(new Uint8Array(event.payload)));
        });
    });

    onDestroy(() => {
        detachConsole?.();
        unlistenUserSerial?.();
    });

    async function start() {
        if (!running) {
            running = true;
            paused = false;
            spawnQemu({
                gdb: false,
                qemu: "qemu-system-arm",
                kernel: await resolveResource("resources/kernel.elf"),
                binary: await resolveResource("resources/pros.bin"),
                qemu_args: [],
            });
        }
    }

    function stop() {
        if (running) {
            running = false;
            paused = false;
            killQemu();
        }
    }

    async function reset() {
        if (running) {
            monitor?.clear();
            killQemu();
            spawnQemu({
                gdb: false,
                qemu: "qemu-system-arm",
                kernel: await resolveResource("resources/kernel.elf"),
                binary: await resolveResource("resources/pros.bin"),
                qemu_args: [],
            });
        }
    }

    async function togglePlayPause() {
        paused = !paused;
        // TODO
    }

    function addDevice() {}
    function handleUpload() {
        start();
    }
</script>

<main class="split-view">
    <DevicesSidebar on:add={addDevice} />
    <div class="app-left">
        <ControlsHeader>
            <svelte:fragment slot="left">
                <Button
                    small
                    title={(paused ? "Unpause" : "Pause") + " execution"}
                    disabled={!running}
                    on:click={togglePlayPause}
                >
                    <svelte:component
                        this={paused ? PlayIcon : PauseIcon}
                        size="16"
                    />
                </Button>
                <Button
                    small
                    title="Reset program"
                    disabled={!running}
                    on:click={reset}
                >
                    <RefreshCwIcon size="16" />
                </Button>
                <Button
                    small
                    title="Unload program"
                    disabled={!running}
                    on:click={stop}
                >
                    <PowerIcon size="16" />
                </Button>
            </svelte:fragment>
            <Button small slot="right" title="Open settings">
                <SettingsIcon size="16" />
            </Button>
        </ControlsHeader>
        <section class="screen-view">
            {#if running}
                <Screen />
            {:else}
                <Uploader on:upload={handleUpload} />
            {/if}
        </section>
        {#if running}
            <SerialMonitor bind:this={monitor} />
        {/if}
    </div>
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

    .screen-view {
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
