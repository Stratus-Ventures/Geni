import { redirect } from '@sveltejs/kit';
import { Polar } from '@polar-sh/sdk';
import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';

const polar = new Polar({
	accessToken: env.POLAR_ACCESS_TOKEN,
	server: env.POLAR_MODE === 'sandbox' ? 'sandbox' : 'production'
});

export const GET: RequestHandler = async () => {
	const baseSuccessUrl = env.SUCCESS_URL || 'http://localhost:5173/checkout/success';
	const successUrl = `${baseSuccessUrl}?checkout_id={CHECKOUT_ID}`;

	const checkout = await polar.checkouts.create({
		products: [env.POLAR_PRODUCT_ID],
		successUrl
	});

	redirect(303, checkout.url);
};
