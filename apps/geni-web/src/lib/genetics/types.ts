/**
 * Genetics Module Types
 * Types for parsing and analyzing genetic data from 23andMe files
 */

/**
 * Raw SNP data extracted from a genetic testing file
 */
export interface SNPData {
	rsid: string;
	chromosome: string;
	position: number;
	genotype: string;
}

/**
 * Map of rsID to SNP data for quick lookups
 */
export type GenotypeMap = Map<string, SNPData>;

/**
 * Confidence level for genetic interpretations
 */
export type ConfidenceLevel = 'high' | 'moderate' | 'low';

/**
 * Result of analyzing a genetic trait
 */
export interface InsightResult {
	/** The trait being analyzed */
	traitId: string;
	/** Display title for the trait */
	title: string;
	/** The user's genotype for this trait's primary SNP */
	genotype: string;
	/** Human-readable summary of what the genotype means */
	summary: string;
	/** Confidence level of this interpretation */
	confidence: ConfidenceLevel;
	/** Explanation of why this confidence level was assigned */
	confidenceNote: string;
	/** Actionable recommendations based on the result */
	whatToDoNext: string[];
}

/**
 * Definition of a genetic trait and how to interpret it
 */
export interface TraitDefinition {
	/** Unique identifier for the trait */
	id: string;
	/** Display name */
	title: string;
	/** Primary SNP (rsID) for this trait */
	rsid: string;
	/** Category for grouping traits */
	category: 'diet' | 'fitness' | 'sleep' | 'wellness' | 'supplements';
	/** Function to interpret a genotype for this trait */
	interpret: (genotype: string) => Omit<InsightResult, 'traitId' | 'title' | 'genotype'> | null;
}

/**
 * Debug info for troubleshooting parsing issues
 */
export interface ParseDebugInfo {
	/** First few raw data lines from the file (for inspection) */
	sampleLines: string[];
	/** First few rsIDs that were successfully parsed */
	sampleSNPs: string[];
	/** Status of specific target SNPs we're looking for */
	targetSNPsStatus: Record<string, { found: boolean; genotype: string | null; line: number | null }>;
	/** Total lines in file */
	totalLines: number;
	/** Lines that were skipped (comments, empty, malformed) */
	skippedLines: number;
}

/**
 * Parser result including any warnings or errors
 */
export interface ParseResult {
	/** Parsed genotype data */
	data: GenotypeMap;
	/** Number of SNPs successfully parsed */
	snpCount: number;
	/** Any warnings encountered during parsing */
	warnings: string[];
	/** File format detected */
	format: '23andme' | 'unknown';
	/** Debug information for troubleshooting */
	debug: ParseDebugInfo;
}
