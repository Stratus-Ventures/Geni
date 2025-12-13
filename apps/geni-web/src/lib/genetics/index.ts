/**
 * Genetics Module
 * Client-side genetic data parsing and trait analysis
 */

// Types
export type {
	SNPData,
	GenotypeMap,
	ConfidenceLevel,
	InsightResult,
	TraitDefinition,
	ParseResult,
	ParseDebugInfo
} from './types';

// Parser functions
export { parse23andMe, getSNP, hasSNP, getGenotype } from './parser';

// Trait analysis functions
export {
	analyzeTrait,
	analyzeAllTraits,
	getAvailableTraits,
	getTraitDefinition,
	TRAITS
} from './traits';
