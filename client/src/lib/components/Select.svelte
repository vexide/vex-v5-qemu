<script lang="ts">
    export let selected: string;
</script>

<select class="nodrag select" bind:value={selected}>
    <slot />
</select>

<style>

</style>
