import { redirect } from '@sveltejs/kit';
import { Polar } from '@polar-sh/sdk';
import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';
import { upsertUser } from '$lib/server/db/operations/users';
import { generateAndLogToken } from '$lib/server/auth/jwt.server';
import { setAuthCookie } from '$lib/server/auth/cookies';
import { extractEmailFromCheckout } from '$lib/server/checkout';

const polar = new Polar({
	accessToken: env.POLAR_ACCESS_TOKEN,
	server: env.POLAR_MODE === 'sandbox' ? 'sandbox' : 'production'
});

export const load: PageServerLoad = async ({ url, cookies }) => {
	const checkoutId = url.searchParams.get('checkout_id');

	if (!checkoutId) {
		redirect(303, '/?error=missing_checkout');
	}

	const checkout = await polar.checkouts.get({ id: checkoutId });

	if (checkout.status !== 'succeeded') {
		redirect(303, '/?error=payment_failed');
	}

	const email = extractEmailFromCheckout(checkout);

	if (!email) {
		console.error('No email found in checkout:', JSON.stringify(checkout, null, 2));
		redirect(303, '/?error=no_email');
	}

	const user = await upsertUser(email.toLowerCase(), 'paid');
	const token = await generateAndLogToken({
		userId: user.id,
		email: user.email,
		plan: user.plan
	});
	setAuthCookie(cookies, token);

	redirect(303, '/report');
};
