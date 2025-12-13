import { env } from '$env/dynamic/private';
import { CustomerPortal } from '@polar-sh/sveltekit';

const serverMode = env.POLAR_MODE === 'production' ? 'production' : 'sandbox';

export const GET = CustomerPortal({
	server: serverMode, // Use sandbox if you're testing Polar - omit the parameter or pass 'production' otherwise
	accessToken: env.POLAR_ACCESS_TOKEN,
	returnUrl: env.POLAR_RETURN_URL ?? 'https://app.geni.health/', // An optional URL which renders a back-button in the Customer Portal
	getCustomerId: async () => '' // Function to resolve a Polar Customer ID
});
