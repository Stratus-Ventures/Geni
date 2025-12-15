import { eq, and, gt, lt } from 'drizzle-orm';
import { db } from '../client';
import { magicLinks, jwtTokenLogs } from '../schemas';
import { createHash, randomBytes } from 'crypto';

export interface MagicLinkRecord {
	identifier: string;
	tokenHash: string;
	expiresAt: Date;
	used: boolean;
	createdAt: Date;
}

export function hashToken(token: string): string {
	return createHash('sha256').update(token).digest('hex');
}

export function generateToken(): string {
	return randomBytes(32).toString('hex');
}

export async function createMagicLink(email: string, expiresInMinutes = 15): Promise<string> {
	const token = generateToken();
	const tokenHash = hashToken(token);
	const expiresAt = new Date(Date.now() + expiresInMinutes * 60 * 1000);

	await db.insert(magicLinks).values({
		identifier: email.toLowerCase(),
		tokenHash,
		expiresAt,
		used: false
	});

	return token;
}

export async function verifyMagicLink(token: string): Promise<string | null> {
	const tokenHash = hashToken(token);
	const now = new Date();

	const [link] = await db
		.select()
		.from(magicLinks)
		.where(
			and(
				eq(magicLinks.tokenHash, tokenHash),
				eq(magicLinks.used, false),
				gt(magicLinks.expiresAt, now)
			)
		)
		.limit(1);

	if (!link) return null;

	await db.update(magicLinks).set({ used: true }).where(eq(magicLinks.tokenHash, tokenHash));

	return link.identifier;
}

export async function logJwtToken(
	userId: string,
	tokenHash: string,
	issuedAt: Date,
	expiresAt: Date
): Promise<void> {
	await db.insert(jwtTokenLogs).values({
		userId,
		tokenHash,
		issuedAt,
		expiresAt
	});
}

export async function cleanupExpiredMagicLinks(): Promise<void> {
	const now = new Date();
	await db.delete(magicLinks).where(lt(magicLinks.expiresAt, now));
}
