<script>
    import { Check } from "svelte-feathers";

</script>
<div class="checkbox-wrapper">
    <input class="checkbox" type="checkbox" {...$$restProps} />
    <Check size="12" class="checkbox-symbol" />
</div>

<style>
    .checkbox-wrapper {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .checkbox {
        appearance: none;
        width: 16px;
        height: 16px;
        border: 1px solid var(--interactive-primary);
        border-radius: 4px;
        background: transparent;
    }

    .checkbox:checked {
        border-color: var(--accent-primary);
        background-color: var(--accent-primary);
    }

    .checkbox:focus {
        outline: none;
        box-shadow: 0 0 0 3px var(--accent-faded);
    }

    .checkbox-wrapper :global(.checkbox-symbol) {
        width: 12px;
        height: 12px;
        position: absolute;
        display: none;
        color: var(--background-primary);
    }

    .checkbox:checked + :global(.checkbox-symbol) {
        display: block;
    }
</style>
