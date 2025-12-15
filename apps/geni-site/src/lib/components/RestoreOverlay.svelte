<script lang="ts">
	import { X, Mail, CheckCircle, AlertCircle, Loader2 } from '@lucide/svelte';

	type OverlayStatus = 'idle' | 'loading' | 'success' | 'error';

	let {
		isOpen = false,
		onClose,
		onSubmit
	}: {
		isOpen: boolean;
		onClose: () => void;
		onSubmit: (email: string) => Promise<boolean>;
	} = $props();

	let email = $state('');
	let status = $state<OverlayStatus>('idle');
	let errorMessage = $state('');

	async function handleSubmit(e: Event) {
		e.preventDefault();

		if (!email || !email.includes('@')) {
			status = 'error';
			errorMessage = 'Please enter a valid email address';
			return;
		}

		status = 'loading';
		errorMessage = '';

		try {
			const success = await onSubmit(email);
			if (success) {
				status = 'success';
			} else {
				status = 'error';
				errorMessage = 'No purchase found for this email';
			}
		} catch {
			status = 'error';
			errorMessage = 'Something went wrong. Please try again.';
		}
	}

	function handleClose() {
		email = '';
		status = 'idle';
		errorMessage = '';
		onClose();
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			handleClose();
		}
	}
</script>

{#if isOpen}
	<div
		class="overlay-backdrop"
		onclick={handleClose}
		onkeydown={handleKeyDown}
		role="button"
		tabindex="0"
		aria-label="Close overlay"
	>
		<div
			class="overlay-card"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="restore-title"
			tabindex="-1"
		>
			<button class="close-btn" onclick={handleClose} aria-label="Close">
				<X size={20} strokeWidth={2} />
			</button>

			{#if status === 'success'}
				<div class="success-content">
					<div class="icon-wrapper success">
						<CheckCircle size={32} strokeWidth={2} />
					</div>
					<h2 id="restore-title">Check Your Email</h2>
					<p>We sent a magic link to <strong>{email}</strong>. Click it to access your report.</p>
					<button class="primary-btn" onclick={handleClose}>Got it</button>
				</div>
			{:else}
				<div class="form-content">
					<div class="icon-wrapper">
						<Mail size={24} strokeWidth={2} />
					</div>
					<h2 id="restore-title">Restore Your Purchase</h2>
					<p>Enter the email you used during checkout and we'll send you a magic link.</p>

					<form onsubmit={handleSubmit}>
						<div class="input-wrapper">
							<input
								type="email"
								placeholder="your@email.com"
								bind:value={email}
								disabled={status === 'loading'}
								class:error={status === 'error'}
							/>
							{#if status === 'error' && errorMessage}
								<div class="error-message">
									<AlertCircle size={14} strokeWidth={2} />
									<span>{errorMessage}</span>
								</div>
							{/if}
						</div>

						<button type="submit" class="primary-btn" disabled={status === 'loading'}>
							{#if status === 'loading'}
								<Loader2 size={18} strokeWidth={2} class="spinner" />
								<span>Sending...</span>
							{:else}
								<span>Send Magic Link</span>
							{/if}
						</button>
					</form>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.overlay-backdrop {
		position: fixed;
		inset: 0;
		background-color: oklch(0 0 0 / 50%);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: flex-end;
		justify-content: center;
		z-index: 100;
		padding: 1rem;
	}

	@media (min-width: 640px) {
		.overlay-backdrop {
			align-items: center;
		}
	}

	.overlay-card {
		position: relative;
		width: 100%;
		max-width: 400px;
		background-color: var(--color-primary-bg);
		border-radius: var(--radius-base) var(--radius-base) 0 0;
		padding: 2rem;
		animation: slide-up 0.3s ease-out;
	}

	@media (min-width: 640px) {
		.overlay-card {
			border-radius: var(--radius-base);
			animation: fade-scale 0.2s ease-out;
		}
	}

	.close-btn {
		position: absolute;
		top: 1rem;
		right: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: none;
		border: none;
		color: var(--color-secondary-fg);
		cursor: pointer;
		border-radius: 8px;
		transition: background-color 0.2s;
	}

	.close-btn:hover {
		background-color: var(--color-secondary-bg);
	}

	.form-content,
	.success-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		text-align: center;
	}

	.icon-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		background-color: var(--color-secondary-bg);
		border-radius: 50%;
		color: var(--color-accent-primary);
	}

	.icon-wrapper.success {
		background-color: oklch(0.9 0.1 145);
		color: oklch(0.4 0.15 145);
	}

	h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--color-primary-fg);
		margin: 0;
	}

	p {
		font-size: 0.95rem;
		color: var(--color-secondary-fg);
		margin: 0;
		line-height: 1.5;
	}

	form {
		width: 100%;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 0.5rem;
	}

	.input-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	input {
		width: 100%;
		padding: 0.875rem 1rem;
		border: 1px solid var(--color-border);
		border-radius: calc(var(--radius-base) / 2);
		font-size: 1rem;
		background-color: var(--color-primary-bg);
		color: var(--color-primary-fg);
		transition: border-color 0.2s;
	}

	input:focus {
		outline: none;
		border-color: var(--color-accent-primary);
	}

	input.error {
		border-color: var(--color-high);
	}

	input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		color: var(--color-high);
	}

	.primary-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.875rem 1.5rem;
		background-color: var(--color-accent-primary);
		color: white;
		border: none;
		border-radius: calc(var(--radius-base) / 2);
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	.primary-btn:hover:not(:disabled) {
		background-color: var(--color-accent-secondary);
	}

	.primary-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.primary-btn :global(.spinner) {
		animation: spin 1s linear infinite;
	}

	@keyframes slide-up {
		from {
			transform: translateY(100%);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}

	@keyframes fade-scale {
		from {
			transform: scale(0.95);
			opacity: 0;
		}
		to {
			transform: scale(1);
			opacity: 1;
		}
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
