<script lang="ts">
    import { Logo } from "$lib/ui";
    import { Mail, Phone } from '@lucide/svelte';

    // TYPES & VARIABLES
    let authMethod: 'email' | 'phone' = 'email';
    let identifier = '';
    let loading = false;
    let error = '';

    // HANDLERS
    function toggleAuthMethod() {
        authMethod = authMethod === 'email' ? 'phone' : 'email';
        identifier = '';
        error = '';
	}


</script>

<form>
	<Logo />
	<h2>Welcome to Geni</h2>
	<p>Please sign in or sign up below.</p>

	{#if error}
		<p class="error">{error}</p>
	{/if}

	<div class="label-phone-email-frame">
		<label for="auth-input">
			{authMethod === 'email' ? 'Email' : 'Phone'}
		</label>
		<button
			type="button"
			onclick={toggleAuthMethod}
		>
			<!--ICON & TEXT -->
			{#if authMethod !== 'email'}
				<Mail size={16} />
				Use Email
			{:else}
				<Phone size={16} />
				Use Phone
			{/if}
		</button>
	</div>
	<input
		id="auth-input"
		bind:value={identifier}
		type={authMethod === 'email' ? 'email' : 'tel'}
		placeholder={authMethod === 'email' ? 'your@email.com' : '+14805551234'}
		required
		disabled={loading}
	/>
	<!--Button (continue with (email | phone))-->
	<button
		type="submit"
		disabled={loading || !identifier}
	>
		{loading ? 'Sending...' : `Continue with ${authMethod === 'email' ? 'Email' : 'Phone'}`}
	</button>
	<!--separator Line-->
	<button
		formaction="?/w_google"
	>
		Continue with Google
	</button>
	<button
		formaction="?/w_apple"
	>
		Continue with Apple
	</button>
	<button
		formaction="?/w_passkey"
	>
		Sign in with Passkey
	</button>
	
	
	<div class="legal">
		<a href="https://geni.health/terms-of-service/" target="_blank">
			<small>Terms of Service</small>
		</a>
		<a href="https://geni.health/privacy_policy/" target="_blank">
			<small>Privacy Policy</small>
		</a>
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
		padding: 2rem;
		border-radius: 24px;
		gap: 2rem;
	}

	input {
		width: 100%;
		height: fit-content;
	}

	.error {
		color: var(--color-error, #ef4444);
		font-size: 0.875rem;
		margin: 0;
	}

	.label-phone-email-frame {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		height: fit-content;
	}


    .legal {
        display: flex;
        flex-direction: row;
        justify-content: start;
        align-items: center;
        width: fit-content;
        height: fit-content;
        gap: 1rem;
    }

</style>