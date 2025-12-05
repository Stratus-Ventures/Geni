<script lang="ts">
    import { enhance } from '$app/forms';
    import { Logo } from "$lib/ui";
    import { Mail, Phone } from '@lucide/svelte';
    
    // TYPES & VARIABLES
    let authMethod: 'email' | 'phone' = 'email';
    
    // HANDLERS
    function toggleAuthMethod() {
        authMethod = authMethod === 'email' ? 'phone' : 'email';
	}
 
</script>

<form method="POST" use:enhance>
	<Logo />
	<h2>Welcome to Geni</h2>
	<p>Please sign in or sign up below.</p>
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
		name="identifier"
		type={authMethod === 'email' ? 'email' : 'tel'}
	/>
	<input type="hidden" name="method" value={authMethod} />
	<!--Button (continue with (email | phone))-->
	<button
		type="submit"
		formaction="?/register"
	>
		Register
	</button>
	<!--separator-->
	<!--Button (Continue with google)-->
	<!--Button (Continue with apple)-->
	<!--Button (Continue with passkey)-->
</form>

<style>
	.label-phone-email-frame {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		height: fit-content;
	}
</style>