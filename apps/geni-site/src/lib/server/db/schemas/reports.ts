import { pgTable, uuid, text, integer, timestamp } from 'drizzle-orm/pg-core';
import { users } from './users';

export const reports = pgTable('reports', {
	id: uuid('id').defaultRandom().primaryKey(),
	userId: uuid('user_id')
		.notNull()
		.references(() => users.id, { onDelete: 'cascade' }),
	encryptedGenotypes: text('encrypted_genotypes').notNull(),
	encryptedInsights: text('encrypted_insights').notNull(),
	iv: text('iv').notNull(),
	authTag: text('auth_tag').notNull(),
	snpCount: integer('snp_count').notNull(),
	format: text('format').notNull(),
	createdAt: timestamp('created_at').notNull().defaultNow()
});
