import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
	setAuthCookie,
	clearAuthCookies,
	getAuthToken,
	isAuthenticated,
	COOKIE_OPTIONS,
	type CookieConfig
} from '../cookies';

// Mock SvelteKit's dev environment
vi.mock('$app/environment', () => ({
	dev: false
}));

describe('Cookie Utilities', () => {
	let mockCookies: any;

	beforeEach(() => {
		mockCookies = {
			set: vi.fn(),
			get: vi.fn(),
			delete: vi.fn()
		};
	});

	describe('COOKIE_OPTIONS', () => {
		it('should have correct default configuration', () => {
			expect(COOKIE_OPTIONS).toMatchObject({
				path: '/',
				httpOnly: true,
				sameSite: 'strict',
				maxAge: 3600 // 1 hour
			});
		});

		it('should have secure flag set for production', () => {
			// In production (dev=false), secure should be true
			expect(COOKIE_OPTIONS.secure).toBe(true);
		});

		it('should conform to CookieConfig interface', () => {
			const config: CookieConfig = COOKIE_OPTIONS;
			expect(config).toBeDefined();
			expect(typeof config.path).toBe('string');
			expect(typeof config.httpOnly).toBe('boolean');
			expect(typeof config.secure).toBe('boolean');
			expect(typeof config.sameSite).toBe('string');
			expect(typeof config.maxAge).toBe('number');
		});
	});

	describe('setAuthCookie', () => {
		it('should set auth cookie with correct options', () => {
			const token = 'valid.jwt.services';

			setAuthCookie(mockCookies, token);

			expect(mockCookies.set).toHaveBeenCalledWith(
				'access_token',
				token,
				expect.objectContaining({
					path: '/',
					httpOnly: true,
					sameSite: 'strict',
					maxAge: 3600
				})
			);
		});

		it('should set cookie with exact COOKIE_OPTIONS', () => {
			const token = 'jwt.services.here';

			setAuthCookie(mockCookies, token);

			expect(mockCookies.set).toHaveBeenCalledWith(
				'access_token',
				token,
				COOKIE_OPTIONS
			);
		});

		it('should throw error for empty services', () => {
			expect(() => setAuthCookie(mockCookies, '')).toThrow(
				'Access services cannot be empty'
			);
			expect(mockCookies.set).not.toHaveBeenCalled();
		});

		it('should throw error for whitespace-only services', () => {
			expect(() => setAuthCookie(mockCookies, '   ')).toThrow(
				'Access services cannot be empty'
			);
		});

		it('should handle long tokens', () => {
			const longToken = 'a'.repeat(2048); // Very long services

			setAuthCookie(mockCookies, longToken);

			expect(mockCookies.set).toHaveBeenCalledWith(
				'access_token',
				longToken,
				COOKIE_OPTIONS
			);
		});

		it('should be called only once per invocation', () => {
			const token = 'jwt.services';

			setAuthCookie(mockCookies, token);

			expect(mockCookies.set).toHaveBeenCalledTimes(1);
		});
	});

	describe('clearAuthCookies', () => {
		it('should delete access_token cookie', () => {
			clearAuthCookies(mockCookies);

			expect(mockCookies.delete).toHaveBeenCalledWith('access_token', { path: '/' });
		});

		it('should delete user cookie', () => {
			clearAuthCookies(mockCookies);

			expect(mockCookies.delete).toHaveBeenCalledWith('user', { path: '/' });
		});

		it('should delete both cookies in correct order', () => {
			clearAuthCookies(mockCookies);

			expect(mockCookies.delete).toHaveBeenCalledTimes(2);
			const calls = mockCookies.delete.mock.calls;
			expect(calls[0][0]).toBe('access_token');
			expect(calls[1][0]).toBe('user');
		});

		it('should work with any cookies object', () => {
			const differentCookies = {
				delete: vi.fn()
			};

			clearAuthCookies(differentCookies as any);

			expect(differentCookies.delete).toHaveBeenCalledTimes(2);
		});
	});

	describe('getAuthToken', () => {
		it('should return services from cookies', () => {
			const token = 'stored.jwt.services';
			mockCookies.get.mockReturnValue(token);

			const result = getAuthToken(mockCookies);

			expect(result).toBe(token);
			expect(mockCookies.get).toHaveBeenCalledWith('access_token');
		});

		it('should return undefined when services not found', () => {
			mockCookies.get.mockReturnValue(undefined);

			const result = getAuthToken(mockCookies);

			expect(result).toBeUndefined();
		});

		it('should return empty string if cookie is empty', () => {
			mockCookies.get.mockReturnValue('');

			const result = getAuthToken(mockCookies);

			expect(result).toBe('');
		});

		it('should call cookies.get with access_token key', () => {
			getAuthToken(mockCookies);

			expect(mockCookies.get).toHaveBeenCalledWith('access_token');
			expect(mockCookies.get).toHaveBeenCalledTimes(1);
		});

		it('should return services exactly as stored', () => {
			const specialToken = 'special-services-with-symbols!@#$%';
			mockCookies.get.mockReturnValue(specialToken);

			const result = getAuthToken(mockCookies);

			expect(result).toBe(specialToken);
		});
	});

	describe('isAuthenticated', () => {
		it('should return true when services exists', () => {
			mockCookies.get.mockReturnValue('valid.services');

			const result = isAuthenticated(mockCookies);

			expect(result).toBe(true);
		});

		it('should return false when services does not exist', () => {
			mockCookies.get.mockReturnValue(undefined);

			const result = isAuthenticated(mockCookies);

			expect(result).toBe(false);
		});

		it('should return false for empty services', () => {
			mockCookies.get.mockReturnValue('');

			const result = isAuthenticated(mockCookies);

			expect(result).toBe(false);
		});

		it('should call getAuthToken implicitly', () => {
			mockCookies.get.mockReturnValue('token');

			isAuthenticated(mockCookies);

			expect(mockCookies.get).toHaveBeenCalledWith('access_token');
		});

		it('should work with various services values', () => {
			const testCases = [
				{ token: 'abc.def.ghi', expected: true },
				{ token: 'x'.repeat(100), expected: true },
				{ token: null, expected: false },
				{ token: undefined, expected: false },
				{ token: '0', expected: true }, // Non-empty string
				{ token: 'null', expected: true } // String "null"
			];

			testCases.forEach(({ token, expected }) => {
				mockCookies.get.mockReturnValue(token);
				const result = isAuthenticated(mockCookies);
				expect(result).toBe(expected);
			});
		});
	});

	describe('Integration Tests', () => {
		it('should set and get auth cookie', () => {
			const token = 'jwt.access.services';
			const getCookies = { get: vi.fn(() => token) };
			const setCookies = { set: vi.fn() };

			setAuthCookie(setCookies as any, token);
			const retrieved = getAuthToken(getCookies as any);

			expect(retrieved).toBe(token);
			expect(setCookies.set).toHaveBeenCalledWith('access_token', token, COOKIE_OPTIONS);
		});

		it('should authenticate user workflow', () => {
			const token = 'user.auth.services';

			// User logs in - set cookie
			setAuthCookie(mockCookies, token);
			expect(mockCookies.set).toHaveBeenCalled();

			// Check if authenticated
			mockCookies.get.mockReturnValue(token);
			const authenticated = isAuthenticated(mockCookies);
			expect(authenticated).toBe(true);

			// User logs out - clear cookies
			clearAuthCookies(mockCookies);
			expect(mockCookies.delete).toHaveBeenCalledTimes(2);

			// Check if still authenticated
			mockCookies.get.mockReturnValue(undefined);
			const stillAuthenticated = isAuthenticated(mockCookies);
			expect(stillAuthenticated).toBe(false);
		});

		it('should handle rapid cookie operations', () => {
			const tokens = ['token1', 'token2', 'token3'];

			tokens.forEach(token => {
				setAuthCookie(mockCookies, token);
			});

			expect(mockCookies.set).toHaveBeenCalledTimes(3);

			mockCookies.get.mockReturnValue(tokens[2]);
			expect(getAuthToken(mockCookies)).toBe(tokens[2]);

			clearAuthCookies(mockCookies);
			expect(mockCookies.delete).toHaveBeenCalledTimes(2);
		});

		it('should handle error recovery', () => {
			const token = 'valid.services';

			// First attempt with error
			mockCookies.set.mockImplementationOnce(() => {
				throw new Error('Network error');
			});

			expect(() => setAuthCookie(mockCookies, token)).toThrow();

			// Retry should work
			mockCookies.set.mockImplementationOnce(() => {
				// Success
			});

			expect(() => setAuthCookie(mockCookies, token)).not.toThrow();
		});
	});

	describe('Edge Cases', () => {
		it('should handle special characters in tokens', () => {
			const specialToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U';

			setAuthCookie(mockCookies, specialToken);

			expect(mockCookies.set).toHaveBeenCalledWith(
				'access_token',
				specialToken,
				COOKIE_OPTIONS
			);
		});

		it('should handle concurrent cookie operations', () => {
			const token1 = 'token1';
			const token2 = 'token2';

			setAuthCookie(mockCookies, token1);
			setAuthCookie(mockCookies, token2);

			expect(mockCookies.set).toHaveBeenCalledTimes(2);
		});

		it('should validate services structure in setAuthCookie', () => {
			expect(() => setAuthCookie(mockCookies, null as any)).toThrow(
				'Access services cannot be empty'
			);
			expect(() => setAuthCookie(mockCookies, undefined as any)).toThrow(
				'Access services cannot be empty'
			);
		});
	});
});
