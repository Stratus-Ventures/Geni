export interface SNPData {
	rsid: string;
	chromosome: string;
	position: number;
	genotype: string;
}

export type GenotypeMap = Map<string, SNPData>;

export type ConfidenceLevel = 'high' | 'moderate' | 'low';

export type RiskLevel = 'low' | 'optimal' | 'high';

export type TraitCategory = 'diet' | 'fitness' | 'wellness' | 'supplements';

export interface InsightResult {
	traitId: string;
	title: string;
	genotype: string;
	summary: string;
	confidence: ConfidenceLevel;
	confidenceNote: string;
	whatToDoNext: string[];
	riskLevel: RiskLevel;
	category: TraitCategory;
}

export interface TraitDefinition {
	id: string;
	title: string;
	rsid: string;
	category: TraitCategory;
	interpret: (
		genotype: string
	) => Omit<InsightResult, 'traitId' | 'title' | 'genotype' | 'category'> | null;
}

export interface ParseDebugInfo {
	sampleLines: string[];
	sampleSNPs: string[];
	targetSNPsStatus: Record<
		string,
		{ found: boolean; genotype: string | null; line: number | null }
	>;
	totalLines: number;
	skippedLines: number;
}

export type FileFormat = '23andme' | 'ancestry' | 'unknown';

export interface ParseResult {
	data: GenotypeMap;
	snpCount: number;
	warnings: string[];
	format: FileFormat;
	debug: ParseDebugInfo;
}

export interface SerializedGenotype {
	rsid: string;
	genotype: string;
}

export interface ReportData {
	genotypes: SerializedGenotype[];
	insights: InsightResult[];
	snpCount: number;
	format: FileFormat;
	createdAt: string;
}
