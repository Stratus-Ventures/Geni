/**
 * Genetic Trait Definitions and Analysis
 * Maps SNPs to traits with interpretation logic
 */

import type { TraitDefinition, InsightResult, GenotypeMap } from './types';
import { getGenotype } from './parser';

/**
 * Lactose Tolerance Trait
 *
 * SNP: rs4988235 (LCT gene, -13910 C>T variant)
 *
 * This SNP is the primary determinant of lactase persistence in European populations.
 * The T allele is associated with continued lactase production into adulthood.
 *
 * Genotypes:
 * - TT: Lactase persistent (lactose tolerant)
 * - CT/TC: Lactase persistent (lactose tolerant, one copy)
 * - CC: Lactase non-persistent (likely lactose intolerant)
 */
const lactoseTolerance: TraitDefinition = {
	id: 'lactose-tolerance',
	title: 'Lactose Tolerance',
	rsid: 'rs4988235',
	category: 'diet',
	interpret: (genotype: string) => {
		// Normalize for comparison (genotype is already uppercase and sorted)
		const g = genotype.toUpperCase();

		if (g === 'AG') {
			return {
				summary:
					'You likely produce lactase throughout adulthood, meaning you can digest lactose (milk sugar) without issues. You have two copies of the lactase persistence variant.',
				confidence: 'high',
				confidenceNote:
					'This is a well-studied genetic variant with strong scientific evidence linking it to lactose tolerance in adults.',
				whatToDoNext: [
					'Dairy products should be well-tolerated in your diet',
					'No genetic reason to avoid milk, cheese, or yogurt',
					'If you experience digestive issues with dairy, consult a healthcare provider as other factors may be involved'
				]
			};
		}

		if (g === 'CT' || g === 'TC') {
			return {
				summary:
					'You likely produce some lactase into adulthood. With one copy of the lactase persistence variant, you may digest moderate amounts of lactose, though tolerance can vary.',
				confidence: 'high',
				confidenceNote:
					'This variant is well-studied. Having one copy typically provides sufficient lactase for moderate dairy consumption.',
				whatToDoNext: [
					'Most dairy products should be tolerable in moderate amounts',
					'Pay attention to how your body responds to larger servings of dairy',
					'Fermented dairy (yogurt, aged cheese) may be easier to digest',
					'Consider lactase supplements if you notice discomfort with large portions'
				]
			};
		}

		if (g === 'CC') {
			return {
				summary:
					'You likely stop producing lactase after childhood, which is common in most of the world population. This means dairy products containing lactose may cause digestive discomfort.',
				confidence: 'high',
				confidenceNote:
					'This is a well-established genetic association. However, individual tolerance can vary based on gut bacteria and other factors.',
				whatToDoNext: [
					'Consider lactose-free dairy products or plant-based alternatives',
					'Fermented dairy (yogurt, kefir, aged cheeses) contains less lactose and may be better tolerated',
					'Lactase enzyme supplements can help when consuming dairy',
					'Small amounts of dairy spread throughout the day may be tolerable',
					'Consult a dietitian if you want to include dairy in your diet'
				]
			};
		}

		// Unknown genotype
		return null;
	}
};

/**
 * Registry of all available traits
 */
export const TRAITS: Record<string, TraitDefinition> = {
	'lactose-tolerance': lactoseTolerance
};

/**
 * Analyze a single trait from a genotype map
 *
 * @param genotypeMap - Parsed genotype data
 * @param traitId - ID of the trait to analyze
 * @returns InsightResult if the trait can be analyzed, null otherwise
 */
export function analyzeTrait(genotypeMap: GenotypeMap, traitId: string): InsightResult | null {
	const trait = TRAITS[traitId];
	if (!trait) {
		return null;
	}

	const genotype = getGenotype(genotypeMap, trait.rsid);
	if (!genotype) {
		return null;
	}

	const interpretation = trait.interpret(genotype);
	if (!interpretation) {
		return null;
	}

	return {
		traitId: trait.id,
		title: trait.title,
		genotype,
		...interpretation
	};
}

/**
 * Analyze all available traits from a genotype map
 *
 * @param genotypeMap - Parsed genotype data
 * @returns Array of InsightResults for all traits that could be analyzed
 */
export function analyzeAllTraits(genotypeMap: GenotypeMap): InsightResult[] {
	const results: InsightResult[] = [];

	for (const traitId of Object.keys(TRAITS)) {
		const result = analyzeTrait(genotypeMap, traitId);
		if (result) {
			results.push(result);
		}
	}

	return results;
}

/**
 * Get a list of all available trait IDs
 */
export function getAvailableTraits(): string[] {
	return Object.keys(TRAITS);
}

/**
 * Get trait definition by ID
 */
export function getTraitDefinition(traitId: string): TraitDefinition | undefined {
	return TRAITS[traitId];
}
