<script>
    import { createEventDispatcher } from "svelte";

    import PackageIcon from "svelte-feathers/Package.svelte";
    import UploadIcon from "svelte-feathers/Upload.svelte";
    import HelpCircleIcon from "svelte-feathers/HelpCircle.svelte";

    import Button from "./Button.svelte";

    const dispatch = createEventDispatcher();
</script>

<div class="uploader">
    <PackageIcon size="64" />
    <h2>No Program Selected</h2>
    <p>Drag + drop or upload a program.bin file here.</p>
    <div class="uploader-actions">
        <Button variant="accent" on:click={() => dispatch("upload")}>
            <UploadIcon size="16" />
            Upload
        </Button>
        <Button>
            <HelpCircleIcon size="16" />
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
        gap: 8px
    }
</style>
