import { createHmac } from 'crypto';
import type { Cookies } from '@sveltejs/kit';
import { getAuthToken } from './cookies';
import { env } from '$env/dynamic/private';


// Re-export cookie functions for backward compatibility
export { setAuthCookie, clearAuthCookies, getAuthToken } from './cookies';

// Types
export interface JwtPayload {
	userId: string;
	email: string;
	role?: string;
	iat?: number;
	exp?: number;
}

export interface TokenResponse {
	success: boolean;
	message?: string;
	user?: Omit<JwtPayload, 'iat' | 'exp'>;
	error?: unknown;
}

export interface DecodedToken {
	payload: JwtPayload | null;
	error?: unknown;
}

// Configuration
const JWT_SECRET = env.JWT_SECRET;
const ACCESS_TOKEN_EXPIRY_MS = 60 * 60 * 1000; // 1 hour in milliseconds

/**
 * Base64URL encode (without padding)
 */
function base64UrlEncode(str: string): string {
	return Buffer.from(str)
		.toString('base64')
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=/g, '');
}

/**
 * Base64URL decode
 */
function base64UrlDecode(str: string): string {
	let base64 = str.replace(/-/g, '+').replace(/_/g, '/');
	// Add padding if needed
	const padding = 4 - (base64.length % 4);
	if (padding !== 4) {
		base64 += '='.repeat(padding);
	}
	return Buffer.from(base64, 'base64').toString('utf-8');
}

/**
 * Generate a JWT token for a user
 * @param payload - User data to encode in token
 * @returns Signed JWT token string
 * @throws Error if token generation fails
 */
export function generateToken(payload: Omit<JwtPayload, 'iat' | 'exp'>): string {
	if (!payload.userId || !payload.email) {
		throw new Error('userId and email are required in token payload');
	}

	const now = Math.floor(Date.now() / 1000);
	const expiresAt = now + Math.floor(ACCESS_TOKEN_EXPIRY_MS / 1000);

	// Create header
	const header = {
		alg: 'HS256',
		typ: 'JWT'
	};

	// Create payload with standard claims
	const fullPayload = {
		...payload,
		iat: now,
		exp: expiresAt
	};

	// Encode header and payload
	const encodedHeader = base64UrlEncode(JSON.stringify(header));
	const encodedPayload = base64UrlEncode(JSON.stringify(fullPayload));
	const message = `${encodedHeader}.${encodedPayload}`;

	// Create signature using HMAC-SHA256
	const signature = createHmac('sha256', JWT_SECRET)
		.update(message)
		.digest('base64')
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=/g, '');

	return `${message}.${signature}`;
}

/**
 * Verify and decode a JWT token
 * @param token - JWT token string to verify
 * @returns TokenResponse with success status and decoded user data
 */
export function verifyToken(token: string): TokenResponse {
	if (!token) {
		return {
			success: false,
			message: 'Token is required'
		};
	}

	try {
		const parts = token.split('.');
		if (parts.length !== 3) {
			return {
				success: false,
				message: 'Invalid token format'
			};
		}

		const [encodedHeader, encodedPayload, signature] = parts;

		// Verify signature
		const message = `${encodedHeader}.${encodedPayload}`;
		const expectedSignature = createHmac('sha256', JWT_SECRET)
			.update(message)
			.digest('base64')
			.replace(/\+/g, '-')
			.replace(/\//g, '_')
			.replace(/=/g, '');

		if (signature !== expectedSignature) {
			return {
				success: false,
				message: 'Invalid token signature'
			};
		}

		// Decode and parse payload
		const payloadString = base64UrlDecode(encodedPayload);
		const decoded = JSON.parse(payloadString) as JwtPayload;

		// Check expiration
		const now = Math.floor(Date.now() / 1000);
		if (decoded.exp && decoded.exp < now) {
			return {
				success: false,
				message: 'Token has expired'
			};
		}

		return {
			success: true,
			user: {
				userId: decoded.userId,
				email: decoded.email,
				role: decoded.role
			}
		};
	} catch (error) {
		return {
			success: false,
			message: error instanceof Error ? error.message : 'Invalid token',
			error
		};
	}
}

/**
 * Decode a token without verification (useful for getting claims)
 * @param token - JWT token string to decode
 * @returns DecodedToken with payload or error
 */
export function decodeToken(token: string): DecodedToken {
	if (!token) {
		return {
			payload: null,
			error: new Error('Token is required')
		};
	}

	try {
		const parts = token.split('.');
		if (parts.length !== 3) {
			return {
				payload: null,
				error: new Error('Invalid token format')
			};
		}

		const payloadString = base64UrlDecode(parts[1]);
		const decoded = JSON.parse(payloadString) as JwtPayload;

		return {
			payload: decoded
		};
	} catch (error) {
		return {
			payload: null,
			error
		};
	}
}

/**
 * Authenticate a request using cookies
 * @param cookies - SvelteKit Cookies object
 * @returns TokenResponse with authentication status and user info
 */
export function authenticateRequest(cookies: Cookies): TokenResponse {
	const accessToken = getAuthToken(cookies);

	if (!accessToken) {
		return {
			success: false,
			message: 'No authentication token found'
		};
	}

	return verifyToken(accessToken);
}