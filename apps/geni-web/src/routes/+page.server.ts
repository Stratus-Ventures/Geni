import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions: Actions = {
	/**
	 * Magic Link Sign-In Action
	 * Sends a magic link to the user's email via /api/sign-in/magic-link
	 */
	magic_link: async ({ request, fetch }) => {
		try {
			const formData = await request.formData();
			const email = formData.get('email') as string;
			const name = formData.get('name') as string | null;

			// Validate email
			if (!email || typeof email !== 'string') {
				return fail(400, {
					error: 'Email is required',
					email
				});
			}

			// Call the magic link API endpoint
			const response = await fetch('/api/sign-in/magic-link', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					email,
					name: name || undefined
				})
			});

			const data = await response.json();

			if (!response.ok) {
				return fail(response.status, {
					error: data.error || 'Failed to send magic link',
					email
				});
			}

			return {
				success: true,
				message: 'Check your email for a sign-in link',
				email
			};
		} catch (error) {
			console.error('Magic link action error:', error);
			return fail(500, {
				error: 'Network error. Please try again.'
			});
		}
	},

	/**
	 * OTP Sign-In Action
	 * Sends an OTP code to the user's phone via /api/sign-in/phone-otp
	 * TODO: Implement phone OTP endpoint
	 */
	otp: async ({ request, fetch }) => {
		try {
			const formData = await request.formData();
			const phone = formData.get('phone') as string;
			const name = formData.get('name') as string | null;

			// Validate phone
			if (!phone || typeof phone !== 'string') {
				return fail(400, {
					error: 'Phone number is required',
					phone
				});
			}

			// Call the OTP API endpoint
			const response = await fetch('/api/sign-in/phone-otp', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					phone,
					name: name || undefined
				})
			});

			const data = await response.json();

			if (!response.ok) {
				return fail(response.status, {
					error: data.error || 'Failed to send OTP',
					phone
				});
			}

			return {
				success: true,
				message: 'Check your phone for a verification code',
				phone
			};
		} catch (error) {
			console.error('OTP action error:', error);
			return fail(500, {
				error: 'Network error. Please try again.'
			});
		}
	}
};
