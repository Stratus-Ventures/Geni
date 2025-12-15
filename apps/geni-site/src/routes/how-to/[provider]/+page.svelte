<script lang="ts">
	import { page } from '$app/stores';
	import { Logo } from '$lib/components';
	import { ArrowLeft, ExternalLink, Download, FileText, Upload } from '@lucide/svelte';

	const provider = $derived($page.params.provider);

	const guides = {
		'23andme': {
			name: '23andMe',
			steps: [
				{
					title: 'Log into 23andMe',
					description: 'Go to 23andme.com and sign in to your account.'
				},
				{
					title: 'Navigate to Settings',
					description:
						'Click on your name in the top right corner, then select "Settings" from the dropdown menu.'
				},
				{
					title: 'Find Raw Data Download',
					description:
						'Scroll down to the "23andMe Data" section and click "View" next to "Download Raw Data".'
				},
				{
					title: 'Request Your Data',
					description:
						'Click "Submit Request" and verify your identity. 23andMe will email you when your data is ready (usually within 30 minutes).'
				},
				{
					title: 'Download the File',
					description:
						'Once ready, return to the same page and click "Download" to get your raw data file (.txt format).'
				},
				{
					title: 'Upload to Geni',
					description:
						'Return to Geni and upload the downloaded .txt file. Your DNA never leaves your device.'
				}
			],
			link: 'https://you.23andme.com/settings/'
		},
		ancestry: {
			name: 'Ancestry',
			steps: [
				{
					title: 'Log into Ancestry',
					description: 'Go to ancestry.com and sign in to your account.'
				},
				{
					title: 'Go to DNA Settings',
					description:
						'Click on "DNA" in the top navigation, then select "Your DNA Results Summary". Click the "Settings" button.'
				},
				{
					title: 'Download Raw DNA Data',
					description: 'In the Actions section, click "Download Raw DNA Data".'
				},
				{
					title: 'Confirm Your Identity',
					description:
						'Enter your password and complete any verification steps. Click "Confirm" to start the download preparation.'
				},
				{
					title: 'Wait for Email',
					description:
						'Ancestry will email you when your download is ready (can take up to a few hours). Click the link in the email.'
				},
				{
					title: 'Upload to Geni',
					description:
						'Download the .zip file, extract the .txt file inside, and upload it to Geni.'
				}
			],
			link: 'https://www.ancestry.com/dna/'
		}
	};

	const guide = $derived(guides[provider as keyof typeof guides]);
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

		{#if guide}
			<div class="guide-header">
				<h1>Download Your {guide.name} Raw Data</h1>
				<p>Follow these steps to export your DNA file for analysis</p>
			</div>

			<div class="steps">
				{#each guide.steps as step, i}
					<div class="step">
						<div class="step-number">{i + 1}</div>
						<div class="step-content">
							<h3>{step.title}</h3>
							<p>{step.description}</p>
						</div>
					</div>
				{/each}
			</div>

			<a href={guide.link} target="_blank" rel="noopener noreferrer" class="external-link">
				<span>Go to {guide.name}</span>
				<ExternalLink size={16} strokeWidth={2} />
			</a>

			<div class="tips">
				<h2>Tips</h2>
				<ul>
					<li>
						<FileText size={16} strokeWidth={2} />
						<span>The raw data file is typically 10-20 MB</span>
					</li>
					<li>
						<Download size={16} strokeWidth={2} />
						<span>The download process can take 30 minutes to a few hours</span>
					</li>
					<li>
						<Upload size={16} strokeWidth={2} />
						<span>Your DNA file is processed entirely on your device</span>
					</li>
				</ul>
			</div>

			<footer class="footer">
				<a href="/">Ready to upload? Go back home</a>
			</footer>
		{:else}
			<div class="not-found">
				<h1>Provider Not Found</h1>
				<p>We currently support 23andMe and Ancestry.</p>
				<div class="provider-links">
					<a href="/how-to/23andme">23andMe Guide</a>
					<a href="/how-to/ancestry">Ancestry Guide</a>
				</div>
			</div>
		{/if}
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

	.guide-header {
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.guide-header h1 {
		color: var(--color-primary-fg);
		font-size: 1.5rem;
	}

	.guide-header p {
		color: var(--color-secondary-fg);
	}

	.steps {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.step {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: calc(var(--radius-base) / 2);
	}

	.step-number {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background-color: var(--color-accent-primary);
		color: white;
		font-weight: 600;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.step-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.step-content h3 {
		font-size: 0.95rem;
		font-weight: 600;
		color: var(--color-primary-fg);
	}

	.step-content p {
		font-size: 0.875rem;
		color: var(--color-secondary-fg);
		line-height: 1.5;
	}

	.external-link {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		background-color: var(--color-accent-primary);
		color: white;
		border-radius: calc(var(--radius-base) / 2);
		font-weight: 500;
		text-decoration: none;
		transition: background-color 0.2s;
	}

	.external-link:hover {
		background-color: var(--color-accent-secondary);
	}

	.tips {
		padding: 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: calc(var(--radius-base) / 2);
	}

	.tips h2 {
		font-size: 0.95rem;
		font-weight: 600;
		color: var(--color-primary-fg);
		margin-bottom: 0.75rem;
	}

	.tips ul {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.tips li {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		font-size: 0.875rem;
		color: var(--color-secondary-fg);
	}

	.tips li :global(svg) {
		flex-shrink: 0;
		color: var(--color-accent-primary);
		margin-top: 0.1rem;
	}

	.footer {
		text-align: center;
		padding-top: 1rem;
		border-top: 1px solid var(--color-border);
	}

	.footer a {
		color: var(--color-accent-primary);
		font-size: 0.9rem;
	}

	.not-found {
		text-align: center;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 2rem 0;
	}

	.not-found h1 {
		color: var(--color-primary-fg);
	}

	.not-found p {
		color: var(--color-secondary-fg);
	}

	.provider-links {
		display: flex;
		gap: 1rem;
		justify-content: center;
	}

	.provider-links a {
		padding: 0.5rem 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: calc(var(--radius-base) / 2);
		color: var(--color-accent-primary);
		text-decoration: none;
	}

	.provider-links a:hover {
		background-color: var(--color-border);
	}
</style>
