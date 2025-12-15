<script lang="ts">
	import type { InsightResult, RiskLevel } from '$lib/domain';

	let {
		insight,
		locked = false
	}: {
		insight: InsightResult;
		locked?: boolean;
	} = $props();

	const riskColors: Record<RiskLevel, string> = {
		low: 'var(--color-low)',
		optimal: 'var(--color-optimal)',
		high: 'var(--color-high)'
	};

	const riskLabels: Record<RiskLevel, string> = {
		low: 'Needs Attention',
		optimal: 'Optimal',
		high: 'High Risk'
	};

	const categoryLabels: Record<string, string> = {
		diet: 'Diet',
		fitness: 'Fitness',
		wellness: 'Wellness',
		supplements: 'Supplements'
	};
</script>

<article class="insight-card" class:locked>
	{#if locked}
		<div class="locked-overlay">
			<p>Unlock to view</p>
		</div>
	{/if}

	<header class="card-header">
		<div class="title-row">
			<span class="category-badge">{categoryLabels[insight.category]}</span>
			<h3 class="card-title">{insight.title}</h3>
		</div>
		<span class="risk-badge" style="background-color: {riskColors[insight.riskLevel]};">
			{riskLabels[insight.riskLevel]}
		</span>
	</header>

	<div class="genotype-display">
		<span class="genotype-label">Your genotype:</span>
		<span class="genotype-value">{insight.genotype}</span>
	</div>

	<section class="card-section">
		<h4>What this means</h4>
		<p>{insight.summary}</p>
	</section>

	<section class="card-section">
		<h4>About this result</h4>
		<p class="confidence-note">{insight.confidenceNote}</p>
	</section>

	<section class="card-section actions-section">
		<h4>What to do next</h4>
		<ul class="actions-list">
			{#each insight.whatToDoNext as action, index (index)}
				<li>{action}</li>
			{/each}
		</ul>
	</section>
</article>

<style>
	.insight-card {
		position: relative;
		background-color: var(--color-primary-bg);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-base);
		padding: 1.5rem;
		width: 100%;
		box-shadow: 0 4px 12px var(--color-shadow);
		overflow: hidden;
	}

	.insight-card.locked {
		filter: blur(4px);
		pointer-events: none;
		user-select: none;
	}

	.locked-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		z-index: 10;
	}

	.locked-overlay p {
		background-color: var(--color-primary-bg);
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-weight: 500;
		color: var(--color-primary-fg);
	}

	.card-header {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1.25rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--color-border);
	}

	.title-row {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.category-badge {
		display: inline-block;
		width: fit-content;
		padding: 0.25rem 0.5rem;
		background-color: var(--color-secondary-bg);
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--color-secondary-fg);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.card-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--color-primary-fg);
		margin: 0;
	}

	.risk-badge {
		display: inline-block;
		width: fit-content;
		padding: 0.35rem 0.75rem;
		border-radius: 999px;
		font-size: 0.8rem;
		font-weight: 500;
		color: white;
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

	.card-section h4 {
		font-size: 0.9rem;
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

	@media (min-width: 500px) {
		.card-header {
			flex-direction: row;
			justify-content: space-between;
			align-items: flex-start;
		}
	}
</style>
