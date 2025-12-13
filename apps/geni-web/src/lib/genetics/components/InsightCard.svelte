<script lang="ts">
	import type { InsightResult } from '../types';

	export let insight: InsightResult;

	const confidenceColors: Record<string, { bg: string; text: string }> = {
		high: { bg: 'var(--confidence-high-bg)', text: 'var(--confidence-high-text)' },
		moderate: { bg: 'var(--confidence-moderate-bg)', text: 'var(--confidence-moderate-text)' },
		low: { bg: 'var(--confidence-low-bg)', text: 'var(--confidence-low-text)' }
	};

	const confidenceLabels: Record<string, string> = {
		high: 'High Confidence',
		moderate: 'Moderate Confidence',
		low: 'Low Confidence'
	};
</script>

<article class="insight-card">
	<header class="card-header">
		<h2 class="card-title">{insight.title}</h2>
		<span
			class="confidence-badge"
			style="background-color: {confidenceColors[insight.confidence].bg}; color: {confidenceColors[insight.confidence].text};"
		>
			{confidenceLabels[insight.confidence]}
		</span>
	</header>

	<div class="genotype-display">
		<span class="genotype-label">Your genotype:</span>
		<span class="genotype-value">{insight.genotype}</span>
	</div>

	<section class="card-section">
		<h3>What this means</h3>
		<p>{insight.summary}</p>
	</section>

	<section class="card-section confidence-section">
		<h3>About this result</h3>
		<p>{insight.confidenceNote}</p>
	</section>

	<section class="card-section actions-section">
		<h3>What to do next</h3>
		<ul class="actions-list">
			{#each insight.whatToDoNext as action}
				<li>{action}</li>
			{/each}
		</ul>
	</section>
</article>

<style>
	/* CSS Variables for confidence colors */
	:global(:root) {
		--confidence-high-bg: oklch(0.9 0.1 145);
		--confidence-high-text: oklch(0.3 0.1 145);
		--confidence-moderate-bg: oklch(0.9 0.1 85);
		--confidence-moderate-text: oklch(0.35 0.1 85);
		--confidence-low-bg: oklch(0.9 0.1 25);
		--confidence-low-text: oklch(0.35 0.1 25);
	}

	@media (prefers-color-scheme: dark) {
		:global(:root) {
			--confidence-high-bg: oklch(0.3 0.1 145);
			--confidence-high-text: oklch(0.85 0.1 145);
			--confidence-moderate-bg: oklch(0.3 0.1 85);
			--confidence-moderate-text: oklch(0.85 0.1 85);
			--confidence-low-bg: oklch(0.3 0.1 25);
			--confidence-low-text: oklch(0.85 0.1 25);
		}
	}

	.insight-card {
		background-color: var(--color-primary-bg);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-base);
		padding: 1.5rem;
		max-width: 600px;
		width: 100%;
		box-shadow: 0 4px 12px var(--color-shadow);
	}

	.card-header {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1.25rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--color-border);
	}

	@media (min-width: 500px) {
		.card-header {
			flex-direction: row;
			justify-content: space-between;
			align-items: center;
		}
	}

	.card-title {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--color-primary-fg);
		margin: 0;
	}

	.confidence-badge {
		display: inline-block;
		padding: 0.35rem 0.75rem;
		border-radius: 999px;
		font-size: 0.8rem;
		font-weight: 500;
		white-space: nowrap;
	}

	.genotype-display {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 1.25rem;
		padding: 0.75rem 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: calc(var(--radius-base) / 2);
	}

	.genotype-label {
		font-size: 0.9rem;
		color: var(--color-secondary-fg);
	}

	.genotype-value {
		font-size: 1.1rem;
		font-weight: 600;
		font-family: monospace;
		color: var(--color-accent-primary);
		letter-spacing: 0.1em;
	}

	.card-section {
		margin-bottom: 1.25rem;
	}

	.card-section:last-child {
		margin-bottom: 0;
	}

	.card-section h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-primary-fg);
		margin-bottom: 0.5rem;
	}

	.card-section p {
		font-size: 0.95rem;
		line-height: 1.6;
		color: var(--color-secondary-fg);
	}

	.actions-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.actions-list li {
		position: relative;
		padding-left: 1.25rem;
		font-size: 0.95rem;
		line-height: 1.5;
		color: var(--color-secondary-fg);
	}

	.actions-list li::before {
		content: '';
		position: absolute;
		left: 0;
		top: 0.5rem;
		width: 6px;
		height: 6px;
		background-color: var(--color-accent-primary);
		border-radius: 50%;
	}
</style>
