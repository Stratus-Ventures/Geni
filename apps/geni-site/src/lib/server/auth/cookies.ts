import type { Cookies } from '@sveltejs/kit';

const AUTH_COOKIE_NAME = 'geni_auth';
const COOKIE_MAX_AGE = 7 * 24 * 60 * 60;

export function setAuthCookie(cookies: Cookies, token: string): void {
	cookies.set(AUTH_COOKIE_NAME, token, {
		path: '/',
		httpOnly: true,
		secure: true,
		sameSite: 'lax',
		maxAge: COOKIE_MAX_AGE
	});
}

export function getAuthToken(cookies: Cookies): string | undefined {
	return cookies.get(AUTH_COOKIE_NAME);
}

export function clearAuthCookies(cookies: Cookies): void {
	cookies.delete(AUTH_COOKIE_NAME, { path: '/' });
}
