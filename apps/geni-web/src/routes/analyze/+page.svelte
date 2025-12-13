<script lang="ts">
	import { FileUpload, InsightCard } from '$lib/genetics/components';
	import { parse23andMe, analyzeAllTraits } from '$lib/genetics';
	import type { InsightResult, ParseResult, ParseDebugInfo } from '$lib/genetics';
	import { Loader, AlertCircle, ArrowLeft, Dna, Bug } from '@lucide/svelte';

	type PageState = 'upload' | 'parsing' | 'results' | 'error';

	let state: PageState = 'upload';
	let error: string | null = null;
	let insights: InsightResult[] = [];
	let parseInfo: { snpCount: number; format: string } | null = null;
	let debugInfo: ParseDebugInfo | null = null;
	let showDebug = false;

	function handleFileContent(content: string) {
		state = 'parsing';
		error = null;
		debugInfo = null;

		// Use setTimeout to allow UI to update before parsing
		setTimeout(() => {
			try {
				const result: ParseResult = parse23andMe(content);
				debugInfo = result.debug;

				if (result.snpCount === 0) {
					error = 'No genetic data found in the file. Please make sure you uploaded a valid 23andMe raw data file.';
					state = 'error';
					return;
				}

				parseInfo = {
					snpCount: result.snpCount,
					format: result.format
				};

				const analysisResults = analyzeAllTraits(result.data);

				if (analysisResults.length === 0) {
					error = 'Could not find the genetic markers needed for analysis. See debug info below.';
					state = 'error';
					showDebug = true;
					return;
				}

				insights = analysisResults;
				state = 'results';
			} catch (err) {
				console.error('Parsing error:', err);
				error = 'Failed to parse the file. Please make sure it is a valid 23andMe raw data file.';
				state = 'error';
			}
		}, 100);
	}

	function handleError(errorMessage: string) {
		error = errorMessage;
		state = 'error';
	}

	function reset() {
		state = 'upload';
		error = null;
		insights = [];
		parseInfo = null;
		debugInfo = null;
		showDebug = false;
	}

	function toggleDebug() {
		showDebug = !showDebug;
	}
</script>

<svelte:head>
	<title>Analyze Your DNA | Geni</title>
	<meta name="description" content="Upload your 23andMe raw data to discover personalized genetic insights. All analysis happens on your device - we never see your DNA." />
</svelte:head>

