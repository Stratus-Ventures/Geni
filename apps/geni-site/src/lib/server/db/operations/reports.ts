import { eq } from 'drizzle-orm';
import { db } from '../client';
import { reports } from '../schemas';

export interface CreateReportInput {
	userId: string;
	encryptedGenotypes: string;
	encryptedInsights: string;
	iv: string;
	authTag: string;
	snpCount: number;
	format: string;
}

export interface ReportRecord {
	id: string;
	userId: string;
	encryptedGenotypes: string;
	encryptedInsights: string;
	iv: string;
	authTag: string;
	snpCount: number;
	format: string;
	createdAt: Date;
}

export async function createReport(input: CreateReportInput): Promise<ReportRecord> {
	const [report] = await db
		.insert(reports)
		.values({
			userId: input.userId,
			encryptedGenotypes: input.encryptedGenotypes,
			encryptedInsights: input.encryptedInsights,
			iv: input.iv,
			authTag: input.authTag,
			snpCount: input.snpCount,
			format: input.format
		})
		.returning();
	return report;
}

export async function getReportByUserId(userId: string): Promise<ReportRecord | null> {
	const [report] = await db
		.select()
		.from(reports)
		.where(eq(reports.userId, userId))
		.orderBy(reports.createdAt)
		.limit(1);
	return report ?? null;
}

export async function getReportById(id: string): Promise<ReportRecord | null> {
	const [report] = await db.select().from(reports).where(eq(reports.id, id)).limit(1);
	return report ?? null;
}

export async function deleteReportsByUserId(userId: string): Promise<void> {
	await db.delete(reports).where(eq(reports.userId, userId));
}
