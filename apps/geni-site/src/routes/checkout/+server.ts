import { redirect } from '@sveltejs/kit';
import { Polar } from '@polar-sh/sdk';
import { POLAR_ACCESS_TOKEN, POLAR_PRODUCT_ID, SUCCESS_URL, POLAR_MODE } from '$env/static/private';
import type { RequestHandler } from './$types';

const polar = new Polar({
	accessToken: POLAR_ACCESS_TOKEN,
	server: POLAR_MODE === 'sandbox' ? 'sandbox' : 'production'
});

export const GET: RequestHandler = async () => {
	const checkout = await polar.checkouts.create({
		products: [POLAR_PRODUCT_ID],
		successUrl: SUCCESS_URL
	});

	redirect(303, checkout.url);
};
