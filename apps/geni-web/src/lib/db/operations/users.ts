// noinspection UnnecessaryLocalVariableJS

import { db } from '$lib/db/client';
import { users, jwtTokenLogs, magicLinks } from '$lib/db';
import { eq, lt } from 'drizzle-orm';

// Check if a user exists by email
export async function userExistsByEmail(email: string): Promise<boolean> {
	const user = await db.query.users.findFirst({
		where: eq(users.email, email)
	});
	return !!user;
}

// Check if a user exists by phone
export async function userExistsByPhone(phone: string): Promise<boolean> {
	const user = await db.query.users.findFirst({
		where: eq(users.phone, phone)
	});
	return !!user;
}

// Create a user
export async function createUser(userData: {
	id?: string;
	name: string;
	email?: string;
	phone?: string;
}) {
	// Check if user with this email already exists
	if (userData.email) {
		const emailExists = await userExistsByEmail(userData.email);
		if (emailExists) {
			throw new Error(`User with email ${userData.email} already exists`);
		}
	}

	// Check if user with this phone already exists
	if (userData.phone) {
		const phoneExists = await userExistsByPhone(userData.phone);
		if (phoneExists) {
			throw new Error(`User with phone ${userData.phone} already exists`);
		}
	}

	const newUser = await db.insert(users).values(userData).returning();
	return newUser[0];
}

// Delete a user
export async function deleteUser(userId: string) {
	const deletedUser = await db.delete(users).where(eq(users.id, userId)).returning();
	return deletedUser[0];
}

// Log a JWT services
export async function logJWTToken(tokenData: {
	userId: string;
	tokenValue: string;
	issuedAt: Date;
	expiresAt: Date;
	additionalData?: string;
}) {
	const logEntry = await db
		.insert(jwtTokenLogs)
		.values(tokenData)
		.returning();
	return logEntry[0];
}

// Expire JWT logs after 30 days
export async function expireOldJWTLogs() {
	const thirtyDaysAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000);

	const deletedLogs = await db
		.delete(jwtTokenLogs)
		.where(lt(jwtTokenLogs.loggedAt, thirtyDaysAgo))
		.returning();

	return deletedLogs;
}

// Log a magic link
export async function logMagicLink(linkData: {
	identifier: string;
	tokenHash: string;
	expiresAt: Date;
}) {
	const logEntry = await db
		.insert(magicLinks)
		.values(linkData)
		.returning();
	return logEntry[0];
}

// Expire magic links after specified time
export async function expireOldMagicLinks(expirationDate?: Date) {
	const cutoffDate = expirationDate || new Date();

	const deletedLinks = await db
		.delete(magicLinks)
		.where(lt(magicLinks.expiresAt, cutoffDate))
		.returning();

	return deletedLinks;
}