<main class="analyze-page">
	<header class="page-header">
		<div class="logo-section">
			<Dna size={32} strokeWidth={1.5} />
			<h1>Genetic Insights</h1>
		</div>
		<p class="subtitle">Discover what your DNA says about you</p>
	</header>

	{#if state === 'upload'}
		<section class="upload-section">
			<FileUpload {onFileContent} {onError} />
			<div class="how-it-works">
				<h3>How to get your raw data</h3>
				<ol>
					<li>Log in to your 23andMe account</li>
					<li>Go to Settings &gt; 23andMe Data</li>
					<li>Click "Download Raw Data"</li>
					<li>Upload the .txt file here</li>
				</ol>
			</div>
		</section>
	{:else if state === 'parsing'}
		<section class="loading-section">
			<Loader size={48} strokeWidth={1.5} class="spinner" />
			<p class="loading-text">Analyzing your DNA...</p>
			<p class="loading-hint">This happens entirely on your device</p>
		</section>
	{:else if state === 'error'}
		<section class="error-section">
			<div class="error-content">
				<AlertCircle size={48} strokeWidth={1.5} />
				<p class="error-message">{error}</p>

				{#if debugInfo}
					<button on:click={toggleDebug} class="debug-toggle">
						<Bug size={16} />
						{showDebug ? 'Hide' : 'Show'} Debug Info
					</button>

					{#if showDebug}
						<div class="debug-panel">
							<h4>Parsing Debug Info</h4>

							<div class="debug-item">
								<strong>Total lines:</strong> {debugInfo.totalLines.toLocaleString()}
							</div>

							<div class="debug-item">
								<strong>SNPs parsed:</strong> {debugInfo.totalLines - debugInfo.skippedLines}
							</div>

							<div class="debug-item">
								<strong>Lines skipped:</strong> {debugInfo.skippedLines.toLocaleString()}
							</div>

							<div class="debug-item">
								<strong>Target SNP (rs4988235):</strong>
								{#if debugInfo.targetSNPsStatus['rs4988235']?.found}
									<span class="found">Found! Genotype: {debugInfo.targetSNPsStatus['rs4988235'].genotype} (line {debugInfo.targetSNPsStatus['rs4988235'].line})</span>
								{:else}
									<span class="not-found">NOT FOUND in file</span>
								{/if}
							</div>

							<div class="debug-item">
								<strong>Sample rsIDs parsed:</strong>
								<code>{debugInfo.sampleSNPs.join(', ') || 'None'}</code>
							</div>

							<div class="debug-item">
								<strong>First data lines:</strong>
								<pre>{debugInfo.sampleLines.join('\n') || 'None'}</pre>
							</div>
						</div>
					{/if}
				{/if}

				<button on:click={reset} class="retry-button">
					<ArrowLeft size={18} />
					Try Again
				</button>
			</div>
		</section>
	{:else if state === 'results'}
		<section class="results-section">
			{#if parseInfo}
				<div class="parse-info">
					<p>Analyzed {parseInfo.snpCount.toLocaleString()} genetic markers</p>
				</div>
			{/if}

			<div class="insights-grid">
				{#each insights as insight (insight.traitId)}
					<InsightCard {insight} />
				{/each}
			</div>

			<button on:click={reset} class="analyze-another">
				<ArrowLeft size={18} />
				Analyze Another File
			</button>
		</section>
	{/if}
</main>

<style>
	.analyze-page {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-height: 100vh;
		padding: 2rem 1rem;
		gap: 2rem;
	}

	.page-header {
		text-align: center;
		max-width: 500px;
	}

	.logo-section {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
	}

	.logo-section :global(svg) {
		color: var(--color-accent-primary);
	}

	.page-header h1 {
		margin: 0;
		font-size: 2rem;
	}

	.subtitle {
		font-size: 1.1rem;
		color: var(--color-secondary-fg);
		margin: 0;
	}

	.upload-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2rem;
		width: 100%;
		max-width: 500px;
	}

	.how-it-works {
		width: 100%;
		padding: 1.25rem;
		background-color: var(--color-secondary-bg);
		border-radius: var(--radius-base);
	}

	.how-it-works h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 0.75rem 0;
		color: var(--color-primary-fg);
	}

	.how-it-works ol {
		margin: 0;
		padding-left: 1.25rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.how-it-works li {
		font-size: 0.9rem;
		color: var(--color-secondary-fg);
		line-height: 1.4;
	}

	.loading-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding: 3rem;
	}

	.loading-section :global(.spinner) {
		color: var(--color-accent-primary);
		animation: spin 1.5s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.loading-text {
		font-size: 1.2rem;
		font-weight: 500;
		color: var(--color-primary-fg);
		margin: 0;
	}

	.loading-hint {
		font-size: 0.9rem;
		color: var(--color-secondary-fg);
		margin: 0;
	}

	.error-section {
		display: flex;
		justify-content: center;
		width: 100%;
		max-width: 600px;
	}

	.error-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		padding: 2rem;
		background-color: var(--color-secondary-bg);
		border-radius: var(--radius-base);
		text-align: center;
		width: 100%;
	}

	.error-content :global(svg) {
		color: oklch(0.6 0.2 25);
	}

	.error-message {
		font-size: 1rem;
		color: var(--color-secondary-fg);
		margin: 0;
		line-height: 1.5;
	}

	.debug-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background-color: var(--color-primary-bg);
		color: var(--color-secondary-fg);
		border: 1px solid var(--color-border);
		border-radius: calc(var(--radius-base) / 2);
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.debug-toggle:hover {
		border-color: var(--color-accent-primary);
		color: var(--color-primary-fg);
	}

	.debug-panel {
		width: 100%;
		padding: 1rem;
		background-color: var(--color-primary-bg);
		border: 1px solid var(--color-border);
		border-radius: calc(var(--radius-base) / 2);
		text-align: left;
		font-size: 0.85rem;
	}

	.debug-panel h4 {
		margin: 0 0 1rem 0;
		font-size: 0.95rem;
		color: var(--color-primary-fg);
	}

	.debug-item {
		margin-bottom: 0.75rem;
		color: var(--color-secondary-fg);
	}

	.debug-item strong {
		color: var(--color-primary-fg);
	}

	.debug-item code {
		display: block;
		margin-top: 0.25rem;
		padding: 0.5rem;
		background-color: var(--color-secondary-bg);
		border-radius: 4px;
		font-family: monospace;
		font-size: 0.8rem;
		word-break: break-all;
	}

	.debug-item pre {
		margin: 0.25rem 0 0 0;
		padding: 0.5rem;
		background-color: var(--color-secondary-bg);
		border-radius: 4px;
		font-family: monospace;
		font-size: 0.75rem;
		overflow-x: auto;
		white-space: pre-wrap;
		word-break: break-all;
	}

	.found {
		color: oklch(0.6 0.15 145);
		font-weight: 500;
	}

	.not-found {
		color: oklch(0.6 0.2 25);
		font-weight: 500;
	}

	.retry-button,
	.analyze-another {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background-color: var(--color-primary-fg);
		color: var(--color-primary-bg);
		border: none;
		border-radius: calc(var(--radius-base) / 2);
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: opacity 0.2s;
	}

	.retry-button:hover,
	.analyze-another:hover {
		opacity: 0.9;
	}

	.results-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
		width: 100%;
		max-width: 700px;
	}

	.parse-info {
		padding: 0.5rem 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: 999px;
	}

	.parse-info p {
		font-size: 0.9rem;
		color: var(--color-secondary-fg);
		margin: 0;
	}

	.insights-grid {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 1.5rem;
		width: 100%;
		height: fit-content;
		max-width: 500px;
	}

	.analyze-another {
		margin-top: 1rem;
	}

	@media (min-width: 800px) {
		.page-header h1 {
			font-size: 2.5rem;
		}

		.subtitle {
			font-size: 1.2rem;
		}
	}
</style>
