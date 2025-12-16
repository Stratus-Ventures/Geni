import { pgTable, uuid, text, timestamp, boolean, primaryKey } from 'drizzle-orm/pg-core';

export const jwtTokenLogs = pgTable('jwt_token_logs', {
	id: uuid('log_id').defaultRandom().primaryKey(),
	userId: uuid('user_id').notNull(),
	tokenHash: text('token_hash').notNull(),
	issuedAt: timestamp('issued_at').notNull(),
	expiresAt: timestamp('expires_at').notNull(),
	createdAt: timestamp('created_at').notNull().defaultNow()
});

export const magicLinks = pgTable(
	'magic_links',
	{
		identifier: text('identifier').notNull(),
		tokenHash: text('token_hash').notNull(),
		expiresAt: timestamp('expires_at').notNull(),
		used: boolean('used').notNull().default(false),
		createdAt: timestamp('created_at').notNull().defaultNow()
	},
	(table) => [
		primaryKey({ name: 'magic_links_pkey', columns: [table.identifier, table.tokenHash] })
	]
);
