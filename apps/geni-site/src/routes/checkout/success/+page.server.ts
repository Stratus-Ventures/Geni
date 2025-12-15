import { redirect } from '@sveltejs/kit';
import { Polar } from '@polar-sh/sdk';
import { POLAR_ACCESS_TOKEN, POLAR_MODE } from '$env/static/private';
import type { PageServerLoad } from './$types';
import { createUser, getUserByEmail, updateUserPlan } from '$lib/server/db/operations/users';
import { generateAndLogToken } from '$lib/server/auth/jwt.server';
import { setAuthCookie } from '$lib/server/auth/cookies';

const polar = new Polar({
	accessToken: POLAR_ACCESS_TOKEN,
	server: POLAR_MODE === 'sandbox' ? 'sandbox' : 'production'
});

export const load: PageServerLoad = async ({ url, cookies }) => {
	const checkoutId = url.searchParams.get('checkout_id');

	if (!checkoutId) {
		redirect(303, '/');
	}

	const checkout = await polar.checkouts.get({ id: checkoutId });

	if (checkout.status !== 'succeeded') {
		redirect(303, '/?error=payment_failed');
	}

	const email = checkout.customerEmail;

	if (!email) {
		redirect(303, '/?error=no_email');
	}

	let user = await getUserByEmail(email);

	if (!user) {
		user = await createUser({ email, plan: 'paid' });
	} else if (user.plan !== 'paid') {
		await updateUserPlan(user.id, 'paid');
		user = { ...user, plan: 'paid' };
	}

	const token = await generateAndLogToken({
		userId: user.id,
		email: user.email,
		plan: user.plan
	});
	setAuthCookie(cookies, token);

	redirect(303, '/report');
};
