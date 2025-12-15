<script lang="ts">
	import { goto } from '$app/navigation';
	import { Logo, FileUpload, LoadingSpinner, RestoreOverlay } from '$lib/components';
	import { parseGeneticFile, analyzeAllTraits, serializeGenotypes, getAllRsids } from '$lib/domain';
	import { AlertCircle, FileQuestion, Shield, Lock, Sparkles } from '@lucide/svelte';

	let isLoading = $state(false);
	let error = $state<string | null>(null);
	let showRestoreOverlay = $state(false);

	function handleFileContent(content: string) {
		isLoading = true;
		error = null;

		setTimeout(() => {
			try {
				const result = parseGeneticFile(content);

				if (result.snpCount < 100) {
					error = 'This file appears to be invalid or empty. Please upload a valid DNA file.';
					isLoading = false;
					return;
				}

				const insights = analyzeAllTraits(result.data);

				if (insights.length === 0) {
					error =
						'Could not find any recognized genetic markers. Please ensure this is a valid DNA file.';
					isLoading = false;
					return;
				}

				const genotypes = serializeGenotypes(result.data, getAllRsids());

				sessionStorage.setItem(
					'geni_pending_report',
					JSON.stringify({
						genotypes,
						insights,
						snpCount: result.snpCount,
						format: result.format
					})
				);

				goto('/report');
			} catch (e) {
				console.error('Parse error:', e);
				error =
					'Failed to parse the DNA file. Please ensure it is a valid 23andMe or Ancestry file.';
				isLoading = false;
			}
		}, 1500);
	}

	function handleError(message: string) {
		error = message;
	}

	async function handleRestoreSubmit(email: string): Promise<boolean> {
		const response = await fetch('/auth/restore', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ email })
		});
		return response.ok;
	}
</script>

<main>
	<section>
		<header class="header">
			<Logo />
			<button class="restore-nav-btn" onclick={() => (showRestoreOverlay = true)}>
				Restore Purchase
			</button>
		</header>

		{#if isLoading}
			<div class="loading-state">
				<LoadingSpinner message="Analyzing your DNA..." />
			</div>
		{:else}
			<div class="hero">
				<h1>Unlock Your DNA Insights</h1>
				<p class="hero-subtitle">
					Transform your raw DNA data into personalized health insights. No account needed.
				</p>
			</div>

			{#if error}
				<div class="error-banner" role="alert">
					<AlertCircle size={20} strokeWidth={2} />
					<span>{error}</span>
				</div>
			{/if}

			<FileUpload onFileContent={handleFileContent} onError={handleError} />

			<div class="features-grid">
				<div class="feature-item">
					<Shield size={20} strokeWidth={2} />
					<div>
						<h3>Privacy First</h3>
						<p>Your DNA stays on your device. We never store genetic files.</p>
					</div>
				</div>
				<div class="feature-item">
					<Sparkles size={20} strokeWidth={2} />
					<div>
						<h3>16+ Traits</h3>
						<p>Diet, fitness, wellness, and supplement insights.</p>
					</div>
				</div>
				<div class="feature-item">
					<Lock size={20} strokeWidth={2} />
					<div>
						<h3>AES-256 Encrypted</h3>
						<p>Your saved reports are protected with bank-level encryption.</p>
					</div>
				</div>
			</div>

			<div class="help-link">
				<FileQuestion size={16} strokeWidth={2} />
				<a href="/how-to/23andme">How to get your raw DNA file</a>
			</div>

			<footer class="disclaimer">
				<small>
					This service is for educational purposes only and does not provide medical advice. Genetic
					insights should not replace professional healthcare consultation. By uploading your file,
					you agree to our <a href="/privacy">Privacy Policy</a>.
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
		gap: 2rem;
		padding-top: 1rem;
		padding-bottom: 2rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
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
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 300px;
	}

	.hero {
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.hero h1 {
		color: var(--color-primary-fg);
	}

	.hero-subtitle {
		font-size: 1.1rem;
		color: var(--color-secondary-fg);
		line-height: 1.5;
	}

	.error-banner {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		background-color: oklch(0.95 0.05 25);
		border: 1px solid var(--color-high);
		border-radius: calc(var(--radius-base) / 2);
		color: var(--color-high);
	}

	.error-banner :global(svg) {
		flex-shrink: 0;
		margin-top: 0.1rem;
	}

	.features-grid {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.feature-item {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: calc(var(--radius-base) / 2);
	}

	.feature-item :global(svg) {
		flex-shrink: 0;
		color: var(--color-accent-primary);
	}

	.feature-item h3 {
		font-size: 0.95rem;
		font-weight: 600;
		color: var(--color-primary-fg);
		margin-bottom: 0.25rem;
	}

	.feature-item p {
		font-size: 0.85rem;
		line-height: 1.4;
	}

	.help-link {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		color: var(--color-secondary-fg);
	}

	.help-link :global(svg) {
		color: var(--color-accent-primary);
	}

	.help-link a {
		font-size: 0.9rem;
	}

	.disclaimer {
		text-align: center;
		padding-top: 1rem;
		border-top: 1px solid var(--color-border);
	}

	.disclaimer small {
		color: var(--color-secondary-fg);
		line-height: 1.6;
	}

	.disclaimer a {
		color: var(--color-accent-primary);
	}

	@media (prefers-color-scheme: dark) {
		.error-banner {
			background-color: oklch(0.2 0.05 25);
		}
	}
</style>
