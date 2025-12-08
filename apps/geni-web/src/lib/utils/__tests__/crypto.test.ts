import { describe, it, expect, beforeEach } from 'vitest';
import {
	GenerateIV,
	DeriveKey,
	AES256_Encrypt,
	AES256_Decrypt
} from '../crypto';

describe('Crypto Utils', () => {
	describe('GenerateIV', () => {
		it('should generate a random IV with default size of 16 bytes', () => {
			const iv = GenerateIV();
			expect(iv).toBeInstanceOf(Buffer);
			expect(iv.length).toBe(16);
		});

		it('should generate an IV of custom size', () => {
			const iv = GenerateIV(32);
			expect(iv.length).toBe(32);
		});

		it('should generate different IVs on each call', () => {
			const iv1 = GenerateIV();
			const iv2 = GenerateIV();
			expect(iv1).not.toEqual(iv2);
		});

		it('should generate cryptographically random IVs', () => {
			const ivs = Array.from({ length: 10 }, () => GenerateIV());
			const uniqueIvs = new Set(ivs.map(iv => iv.toString('hex')));
			expect(uniqueIvs.size).toBe(10);
		});
	});

	describe('DeriveKey', () => {
		it('should derive a 32-byte key from password', () => {
			const { key } = DeriveKey('testpassword');
			expect(key).toBeInstanceOf(Buffer);
			expect(key.length).toBe(32);
		});

		it('should generate and return a salt when not provided', () => {
			const { salt } = DeriveKey('testpassword');
			expect(salt).toBeInstanceOf(Buffer);
			expect(salt.length).toBe(32);
		});

		it('should use provided salt', () => {
			const providedSalt = Buffer.from('a'.repeat(64), 'hex');
			const { salt } = DeriveKey('testpassword', providedSalt);
			expect(salt).toEqual(providedSalt);
		});

		it('should produce same key with same password and salt', () => {
			const password = 'testpassword';
			const salt = Buffer.from('a'.repeat(64), 'hex');
			const { key: key1 } = DeriveKey(password, salt);
			const { key: key2 } = DeriveKey(password, salt);
			expect(key1).toEqual(key2);
		});

		it('should produce different keys for different passwords', () => {
			const salt = Buffer.from('a'.repeat(64), 'hex');
			const { key: key1 } = DeriveKey('password1', salt);
			const { key: key2 } = DeriveKey('password2', salt);
			expect(key1).not.toEqual(key2);
		});

		it('should produce different keys for different salts', () => {
			const password = 'testpassword';
			const salt1 = Buffer.from('a'.repeat(64), 'hex');
			const salt2 = Buffer.from('b'.repeat(64), 'hex');
			const { key: key1 } = DeriveKey(password, salt1);
			const { key: key2 } = DeriveKey(password, salt2);
			expect(key1).not.toEqual(key2);
		});

		it('should support custom iteration count', () => {
			const password = 'testpassword';
			const salt = Buffer.from('a'.repeat(64), 'hex');
			const { key: key1 } = DeriveKey(password, salt, 100000);
			const { key: key2 } = DeriveKey(password, salt, 200000);
			expect(key1).not.toEqual(key2);
		});
	});

	describe('AES256_Encrypt & AES256_Decrypt', () => {
		let key: Buffer;

		beforeEach(() => {
			// Create a fixed 32-byte key for testing
			key = Buffer.from('a'.repeat(64), 'hex');
		});

		describe('GCM Mode', () => {
			it('should encrypt string data', () => {
				const data = 'Hello, World!';
				const encrypted = AES256_Encrypt(data, key, 'gcm');
				expect(encrypted.ciphertext).toBeInstanceOf(Buffer);
				expect(encrypted.iv).toBeInstanceOf(Buffer);
				expect(encrypted.authTag).toBeInstanceOf(Buffer);
				expect(encrypted.mode).toBe('gcm');
			});

			it('should encrypt buffer data', () => {
				const data = Buffer.from('Hello, World!');
				const encrypted = AES256_Encrypt(data, key, 'gcm');
				expect(encrypted.ciphertext).toBeInstanceOf(Buffer);
				expect(encrypted.iv).toBeInstanceOf(Buffer);
				expect(encrypted.authTag).toBeInstanceOf(Buffer);
			});

			it('should decrypt encrypted data correctly', () => {
				const originalData = 'Hello, World!';
				const encrypted = AES256_Encrypt(originalData, key, 'gcm');
				const decrypted = AES256_Decrypt(encrypted, key);
				expect(decrypted.toString('utf8')).toBe(originalData);
			});

			it('should produce different ciphertexts for same data (due to random IV)', () => {
				const data = 'Hello, World!';
				const encrypted1 = AES256_Encrypt(data, key, 'gcm');
				const encrypted2 = AES256_Encrypt(data, key, 'gcm');
				expect(encrypted1.ciphertext).not.toEqual(encrypted2.ciphertext);
				expect(encrypted1.iv).not.toEqual(encrypted2.iv);
			});

			it('should fail decryption with wrong key', () => {
				const wrongKey = Buffer.from('b'.repeat(64), 'hex');
				const encrypted = AES256_Encrypt('secret data', key, 'gcm');
				expect(() => AES256_Decrypt(encrypted, wrongKey)).toThrow();
			});

			it('should detect tampered ciphertext', () => {
				const encrypted = AES256_Encrypt('secret data', key, 'gcm');
				// Tamper with ciphertext
				encrypted.ciphertext[0] ^= 0xff;
				expect(() => AES256_Decrypt(encrypted, key)).toThrow();
			});

			it('should detect tampered authTag', () => {
				const encrypted = AES256_Encrypt('secret data', key, 'gcm');
				// Tamper with authTag
				if (encrypted.authTag) {
					encrypted.authTag[0] ^= 0xff;
				}
				expect(() => AES256_Decrypt(encrypted, key)).toThrow();
			});

			it('should handle large data', () => {
				const largeData = 'x'.repeat(1000000);
				const encrypted = AES256_Encrypt(largeData, key, 'gcm');
				const decrypted = AES256_Decrypt(encrypted, key);
				expect(decrypted.toString('utf8')).toBe(largeData);
			});

			it('should handle empty string', () => {
				const encrypted = AES256_Encrypt('', key, 'gcm');
				const decrypted = AES256_Decrypt(encrypted, key);
				expect(decrypted.toString('utf8')).toBe('');
			});
		});

		describe('CBC Mode', () => {
			it('should encrypt and decrypt in CBC mode', () => {
				const data = 'Hello, World!';
				const encrypted = AES256_Encrypt(data, key, 'cbc');
				expect(encrypted.mode).toBe('cbc');
				const decrypted = AES256_Decrypt(encrypted, key);
				expect(decrypted.toString('utf8')).toBe(data);
			});

			it('should not have authTag in CBC mode', () => {
				const encrypted = AES256_Encrypt('test', key, 'cbc');
				expect(encrypted.authTag).toBeUndefined();
			});
		});

		describe('CTR Mode', () => {
			it('should encrypt and decrypt in CTR mode', () => {
				const data = 'Hello, World!';
				const encrypted = AES256_Encrypt(data, key, 'ctr');
				expect(encrypted.mode).toBe('ctr');
				const decrypted = AES256_Decrypt(encrypted, key);
				expect(decrypted.toString('utf8')).toBe(data);
			});

			it('should not have authTag in CTR mode', () => {
				const encrypted = AES256_Encrypt('test', key, 'ctr');
				expect(encrypted.authTag).toBeUndefined();
			});
		});

		it('should throw error with invalid key size', () => {
			const invalidKey = Buffer.from('tooshort');
			expect(() => AES256_Encrypt('data', invalidKey, 'gcm')).toThrow(
				'Key must be 32 bytes for AES-256'
			);
		});

		it('should throw error on decrypt with invalid key size', () => {
			const invalidKey = Buffer.from('tooshort');
			const encrypted = AES256_Encrypt('data', key, 'gcm');
			expect(() => AES256_Decrypt(encrypted, invalidKey)).toThrow(
				'Key must be 32 bytes for AES-256'
			);
		});

		it('should accept custom IV', () => {
			const customIv = GenerateIV();
			const encrypted1 = AES256_Encrypt('test', key, 'gcm', customIv);
			const encrypted2 = AES256_Encrypt('test', key, 'gcm', customIv);
			// With same IV, ciphertexts should match (though in real use you'd want random IVs)
			expect(encrypted1.ciphertext).toEqual(encrypted2.ciphertext);
		});
	});

	describe('Integration Tests', () => {
		it('should perform full encryption/decryption cycle with derived key', () => {
			const password = 'mysecretpassword';
			const data = 'sensitive information';

			// Derive key from password
			const { key, salt } = DeriveKey(password);

			// Encrypt data
			const encrypted = AES256_Encrypt(data, key, 'gcm');

			// Decrypt data
			const decrypted = AES256_Decrypt(encrypted, key);
			expect(decrypted.toString('utf8')).toBe(data);

			// Verify we can recover the key with same password and salt
			const { key: recoveredKey } = DeriveKey(password, salt);
			const decryptedWithRecovered = AES256_Decrypt(encrypted, recoveredKey);
			expect(decryptedWithRecovered.toString('utf8')).toBe(data);
		});

		it('should handle multiple encryption/decryption operations', () => {
			const { key } = DeriveKey('password');
			const dataList = [
				'First message',
				'Second message',
				'Third message with special chars: !@#$%'
			];

			// Encrypt all
			const encryptedList = dataList.map(data => AES256_Encrypt(data, key, 'gcm'));

			// Decrypt all
			const decryptedList = encryptedList.map(encrypted =>
				AES256_Decrypt(encrypted, key).toString('utf8')
			);

			expect(decryptedList).toEqual(dataList);
		});

		it('should encrypt/decrypt JSON data', () => {
			const { key } = DeriveKey('password');
			const jsonData = JSON.stringify({ user: 'john', email: 'john@example.com' });

			const encrypted = AES256_Encrypt(jsonData, key, 'gcm');
			const decrypted = AES256_Decrypt(encrypted, key).toString('utf8');
			const parsedData = JSON.parse(decrypted);

			expect(parsedData).toEqual({ user: 'john', email: 'john@example.com' });
		});
	});
});
