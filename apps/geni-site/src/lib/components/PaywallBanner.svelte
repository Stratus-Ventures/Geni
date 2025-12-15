<script lang="ts">
	import { Lock, Sparkles } from '@lucide/svelte';

	let {
		onUnlock,
		onRestore,
		remainingCount = 0
	}: {
		onUnlock: () => void;
		onRestore: () => void;
		remainingCount?: number;
	} = $props();
</script>

<div class="paywall-container">
	<div class="paywall-content">
		<div class="icon-wrapper">
			<Lock size={24} strokeWidth={2} />
		</div>

		<div class="text-content">
			<h3>Unlock Your Full Report</h3>
			<p>
				{remainingCount > 0
					? `Get ${remainingCount} more personalized insights about your diet, fitness, and wellness.`
					: 'Access all your personalized genetic insights.'}
			</p>
		</div>

		<button class="unlock-btn" onclick={onUnlock}>
			<Sparkles size={18} strokeWidth={2} />
			<span>Unlock Full Report</span>
		</button>

		<button class="restore-btn" onclick={onRestore}> Already purchased? Restore access </button>
	</div>

	<div class="blur-gradient"></div>
</div>

<style>
	.paywall-container {
		position: relative;
		width: 100%;
		margin-top: -2rem;
		padding-top: 4rem;
	}

	.blur-gradient {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 4rem;
		background: linear-gradient(to bottom, transparent 0%, var(--color-primary-bg) 100%);
		pointer-events: none;
	}

	.paywall-content {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding: 2rem;
		background-color: var(--color-secondary-bg);
		border-radius: var(--radius-base);
		text-align: center;
		z-index: 1;
	}

	.icon-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background-color: var(--color-accent-primary);
		border-radius: 50%;
		color: white;
	}

	.text-content {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.text-content h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--color-primary-fg);
		margin: 0;
	}

	.text-content p {
		font-size: 0.95rem;
		color: var(--color-secondary-fg);
		margin: 0;
		line-height: 1.5;
	}

	.unlock-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		max-width: 280px;
		padding: 1rem 1.5rem;
		background-color: var(--color-accent-primary);
		color: white;
		border: none;
		border-radius: calc(var(--radius-base) / 2);
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition:
			background-color 0.2s,
			transform 0.1s;
	}

	.unlock-btn:hover {
		background-color: var(--color-accent-secondary);
	}

	.unlock-btn:active {
		transform: scale(0.98);
	}

	.restore-btn {
		padding: 0.5rem 1rem;
		background: none;
		border: none;
		color: var(--color-secondary-fg);
		font-size: 0.9rem;
		cursor: pointer;
		text-decoration: underline;
		transition: color 0.2s;
	}

	.restore-btn:hover {
		color: var(--color-accent-primary);
	}
</style>
