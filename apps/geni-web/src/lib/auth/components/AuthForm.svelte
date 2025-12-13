<script lang="ts">
	import { Logo } from '$lib/ui';
	import { Mail, Phone } from '@lucide/svelte';
	import { enhance } from '$app/forms';
	import type { SubmitFunction } from '@sveltejs/kit';

	// TYPES & VARIABLES
	let authMethod: 'email' | 'phone' = 'email';
	let identifier = '';
	let loading = false;
	let error: string = '';
	let successMessage: string = '';

	// HANDLERS
	function toggleAuthMethod() {
		authMethod = authMethod === 'email' ? 'phone' : 'email';
		identifier = '';
		error = '';
		successMessage = '';
	}

	const handleSubmit: SubmitFunction = () => {
		loading = true;
		return async ({ result }) => {
			if (result.type === 'failure' && result.data) {
				error = (result.data as any).error || 'An error occurred';
				successMessage = '';
			} else if (result.type === 'success' && result.data) {
				successMessage = (result.data as any).message || 'Success!';
				error = '';
				identifier = '';
			}
			loading = false;
		};
	};
</script>

<form
	method="POST"
	use:enhance={handleSubmit}
>
	<div class="main-field">
		<Logo />
		
		<div class="heading-text">
			<h2>Welcome to Geni</h2>
			<p>Please sign in or sign up below.</p>
		</div>
		
		<div class="primary-field">
			<div class="label-phone-email-frame">
				<label
					for="email-or-phone">
					{authMethod === 'email' ? 'Email' : 'Phone'}
				</label>
				<button
					class="toggle"
					type="button"
					onclick={toggleAuthMethod}>
					<!--ICON & TEXT -->
					{#if authMethod !== 'email'}
						<Mail size={14} />
						Use Email
					{:else}
						<Phone size={14} />
						Use Phone
					{/if}
				</button>
			</div>
			
			<input
				id="email-or-phone"
				name={authMethod === 'email' ? 'email' : 'phone'}
				bind:value={identifier}
				type={authMethod === 'email' ? 'email' : 'tel'}
				placeholder={authMethod === 'email' ? 'you@email.com' : '+1 480 555 1234'}
				required
				disabled={loading}
			/>
			
			<!--Button (continue with (email | phone))-->
			<button
				class="submit-button"
				type="submit"
				formaction="?/{authMethod === 'email' ? 'magic_link' : 'otp'}"
				disabled={loading || !identifier}
			>
				{loading ? 'Sending...' : `Continue with ${authMethod === 'email' ? 'Email' : 'Phone'}`}
			</button>
		</div>
		
		
		{#if error}
			<p class="error">{error}</p>
		{/if}
		
		{#if successMessage}
			<p class="success">{successMessage}</p>
		{/if}
	</div>
	
	<div class="separator"></div>
	
	<div class="bottom-wrapper">
		<div class="oauth-button-wrapper">
			<button
				class="oauth-button"
				formaction="?/w_google">
				Continue with Google
			</button>
			<button
				class="oauth-button"
				formaction="?/w_apple">
				Continue with Apple
			</button>
			<button
				class="oauth-button"
				formaction="?/w_passkey">
				Sign in with Passkey
			</button>
		</div>
		
		
		<div class="legal">
			<a href="https://geni.health/terms-of-service/" target="_blank">
				<small class="link-text">Terms of Service</small>
			</a>
			<a href="https://geni.health/privacy_policy/" target="_blank">
				<small class="link-text">Privacy Policy</small>
			</a>
		</div>
	</div>
</form>

<style>
	
	form {
		display: flex;
		flex-direction: column;
		justify-content: start;
		align-items: start;
		width: 100%;
		max-width: 400px;
		height: fit-content;
		background: var(--color-primary-bg);
		border-radius: 24px;
        border: 1px solid var(--color-border);
		padding: 2rem 0;
		gap: 1.5rem;
	}
	
	.main-field {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: start;
		width: 100%;
		height: fit-content;
        padding: 0 2rem;
	}

	input {
		width: 100%;
		height: fit-content;
		border: 1px solid var(--color-border);
		background: var(--color-secondary-bg);
		border-radius: 12px;
		padding: 1rem;
	}
	
	.heading-text {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: start;
		gap: 0.5rem;
		margin: 2.5rem 0 2.5rem 0;
	}
	
	.primary-field {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: start;
		width: 100%;
		height: fit-content;
		gap: 0.75rem;
	}

    .label-phone-email-frame {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        width: 100%;
        height: fit-content;
    }
	
	.toggle {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
        width: fit-content;
        height: fit-content;
		gap: 0.35rem;
        background: none;
        border: none;
		font: inherit;
		font-weight: 500;
		letter-spacing: -0.03rem;
		line-height: 1;
		color: var(--color-secondary-fg);
		transition: color 300ms ease-in;
		cursor: pointer;
	}

	.toggle:hover {
		color: var(--color-primary-fg);
	}

    .submit-button {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: fit-content;
		color: var(--color-primary-fg-inverted);
        background: var(--color-primary-fg);
        border: none;
        border-radius: 12px;
        padding: 0.75rem 2rem;
    }

	.error {
		color: var(--color-error, #ef4444);
		font-size: 0.875rem;
		margin: 0;
	}

	.success {
		color: var(--color-success, #10b981);
		font-size: 0.875rem;
		margin: 0;
	}
	
    .bottom-wrapper {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: start;
		width: 100%;
		height: fit-content;
		gap: 1.5rem;
	}
	
    .separator {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 1px;
        background: var(--color-border);
    }
	
	.oauth-button-wrapper {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: fit-content;
		gap: 0.5rem;
        padding: 0 2rem;
	}
	
	.oauth-button {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: fit-content;
		background: var(--color-secondary-bg);
		border: none;
		border-radius: 12px;
		padding: 0.75rem 2rem;
	}
	
	.legal {
		display: flex;
		flex-direction: row;
		justify-content: start;
		align-items: center;
		width: 100%;
		height: fit-content;
		gap: 1rem;
		padding: 0 2rem;
	}

	.legal a {
		color: var(--color-secondary-fg);
		transition: color 300ms ease-in;
		text-decoration: none;
	}

	.legal a:hover {
		color: var(--color-primary-fg);
	}


    .link-text {
        position: relative;
        display: inline-block;
        text-decoration: none;
    }

    .link-text::after {
        content: '';
        position: absolute;
        left: 0;
        bottom: 0;
        height: 1px;
        width: 100%;
        background-color: currentColor;
        transform: scaleX(0);
        transform-origin: left;
        transition: transform 300ms ease-in-out;
    }

    .link-text:hover::after {
        transform: scaleX(1);
    }
</style>


<!--TODO: beautify the elements-->
<!--TODO: mesh background that moves in bg-->
<!--TODO: add interactivity to css-->
<!--TODO: implement the cleaning functions for Phone and Email-->
<!--TODO: Make MagicLink Email & Phone OTP Work, Create a JWT token and log it, then route them to dashboard-->
<!--TODO: Create a cohesive code style. Document it well. functions, api calls, server actions, markdown, styles, layouts, try catch, error handling.-->