import { createHmac } from 'crypto';
import { env } from '$env/dynamic/private';
import { hashToken, logJwtToken } from '../db';

export interface JwtPayload {
	userId: string;
	email: string;
	plan: 'free' | 'paid';
	iat?: number;
	exp?: number;
}

export interface TokenResponse {
	success: boolean;
	message?: string;
	user?: Omit<JwtPayload, 'iat' | 'exp'>;
}

const JWT_SECRET = env.JWT_SECRET || 'fallback-secret-change-in-production';
const TOKEN_EXPIRY_DAYS = 7;
const TOKEN_EXPIRY_MS = TOKEN_EXPIRY_DAYS * 24 * 60 * 60 * 1000;

function base64UrlEncode(str: string): string {
	return Buffer.from(str)
		.toString('base64')
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=/g, '');
}

function base64UrlDecode(str: string): string {
	let base64 = str.replace(/-/g, '+').replace(/_/g, '/');
	const padding = 4 - (base64.length % 4);
	if (padding !== 4) base64 += '='.repeat(padding);
	return Buffer.from(base64, 'base64').toString('utf-8');
}

export function generateToken(payload: Omit<JwtPayload, 'iat' | 'exp'>): string {
	if (!payload.userId || !payload.email) {
		throw new Error('userId and email are required');
	}

	const now = Math.floor(Date.now() / 1000);
	const expiresAt = now + Math.floor(TOKEN_EXPIRY_MS / 1000);

	const header = { alg: 'HS256', typ: 'JWT' };
	const fullPayload = { ...payload, iat: now, exp: expiresAt };

	const encodedHeader = base64UrlEncode(JSON.stringify(header));
	const encodedPayload = base64UrlEncode(JSON.stringify(fullPayload));
	const message = `${encodedHeader}.${encodedPayload}`;

	const signature = createHmac('sha256', JWT_SECRET)
		.update(message)
		.digest('base64')
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=/g, '');

	return `${message}.${signature}`;
}

export function verifyToken(token: string): TokenResponse {
	if (!token) {
		return { success: false, message: 'Token is required' };
	}

	try {
		const parts = token.split('.');
		if (parts.length !== 3) {
			return { success: false, message: 'Invalid token format' };
		}

		const [encodedHeader, encodedPayload, signature] = parts;
		const message = `${encodedHeader}.${encodedPayload}`;

		const expectedSignature = createHmac('sha256', JWT_SECRET)
			.update(message)
			.digest('base64')
			.replace(/\+/g, '-')
			.replace(/\//g, '_')
			.replace(/=/g, '');

		if (signature !== expectedSignature) {
			return { success: false, message: 'Invalid token signature' };
		}

		const decoded = JSON.parse(base64UrlDecode(encodedPayload)) as JwtPayload;
		const now = Math.floor(Date.now() / 1000);

		if (decoded.exp && decoded.exp < now) {
			return { success: false, message: 'Token has expired' };
		}

		return {
			success: true,
			user: { userId: decoded.userId, email: decoded.email, plan: decoded.plan }
		};
	} catch {
		return { success: false, message: 'Invalid token' };
	}
}

export async function generateAndLogToken(
	payload: Omit<JwtPayload, 'iat' | 'exp'>
): Promise<string> {
	const token = generateToken(payload);
	const tokenHash = hashToken(token);
	const now = new Date();
	const expiresAt = new Date(now.getTime() + TOKEN_EXPIRY_MS);

	await logJwtToken(payload.userId, tokenHash, now, expiresAt);
	return token;
}
