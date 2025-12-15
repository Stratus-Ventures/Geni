<script lang="ts">
	import { Logo, InsightCard } from '$lib/components';
	import { ArrowLeft, Sparkles } from '@lucide/svelte';
	import type { InsightResult, TraitCategory } from '$lib/domain';

	const categoryOrder: TraitCategory[] = ['diet', 'fitness', 'wellness', 'supplements'];

	const sampleInsights: InsightResult[] = [
		{
			traitId: 'lactose-tolerance',
			title: 'Lactose Tolerance',
			category: 'diet',
			genotype: 'CT',
			riskLevel: 'optimal',
			summary: 'You likely maintain lactose tolerance into adulthood',
			confidence: 'high',
			confidenceNote: 'This variant is well-studied with strong research support.',
			whatToDoNext: [
				'Enjoy dairy products normally',
				'Consider lactase supplements if you experience occasional discomfort'
			]
		},
		{
			traitId: 'caffeine-metabolism',
			title: 'Caffeine Metabolism',
			category: 'diet',
			genotype: 'AC',
			riskLevel: 'low',
			summary: 'You metabolize caffeine at a moderate rate',
			confidence: 'high',
			confidenceNote: 'CYP1A2 variants are well-characterized for caffeine metabolism.',
			whatToDoNext: ['Limit coffee to 2-3 cups daily', 'Avoid caffeine after 2pm for better sleep']
		},
		{
			traitId: 'muscle-fiber-type',
			title: 'Muscle Fiber Type',
			category: 'fitness',
			genotype: 'CT',
			riskLevel: 'optimal',
			summary: 'You have a balanced mix of muscle fiber types',
			confidence: 'high',
			confidenceNote: 'ACTN3 is one of the most studied athletic performance genes.',
			whatToDoNext: [
				'Include both strength training and cardio',
				'You may excel at sports requiring both power and endurance'
			]
		},
		{
			traitId: 'endurance-potential',
			title: 'Endurance Potential',
			category: 'fitness',
			genotype: 'AG',
			riskLevel: 'optimal',
			summary: 'You have above-average endurance capacity',
			confidence: 'moderate',
			confidenceNote: 'This association has moderate research support.',
			whatToDoNext: [
				'Consider activities like running, cycling, or swimming',
				'You may respond well to high-volume training'
			]
		},
		{
			traitId: 'vitamin-d-levels',
			title: 'Vitamin D Levels',
			category: 'wellness',
			genotype: 'GT',
			riskLevel: 'low',
			summary: 'You may have lower vitamin D levels',
			confidence: 'high',
			confidenceNote: 'GC gene variants are well-established for vitamin D binding.',
			whatToDoNext: [
				'Consider vitamin D supplementation (1000-2000 IU daily)',
				'Get regular sun exposure',
				'Get levels tested annually'
			]
		},
		{
			traitId: 'sleep-depth',
			title: 'Sleep Depth',
			category: 'wellness',
			genotype: 'TC',
			riskLevel: 'optimal',
			summary: 'You likely have normal sleep patterns',
			confidence: 'moderate',
			confidenceNote: 'CLOCK gene variants have moderate associations with sleep traits.',
			whatToDoNext: [
				'Maintain consistent sleep and wake times',
				'Your body clock should respond well to regular routines'
			]
		},
		{
			traitId: 'folate-metabolism',
			title: 'Folate Metabolism',
			category: 'supplements',
			genotype: 'CT',
			riskLevel: 'low',
			summary: 'You have reduced folate processing ability',
			confidence: 'high',
			confidenceNote: 'MTHFR C677T is one of the most studied nutrigenetic variants.',
			whatToDoNext: [
				'Consider methylfolate supplements instead of regular folic acid',
				'Eat folate-rich foods like leafy greens and legumes'
			]
		},
		{
			traitId: 'omega3-conversion',
			title: 'Omega-3 Conversion',
			category: 'supplements',
			genotype: 'TT',
			riskLevel: 'low',
			summary: 'You have limited omega-3 conversion ability',
			confidence: 'high',
			confidenceNote: 'FADS1 variants are well-characterized for fatty acid metabolism.',
			whatToDoNext: [
				'Prioritize direct sources of EPA/DHA like fatty fish',
				'Consider fish oil supplements rather than relying on flax or chia seeds'
			]
		}
	];

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

	const groupedInsights = groupByCategory(sampleInsights);
</script>

<main>
	<section>
		<header class="header">
			<a href="/" class="back-link">
				<ArrowLeft size={20} strokeWidth={2} />
				<span>Back</span>
			</a>
			<Logo />
		</header>

		<div class="example-banner">
			<Sparkles size={20} strokeWidth={2} />
			<span>This is an example report showing what you'll get</span>
		</div>

		<div class="report-header">
			<h1>Example DNA Report</h1>
			<p class="subtitle">
				See what a full Geni report looks like with {sampleInsights.length} personalized insights
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

		<div class="cta-section">
			<h2>Ready to see your own insights?</h2>
			<p>Upload your DNA file to get personalized recommendations based on your genetics.</p>
			<a href="/" class="cta-button">Upload Your DNA File</a>
		</div>

		<footer class="footer">
			<small>
				Example data shown for demonstration. Your actual results will be based on your unique
				genetic profile.
			</small>
		</footer>
	</section>
</main>

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
	}

	.back-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--color-secondary-fg);
		font-size: 0.9rem;
		text-decoration: none;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: var(--color-accent-primary);
	}

	.example-banner {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background-color: oklch(0.95 0.05 85);
		border: 1px solid oklch(0.85 0.1 85);
		border-radius: calc(var(--radius-base) / 2);
		color: oklch(0.45 0.12 85);
		font-size: 0.875rem;
		font-weight: 500;
	}

	.example-banner :global(svg) {
		color: oklch(0.55 0.15 85);
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

	.cta-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding: 2rem 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: var(--radius-base);
		text-align: center;
	}

	.cta-section h2 {
		font-size: 1.25rem;
		color: var(--color-primary-fg);
	}

	.cta-section p {
		color: var(--color-secondary-fg);
		font-size: 0.95rem;
	}

	.cta-button {
		display: inline-flex;
		align-items: center;
		padding: 0.875rem 1.5rem;
		background-color: var(--color-accent-primary);
		color: white;
		border-radius: calc(var(--radius-base) / 2);
		font-weight: 500;
		text-decoration: none;
		transition: background-color 0.2s;
	}

	.cta-button:hover {
		background-color: var(--color-accent-secondary);
	}

	.footer {
		text-align: center;
		padding-top: 1rem;
		border-top: 1px solid var(--color-border);
	}

	.footer small {
		color: var(--color-secondary-fg);
		line-height: 1.6;
	}

	@media (prefers-color-scheme: dark) {
		.example-banner {
			background-color: oklch(0.25 0.04 85);
			border-color: oklch(0.35 0.08 85);
			color: oklch(0.75 0.1 85);
		}

		.example-banner :global(svg) {
			color: oklch(0.7 0.12 85);
		}
	}
</style>
