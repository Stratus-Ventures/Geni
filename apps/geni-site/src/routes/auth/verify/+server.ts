import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { verifyMagicLink } from '$lib/server/db/operations/sessions';
import { getUserByEmail } from '$lib/server/db/operations/users';
import { generateAndLogToken } from '$lib/server/auth/jwt.server';
import { setAuthCookie } from '$lib/server/auth/cookies';

export const GET: RequestHandler = async ({ url, cookies }) => {
	const token = url.searchParams.get('token');

	if (!token) {
		redirect(303, '/?error=invalid_link');
	}

	const email = await verifyMagicLink(token);

	if (!email) {
		redirect(303, '/?error=expired_link');
	}

	const user = await getUserByEmail(email);

	if (!user) {
		redirect(303, '/?error=user_not_found');
	}

	const jwt = await generateAndLogToken({
		userId: user.id,
		email: user.email,
		plan: user.plan
	});
	setAuthCookie(cookies, jwt);

	redirect(303, '/report');
};
