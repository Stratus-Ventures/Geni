<script lang="ts">
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { Logo, InsightCard, PaywallBanner, RestoreOverlay } from '$lib/components';
	import type { InsightResult, TraitCategory } from '$lib/domain';

	let { data } = $props();

	let insights = $state<InsightResult[]>([]);
	let showRestoreOverlay = $state(false);
	let isLoaded = $state(false);

	const FREE_LIMIT = 5;

	const categoryOrder: TraitCategory[] = ['diet', 'fitness', 'wellness', 'supplements'];

	$effect(() => {
		if (!browser) return;

		if (data.savedInsights && data.savedInsights.length > 0) {
			insights = data.savedInsights;
			isLoaded = true;
			return;
		}

		const stored = sessionStorage.getItem('geni_pending_report');
		if (!stored) {
			goto('/');
			return;
		}

		try {
			const parsed = JSON.parse(stored);
			insights = parsed.insights || [];
			isLoaded = true;
		} catch {
			goto('/');
		}
	});

	function groupByCategory(items: InsightResult[]): Map<TraitCategory, InsightResult[]> {
		const groups = new Map<TraitCategory, InsightResult[]>();
		for (const item of items) {
			const existing = groups.get(item.category) || [];
			existing.push(item);
			groups.set(item.category, existing);
		}
		return groups;
	}

	function getCategoryLabel(category: TraitCategory): string {
		const labels: Record<TraitCategory, string> = {
			diet: 'Diet & Nutrition',
			fitness: 'Fitness & Performance',
			wellness: 'Wellness & Sleep',
			supplements: 'Supplements'
		};
		return labels[category];
	}

	async function handleRestoreSubmit(email: string): Promise<boolean> {
		const response = await fetch('/auth/restore', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ email })
		});
		return response.ok;
	}

	const freeInsights = $derived(insights.slice(0, FREE_LIMIT));
	const lockedCount = $derived(Math.max(0, insights.length - FREE_LIMIT));
	const groupedInsights = $derived(groupByCategory(insights));
</script>

<main>
	<section>
		<header class="header">
			<a href="/" class="logo-link">
				<Logo />
			</a>
			{#if !data.isPaid}
				<button class="restore-nav-btn" onclick={() => (showRestoreOverlay = true)}>
					Restore Purchase
				</button>
			{/if}
		</header>

		{#if !isLoaded}
			<div class="loading-state">
				<p>Loading your report...</p>
			</div>
		{:else if data.isPaid}
			<div class="report-header">
				<h1>Your DNA Report</h1>
				<p class="subtitle">
					{insights.length} personalized insights based on your genetic profile
				</p>
			</div>

			<div class="categories">
				{#each categoryOrder as category}
					{@const categoryInsights = groupedInsights.get(category)}
					{#if categoryInsights && categoryInsights.length > 0}
						<div class="category-section">
							<h2 class="category-title">{getCategoryLabel(category)}</h2>
							<div class="insights-grid">
								{#each categoryInsights as insight}
									<InsightCard {insight} />
								{/each}
							</div>
						</div>
					{/if}
				{/each}
			</div>

			<footer class="report-footer">
				<small>
					Report generated from your DNA file. These insights are for educational purposes only.
				</small>
			</footer>
		{:else}
			<div class="report-header">
				<h1>Your DNA Insights Preview</h1>
				<p class="subtitle">
					Showing {FREE_LIMIT} of {insights.length} available insights
				</p>
			</div>

			<div class="insights-grid">
				{#each freeInsights as insight}
					<InsightCard {insight} />
				{/each}
			</div>

			<PaywallBanner
				remainingCount={lockedCount}
				onUnlock={() => (window.location.href = '/checkout')}
				onRestore={() => (showRestoreOverlay = true)}
			/>

			<div class="locked-preview">
				{#each insights.slice(FREE_LIMIT, FREE_LIMIT + 3) as insight}
					<InsightCard {insight} locked />
				{/each}
			</div>

			<footer class="disclaimer">
				<small>
					Your DNA data stays on your device. We only store your encrypted report after purchase.
				</small>
			</footer>
		{/if}
	</section>
</main>

<RestoreOverlay
	isOpen={showRestoreOverlay}
	onClose={() => (showRestoreOverlay = false)}
	onSubmit={handleRestoreSubmit}
/>

<style>
	section {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		padding-top: 1rem;
		padding-bottom: 2rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
	}

	.logo-link {
		display: flex;
	}

	.restore-nav-btn {
		padding: 0.5rem 1rem;
		background: none;
		border: 1px solid var(--color-border);
		border-radius: calc(var(--radius-base) / 2);
		color: var(--color-secondary-fg);
		font-size: 0.875rem;
		cursor: pointer;
		transition:
			border-color 0.2s,
			color 0.2s;
	}

	.restore-nav-btn:hover {
		border-color: var(--color-accent-primary);
		color: var(--color-accent-primary);
	}

	.loading-state {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 200px;
		color: var(--color-secondary-fg);
	}

	.report-header {
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.report-header h1 {
		color: var(--color-primary-fg);
	}

	.subtitle {
		color: var(--color-secondary-fg);
		font-size: 1rem;
	}

	.categories {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.category-section {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.category-title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-accent-primary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.insights-grid {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.locked-preview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		filter: blur(4px);
		opacity: 0.6;
		pointer-events: none;
	}

	.report-footer,
	.disclaimer {
		text-align: center;
		padding-top: 1rem;
		border-top: 1px solid var(--color-border);
	}

	.report-footer small,
	.disclaimer small {
		color: var(--color-secondary-fg);
		line-height: 1.6;
	}
</style>
