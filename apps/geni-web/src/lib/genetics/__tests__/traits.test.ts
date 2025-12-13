import { describe, it, expect } from 'vitest';
import {
	analyzeTrait,
	analyzeAllTraits,
	getAvailableTraits,
	getTraitDefinition,
	TRAITS
} from '../traits';
import { parse23andMe } from '../parser';
import type { GenotypeMap } from '../types';

// Helper to create a GenotypeMap with specific SNPs
function createGenotypeMap(snps: Array<{ rsid: string; genotype: string }>): GenotypeMap {
	const lines = snps
		.map((snp, i) => `${snp.rsid}\t1\t${i * 1000 + 1000}\t${snp.genotype}`)
		.join('\n');
	return parse23andMe(lines).data;
}

describe('TRAITS registry', () => {
	it('should have lactose-tolerance trait defined', () => {
		expect(TRAITS['lactose-tolerance']).toBeDefined();
	});

	it('should have correct rsID for lactose tolerance', () => {
		expect(TRAITS['lactose-tolerance'].rsid).toBe('rs4988235');
	});

	it('should have all required fields for each trait', () => {
		for (const [id, trait] of Object.entries(TRAITS)) {
			expect(trait.id).toBe(id);
			expect(trait.title).toBeTruthy();
			expect(trait.rsid).toBeTruthy();
			expect(trait.category).toBeTruthy();
			expect(typeof trait.interpret).toBe('function');
		}
	});
});

describe('analyzeTrait', () => {
	it('should return InsightResult for valid genotype', () => {
		const map = createGenotypeMap([{ rsid: 'rs4988235', genotype: 'TT' }]);
		const result = analyzeTrait(map, 'lactose-tolerance');

		expect(result).not.toBeNull();
		expect(result?.traitId).toBe('lactose-tolerance');
		expect(result?.title).toBe('Lactose Tolerance');
		expect(result?.genotype).toBe('TT');
		expect(result?.summary).toBeTruthy();
		expect(result?.confidence).toBeTruthy();
		expect(result?.whatToDoNext).toBeInstanceOf(Array);
	});

	it('should return null for missing SNP in map', () => {
		const map = createGenotypeMap([{ rsid: 'rs1234567', genotype: 'AA' }]);
		const result = analyzeTrait(map, 'lactose-tolerance');
		expect(result).toBeNull();
	});

	it('should return null for unknown trait ID', () => {
		const map = createGenotypeMap([{ rsid: 'rs4988235', genotype: 'TT' }]);
		const result = analyzeTrait(map, 'nonexistent-trait');
		expect(result).toBeNull();
	});
});

describe('lactose tolerance trait', () => {
	const trait = TRAITS['lactose-tolerance'];

	it('should interpret TT as lactose tolerant', () => {
		const result = trait.interpret('TT');
		expect(result).not.toBeNull();
		expect(result?.summary).toContain('lactase');
		expect(result?.confidence).toBe('high');
	});

	it('should interpret CT as lactose tolerant (partial)', () => {
		const result = trait.interpret('CT');
		expect(result).not.toBeNull();
		expect(result?.summary).toContain('lactase');
		expect(result?.confidence).toBe('high');
	});

	it('should interpret TC as lactose tolerant (partial) - normalized', () => {
		// TC should work same as CT due to normalization
		const result = trait.interpret('TC');
		expect(result).not.toBeNull();
	});

	it('should interpret CC as lactose intolerant', () => {
		const result = trait.interpret('CC');
		expect(result).not.toBeNull();
		expect(result?.summary).toContain('stop producing lactase');
		expect(result?.confidence).toBe('high');
	});

	it('should return null for unknown genotype', () => {
		const result = trait.interpret('XX');
		expect(result).toBeNull();
	});

	it('should handle lowercase genotype', () => {
		const result = trait.interpret('tt');
		expect(result).not.toBeNull();
	});

	it('should provide actionable recommendations', () => {
		const resultTT = trait.interpret('TT');
		const resultCC = trait.interpret('CC');

		expect(resultTT?.whatToDoNext.length).toBeGreaterThan(0);
		expect(resultCC?.whatToDoNext.length).toBeGreaterThan(0);

		// CC (intolerant) should have more/different recommendations
		expect(resultCC?.whatToDoNext.some((r) => r.toLowerCase().includes('lactose-free'))).toBe(
			true
		);
	});
});

describe('analyzeAllTraits', () => {
	it('should return array of results for all found traits', () => {
		const map = createGenotypeMap([{ rsid: 'rs4988235', genotype: 'TT' }]);
		const results = analyzeAllTraits(map);

		expect(results).toBeInstanceOf(Array);
		expect(results.length).toBe(1);
		expect(results[0].traitId).toBe('lactose-tolerance');
	});

	it('should return empty array when no traits can be analyzed', () => {
		const map = createGenotypeMap([{ rsid: 'rs1234567', genotype: 'AA' }]);
		const results = analyzeAllTraits(map);

		expect(results).toBeInstanceOf(Array);
		expect(results.length).toBe(0);
	});

	it('should return empty array for empty genotype map', () => {
		const map: GenotypeMap = new Map();
		const results = analyzeAllTraits(map);

		expect(results).toBeInstanceOf(Array);
		expect(results.length).toBe(0);
	});
});

describe('getAvailableTraits', () => {
	it('should return array of trait IDs', () => {
		const traits = getAvailableTraits();
		expect(traits).toBeInstanceOf(Array);
		expect(traits).toContain('lactose-tolerance');
	});

	it('should match TRAITS registry keys', () => {
		const traits = getAvailableTraits();
		expect(traits).toEqual(Object.keys(TRAITS));
	});
});

describe('getTraitDefinition', () => {
	it('should return trait definition for valid ID', () => {
		const trait = getTraitDefinition('lactose-tolerance');
		expect(trait).toBeDefined();
		expect(trait?.id).toBe('lactose-tolerance');
	});

	it('should return undefined for invalid ID', () => {
		const trait = getTraitDefinition('nonexistent');
		expect(trait).toBeUndefined();
	});
});
