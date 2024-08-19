<script>
    import { createEventDispatcher } from "svelte";

    import { Button } from "~/lib/components";
    import { HelpCircle, Package, Upload } from "svelte-feathers";

    const dispatch = createEventDispatcher();
</script>

<div class="uploader">
    <Package size="64" />
    <h2>No Program Selected</h2>
    <p>Drag + drop or upload a program.bin file here.</p>
    <div class="uploader-actions">
        <Button variant="accent" on:click={() => dispatch("upload")}>
            <Upload size="16" />
            Upload
        </Button>
        <Button>
            <HelpCircle size="16" />
            Help
        </Button>
    </div>
</div>

<style>
    .uploader {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }

    .uploader > :global(svg) {
        color: var(--accent-primary);
    }

    .uploader p {
        font-size: 16px;
        font-weight: 500;
        margin-block: 0 24px;
        color: var(--foreground-secondary);
    }

    .uploader h2 {
        margin-block: 24px 16px;
        font-size: 24px;
        color: var(--foreground-primary);
    }

    .uploader-actions {
        display: flex;
        gap: 8px;
    }
</style>
