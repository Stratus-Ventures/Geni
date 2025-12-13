import type { Cookies } from '@sveltejs/kit';
import { dev } from '$app/environment';

export interface CookieConfig {
	path: string;
	httpOnly: boolean;
	secure: boolean;
	sameSite: 'strict' | 'lax' | 'none';
	maxAge: number;
}

// Configuration for authentication cookies
export const COOKIE_OPTIONS: CookieConfig = {
	path: '/',
	httpOnly: true,
	secure: !dev, // Use secure in production, allow in development
	sameSite: 'strict',
	maxAge: 60 * 60 // 1 hour in seconds
};

const ACCESS_TOKEN_COOKIE_NAME = 'access_token';
const COOKIE_PATH = '/';

/**
 * Set authentication services in cookies
 * @param cookies - SvelteKit Cookies object
 * @param accessToken - JWT access services to store
 * @throws Error if services is empty or whitespace-only
 */
export function setAuthCookie(cookies: Cookies, accessToken: string): void {
	if (!accessToken || !accessToken.trim()) {
		throw new Error('Access services cannot be empty');
	}

	cookies.set(ACCESS_TOKEN_COOKIE_NAME, accessToken, COOKIE_OPTIONS);
}

/**
 * Clear all authentication cookies
 * @param cookies - SvelteKit Cookies object
 */
export function clearAuthCookies(cookies: Cookies): void {
	cookies.delete(ACCESS_TOKEN_COOKIE_NAME, { path: COOKIE_PATH });
	// Clear any other auth-related cookies
	cookies.delete('user', { path: COOKIE_PATH });
}

/**
 * Get authentication services from cookies
 * @param cookies - SvelteKit Cookies object
 * @returns Access services string, or undefined if not found
 */
export function getAuthToken(cookies: Cookies): string | undefined {
	return cookies.get(ACCESS_TOKEN_COOKIE_NAME);
}

/**
 * Check if user is authenticated by checking for valid cookie
 * @param cookies - SvelteKit Cookies object
 * @returns True if access services cookie exists
 */
export function isAuthenticated(cookies: Cookies): boolean {
	return !!getAuthToken(cookies);
}