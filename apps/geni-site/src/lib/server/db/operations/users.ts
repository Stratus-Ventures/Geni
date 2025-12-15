import { eq } from 'drizzle-orm';
import { db } from '../client';
import { users } from '../schemas';

export interface CreateUserInput {
	email: string;
	name?: string;
	plan?: 'free' | 'paid';
}

export interface UserRecord {
	id: string;
	email: string;
	name: string | null;
	plan: 'free' | 'paid';
	createdAt: Date;
	updatedAt: Date;
}

export async function createUser(input: CreateUserInput): Promise<UserRecord> {
	const [user] = await db
		.insert(users)
		.values({
			email: input.email,
			name: input.name ?? null,
			plan: input.plan ?? 'free'
		})
		.returning();
	return user;
}

export async function getUserByEmail(email: string): Promise<UserRecord | null> {
	const [user] = await db.select().from(users).where(eq(users.email, email)).limit(1);
	return user ?? null;
}

export async function getUserById(id: string): Promise<UserRecord | null> {
	const [user] = await db.select().from(users).where(eq(users.id, id)).limit(1);
	return user ?? null;
}

export async function updateUserPlan(
	userId: string,
	plan: 'free' | 'paid'
): Promise<UserRecord | null> {
	const [user] = await db
		.update(users)
		.set({ plan, updatedAt: new Date() })
		.where(eq(users.id, userId))
		.returning();
	return user ?? null;
}

export async function upsertUser(
	email: string,
	plan: 'free' | 'paid' = 'free'
): Promise<UserRecord> {
	const existing = await getUserByEmail(email);
	if (existing) {
		if (plan === 'paid' && existing.plan !== 'paid') {
			const updated = await updateUserPlan(existing.id, plan);
			return updated ?? existing;
		}
		return existing;
	}
	return createUser({ email, plan });
}
