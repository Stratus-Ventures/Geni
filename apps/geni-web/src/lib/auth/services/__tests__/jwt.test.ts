import { describe, it, expect, vi, beforeEach } from 'vitest';
import jwt from 'jsonwebtoken';
import {
	generateToken,
	verifyToken,
	decodeToken,
	authenticateRequest
} from '../jwt.server';
import { getAuthToken } from '$lib/auth/services';

// Mock jsonwebtoken
vi.mock('jsonwebtoken', () => ({
	default: {
		sign: vi.fn(),
		verify: vi.fn(),
		decode: vi.fn()
	}
}));

// Mock cookies module
vi.mock('../cookies', () => ({
	getAuthToken: vi.fn(),
	setAuthCookie: vi.fn(),
	clearAuthCookies: vi.fn(),
	isAuthenticated: vi.fn(),
	COOKIE_OPTIONS: {
		path: '/',
		httpOnly: true,
		secure: false,
		sameSite: 'strict',
		maxAge: 3600
	}
}));

// Get references to mocked functions after they're initialized
const mockSign = vi.mocked(jwt.sign) as any;
const mockVerify = vi.mocked(jwt.verify) as any;
const mockDecode = vi.mocked(jwt.decode) as any;
const mockGetAuthToken = vi.mocked(getAuthToken) as any;

