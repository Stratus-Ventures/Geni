<script lang="ts">
	let { message = 'Analyzing your DNA...' }: { message?: string } = $props();
</script>

<div class="loading-container">
	<div class="dna-helix">
		<div class="strand strand-1">
			<div class="dot"></div>
			<div class="dot"></div>
			<div class="dot"></div>
			<div class="dot"></div>
		</div>
		<div class="strand strand-2">
			<div class="dot"></div>
			<div class="dot"></div>
			<div class="dot"></div>
			<div class="dot"></div>
		</div>
	</div>
	<p class="loading-message">{message}</p>
</div>

<style>
	.loading-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
		padding: 2rem;
	}

	.dna-helix {
		position: relative;
		width: 60px;
		height: 80px;
	}

	.strand {
		position: absolute;
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
	}

	.strand-1 {
		animation: rotate-strand-1 2s ease-in-out infinite;
	}

	.strand-2 {
		animation: rotate-strand-2 2s ease-in-out infinite;
	}

	.dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background-color: var(--color-accent-primary);
	}

	.strand-1 .dot:nth-child(1) {
		animation: pulse 2s ease-in-out infinite 0s;
	}
	.strand-1 .dot:nth-child(2) {
		animation: pulse 2s ease-in-out infinite 0.2s;
	}
	.strand-1 .dot:nth-child(3) {
		animation: pulse 2s ease-in-out infinite 0.4s;
	}
	.strand-1 .dot:nth-child(4) {
		animation: pulse 2s ease-in-out infinite 0.6s;
	}

	.strand-2 .dot:nth-child(1) {
		animation: pulse 2s ease-in-out infinite 0.5s;
	}
	.strand-2 .dot:nth-child(2) {
		animation: pulse 2s ease-in-out infinite 0.7s;
	}
	.strand-2 .dot:nth-child(3) {
		animation: pulse 2s ease-in-out infinite 0.9s;
	}
	.strand-2 .dot:nth-child(4) {
		animation: pulse 2s ease-in-out infinite 1.1s;
	}

	.loading-message {
		font-size: 1rem;
		color: var(--color-secondary-fg);
		text-align: center;
	}

	@keyframes rotate-strand-1 {
		0%,
		100% {
			transform: rotateY(0deg) translateX(-15px);
		}
		50% {
			transform: rotateY(180deg) translateX(15px);
		}
	}

	@keyframes rotate-strand-2 {
		0%,
		100% {
			transform: rotateY(180deg) translateX(15px);
		}
		50% {
			transform: rotateY(0deg) translateX(-15px);
		}
	}

	@keyframes pulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 1;
		}
		50% {
			transform: scale(0.7);
			opacity: 0.6;
		}
	}
</style>
