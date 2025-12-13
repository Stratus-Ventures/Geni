import { env } from '$env/dynamic/private';
import { Checkout } from '@polar-sh/sveltekit';

const serverMode = env.POLAR_MODE === 'production' ? 'production' : 'sandbox';

export const GET = Checkout({
	accessToken: env.POLAR_ACCESS_TOKEN,
	successUrl: env.SUCCESS_URL,
	returnUrl: env.POLAR_RETURN_URL ?? 'https://app.geni.health', // An optional URL which renders a back-button in the Checkout
	server: serverMode, // Use sandbox if you're testing Polar - omit the parameter or pass 'production' otherwise
	theme: 'dark' // Enforces the theme - System-preferred theme will be set if left omitted
});
