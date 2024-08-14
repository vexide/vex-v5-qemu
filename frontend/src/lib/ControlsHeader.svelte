<header class="controls-header">
    <div class="controls-left">
        <slot name="left" />
    </div>
    <div class="controls-right">
        <slot name="right" />
    </div>
</header>

<style>
    .controls-header {
        display: flex;
        align-items: center;
        position: relative;
        flex: 0 0 auto;
        height: 48px;
        padding-inline: 10px;
        box-shadow: 0 0 8px var(--background-primary);
        background: var(--background-primary);
        border-bottom: 1px solid var(--interactive-primary);
    }

    .controls-left,
    .controls-right {
        display: flex;
        gap: 8px;
    }

    .controls-left {
        flex: 1 1 auto;
    }

    .controls-right {
        flex: 0 0 auto;
    }
</style>
