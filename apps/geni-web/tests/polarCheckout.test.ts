import { afterEach, describe, expect, it, vi } from 'vitest';

const routeModule = '../src/routes/checkout/test/+server';

let mockedEnv: Record<string, string | undefined> = {};

vi.mock('$env/dynamic/private', () => ({
	get env() {
		return mockedEnv;
	}
}));

const loadHandler = async () => {
	const { GET } = await import(routeModule);
	return GET;
};

describe('checkout test route', () => {
	afterEach(() => {
		mockedEnv = {};
		vi.resetModules();
	});

	it('redirects to checkout with the configured product id', async () => {
		mockedEnv = { POLAR_PRODUCT_ID: 'prod_test_123' };
		const GET = await loadHandler();

		try {
			GET();
		} catch (err) {
			expect(err).toMatchObject({
				status: 302,
				location: '/checkout?products=prod_test_123'
			});
			return;
		}

		throw new Error('Expected redirect to be thrown');
	});

	it('returns a server error when the product id is missing', async () => {
		mockedEnv = {};
		const GET = await loadHandler();

		try {
			GET();
		} catch (err) {
			expect(err).toMatchObject({ status: 500 });
			return;
		}

		throw new Error('Expected error to be thrown');
	});
});
