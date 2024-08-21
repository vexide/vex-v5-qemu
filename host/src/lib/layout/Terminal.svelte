<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";
    import { WebglAddon } from "@xterm/addon-webgl";

    import TerminalIcon from "svelte-feathers/Terminal.svelte";

    import { drag } from "~/lib/actions";
    import { terminal } from "~/lib/stores";

    import "@xterm/xterm/css/xterm.css";

    let fitAddon: FitAddon | undefined;

    let terminalContainer: HTMLDivElement | undefined;
    let monitorElement: HTMLElement | undefined;

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
        $terminal = new Terminal({
            convertEol: true,
            theme: {
                background: "#141415",
            },
            fontFamily: "ui-monospace, Consolas, monospace",
        });

        $terminal.loadAddon(fitAddon);
        $terminal.loadAddon(new WebglAddon());
        $terminal.open(terminalContainer!);

        $terminal?.onWriteParsed(() => {
            if (height == "0px") {
                unreadMessages++;
            }
        });

        observer.observe(terminalContainer!);
        fitAddon.fit();
    });

    onDestroy(() => {
        $terminal?.dispose();
        observer.disconnect();
    });

    function roundStep(number: number, step: number, offset: number = 0) {
        return Math.ceil((number - offset) / step) * step + offset;
    }

    function handleDrag(event: PointerEvent) {
        if (!monitorElement) return;

        const monitorRect = monitorElement.getBoundingClientRect();
        const newHeight =
            Math.max(
                roundStep(
                    monitorRect.bottom -
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
</script>

<section class="serial-monitor" bind:this={monitorElement}>
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
        Terminal
        {#if unreadMessages}
            <span class="unread-messages">
                {unreadMessages}
            </span>
        {/if}
    </button>
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

    :global(.terminal-container[style="height: 0px;"]) {
        display: none;
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
        transition:
            150ms ease background,
            150ms ease box-shadow;
        font-family: inherit;
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

    .monitor-header > :global(svg) {
        margin-right: 8px;
        color: var(--foreground-secondary);
    }
</style>
