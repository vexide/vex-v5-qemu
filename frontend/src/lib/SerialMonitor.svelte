<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import { Terminal } from "@xterm/xterm";
	import { FitAddon } from '@xterm/addon-fit';

    import TerminalIcon from "svelte-feathers/Terminal.svelte";
    import ExternalLinkIcon from "svelte-feathers/ExternalLink.svelte";

    import '@xterm/xterm/css/xterm.css';
    import Button from "./Button.svelte";

    let terminal: Terminal | undefined;
    let fitAddon: FitAddon | undefined;

    const observer = new ResizeObserver(() => {
        fitAddon?.fit();
    });

    let terminalContainer: HTMLDivElement | undefined;

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

	export function write(text: string) {
		terminal?.write(text);
        console.log("Text");
	}

	export function clear() {
		terminal?.clear();
	}
</script>

<svelte:options accessors={true} />

<section class="serial-monitor">
    <header>
        <TerminalIcon size="16" />
        <strong>
            Serial Monitor
        </strong>
        <Button title="Detach terminal" class="detach-button" small>
            <ExternalLinkIcon size="16" />
        </Button>
    </header>
    <div class="terminal-container" bind:this={terminalContainer}></div>
</section>

<style>
    .terminal-container {
        height: 35vh;
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

    .serial-monitor header {
        display: flex;
        align-items: center;
        width: 100%;
        height: 36px;
        padding-inline: 16px 6px;
        border-radius: 8px 8px 0 0;
        background: var(--background-tertiary);
        border-top: 1px solid var(--interactive-primary);
    }

    .serial-monitor :global(.detach-button) {
        padding: 6px 12px;
    }

    .serial-monitor header strong {
        flex: 1 1 auto;
        font-size: 14px;
        font-weight: 600;
    }

    .serial-monitor header > :global(svg) {
        margin-right: 8px;
        color: var(--foreground-secondary);
    }
</style>
