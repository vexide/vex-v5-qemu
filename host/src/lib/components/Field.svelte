<script lang="ts">
    export let layout: "horizontal" | "vertical" = "horizontal";
    export let label: string;
</script>

<label class="field {layout}">
    <span class="field-label">{label}</span>
    <slot />
</label>

<style>
    .field {
        display: flex;
        justify-content: space-between;
        gap: 16px;
        margin-bottom: 8px;
    }

    .field:last-child {
        margin-bottom: 0;
    }

    .field.horizontal {
        align-items: center;
        flex-direction: row;
    }

    .field.vertical {
        justify-content: center;
        flex-direction: column;
    }

    .field-label {
        color: var(--foreground-primary);
        font-size: 14px;
        font-weight: 600;
    }

</style>
