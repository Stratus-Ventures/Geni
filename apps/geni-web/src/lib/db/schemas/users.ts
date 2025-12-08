import {
	pgTable,
	uuid,
	text,
	boolean,
	timestamp,
	primaryKey,
	index,
	uniqueIndex,
	bigint as bigInt,
	pgEnum
} from 'drizzle-orm/pg-core';
import { sql } from 'drizzle-orm';

export const planEnum = pgEnum('plan', ['FreeTrial', 'Plus', 'Premium', 'Lifetime']);

export const users = pgTable(
	'users',
	{
		id: uuid('id').defaultRandom().primaryKey(),
		name: text('name').notNull(),
		email: text('email').unique(),
		phone: text('phone'),
		plan: planEnum('plan').notNull().default('FreeTrial'),
		onboardingCompleted: boolean('onboarding_completed').notNull().default(false),
		lastSignIn: timestamp('last_sign_in'),
		lastSignInMethod: text('last_sign_in_method'),
		createdAt: timestamp('created_at').notNull().defaultNow(),
		updatedAt: timestamp('updated_at').notNull().defaultNow()
	},
	(table) => [
		index("idx_users_phone")
			.on(table.phone)
			.where(sql`"phone" IS NOT NULL`),
	]
);

export const userIdentities = pgTable(
	'user_identities',
	{
		id: uuid('id').defaultRandom().primaryKey(),
		userId: uuid('user_id')
			.notNull()
			.references(() => users.id, { onDelete: 'cascade' }),
		provider: text('provider').notNull(),
		providerId: text('provider_id').notNull(),
		email: text('email').notNull(),
		createdAt: timestamp('created_at').notNull().defaultNow()
	},
	(table) => [
		uniqueIndex("user_identities_provider_provider_id_key").on(
			table.provider,
			table.providerId
		),
	]
);

export const passkeys = pgTable('passkeys', {
	id: uuid('id').defaultRandom().primaryKey(),
	userId: uuid('user_id')
		.notNull()
		.references(() => users.id, { onDelete: 'cascade' }),
	name: text('name'),
	publicKey: text('public_key').notNull(),
	credentialId: text('credential_id').notNull().unique(),
	counter: bigInt('counter', { mode: 'number' }).notNull().default(0),
	deviceType: text('device_type').notNull(),
	backedUp: boolean('backed_up').notNull().default(false),
	transports: text('transports').array(),
	aaguid: uuid('aaguid'),
	lastUsedAt: timestamp('last_used_at').notNull().defaultNow(),
	createdAt: timestamp('created_at').notNull().defaultNow()
});

export const magicLinks = pgTable(
	'magic_links',
	{
		identifier: text('identifier').notNull(),
		tokenHash: text('token_hash').notNull(),
		expiresAt: timestamp('expires_at').notNull(),
		used: boolean('used').notNull().default(false)
	},
	(table) => [
		primaryKey({
			name: "magic_links_pkey",
			columns: [table.identifier, table.tokenHash],
		}),
	]
);

export const jwtTokenLogs = pgTable(
	'jwt_token_logs',
	{
		id: uuid('log_id').defaultRandom().primaryKey(),
		userId: text('user_id').notNull(),
		tokenValue: text('token_value').notNull(),
		issuedAt: timestamp('issued_at').notNull(),
		expiresAt: timestamp('expires_at').notNull(),
		loggedAt: timestamp('logged_at').notNull().defaultNow(),
		additionalData: text('additional_data')
	}
);