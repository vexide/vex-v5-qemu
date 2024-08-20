<script lang="ts">
    /** The minimum value that can be set by the user. */
    export let min = 0;

    /** The maximum value that can be set by the user. */
    export let max = 100;

    /** The slider's value on the track. */
    export let value = min;

    /** The amount that the value should change per slider step. */
    export let step = 1;

    /** Determines if the slider can be interacted with. */
    export let disabled = false;

    /** An accessible description of the slider's purpose. */
    export let label: string;

    /** A set of numbers that will be visually displayed along the slider track to the user. */
    export let markers: number[] | undefined = undefined;

    /** Limits the available values to exact marker values. Replaces the `step` property if set. */
    export let snapToMarkers = false;

    $: sortedMarkers = [...(markers ?? [])].sort((a, b) => a - b);

    let trackElement: HTMLSpanElement | undefined;
    let thumbElement: HTMLSpanElement | undefined;

    let dragging = false;

    const clamp = (n: number, min: number, max: number) =>
        Math.min(Math.max(n, min), max);
    const findNearestMarker = (n: number) =>
        sortedMarkers.reduce((prev, curr) =>
            Math.abs(curr - n) < Math.abs(prev - n) ? curr : prev,
        );
    const valueToPercent = (value: number) =>
        Math.round(((value - min) / (max - min)) * 100);

    function valueFromPosition(x: number) {
        if (!trackElement) return 0;

        const { left, width } = trackElement.getBoundingClientRect();
        const percentage = (x - left) / width; // Raw percentage extrapolated from cursor position and slider offsets

        if (markers?.length && snapToMarkers) {
            return clamp(
                findNearestMarker(min + (max - min) * percentage),
                min,
                max,
            );
        } else {
            return clamp(
                Math.round((min + (max - min) * percentage) / step) * step,
                min,
                max,
            );
        }
    }

    function handlePointerMove(event: PointerEvent) {
        if (!dragging) return;
        value = valueFromPosition(event.clientX);
    }

    function handlePointerDown(event: PointerEvent) {
        if (
            (event.pointerType === "mouse" && event.button === 2) ||
            (event.pointerType !== "mouse" && !event.isPrimary)
        )
            return;

        thumbElement?.focus();
        event.preventDefault();
        value = valueFromPosition(event.clientX);
        dragging = true;
    }

    function handleKeyDown(event: KeyboardEvent) {
        const { key } = event;

        // Finds the index of the closest marker to the current (non-updated) value
        const nearestMarkerIndex =
            sortedMarkers.length > 0 && snapToMarkers
                ? sortedMarkers.indexOf(findNearestMarker(value))
                : -1;

        // I really could not care less about DRY at this point. If you think this is ugly
        // go bother TC39 for some pattern matching syntax.
        if (key == "ArrowLeft" || key == "ArrowDown" || key == "PageDown") {
            const previousMarker = sortedMarkers[nearestMarkerIndex - 1];
            const previousSnapValue = clamp(
                previousMarker != null && snapToMarkers
                    ? previousMarker
                    : value - step,
                snapToMarkers ? sortedMarkers?.[0] : min,
                snapToMarkers ? sortedMarkers?.[sortedMarkers.length - 1] : max,
            );

            event.preventDefault();
            value = previousSnapValue;
        } else if (key == "ArrowRight" || key == "ArrowUp" || key == "PageUp") {
            const nextMarker = sortedMarkers[nearestMarkerIndex + 1];
            const nextSnapValue = clamp(
                nextMarker != null && snapToMarkers ? nextMarker : value + step,
                snapToMarkers ? sortedMarkers?.[0] : min,
                snapToMarkers ? sortedMarkers?.[sortedMarkers.length - 1] : max,
            );

            event.preventDefault();
            value = nextSnapValue;
        } else if (key == "Home") {
            event.preventDefault();
            value = snapToMarkers ? sortedMarkers?.[0] : min;
        } else if (key == "End") {
            event.preventDefault();
            value = snapToMarkers
                ? sortedMarkers?.[sortedMarkers.length - 1]
                : max;
        }
    }
</script>

<svelte:window
    on:pointermove={handlePointerMove}
    on:pointerup={() => (dragging = false)}
/>

<div
    class="slider nodrag"
    class:disabled
    style:--value-percentage="{valueToPercent(value)}%"
    on:pointerdown={handlePointerDown}
>
    {#if markers}
        <div class="slider-markers">
            {#each markers as marker}
                <div class="slider-marker" style="left: {valueToPercent(marker)}%"></div>
            {/each}
        </div>
    {/if}
    <div class="slider-track" bind:this={trackElement}>
        <div class="slider-fill"></div>
    </div>
    <span
        class="slider-thumb"
        role="slider"
        aria-label={label}
        aria-valuemin={min}
        aria-valuemax={max}
        aria-valuenow={value}
        aria-disabled={disabled}
        aria-orientation="horizontal"
        tabindex={disabled ? -1 : 0}
        bind:this={thumbElement}
        on:keydown={handleKeyDown}
        {...$$restProps}
    ></span>
</div>

<style>
    .slider {
        position: relative;
        display: flex;
        align-items: center;
        height: 16px;
        cursor: ew-resize;
    }

    .slider-track {
        overflow: hidden;
        position: absolute;
        width: 100%;
        height: 4px;
        border-radius: 50px;
        background-color: var(--interactive-primary);
    }

    .slider-fill {
        position: absolute;
        height: 100%;
        width: var(--value-percentage);
        background-color: var(--accent-primary);
    }

    .slider-thumb {
        width: 16px;
        height: 16px;
        border-radius: 50px;
        background-color: var(--accent-primary);
        left: var(--value-percentage);
        transform: translateX(calc(var(--value-percentage) * -1));
        position: absolute;
    }

    .slider-thumb:focus {
        outline: none;
        box-shadow: 0 0 0 3px var(--accent-faded);
    }

    .slider-markers {
        position: absolute;
        width: 100%;
        height: 100%;
    }

    .slider-marker {
        position: absolute;
        background-color: var(--interactive-secondary);
        width: 2px;
        border-radius: 50px;
        height: 100%;
    }
</style>
