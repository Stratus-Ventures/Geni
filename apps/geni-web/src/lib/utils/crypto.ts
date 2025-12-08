import { randomBytes, pbkdf2Sync, createCipheriv, createDecipheriv } from 'crypto';

type EncryptionMode = 'cbc' | 'ctr' | 'gcm';

interface EncryptedData {
	ciphertext: Buffer;
	iv: Buffer;
	salt?: Buffer;
	authTag?: Buffer;
	mode: EncryptionMode;
}

/**
 * Generates a random initialization vector (IV) for encryption
 * @param size - Size of IV in bytes (default: 16 for AES)
 * @returns Random IV buffer
 */
export function GenerateIV(size = 16): Buffer {
	return randomBytes(size);
}

/**
 * Derives a strong encryption key from a password using PBKDF2
 * @param password - User password
 * @param salt - Random salt (generated if not provided)
 * @param iterations - Number of iterations (default: 310000 for PBKDF2-SHA256)
 * @returns Object containing derived key and salt
 */
export function DeriveKey(
	password: string,
	salt?: Buffer,
	iterations = 310000
): { key: Buffer; salt: Buffer } {
	const actualSalt = salt || randomBytes(32);
	const key = pbkdf2Sync(password, actualSalt, iterations, 32, 'sha256');
	return { key, salt: actualSalt };
}

/**
 * Encrypts data using AES-256 in the specified mode
 * @param data - Data to encrypt (string or Buffer)
 * @param key - Encryption key (32 bytes for AES-256)
 * @param mode - Encryption mode: 'cbc', 'ctr', or 'gcm'
 * @param iv - Initialization vector (generated if not provided)
 * @returns Encrypted data with metadata
 */
export function AES256_Encrypt(
	data: string | Buffer,
	key: Buffer,
	mode: EncryptionMode = 'gcm',
	iv?: Buffer
): EncryptedData {
	if (key.length !== 32) {
		throw new Error('Key must be 32 bytes for AES-256');
	}

	const dataBuffer = typeof data === 'string' ? Buffer.from(data, 'utf8') : data;
	const actualIv = iv || GenerateIV();

	const cipher = createCipheriv(`aes-256-${mode}`, key, actualIv);
	const ciphertext = Buffer.concat([cipher.update(dataBuffer), cipher.final()]);

	const result: EncryptedData = {
		ciphertext,
		iv: actualIv,
		mode
	};

	// GCM mode requires an authentication tag for integrity verification
	if (mode === 'gcm') {
		result.authTag = (cipher as any).getAuthTag();
	}

	return result;
}

/**
 * Decrypts data using AES-256 in the specified mode
 * @param encryptedData - Encrypted data object with metadata
 * @param key - Decryption key (must match encryption key)
 * @returns Decrypted data as Buffer
 */
export function AES256_Decrypt(encryptedData: EncryptedData, key: Buffer): Buffer {
	if (key.length !== 32) {
		throw new Error('Key must be 32 bytes for AES-256');
	}

	const { ciphertext, iv, mode, authTag } = encryptedData;

	const decipher = createDecipheriv(`aes-256-${mode}`, key, iv);

	// For GCM mode, set the authentication tag for verification
	if (mode === 'gcm' && authTag) {
		(decipher as any).setAuthTag(authTag);
	}

	return Buffer.concat([decipher.update(ciphertext), decipher.final()]);
}