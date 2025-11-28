<script lang="ts">

	//TODO: Seperate components... into their own file strucutre
	//TODO: make State nicer
	//TODO: remove tailwindcss. and switch to svelte default styling
	//TODO: Make email field have label on it on click.
	//TODO: Handle Oath & Email with Supabase.
	//TODO: make a handler so that if they have the wrong screen based if we have the email or not.. Direct them to the correct page.
	
	
	import { GoogleIcon, AppleIcon, MicrosoftIcon } from "$lib/components/icons/";

	import { Phone } from '@lucide/svelte';
	import { Button, LogoIcon, ThemeToggle } from '$lib';

	// AUTHENTICATION CONTENT STATE //
	let authState = $state<'register' | 'sign_in'>('register');
	const AUTH_META = {
			register: {
					title: "Register - Geni",
					description: "Create your Geni account in seconds",
		section: "Register card",
			},
			sign_in: {
					title: "Sign in - Geni",
					description: "Welcome back. Let's get you signed in.",
					section: "Sign in card",
			}
	}

	let isRegister = $derived(authState === 'register');
	let currentMeta = $derived(AUTH_META[authState]);

	function toggleAuthState() {
			authState = isRegister ? 'sign_in' : 'register';
	}

	let email = $state('');
	
	function isValidEmail(email: string) {
			const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
			return emailRegex.test(email);
	}
	
	function createAccount() {
	
	}
	
	function signIn() {
	
	}
	
</script>

<svelte:head>
	<!-- PAGE TITLE AND DESCRIPTION	-->
	<title>{currentMeta.title}</title>
	<meta name="description" content={currentMeta.description} />
</svelte:head>


<main
	class="flex flex-col justify-start items-center w-full min-h-screen h-fit
	sm:p-8 background-gradient"
>
	<div class="flex flex-row w-full h-fit items-center justify-start p-5 mb-3">
		<!-- [MOBILE] GENI LOGO -->
		<a
			href="https://geni.health"
			class="sm:hidden flex flex-row items-center justify-start w-fit gap-2"
		>
			<LogoIcon class="h-7 w-7 "/>
			<h2 class="text-2xl font-semibold select-none">Geni</h2>
		</a>
	</div>

	<!-- REGISTER / LOGIN CARD -->
	<section
		aria-label={currentMeta.section}
		class="flex flex-col justify-between items-start w-full sm:max-w-lg h-full sm:h-fit
		p-6 py-8 pb-16 bg-background/80
		drop-shadow-[0_-8px_24px_rgba(0,0,0,0.08)]
		dark:drop-shadow-[0_-8px_24px_rgba(0,0,0,0.5)] backdrop-blur-lg rounded-2xl
		inset-shadow-white border-t border-border/20 dark:border-border/50
		rounded-b-none sm:rounded-2xl gap-12 sm:p-8 sm:gap-12"
	>
		<!-- LOGO -->
		<a
			href="https://geni.health"
			class="hidden sm:flex flex-row items-center justify-start w-fit gap-2"
		>
			<LogoIcon class="h-10 w-10 "/>
			<h2 class="font-semibold select-none">Geni</h2>
		</a>
		
		<div class="flex flex-col justify-center items-start w-full h-fit gap-1">
			<!-- H2 -->
			<h2>{authState === 'register' ? 'Let’s create your account' : 'Welcome back'}</h2>
			<!-- SUBTITLE-->
			<p class="text-muted-foreground">
				
				{authState === 'register' ? 'Already have an account?' : 'Don\'t have an account?'}
				
				<Button
					onclick={toggleAuthState}
					variant="link"
					class="ml-0.5 sm:ml-1 text-action-orange cursor-pointer"
				>
					<p>{authState === 'register' ? 'Sign in here' : 'Create one here'}</p>
				</Button>
			</p>
		</div>
		
		<!-- EMAIL AND OATH BUTTONS -->
		<div class="flex flex-col items-center justify-start w-full h-fit gap-6">
			
			<div class="flex flex-col justify-start items-center w-full h-fit gap-3">
				<!-- EMAIL -->
				<input class="w-full h-fit flex-row items-start justify-center p-3 rounded-lg border border-border
				focus:outline focus:outline-action-orange bg-accent/40 hover:bg-accent/90
				dark:bg-input/10 dark:hover:bg-input/40 transition-colors duration-200 ease-in-out
				placeholder:text-base placeholder:text-muted-foreground placeholder:font-normal placeholder:tracking-tight placeholder:leading-2"
					   bind:value={email} placeholder="Email address" type="email"/>
				<!-- LOGIN BUTTON -->
				<Button class="w-full h-fit button-text py-3 rounded-lg"
						>
					{authState === 'register' ? 'Create free account' : 'Sign in'}
				</Button>
			</div>
			
			<!-- OR -->
			<div class="flex flex-row items-center justify-center w-full h-fit gap-3">
				<div class="w-full h-[1px] bg-border rounded-lg"></div>
				<small>OR</small>
				<div class="w-full h-[1px] bg-border rounded-lg"></div>
			</div>
			<!-- SOCIAL OATH BUTTONS -->
			<div class="flex flex-col w-full h-fit justify-start items-center gap-3">
				<Button class="w-full h-fit button-text py-3
					rounded-lg border-border shadow-none"
						variant="outline"
						>
					<GoogleIcon class="w-6 h-6"/>
					Continue with Google
				</Button>
				<Button class="w-full h-fit button-text py-3
					rounded-lg border-border shadow-none"
						variant="outline"
						>
					<AppleIcon class="w-6 h-6"/>
					Continue with Apple
				</Button>
				<Button class="w-full h-fit button-text py-3
					rounded-lg border-border shadow-none"
						variant="outline"
						>
					<MicrosoftIcon class="w-6 h-6"/>
					Continue with Microsoft
				</Button>
				<Button class="w-full h-fit button-text py-3
					rounded-lg border-border shadow-none"
						variant="outline"
						>
					<Phone size={20} />
					Continue with Phone
				</Button>
			</div>
		</div>
		
		<!-- LEGAL WRAPPER -->
		<div class="flex flex-row justify-start items-center w-full h-fit gap-6">
			<Button
				href="https://geni.health/privacy-policy"
				variant="link"
				class="text-muted-foreground hover:text-primary font-normal cursor-pointer"
			>
				Privacy Policy
			</Button>
			<Button
				href="https://geni.health/terms-of-service"
				variant="link"
				class="text-muted-foreground hover:text-primary font-normal cursor-pointer mr-auto"
			>
				Terms of Service
			</Button>
			<ThemeToggle />
		</div>
	</section>
	
	<!-- AI ORB -->
</main>