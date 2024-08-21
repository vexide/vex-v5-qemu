<script lang="ts">
    export let layout: "horizontal" | "vertical" = "horizontal";
    export let label: string;

    let field: HTMLLabelElement;
</script>

<label class="field {layout}" bind:this={field}>
    <slot name="handle" class="handle" />
    <span class="field-label">{label}</span>
    <slot />
</label>

<style>
    .field {
        display: flex;
        position: relative;
        justify-content: space-between;
        width: calc(100% + 16px);
        gap: 16px;
        padding-inline-end: 16px;
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
