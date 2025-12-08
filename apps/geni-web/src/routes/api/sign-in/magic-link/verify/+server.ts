import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { DeriveKey, AES256_Decrypt } from '$lib/utils/crypto';
import { db } from '$lib/db/client';
import { magicLinks, users } from '$lib/db/schemas/users';
import { eq, and } from 'drizzle-orm';
import { generateToken } from '$lib/auth/services/jwt.server';
import { setAuthCookie } from '$lib/auth/services/cookies';

export const GET: RequestHandler = async ({ url, cookies }) => {
	try {
		const token = url.searchParams.get('token');
		const email = url.searchParams.get('email');

		// Validate parameters
		if (!token || !email) {
			return json(
				{ error: 'Token and email are required' },
				{ status: 400 }
			);
		}

		// Validate email format
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(email)) {
			return json({ error: 'Invalid email format' }, { status: 400 });
		}

		// Create encryption key from email
		const { key } = DeriveKey(email);

		// Find the magic link in database
		const magicLink = await db.query.magicLinks.findFirst({
			where: and(
				eq(magicLinks.identifier, email),
				eq(magicLinks.used, false)
			)
		});

		if (!magicLink) {
			return json(
				{ error: 'Magic link not found or already used' },
				{ status: 404 }
			);
		}

		// Check if link has expired
		if (new Date() > new Date(magicLink.expiresAt)) {
			return json(
				{ error: 'Magic link has expired' },
				{ status: 401 }
			);
		}

		// Decrypt stored token hash to verify the token
		let storedEncryptedData;
		try {
			storedEncryptedData = JSON.parse(magicLink.tokenHash);
		} catch (error) {
			return json(
				{ error: 'Invalid stored token format' },
				{ status: 500 }
			);
		}

		// Reconstruct EncryptedData object
		const encryptedDataBuffer = {
			ciphertext: Buffer.from(storedEncryptedData.ciphertext, 'hex'),
			iv: Buffer.from(storedEncryptedData.iv, 'hex'),
			authTag: storedEncryptedData.authTag ? Buffer.from(storedEncryptedData.authTag, 'hex') : undefined,
			mode: storedEncryptedData.mode as 'cbc' | 'ctr' | 'gcm'
		};

		// Decrypt the token
		let decryptedTokenBuffer;
		try {
			decryptedTokenBuffer = AES256_Decrypt(encryptedDataBuffer, key);
		} catch (error) {
			return json(
				{ error: 'Failed to decrypt token. Link may be invalid.' },
				{ status: 401 }
			);
		}

		const decryptedToken = decryptedTokenBuffer.toString('hex');

		// Verify the decrypted token matches the provided token
		if (decryptedToken !== token) {
			return json(
				{ error: 'Invalid token' },
				{ status: 401 }
			);
		}

		// Get user by email
		const user = await db.query.users.findFirst({
			where: eq(users.email, email)
		});

		if (!user) {
			return json(
				{ error: 'User not found' },
				{ status: 404 }
			);
		}

		// Mark magic link as used
		await db
			.update(magicLinks)
			.set({ used: true })
			.where(
				and(
					eq(magicLinks.identifier, email),
					eq(magicLinks.tokenHash, magicLink.tokenHash)
				)
			);

		// Generate JWT token
		const jwtToken = generateToken({
			userId: user.id,
			email: user.email || ''
		});

		// Set authentication cookie
		setAuthCookie(cookies, jwtToken);

		// Log JWT token
		const { logJWTToken } = await import('$lib/db/operations/users');
		const now = new Date();
		const expiresAt = new Date(now.getTime() + 60 * 60 * 1000); // 1 hour

		await logJWTToken({
			userId: user.id,
			tokenValue: jwtToken,
			issuedAt: now,
			expiresAt,
			additionalData: 'magic_link_sign_in'
		});

		// Return success response
		return json({
			success: true,
			message: 'Successfully signed in',
			user: {
				id: user.id,
				email: user.email,
				name: user.name
			}
		});
	} catch (error) {
		console.error('Verify magic link error:', error);
		return json(
			{ error: error instanceof Error ? error.message : 'An error occurred' },
			{ status: 500 }
		);
	}
};

