<script>
    export let flipped = false;
</script>

<div class="switch-wrapper nodrag">
    <input
        bind:checked={flipped}
        class="checkbox"
        type="checkbox"
        {...$$restProps}
    />
    <div class="slider" class:flipped></div>
</div>

<style>
    .switch-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .checkbox {
        appearance: none;
        width: 32px;
        height: 16px;
        /* Outline instead of border for slider positioning */
        outline: 1px solid var(--interactive-primary);
        border-radius: 16px;
        background: transparent;

        transition: background-color 0.2s;
    }

    .checkbox:checked {
        border-color: var(--accent-primary);
        background-color: var(--accent-primary);
    }

    .checkbox:focus {
        outline: none;
        box-shadow: 0 0 0 3px var(--accent-faded);
    }

    .slider {
        position: absolute;
        background-color: var(--accent-primary);
        width: 12px;
        height: 12px;
        left: 4px;
        border-radius: 50%;

        transition: all 0.2s;
    }
    .slider.flipped {
        background-color: var(--background-secondary);
        transform: translate(16px, 0);
    }
</style>
