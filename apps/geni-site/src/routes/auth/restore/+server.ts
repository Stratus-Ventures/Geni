import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { getUserByEmail } from '$lib/server/db/operations/users';
import { createMagicLink } from '$lib/server/db/operations/sessions';
import { sendMagicLinkEmail } from '$lib/server/email/resend';

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();
	const email = body.email?.toLowerCase()?.trim();

	if (!email || !email.includes('@')) {
		return json({ error: 'Invalid email' }, { status: 400 });
	}

	const user = await getUserByEmail(email);

	if (!user || user.plan !== 'paid') {
		return json({ error: 'No purchase found' }, { status: 404 });
	}

	const token = await createMagicLink(email);
	await sendMagicLinkEmail(email, token);

	return json({ success: true });
};