export const POST: RequestHandler = async ({ request, cookies }) => {
	try {
		const body = await request.json();
		const { token, email } = body;

		// Validate parameters
		if (!token || !email) {
			return json(
				{ error: 'Token and email are required' },
				{ status: 400 }
			);
		}

		// Validate email format
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(email)) {
			return json({ error: 'Invalid email format' }, { status: 400 });
		}

		// Create encryption key from email
		const { key } = DeriveKey(email);

		// Find the magic link in database
		const magicLink = await db.query.magicLinks.findFirst({
			where: and(
				eq(magicLinks.identifier, email),
				eq(magicLinks.used, false)
			)
		});

		if (!magicLink) {
			return json(
				{ error: 'Magic link not found or already used' },
				{ status: 404 }
			);
		}

		// Check if link has expired
		if (new Date() > new Date(magicLink.expiresAt)) {
			return json(
				{ error: 'Magic link has expired' },
				{ status: 401 }
			);
		}

		// Decrypt stored token hash to verify the token
		let storedEncryptedData;
		try {
			storedEncryptedData = JSON.parse(magicLink.tokenHash);
		} catch (error) {
			return json(
				{ error: 'Invalid stored token format' },
				{ status: 500 }
			);
		}

		// Reconstruct EncryptedData object
		const encryptedDataBuffer = {
			ciphertext: Buffer.from(storedEncryptedData.ciphertext, 'hex'),
			iv: Buffer.from(storedEncryptedData.iv, 'hex'),
			authTag: storedEncryptedData.authTag ? Buffer.from(storedEncryptedData.authTag, 'hex') : undefined,
			mode: storedEncryptedData.mode as 'cbc' | 'ctr' | 'gcm'
		};

		// Decrypt the token
		let decryptedTokenBuffer;
		try {
			decryptedTokenBuffer = AES256_Decrypt(encryptedDataBuffer, key);
		} catch (error) {
			return json(
				{ error: 'Failed to decrypt token. Link may be invalid.' },
				{ status: 401 }
			);
		}

		const decryptedToken = decryptedTokenBuffer.toString('hex');

		// Verify the decrypted token matches the provided token
		if (decryptedToken !== token) {
			return json(
				{ error: 'Invalid token' },
				{ status: 401 }
			);
		}

		// Get user by email
		const user = await db.query.users.findFirst({
			where: eq(users.email, email)
		});

		if (!user) {
			return json(
				{ error: 'User not found' },
				{ status: 404 }
			);
		}

		// Mark magic link as used
		await db
			.update(magicLinks)
			.set({ used: true })
			.where(
				and(
					eq(magicLinks.identifier, email),
					eq(magicLinks.tokenHash, magicLink.tokenHash)
				)
			);

		// Generate JWT token
		const jwtToken = generateToken({
			userId: user.id,
			email: user.email || ''
		});

		// Set authentication cookie
		setAuthCookie(cookies, jwtToken);

		// Log JWT token
		const { logJWTToken } = await import('$lib/db/operations/users');
		const now = new Date();
		const expiresAt = new Date(now.getTime() + 60 * 60 * 1000); // 1 hour

		await logJWTToken({
			userId: user.id,
			tokenValue: jwtToken,
			issuedAt: now,
			expiresAt,
			additionalData: 'magic_link_sign_in'
		});

		// Return success response
		return json({
			success: true,
			message: 'Successfully signed in',
			user: {
				id: user.id,
				email: user.email,
				name: user.name
			}
		});
	} catch (error) {
		console.error('Verify magic link error:', error);
		return json(
			{ error: error instanceof Error ? error.message : 'An error occurred' },
			{ status: 500 }
		);
	}
};
