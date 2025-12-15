export {
	generateToken as generateJwtToken,
	verifyToken,
	generateAndLogToken,
	type JwtPayload,
	type TokenResponse,
	setAuthCookie,
	getAuthToken,
	clearAuthCookies
} from './auth';

export {
	db,
	users,
	reports,
	magicLinks,
	jwtTokenLogs,
	createUser,
	getUserByEmail,
	getUserById,
	updateUserPlan,
	createReport,
	getReportByUserId,
	hashToken,
	generateToken as generateMagicToken,
	createMagicLink,
	verifyMagicLink,
	logJwtToken,
	cleanupExpiredMagicLinks
} from './db';

export { sendMagicLinkEmail } from './email';
export { encryptData, decryptData, encryptReport, decryptReport } from './crypto';
