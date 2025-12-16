export function extractEmailFromCheckout(checkout: unknown): string | null {
	if (!checkout || typeof checkout !== 'object') {
		return null;
	}
	const co = checkout as Record<string, unknown>;
	if (typeof co.customerEmail === 'string' && co.customerEmail) {
		return co.customerEmail;
	}
	const customer = co.customer as Record<string, unknown> | undefined;
	if (customer && typeof customer.email === 'string' && customer.email) {
		return customer.email;
	}
	return null;
}
