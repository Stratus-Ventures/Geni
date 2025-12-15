import { randomBytes, pbkdf2Sync, createCipheriv, createDecipheriv } from 'crypto';
import { env } from '$env/dynamic/private';

interface EncryptedData {
	ciphertext: string;
	iv: string;
	authTag: string;
}

const ALGORITHM = 'aes-256-gcm';
const KEY_LENGTH = 32;
const IV_LENGTH = 16;
const ITERATIONS = 310000;

function deriveKey(userId: string): Buffer {
	const secret = env.JWT_SECRET || 'fallback-secret';
	const salt = Buffer.from(userId, 'utf-8');
	return pbkdf2Sync(secret, salt, ITERATIONS, KEY_LENGTH, 'sha256');
}

export function encryptData(data: string, userId: string): EncryptedData {
	const key = deriveKey(userId);
	const iv = randomBytes(IV_LENGTH);

	const cipher = createCipheriv(ALGORITHM, key, iv);
	const encrypted = Buffer.concat([cipher.update(data, 'utf8'), cipher.final()]);
	const authTag = cipher.getAuthTag();

	return {
		ciphertext: encrypted.toString('base64'),
		iv: iv.toString('base64'),
		authTag: authTag.toString('base64')
	};
}

export function decryptData(encrypted: EncryptedData, userId: string): string {
	const key = deriveKey(userId);
	const iv = Buffer.from(encrypted.iv, 'base64');
	const ciphertext = Buffer.from(encrypted.ciphertext, 'base64');
	const authTag = Buffer.from(encrypted.authTag, 'base64');

	const decipher = createDecipheriv(ALGORITHM, key, iv);
	decipher.setAuthTag(authTag);

	const decrypted = Buffer.concat([decipher.update(ciphertext), decipher.final()]);
	return decrypted.toString('utf8');
}

export function encryptReport(
	genotypes: object,
	insights: object,
	userId: string
): { encryptedGenotypes: EncryptedData; encryptedInsights: EncryptedData } {
	return {
		encryptedGenotypes: encryptData(JSON.stringify(genotypes), userId),
		encryptedInsights: encryptData(JSON.stringify(insights), userId)
	};
}

export function decryptReport(
	encryptedGenotypes: EncryptedData,
	encryptedInsights: EncryptedData,
	userId: string
): { genotypes: object; insights: object } {
	return {
		genotypes: JSON.parse(decryptData(encryptedGenotypes, userId)),
		insights: JSON.parse(decryptData(encryptedInsights, userId))
	};
}
