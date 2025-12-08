import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { Resend } from 'resend';
import { DEV_URL, PROD_URL, RESEND_API_KEY, FROM_EMAIL } from '$env/static/private';
import { randomBytes } from 'crypto';
import { dev } from '$app/environment';
import { createUser, logMagicLink } from '$lib/db';
import { DeriveKey, AES256_Encrypt } from '$lib/utils/crypto';

const resend = new Resend(RESEND_API_KEY);

// Get base URL based on environment
const BASE_URL = dev ? DEV_URL : PROD_URL;

// Magic link expiration time (10 minutes)
const MAGIC_LINK_EXPIRY_MS = 10 * 60 * 1000;

/**
 * Helper to check if error is a database connection error
 */
function isConnectionError(error: unknown): boolean {
	const err = error as any;
	return err?.code === 'ECONNREFUSED' || err?.cause?.code === 'ECONNREFUSED';
}

/**
 * Helper to get readable error message from database error
 */
function getErrorMessage(error: unknown): string {
	const err = error as any;

	if (isConnectionError(error)) {
		return 'Cannot connect to database. Please check your database is running.';
	}

	if (err?.code === 'UNIQUE_VIOLATION' || err?.message?.includes('unique')) {
		return 'This email is already registered.';
	}

	if (err?.message) {
		return `Database error: ${err.message}`;
	}

	return 'An error occurred while processing your request.';
}

