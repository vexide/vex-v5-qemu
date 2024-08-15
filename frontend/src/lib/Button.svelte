<script lang="ts">
	export let variant: "default" | "accent" | "danger" = "default";
	export let href = "";
	export let disabled = false;
	export let small = false;
	export let ref: HTMLElement | null = null;

	let className = "";
	export { className as class };
</script>

<svelte:element
	this={href && !disabled ? "a" : "button"}
	bind:this={ref}
	role={href && !disabled ? "button" : undefined}
	href={href && !disabled ? href : undefined}
	class="button variant-{variant} {className}"
	class:small
	{disabled}
	on:click
	on:focus
	on:blur
	on:dragstart
	{...$$restProps}
>
	<slot />
</svelte:element>

<style>
	.button {
		display: flex;
		justify-content: center;
		align-items: center;
		vertical-align: middle;
		box-sizing: border-box;
		text-align: center;
		font-family: inherit;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		letter-spacing: 1px;
		padding-inline: 24px;
		padding-block: 12px;
		gap: 8px;
		border: none;
		outline: none;
		cursor: pointer;
		text-transform: uppercase;
		transition: 150ms ease;
	}

	.button:focus {
		box-shadow: 0 0 0 3px var(--accent-faded);
	}

	.button.variant-default {
		background-color: var(--interactive-primary);
		color: var(--foreground-secondary);
	}

	.button.variant-default:hover {
		background-color: var(--interactive-secondary);
	}

	.button.variant-danger,
	.button.variant-accent {
		font-weight: 700;
		color: var(--background-tertiary);
	}

	.button.variant-danger {
		background-color: var(--background-danger);
	}

	.button.variant-accent {
		background-color: var(--accent-primary);
	}

	.button.small {
		padding-inline: 16px;
		padding-block: 8px;
		line-height: 16px;
		gap: 4px;
	}

	.button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
