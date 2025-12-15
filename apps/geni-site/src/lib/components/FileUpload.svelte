<script lang="ts">
	import { Dna, Shield, FileText } from '@lucide/svelte';

	let {
		onFileContent,
		onError
	}: {
		onFileContent: (content: string) => void;
		onError: (error: string) => void;
	} = $props();

	let isDragging = $state(false);
	let fileName = $state<string | null>(null);
	let fileInputRef = $state<HTMLInputElement | null>(null);

	function handleDragEnter(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		isDragging = true;
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		isDragging = false;
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		isDragging = false;

		const files = e.dataTransfer?.files;
		if (files && files.length > 0) {
			processFile(files[0]);
		}
	}

	function handleFileSelect(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			processFile(input.files[0]);
		}
	}

	function processFile(file: File) {
		const validExtensions = ['.txt', '.csv'];
		const hasValidExtension = validExtensions.some((ext) => file.name.toLowerCase().endsWith(ext));

		if (!hasValidExtension) {
			onError('Please upload a .txt or .csv file from 23andMe or Ancestry');
			return;
		}

		const maxSize = 50 * 1024 * 1024;
		if (file.size > maxSize) {
			onError('File is too large. Maximum size is 50MB.');
			return;
		}

		fileName = file.name;

		const reader = new FileReader();

		reader.onload = (event) => {
			const content = event.target?.result as string;
			if (content) {
				onFileContent(content);
			} else {
				onError('Failed to read file content');
			}
		};

		reader.onerror = () => {
			onError('Error reading file. Please try again.');
		};

		reader.readAsText(file);
	}

	function handleClick() {
		fileInputRef?.click();
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleClick();
		}
	}
</script>

<div class="upload-container">
	<div
		class="dropzone"
		class:dragging={isDragging}
		ondragenter={handleDragEnter}
		ondragleave={handleDragLeave}
		ondragover={handleDragOver}
		ondrop={handleDrop}
		onclick={handleClick}
		onkeydown={handleKeyDown}
		role="button"
		tabindex="0"
		aria-label="Upload your DNA raw data file"
	>
		<input
			bind:this={fileInputRef}
			type="file"
			accept=".txt,.csv"
			onchange={handleFileSelect}
			class="file-input"
			aria-hidden="true"
		/>

		<div class="dropzone-content">
			{#if fileName}
				<FileText size={48} strokeWidth={1.5} class="upload-icon selected" />
				<p class="selected-file">{fileName}</p>
				<p class="hint">Click or drop to change file</p>
			{:else}
				<Dna size={48} strokeWidth={1.5} class="dna-icon" />
				<p class="main-text">
					{isDragging ? 'Drop your file here' : 'Upload your raw DNA data'}
				</p>
				<p class="hint">Drag and drop or click to browse</p>
				<p class="file-type">Accepts .txt (23andMe) or .csv (Ancestry) files</p>
			{/if}
		</div>
	</div>

	<div class="privacy-notice">
		<Shield size={16} strokeWidth={2} />
		<span>Your DNA data stays on your device. We never upload or store your genetic file.</span>
	</div>
</div>

<style>
	.upload-container {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.dropzone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2.5rem 2rem;
		border: 2px dashed var(--color-border);
		border-radius: var(--radius-base);
		background-color: var(--color-secondary-bg);
		cursor: pointer;
		transition:
			border-color 0.2s,
			background-color 0.2s;
	}

	.dropzone:hover,
	.dropzone:focus {
		border-color: var(--color-accent-primary);
		outline: none;
	}

	.dropzone:focus-visible {
		box-shadow: 0 0 0 3px var(--color-accent-primary-grade);
	}

	.dropzone.dragging {
		border-color: var(--color-accent-primary);
		background-color: var(--color-accent-primary-grade);
	}

	.file-input {
		display: none;
	}

	.dropzone-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		text-align: center;
	}

	.dropzone-content :global(.upload-icon),
	.dropzone-content :global(.dna-icon) {
		color: var(--color-secondary-fg);
		transition: color 0.2s;
	}

	.dropzone:hover :global(.upload-icon),
	.dropzone:hover :global(.dna-icon),
	.dropzone.dragging :global(.upload-icon),
	.dropzone.dragging :global(.dna-icon) {
		color: var(--color-accent-primary);
	}

	.dropzone-content :global(.upload-icon.selected) {
		color: var(--color-accent-primary);
	}

	.main-text {
		font-size: 1.1rem;
		font-weight: 500;
		color: var(--color-primary-fg);
		margin: 0;
	}

	.selected-file {
		font-size: 1rem;
		font-weight: 500;
		color: var(--color-accent-primary);
		margin: 0;
		word-break: break-all;
	}

	.hint {
		font-size: 0.9rem;
		color: var(--color-secondary-fg);
		margin: 0;
	}

	.file-type {
		font-size: 0.8rem;
		color: var(--color-secondary-fg);
		margin: 0;
		opacity: 0.8;
	}

	.privacy-notice {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background-color: var(--color-secondary-bg);
		border-radius: calc(var(--radius-base) / 2);
		font-size: 0.85rem;
		color: var(--color-secondary-fg);
		line-height: 1.4;
	}

	.privacy-notice :global(svg) {
		flex-shrink: 0;
		margin-top: 0.1rem;
		color: var(--color-accent-primary);
	}
</style>
