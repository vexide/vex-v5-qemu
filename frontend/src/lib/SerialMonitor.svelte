<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";

    import TerminalIcon from "svelte-feathers/Terminal.svelte";
    import ExternalLinkIcon from "svelte-feathers/ExternalLink.svelte";

    import drag from "./drag";

    import "@xterm/xterm/css/xterm.css";
    import Button from "./Button.svelte";

    let terminal: Terminal | undefined;
    let fitAddon: FitAddon | undefined;

    let terminalContainer: HTMLDivElement | undefined;

    let height = "0px";
    let heightBeforeCollapse = "35vh";
    let holding = false;
    let dragging = false;
    let unreadMessages = 0;

    const XTERM_DEFAULT_LINE_HEIGHT = 20;
    const observer = new ResizeObserver(() => {
        fitAddon?.fit();
    });

    $: if (height != "0px") {
        unreadMessages = 0;
    }

    onMount(() => {
        fitAddon = new FitAddon();
        terminal = new Terminal({
            convertEol: true,
            theme: {
                background: "#141415",
            },
            fontFamily: "ui-monospace, Consolas, monospace",
        });

        terminal.loadAddon(fitAddon);
        terminal.open(terminalContainer!);
        observer.observe(terminalContainer!);
        fitAddon.fit();
    });

    onDestroy(() => {
        terminal?.dispose();
        observer.disconnect();
    });

    function roundStep(number: number, step: number, offset: number = 0) {
        return Math.ceil((number - offset) / step) * step + offset;
    }

    function handleDrag(event: PointerEvent) {
        if (!terminalContainer) return;

        const containerRect = terminalContainer.getBoundingClientRect();
        const newHeight =
            Math.max(
                roundStep(
                    containerRect.bottom -
                        event.clientY -
                        (event?.target as HTMLElement).offsetHeight / 2,
                    XTERM_DEFAULT_LINE_HEIGHT,
                ),
                0,
            ) + "px";

        if (height != newHeight) {
            height = newHeight;
        }
    }

    function toggleCollapse() {
        if (height == "0px") {
            height = heightBeforeCollapse;
        } else {
            heightBeforeCollapse = height;
            height = "0px";
        }
    }

    export function write(text: string) {
        terminal?.write(text);

        if (height == "0px") {
            unreadMessages += 1;
        }
    }

    export function clear() {
        terminal?.clear();
    }
</script>

<section class="serial-monitor">
    <button
        class="monitor-header"
        on:click={() => {
            if (!dragging) toggleCollapse();
        }}
        on:pointerdown={() => (holding = true)}
        on:pointerup={() => (holding = false)}
        on:pointermove={() => {
            if (holding) {
                dragging = true;
            } else {
                dragging = false;
            }
        }}
        use:drag={handleDrag}
    >
        <TerminalIcon size="16" />
        Serial Monitor
        {#if unreadMessages}
            <span class="unread-messages">
                {unreadMessages}
            </span>
        {/if}
    </button>
    <Button title="Detach terminal" class="detach-button" small>
        <ExternalLinkIcon size="16" />
    </Button>
    <div
        class="terminal-container"
        style:height
        bind:this={terminalContainer}
    ></div>
</section>

<style>
    .serial-monitor {
        position: relative;
    }

    .unread-messages {
        background: var(--accent-primary);
        color: var(--background-primary);
        font-size: 12px;
        padding: 1px 8px;
        border-radius: 50px;
        margin-left: 8px;
    }

    .terminal-container {
        max-height: calc(100vh - 48px - 36px);
        overflow: hidden;
        background: #141415;
    }

    .terminal-container :global(::-webkit-scrollbar) {
        background-color: #141415;
    }

    .terminal-container :global(::-webkit-scrollbar-thumb) {
        border-color: #141415;
    }

    .terminal-container :global(.terminal) {
        padding: 8px;
    }

    .monitor-header {
        display: flex;
        align-items: center;
        width: 100%;
        height: 36px;
        padding-inline: 16px;
        border-radius: 8px 8px 0 0;
        background: var(--background-tertiary);
        border: none;
        border-top: 1px solid var(--interactive-primary);
        transition: 150ms ease background, 150ms ease box-shadow;
        font-size: 14px;
        font-weight: 600;
        outline: none;
    }

    .monitor-header:focus-visible {
        box-shadow: inset 0 0 0 3px var(--accent-faded);
    }

    .monitor-header:hover {
        background: var(--interactive-primary);
        border-color: var(--interactive-secondary);
    }

    .monitor-header:active {
        background: var(--interactive-secondary);
        border-color: var(--interactive-tertiary);
    }

    .serial-monitor :global(.detach-button) {
        position: absolute;
        right: 6px;
        top: 4.5px;
        padding: 6px 12px;
    }

    .monitor-header:hover + :global(.detach-button) {
        background-color: var(--interactive-secondary);
    }

    .monitor-header > :global(svg) {
        margin-right: 8px;
        color: var(--foreground-secondary);
    }
</style>