export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();
		const { email, name } = body;

		// Validate email
		if (!email || typeof email !== 'string') {
			return json(
				{ error: 'Email is required and must be a string' },
				{ status: 400 }
			);
		}

		// Validate email format
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		if (!emailRegex.test(email)) {
			return json({ error: 'Invalid email format' }, { status: 400 });
		}

		// Create or get user
		let user;
		try {
			user = await createUser({
				name: name || email.split('@')[0],
				email
			});
			console.log('✅ User created/found:', { email, userId: user.id });
		} catch (createError) {
			console.error('❌ Create user error:', {
				message: createError instanceof Error ? createError.message : String(createError),
				code: (createError as any)?.code,
				email
			});

			// User might already exist, try to look them up
			try {
				const { db } = await import('$lib/db/client');
				const { users } = await import('$lib/db/schemas/users');
				const { eq } = await import('drizzle-orm');

				console.log('🔍 Looking up existing user:', email);
				const existingUser = await db.query.users.findFirst({
					where: eq(users.email, email)
				});

				if (!existingUser) {
					const statusCode = isConnectionError(createError) ? 503 : 500;
					return json(
						{ error: getErrorMessage(createError) },
						{ status: statusCode }
					);
				}

				user = existingUser;
				console.log('✅ Found existing user:', { email, userId: user.id });
			} catch (dbError) {
				console.error('❌ Database lookup error:', {
					message: dbError instanceof Error ? dbError.message : String(dbError),
					code: (dbError as any)?.code,
					email
				});

				const statusCode = isConnectionError(dbError) ? 503 : 500;
				return json(
					{ error: getErrorMessage(dbError) },
					{ status: statusCode }
				);
			}
		}

		// Generate a strong random token (32 bytes = 256 bits)
		const tokenBuffer = randomBytes(32);
		const token = tokenBuffer.toString('hex');

		// Create encryption key from email using PBKDF2
		const { key } = DeriveKey(email);

		// Encrypt the token
		const encryptedData = AES256_Encrypt(token, key);

		// Create token hash for storage (encrypted data as hex string)
		const encryptedHex = JSON.stringify({
			ciphertext: encryptedData.ciphertext.toString('hex'),
			iv: encryptedData.iv.toString('hex'),
			authTag: encryptedData.authTag?.toString('hex'),
			mode: encryptedData.mode
		});

		// Set expiration time
		const expiresAt = new Date(Date.now() + MAGIC_LINK_EXPIRY_MS);

		// Log magic link to database
		try {
			await logMagicLink({
				identifier: email,
				tokenHash: encryptedHex,
				expiresAt
			});
			console.log('✅ Magic link logged to database:', email);
		} catch (magicLinkError) {
			console.error('❌ Magic link logging error:', {
				message: magicLinkError instanceof Error ? magicLinkError.message : String(magicLinkError),
				code: (magicLinkError as any)?.code,
				email
			});

			const statusCode = isConnectionError(magicLinkError) ? 503 : 500;
			return json(
				{ error: getErrorMessage(magicLinkError) },
				{ status: statusCode }
			);
		}

		// Create magic link URL
		const magicLinkUrl = new URL(`/api/sign-in/magic-link/verify`, BASE_URL);
		magicLinkUrl.searchParams.set('token', token);
		magicLinkUrl.searchParams.set('email', email);

		// Create email HTML
		const emailHtml = `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
      margin: 0;
      padding: 0;
    }
    .container {
      background-color: hsl(var(--color-primary-bg));
      padding: 2rem;
      display: flex;
      flex-direction: column;
      justify-content: flex-start;
      align-items: center;
      gap: 1rem;
      min-height: 100vh;
    }
    .logo {
      color: hsl(var(--color-secondary-fg));
      font-size: 2rem;
      font-weight: bold;
      margin-bottom: 1rem;
    }
    .content {
      max-width: 600px;
      text-align: center;
    }
    .heading {
      font-size: 1.5rem;
      color: hsl(var(--color-secondary-fg));
      margin: 1rem 0;
    }
    .paragraph {
      font-size: 1rem;
      color: hsl(var(--color-secondary-fg));
      line-height: 1.6;
      margin: 1rem 0;
    }
    .magic-link {
      background-color: hsl(var(--color-secondary-fg));
      color: hsl(var(--color-primary-bg));
      padding: 1rem 2rem;
      border-radius: 0.5rem;
      font-weight: bold;
      font-size: 1.25rem;
      letter-spacing: 0.1em;
      display: inline-block;
      margin: 2rem 0;
      font-family: 'Courier New', monospace;
    }
    .expiry {
      font-size: 0.875rem;
      color: hsl(var(--color-secondary-fg) / 0.7);
      margin-top: 2rem;
    }
  </style>
</head>
<body>
  <div class="container">
    <div class="logo">Geni</div>
    <div class="content">
      <h2 class="heading">Welcome to Geni!</h2>
      <p class="paragraph">Your secure sign-in link is ready. Use the code below to complete your authentication:</p>
      <div class="magic-link">${token.substring(0, 8).toUpperCase()}</div>
      <p class="paragraph">Or click the link to sign in automatically:</p>
      <a href="${magicLinkUrl.toString()}" style="color: hsl(var(--color-secondary-fg)); text-decoration: none; font-weight: bold;">Sign in to Geni</a>
      <p class="expiry">This link expires in 10 minutes. If you didn't request this, please ignore this email.</p>
    </div>
  </div>
</body>
</html>
		`;

		// Send email via Resend
		try {
			const { error: sendError } = await resend.emails.send({
				from: FROM_EMAIL,
				to: email,
				subject: 'Your Geni Sign-In Link',
				html: emailHtml
			});

			if (sendError) {
				console.error('❌ Resend error:', sendError);
				return json(
					{ error: 'Failed to send magic link email. Please try again.' },
					{ status: 500 }
				);
			}

			console.log('✅ Email sent successfully:', email);
		} catch (emailError) {
			console.error('❌ Email sending exception:', {
				message: emailError instanceof Error ? emailError.message : String(emailError)
			});

			return json(
				{ error: 'Failed to send email. Please try again.' },
				{ status: 500 }
			);
		}

		return json({
			success: true,
			message: 'Magic link sent to your email',
			userId: user.id
		});
	} catch (error) {
		console.error('❌ Unexpected error in sign-in:', {
			message: error instanceof Error ? error.message : String(error),
			stack: error instanceof Error ? error.stack : undefined
		});

		return json(
			{ error: 'An unexpected error occurred. Please try again later.' },
			{ status: 500 }
		);
	}
};
