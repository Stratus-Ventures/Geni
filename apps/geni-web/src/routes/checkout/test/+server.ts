import { env } from '$env/dynamic/private';
import { error, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = () => {
	const productId = env.POLAR_PRODUCT_ID;

	if (!productId) {
		throw error(500, 'POLAR_PRODUCT_ID is not configured');
	}

	const params = new URLSearchParams({ products: productId });

	throw redirect(302, `/checkout?${params.toString()}`);
};
