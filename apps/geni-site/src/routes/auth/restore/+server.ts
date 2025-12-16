import { json } from '@sveltejs/kit';
import { Polar } from '@polar-sh/sdk';
import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';
import { getUserByEmail, upsertUser } from '$lib/server/db/operations/users';
import { createMagicLink } from '$lib/server/db/operations/sessions';
import { sendMagicLinkEmail } from '$lib/server/email/resend';

const polar = new Polar({
	accessToken: env.POLAR_ACCESS_TOKEN,
	server: env.POLAR_MODE === 'sandbox' ? 'sandbox' : 'production'
});

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();
	const email = body.email?.toLowerCase()?.trim();

	if (!email || !email.includes('@')) {
		return json({ error: 'Invalid email' }, { status: 400 });
	}

	let user = await getUserByEmail(email);

	if (!user || user.plan !== 'paid') {
		const isPolarCustomer = await checkPolarCustomer(email);
		if (!isPolarCustomer) {
			return json({ error: 'No purchase found' }, { status: 404 });
		}
		user = await upsertUser(email, 'paid');
	}

	const token = await createMagicLink(email);
	const sent = await sendMagicLinkEmail(email, token);

	if (!sent) {
		return json({ error: 'Failed to send email' }, { status: 500 });
	}

	return json({ success: true });
};

async function checkPolarCustomer(email: string): Promise<boolean> {
	try {
		const customers = await polar.customers.list({
			email: email,
			limit: 1
		});
		return customers.result.items.length > 0;
	} catch {
		return false;
	}
}
