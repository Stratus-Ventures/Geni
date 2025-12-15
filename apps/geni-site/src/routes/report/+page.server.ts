import type { PageServerLoad } from './$types';
import { getAuthToken } from '$lib/server/auth/cookies';
import { verifyToken } from '$lib/server/auth/jwt.server';
import { getReportByUserId } from '$lib/server/db/operations/reports';
import { decryptReport } from '$lib/server/crypto';
import type { InsightResult } from '$lib/domain';

export const load: PageServerLoad = async ({ cookies }) => {
	const token = getAuthToken(cookies);

	if (!token) {
		return { isPaid: false, savedInsights: null };
	}

	const result = verifyToken(token);

	if (!result.success || !result.user) {
		return { isPaid: false, savedInsights: null };
	}

	const userId = result.user.userId;
	const report = await getReportByUserId(userId);

	if (!report) {
		return { isPaid: true, savedInsights: null };
	}

	const decrypted = decryptReport(
		{ ciphertext: report.encryptedGenotypes, iv: report.iv, authTag: report.authTag },
		{ ciphertext: report.encryptedInsights, iv: report.iv, authTag: report.authTag },
		userId
	);

	return {
		isPaid: true,
		savedInsights: decrypted.insights as InsightResult[],
		snpCount: report.snpCount,
		format: report.format
	};
};
