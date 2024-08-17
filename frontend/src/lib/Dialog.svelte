<script lang="ts">
    export let open = false;
    export let dialogRef: HTMLDialogElement | null = null;

    $: if (open) {
        dialogRef?.showModal();
    } else {
        dialogRef?.close();
    }
</script>

<dialog class="dialog" bind:this={dialogRef} {...$$restProps}>
    {#if $$slots.header}
        <header class="dialog-header">
            <strong>
                <slot name="header" />
            </strong>
        </header>
    {/if}
    <div class="dialog-body">
        <slot />
    </div>
    <footer class="dialog-footer">
        <slot name="footer" />
    </footer>
</dialog>

<style>
    .dialog {
        font-family: var(--font-family-text);
        display: flex;
        flex-direction: column;
        box-sizing: border-box;
        inset-inline-start: 50%;
        inset-block-start: 50%;
        inline-size: 388px;
        max-inline-size: calc(100% - 24px);
        min-block-size: 178px;
        margin: 0;
        padding: 16px;
        font-size: 14px;
        font-weight: 500;
        line-height: 1.6;
        border-radius: 6px;
        border: 1px solid var(--background-tertiary);
        z-index: 9999;
        box-shadow:
            inset 0 1px 0 var(--background-tertiary),
            0px 2.8px 2.8px rgba(0, 0, 0, 0.056),
            0px 6.7px 6.7px rgba(0, 0, 0, 0.081),
            0px 12.5px 12.5px rgba(0, 0, 0, 0.1),
            0px 22.3px 22.3px rgba(0, 0, 0, 0.119),
            0px 41.8px 41.8px rgba(0, 0, 0, 0.144),
            0px 100px 100px rgba(0, 0, 0, 0.2);
        background-color: var(--background-secondary);
        color: var(--foreground-secondary);
        opacity: 0;
        visibility: hidden;
        will-change: transform;
        transform: translate(-50%, -50%) scale(0.95) perspective(64px)
            rotateX(4deg);
        transition: 400ms cubic-bezier(0.175, 0.885, 0.32, 1.275);
    }

    .dialog[open] {
        display: flex;
        opacity: 1;
        visibility: visible;
        transform: translate(-50%, -50%) translateZ(0);
    }

    .dialog::backdrop {
        background-color: var(--background-primary);
        opacity: 0;
        backdrop-filter: blur(8px);
        -webkit-backdrop-filter: blur(8px);
        transition: 250ms ease;
    }

    .dialog[open]::backdrop {
        opacity: 0.8;
    }

    .dialog-header {
        margin-block-end: 12px;
        font-size: 16px;
        font-weight: 500;
        color: var(--foreground-primary);
    }

    .dialog-footer {
        display: grid;
        grid-gap: 8px;
        grid-auto-flow: column;
        grid-auto-columns: 1fr 1fr;
    }

    .dialog-body {
        flex: 1 1 auto;
    }

    .dialog-header,
    .dialog-footer {
        flex: 0 0 auto;
    }
</style>
