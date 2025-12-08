<script lang="ts">
	import LogoIcon from './LogoIcon.svelte';
</script>

<div class="logo-frame">
	<LogoIcon />
	<h2 class="logo-text">Geni</h2>
</div>

<style>
	.logo-frame {
		display: flex;
		flex-direction: row;
		justify-content: start;
		align-items: center;
		width: fit-content;
		height: fit-content;
		gap: 0.5rem;
	}

	.logo-text {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--color-primary-fg);
	}

	@media (min-width: 800px) {
		.logo-text {
			font-size: 2rem;
		}
	}
</style>
