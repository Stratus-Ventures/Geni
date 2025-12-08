import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
	userExistsByEmail,
	userExistsByPhone,
	createUser,
	deleteUser,
	logJWTToken,
	expireOldJWTLogs,
	logMagicLink,
	expireOldMagicLinks
} from '../users';
import { db } from '$lib/db/client';

vi.mock('$lib/db/client');

describe('User Operations', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	describe('userExistsByEmail', () => {
		it('should return true if user with email exists', async () => {
			const mockUser = { id: '123', email: 'test@example.com' };
			vi.mocked(db.query.users.findFirst).mockResolvedValue(mockUser as any);

			const result = await userExistsByEmail('test@example.com');

			expect(result).toBe(true);
			expect(db.query.users.findFirst).toHaveBeenCalledWith({
				where: expect.anything()
			});
		});

		it('should return false if user with email does not exist', async () => {
			vi.mocked(db.query.users.findFirst).mockResolvedValue(undefined);

			const result = await userExistsByEmail('nonexistent@example.com');

			expect(result).toBe(false);
		});
	});

	describe('userExistsByPhone', () => {
		it('should return true if user with phone exists', async () => {
			const mockUser = { id: '123', phone: '+1234567890' };
			vi.mocked(db.query.users.findFirst).mockResolvedValue(mockUser as any);

			const result = await userExistsByPhone('+1234567890');

			expect(result).toBe(true);
			expect(db.query.users.findFirst).toHaveBeenCalledWith({
				where: expect.anything()
			});
		});

		it('should return false if user with phone does not exist', async () => {
			vi.mocked(db.query.users.findFirst).mockResolvedValue(undefined);

			const result = await userExistsByPhone('+9999999999');

			expect(result).toBe(false);
		});
	});

	describe('createUser', () => {
		it('should create a user with valid data', async () => {
			const userData = {
				name: 'John Doe',
				email: 'john@example.com',
				phone: '+1234567890'
			};

			const createdUser = {
				id: '123',
				...userData,
				plan: 'FreeTrial',
				onboardingCompleted: false,
				lastSignIn: null,
				lastSignInMethod: null,
				createdAt: new Date(),
				updatedAt: new Date()
			};

			vi.mocked(db.query.users.findFirst).mockResolvedValue(undefined);
			vi.mocked(db.insert).mockReturnValue({
				values: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue([createdUser])
				})
			} as any);

			const result = await createUser(userData);

			expect(result).toEqual(createdUser);
		});

		it('should throw error if email already exists', async () => {
			const userData = {
				name: 'John Doe',
				email: 'existing@example.com'
			};

			vi.mocked(db.query.users.findFirst).mockResolvedValue({
				id: '456',
				email: 'existing@example.com'
			} as any);

			await expect(createUser(userData)).rejects.toThrow(
				'User with email existing@example.com already exists'
			);
		});

		it('should throw error if phone already exists', async () => {
			const userData = {
				name: 'John Doe',
				phone: '+1234567890'
			};

			vi.mocked(db.query.users.findFirst).mockResolvedValue({
				id: '456',
				phone: '+1234567890'
			} as any);

			await expect(createUser(userData)).rejects.toThrow(
				'User with phone +1234567890 already exists'
			);
		});

		it('should create user without email and phone', async () => {
			const userData = {
				name: 'John Doe'
			};

			const createdUser = {
				id: '123',
				...userData,
				email: null,
				phone: null,
				plan: 'FreeTrial',
				onboardingCompleted: false,
				lastSignIn: null,
				lastSignInMethod: null,
				createdAt: new Date(),
				updatedAt: new Date()
			};

			vi.mocked(db.insert).mockReturnValue({
				values: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue([createdUser])
				})
			} as any);

			const result = await createUser(userData);

			expect(result).toEqual(createdUser);
		});
	});

	describe('deleteUser', () => {
		it('should delete user and return deleted user', async () => {
			const deletedUser = { id: '123', name: 'John Doe' };

			vi.mocked(db.delete).mockReturnValue({
				where: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue([deletedUser])
				})
			} as any);

			const result = await deleteUser('123');

			expect(result).toEqual(deletedUser);
			expect(db.delete).toHaveBeenCalled();
		});
	});

	describe('logJWTToken', () => {
		it('should log a JWT token', async () => {
			const now = new Date();
			const tokenData = {
				userId: 'user123',
				tokenValue: 'jwt_token_value',
				issuedAt: now,
				expiresAt: new Date(now.getTime() + 3600000),
				additionalData: 'user-agent: Mozilla/5.0'
			};

			const logEntry = {
				id: 'log123',
				...tokenData,
				loggedAt: now
			};

			vi.mocked(db.insert).mockReturnValue({
				values: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue([logEntry])
				})
			} as any);

			const result = await logJWTToken(tokenData);

			expect(result).toEqual(logEntry);
		});

		it('should log JWT token without additional data', async () => {
			const now = new Date();
			const tokenData = {
				userId: 'user123',
				tokenValue: 'jwt_token_value',
				issuedAt: now,
				expiresAt: new Date(now.getTime() + 3600000)
			};

			const logEntry = {
				id: 'log123',
				...tokenData,
				additionalData: null,
				loggedAt: now
			};

			vi.mocked(db.insert).mockReturnValue({
				values: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue([logEntry])
				})
			} as any);

			const result = await logJWTToken(tokenData);

			expect(result).toEqual(logEntry);
		});
	});

	describe('expireOldJWTLogs', () => {
		it('should delete JWT logs older than 30 days', async () => {
			const deletedLogs = [
				{
					id: 'log1',
					userId: 'user1',
					loggedAt: new Date(Date.now() - 31 * 24 * 60 * 60 * 1000)
				}
			];

			vi.mocked(db.delete).mockReturnValue({
				where: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue(deletedLogs)
				})
			} as any);

			const result = await expireOldJWTLogs();

			expect(result).toEqual(deletedLogs);
			expect(db.delete).toHaveBeenCalled();
		});

		it('should return empty array if no logs to delete', async () => {
			vi.mocked(db.delete).mockReturnValue({
				where: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue([])
				})
			} as any);

			const result = await expireOldJWTLogs();

			expect(result).toEqual([]);
		});
	});

	describe('logMagicLink', () => {
		it('should log a magic link', async () => {
			const expiresAt = new Date(Date.now() + 15 * 60 * 1000);
			const linkData = {
				identifier: 'user@example.com',
				tokenHash: 'hash123456',
				expiresAt
			};

			const logEntry = { ...linkData };

			vi.mocked(db.insert).mockReturnValue({
				values: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue([logEntry])
				})
			} as any);

			const result = await logMagicLink(linkData);

			expect(result).toEqual(logEntry);
		});
	});

	describe('expireOldMagicLinks', () => {
		it('should delete magic links expired before current time', async () => {
			const deletedLinks = [
				{
					identifier: 'user@example.com',
					tokenHash: 'hash1',
					expiresAt: new Date(Date.now() - 1000)
				}
			];

			vi.mocked(db.delete).mockReturnValue({
				where: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue(deletedLinks)
				})
			} as any);

			const result = await expireOldMagicLinks();

			expect(result).toEqual(deletedLinks);
		});

		it('should delete magic links expired before specified date', async () => {
			const customDate = new Date('2024-01-01');
			const deletedLinks = [
				{
					identifier: 'user@example.com',
					tokenHash: 'hash1',
					expiresAt: new Date('2023-12-31')
				}
			];

			vi.mocked(db.delete).mockReturnValue({
				where: vi.fn().mockReturnValue({
					returning: vi.fn().mockResolvedValue(deletedLinks)
				})
			} as any);

			const result = await expireOldMagicLinks(customDate);

			expect(result).toEqual(deletedLinks);
		});
	});
});
