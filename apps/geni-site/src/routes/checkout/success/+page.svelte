<script lang="ts">
	import { LoadingSpinner } from '$lib/components';
</script>

<main>
	<section>
		<div class="loading-state">
			<LoadingSpinner message="Activating your report..." />
		</div>
	</section>
</main>

<style>
	section {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		padding: 2rem;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}
</style>
