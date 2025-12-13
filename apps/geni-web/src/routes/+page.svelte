<script lang="ts">
	import { Logo } from '$lib/ui'
    import { FileUpload, InsightCard } from '$lib/genetics/components';
    import { parse23andMe, analyzeAllTraits } from '$lib/genetics';
    import type { InsightResult, ParseResult } from '$lib/genetics';
    import { goto } from '$app/navigation';
    import { Loader, CircleAlert, ArrowLeft, ShoppingCart } from '@lucide/svelte';

    type PageState = 'upload' | 'parsing' | 'results' | 'error';

    let state: PageState = 'upload';
    let error: string | null = null;
    let insights: InsightResult[] = [];
    let parseInfo: { snpCount: number; format: string } | null = null;

    function handleFileContent(content: string) {
        state = 'parsing';
        error = null;

        // Use setTimeout to allow UI to update before parsing
        setTimeout(() => {
            try {
                const result: ParseResult = parse23andMe(content);

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
                    error = 'Could not find the genetic markers needed for analysis. The file may be from an older test version.';
                    state = 'error';
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
    }

    async function requestCheckout() {
        await goto('/checkout/test');
    }
</script>

<svelte:head>
	<title>Analyze Your DNA | Geni</title>
	<meta name="description" content="Upload your 23andMe raw data to discover personalized genetic insights. All analysis happens on your device - we never see your DNA." />
</svelte:head>

<main class="analyze-page">
	
	<header class="page-header">
		<Logo />
		
		<div class="heading">
			<h1>Turn your 23andMe data into actionable health insights.</h1>
			<p class="subtitle">Discover what your DNA says about you</p>
			<button class="checkout-button" onclick={requestCheckout}>
				<ShoppingCart size={18} />
				Get Premium
			</button>
		</div>
	</header>
	
	{#if state === 'upload'}
		<section class="upload-section">
			<FileUpload onFileContent={handleFileContent} onError={handleError} />
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
				<CircleAlert size={48} strokeWidth={1.5} />
				<p class="error-message">{error}</p>
				<button onclick={reset} class="retry-button">
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
			
			<button onclick={reset} class="analyze-another">
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
		background: var(--gradient-main), var(--color-primary-bg);
    }

    .page-header {
		display: flex;
		flex-direction: column;
		justify-content: start;
		align-items: start;
        text-align: start;
        max-width: 500px;
        width: 100%;
		gap: 4rem;
    }
	
	.heading {
		display: flex;
		flex-direction: column;
		justify-content: start;
		align-items: start;
		text-align: start;
		width: 100%;
		height: fit-content;
		gap: 1rem;
	}

    .checkout-button {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.85rem 1rem;
        border-radius: var(--radius-base);
        background: var(--color-primary-fg);
        color: var(--color-primary-bg);
        border: none;
        cursor: pointer;
        font-weight: 600;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
        transition: transform 150ms ease, box-shadow 150ms ease, opacity 150ms ease;
    }

    .checkout-button:hover {
        transform: translateY(-1px);
        box-shadow: 0 12px 28px rgba(0, 0, 0, 0.12);
    }

    .checkout-button:active {
        transform: translateY(0);
        opacity: 0.9;
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
        max-width: 500px;
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
        gap: 1.5rem;
        width: 100%;
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