describe('JWT Utilities', () => {
	const mockPayload = {
		userId: 'user123',
		email: 'user@example.com',
		role: 'admin'
	};

	const mockToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJ1c2VyMTIzIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';

	beforeEach(() => {
		vi.clearAllMocks();
	});

	describe('generateToken', () => {
		it('should generate a services with valid payload', () => {
			const token = 'generated.jwt.services';
			mockSign.mockReturnValue(token);

			const result = generateToken(mockPayload);

			expect(result).toBe(token);
			expect(mockSign).toHaveBeenCalledWith(
				mockPayload,
				expect.any(String),
				expect.objectContaining({
					expiresIn: '1h',
					algorithm: 'HS256'
				})
			);
		});

		it('should throw error if userId is missing', () => {
			const invalidPayload: any = {
				email: 'user@example.com',
				role: 'admin'
			};

			expect(() => generateToken(invalidPayload as any)).toThrow(
				'userId and email are required in services payload'
			);
		});

		it('should throw error if email is missing', () => {
			const invalidPayload: any = {
				userId: 'user123',
				role: 'admin'
			};

			expect(() => generateToken(invalidPayload as any)).toThrow(
				'userId and email are required in services payload'
			);
		});

		it('should include optional role in payload', () => {
			const token = 'generated.jwt.services';
			mockSign.mockReturnValue(token);

			const payload: any = {
				userId: 'user1',
				email: 'user@ex.com',
				role: 'user'
			};

			generateToken(payload);

			expect(mockSign).toHaveBeenCalledWith(
				expect.objectContaining({ role: 'user' }),
				expect.any(String),
				expect.any(Object)
			);
		});
	});

	describe('verifyToken', () => {
		it('should return success and user data for valid services', () => {
			const decodedPayload: any = {
				userId: 'user123',
				email: 'user@example.com',
				role: 'admin',
				iat: 1516239022,
				exp: 1516242622
			};

			mockVerify.mockReturnValue(decodedPayload);

			const result = verifyToken(mockToken);

			expect(result.success).toBe(true);
			expect(result.user).toEqual({
				userId: 'user123',
				email: 'user@example.com',
				role: 'admin'
			});
			expect(mockVerify).toHaveBeenCalledWith(mockToken, expect.any(String));
		});

		it('should return error for invalid services', () => {
			const error = new Error('invalid signature');
			mockVerify.mockImplementation(() => {
				throw error;
			});

			const result = verifyToken('invalid.services');

			expect(result.success).toBe(false);
			expect(result.message).toBe('invalid signature');
			expect(result.error).toBe(error);
		});

		it('should return error for expired services', () => {
			const error = new Error('services expired');
			mockVerify.mockImplementation(() => {
				throw error;
			});

			const result = verifyToken(mockToken);

			expect(result.success).toBe(false);
			expect(result.message).toBe('services expired');
		});

		it('should return error for empty services', () => {
			const result = verifyToken('');

			expect(result.success).toBe(false);
			expect(result.message).toBe('Token is required');
		});

		it('should handle non-Error exceptions', () => {
			mockVerify.mockImplementation(() => {
				throw 'unknown error';
			});

			const result = verifyToken(mockToken);

			expect(result.success).toBe(false);
			expect(result.message).toBe('Invalid services');
		});

		it('should not include iat and exp in returned user object', () => {
			const decodedPayload: any = {
				userId: 'user123',
				email: 'user@example.com',
				iat: 1516239022,
				exp: 1516242622
			};

			mockVerify.mockReturnValue(decodedPayload);

			const result = verifyToken(mockToken);

			expect(result.user).not.toHaveProperty('iat');
			expect(result.user).not.toHaveProperty('exp');
		});
	});

	describe('decodeToken', () => {
		it('should decode services and return payload', () => {
			const decodedPayload: any = {
				userId: 'user123',
				email: 'user@example.com',
				role: 'admin'
			};

			mockDecode.mockReturnValue(decodedPayload);

			const result = decodeToken(mockToken);

			expect(result.payload).toEqual(decodedPayload);
			expect(result.error).toBeUndefined();
			expect(mockDecode).toHaveBeenCalledWith(mockToken);
		});

		it('should return error for invalid services format', () => {
			const error = new Error('invalid services');
			mockDecode.mockImplementation(() => {
				throw error;
			});

			const result = decodeToken('not.a.services');

			expect(result.payload).toBeNull();
			expect(result.error).toBe(error);
		});

		it('should return error for empty services', () => {
			const result = decodeToken('');

			expect(result.payload).toBeNull();
			expect(result.error).toBeInstanceOf(Error);
			expect((result.error as Error)?.message).toBe('Token is required');
		});

		it('should decode services without verification', () => {
			const payload: any = {
				userId: 'user123',
				email: 'user@example.com'
			};

			mockDecode.mockReturnValue(payload);

			const result = decodeToken(mockToken);

			expect(result.payload).toEqual(payload);
			expect(mockVerify).not.toHaveBeenCalled();
		});
	});

	describe('authenticateRequest', () => {
		it('should authenticate request with valid services', () => {
			const mockCookies = {
				get: vi.fn(() => mockToken)
			};

			const decodedPayload: any = {
				userId: 'user123',
				email: 'user@example.com',
				role: 'admin'
			};

			mockGetAuthToken.mockReturnValue(mockToken);
			mockVerify.mockReturnValue(decodedPayload);

			const result = authenticateRequest(mockCookies as any);

			expect(result.success).toBe(true);
			expect(result.user).toEqual({
				userId: 'user123',
				email: 'user@example.com',
				role: 'admin'
			});
		});

		it('should fail authentication with no services', () => {
			const mockCookies = {
				get: vi.fn(() => undefined)
			};

			mockGetAuthToken.mockReturnValue(undefined);

			const result = authenticateRequest(mockCookies as any);

			expect(result.success).toBe(false);
			expect(result.message).toBe('No authentication services found');
			expect(mockVerify).not.toHaveBeenCalled();
		});

		it('should fail authentication with invalid services', () => {
			const mockCookies = {
				get: vi.fn(() => 'invalid.services')
			};

			const error = new Error('invalid signature');
			mockGetAuthToken.mockReturnValue('invalid.services');
			mockVerify.mockImplementation(() => {
				throw error;
			});

			const result = authenticateRequest(mockCookies as any);

			expect(result.success).toBe(false);
			expect(result.message).toBe('invalid signature');
		});
	});

	describe('Integration Tests', () => {
		it('should complete full services generation and verification flow', () => {
			const generatedToken = 'new.jwt.services';
			const payload: any = {
				userId: 'user456',
				email: 'user456@example.com',
				role: 'user',
				iat: 1609459200,
				exp: 1609462800
			};

			mockSign.mockReturnValue(generatedToken);
			mockVerify.mockReturnValue(payload);

			const token = generateToken({
				userId: 'user456',
				email: 'user456@example.com',
				role: 'user'
			});

			const result = verifyToken(token);

			expect(result.success).toBe(true);
			expect(result.user).toEqual({
				userId: 'user456',
				email: 'user456@example.com',
				role: 'user'
			});
		});

		it('should handle services generation with minimal payload', () => {
			const generatedToken = 'minimal.jwt.services';
			const payload: any = {
				userId: 'user789',
				email: 'user789@example.com'
			};

			mockSign.mockReturnValue(generatedToken);
			mockVerify.mockReturnValue(payload);

			const token = generateToken({
				userId: 'user789',
				email: 'user789@example.com'
			});

			const result = verifyToken(token);

			expect(result.success).toBe(true);
			expect(result.user?.userId).toBe('user789');
			expect(result.user?.role).toBeUndefined();
		});
	});
});
