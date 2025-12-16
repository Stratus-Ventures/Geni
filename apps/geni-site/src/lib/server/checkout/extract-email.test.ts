import { describe, it, expect } from 'vitest';
import { extractEmailFromCheckout } from './extract-email';

describe('extractEmailFromCheckout', () => {
	it('should extract email from customerEmail field', () => {
		const checkout = { customerEmail: 'test@example.com', status: 'succeeded' };
		expect(extractEmailFromCheckout(checkout)).toBe('test@example.com');
	});

	it('should extract email from nested customer.email field', () => {
		const checkout = { customer: { email: 'nested@example.com' }, status: 'succeeded' };
		expect(extractEmailFromCheckout(checkout)).toBe('nested@example.com');
	});

	it('should prefer customerEmail over customer.email', () => {
		const checkout = {
			customerEmail: 'primary@example.com',
			customer: { email: 'nested@example.com' },
			status: 'succeeded'
		};
		expect(extractEmailFromCheckout(checkout)).toBe('primary@example.com');
	});

	it('should return null when no email is found', () => {
		const checkout = { status: 'succeeded' };
		expect(extractEmailFromCheckout(checkout)).toBeNull();
	});

	it('should return null for empty customerEmail', () => {
		const checkout = { customerEmail: '', status: 'succeeded' };
		expect(extractEmailFromCheckout(checkout)).toBeNull();
	});

	it('should return null for non-string customerEmail', () => {
		const checkout = { customerEmail: 123, status: 'succeeded' };
		expect(extractEmailFromCheckout(checkout)).toBeNull();
	});

	it('should handle undefined checkout gracefully', () => {
		expect(extractEmailFromCheckout(undefined)).toBeNull();
	});

	it('should handle null checkout gracefully', () => {
		expect(extractEmailFromCheckout(null)).toBeNull();
	});
});
