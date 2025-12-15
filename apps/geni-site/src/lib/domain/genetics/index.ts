export type {
	SNPData,
	GenotypeMap,
	ConfidenceLevel,
	RiskLevel,
	TraitCategory,
	InsightResult,
	TraitDefinition,
	ParseDebugInfo,
	FileFormat,
	ParseResult,
	SerializedGenotype,
	ReportData
} from './types';

export {
	parse23andMe,
	parseAncestry,
	parseGeneticFile,
	getSNP,
	hasSNP,
	getGenotype,
	serializeGenotypes,
	deserializeGenotypes
} from './parser';

export {
	TRAITS,
	TRAIT_ORDER,
	analyzeTrait,
	analyzeAllTraits,
	getTraitsByCategory,
	getAvailableTraits,
	getTraitDefinition,
	getAllRsids
} from './traits';
