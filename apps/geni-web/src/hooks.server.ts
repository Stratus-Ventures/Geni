import { authenticateRequest } from '$lib/auth/services/jwt.server';
import { clearAuthCookies } from '$lib/auth/services/cookies';
import { db } from '$lib/db/client';
import { users } from '$lib/db/schemas/users';
import { eq } from 'drizzle-orm';
import type { Handle } from '@sveltejs/kit';
import type { JwtPayload } from '$lib/auth/services/jwt.server';

// Extend the Locals interface to include user property
declare global {
	namespace App {
		interface Locals {
			user?: Omit<JwtPayload, 'iat' | 'exp'> & { onboardingCompleted?: boolean };
		}
	}
}

// Public routes that don't require authentication
const PUBLIC_ROUTES = [
	'/',
	'/api/sign-in/magic-link',
	'/api/sign-in/magic-link/verify',
	'/api/sign-in/phone-otp',
	'/api/sign-in/phone-otp/verify'
];

// Protected routes that require authentication
const PROTECTED_ROUTES = [
	'/dashboard',
	'/onboarding',
];

/**
 * Check if a route is public (doesn't require authentication)
 */
function isPublicRoute(pathname: string): boolean {
	return PUBLIC_ROUTES.some(
		(route) => pathname === route || pathname.startsWith(route)
	);
}

/**
 * Check if a route is a protected route that requires authentication
 */
function isProtectedRoute(pathname: string): boolean {
	return PROTECTED_ROUTES.some((route) => pathname.startsWith(route));
}

export const handle: Handle = async ({ event, resolve }) => {
	const { cookies, url } = event;
	const pathname = url.pathname;

	// Verify JWT authentication from cookies
	const authResult = authenticateRequest(cookies);

	if (authResult.success && authResult.user) {
		// User is authenticated - fetch user details from database
		try {
			const dbUser = await db.query.users.findFirst({
				where: eq(users.id, authResult.user.userId)
			});

			if (dbUser) {
				// Set user in locals with onboarding status
				event.locals.user = {
					...authResult.user,
					onboardingCompleted: dbUser.onboardingCompleted
				};

				// If authenticated user tries to access sign-in page, redirect them
				if (pathname === '/sign-in') {
					const redirectUrl = dbUser.onboardingCompleted ? '/dashboard' : '/onboarding';
					return Response.redirect(new URL(redirectUrl, url));
				}

				// Check onboarding status for protected routes
				if (pathname.startsWith('/dashboard') && !dbUser.onboardingCompleted) {
					// User hasn't completed onboarding, send to onboarding
					return Response.redirect(new URL('/onboarding', url));
				}

				if (pathname.startsWith('/onboarding') && dbUser.onboardingCompleted) {
					// User already completed onboarding, send to dashboard
					return Response.redirect(new URL('/dashboard', url));
				}
			} else {
				// User in token doesn't exist in database - clear auth and redirect
				clearAuthCookies(cookies);
				return Response.redirect(new URL('/sign-in', url));
			}
		} catch (error) {
			console.error('Error fetching user from database:', error);
			// On database error, clear cookies to prevent infinite loops
			clearAuthCookies(cookies);
			// Only redirect if accessing protected route
			if (isProtectedRoute(pathname)) {
				return Response.redirect(new URL('/sign-in', url));
			}
			// Allow public routes to continue loading
		}
	} else {
		// User is not authenticated
		if (isProtectedRoute(pathname)) {
			// Trying to access protected route without authentication
			clearAuthCookies(cookies);
			return Response.redirect(new URL('/sign-in', url));
		}

		// Allow access to public routes
		event.locals.user = undefined;
	}

	return resolve(event);
};
