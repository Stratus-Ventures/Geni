/**
 * 23andMe File Parser
 * Parses raw genetic data files from 23andMe into a normalized format
 */

import type { SNPData, GenotypeMap, ParseResult, ParseDebugInfo } from './types';

/** Target SNPs we're looking for (used for debug output) */
const TARGET_SNPS = ['rs4988235']; // Lactose tolerance

/**
 * Parse a 23andMe raw data file into a GenotypeMap
 *
 * 23andMe format is tab-delimited with columns:
 * rsid \t chromosome \t position \t genotype
 *
 * Lines starting with # are comments/headers
 * Genotype may be -- for no-calls
 */
export function parse23andMe(fileContent: string): ParseResult {
	const data: GenotypeMap = new Map();
	const warnings: string[] = [];
	let is23andMe = false;

	// Debug collection
	const sampleLines: string[] = [];
	const sampleSNPs: string[] = [];
	const targetSNPsStatus: ParseDebugInfo['targetSNPsStatus'] = {};
	let skippedLines = 0;

	// Initialize target SNPs status
	for (const snp of TARGET_SNPS) {
		targetSNPsStatus[snp] = { found: false, genotype: null, line: null };
	}

	const lines = fileContent.split(/\r?\n/);
	const totalLines = lines.length;

	for (let i = 0; i < lines.length; i++) {
		const line = lines[i].trim();

		// Skip empty lines
		if (!line) {
			skippedLines++;
			continue;
		}

		// Check for 23andMe header signature
		if (line.includes('23andMe')) {
			is23andMe = true;
			skippedLines++;
			continue;
		}

		// Skip comment lines
		if (line.startsWith('#')) {
			skippedLines++;
			continue;
		}

		// Collect sample lines (first 5 data lines)
		if (sampleLines.length < 5) {
			sampleLines.push(line);
		}

		// Parse data line
		const parts = line.split('\t');

		if (parts.length < 4) {
			// Try space-separated as fallback
			const spaceParts = line.split(/\s+/);
			if (spaceParts.length >= 4) {
				parts.length = 0;
				parts.push(...spaceParts);
			} else {
				// Skip malformed lines
				skippedLines++;
				continue;
			}
		}

		const [rsid, chromosome, positionStr, genotype] = parts;

		// Validate rsid format (should start with rs or i, case-insensitive)
		const rsidLower = rsid?.toLowerCase() || '';
		if (!rsidLower.startsWith('rs') && !rsidLower.startsWith('i')) {
			skippedLines++;
			continue;
		}

		// Parse position
		const position = parseInt(positionStr, 10);
		if (isNaN(position)) {
			warnings.push(`Invalid position at line ${i + 1}: ${positionStr}`);
			skippedLines++;
			continue;
		}

		// Skip no-calls (--) but still count them
		if (genotype === '--' || !genotype) {
			skippedLines++;
			continue;
		}

		// Normalize genotype (uppercase, sorted alphabetically for consistency)
		const normalizedGenotype = normalizeGenotype(genotype);
		const normalizedRsid = rsid.toLowerCase();

		const snpData: SNPData = {
			rsid: normalizedRsid,
			chromosome: chromosome.toUpperCase(),
			position,
			genotype: normalizedGenotype
		};

		data.set(snpData.rsid, snpData);

		// Collect sample SNPs (first 10)
		if (sampleSNPs.length < 10) {
			sampleSNPs.push(normalizedRsid);
		}

		// Check if this is a target SNP
		if (TARGET_SNPS.includes(normalizedRsid)) {
			targetSNPsStatus[normalizedRsid] = {
				found: true,
				genotype: normalizedGenotype,
				line: i + 1
			};
		}
	}

	return {
		data,
		snpCount: data.size,
		warnings,
		format: is23andMe ? '23andme' : 'unknown',
		debug: {
			sampleLines,
			sampleSNPs,
			targetSNPsStatus,
			totalLines,
			skippedLines
		}
	};
}

/**
 * Normalize a genotype string for consistent comparison
 * - Uppercase
 * - Sort alleles alphabetically (so AG and GA both become AG)
 */
function normalizeGenotype(genotype: string): string {
	const upper = genotype.toUpperCase().trim();

	// Handle single nucleotide (homozygous shorthand in some files)
	if (upper.length === 1) {
		return upper + upper;
	}

	// Sort the two alleles alphabetically
	if (upper.length === 2) {
		const alleles = upper.split('').sort();
		return alleles.join('');
	}

	// Return as-is for unusual cases (insertions/deletions)
	return upper;
}

/**
 * Get a specific SNP from a GenotypeMap
 */
export function getSNP(map: GenotypeMap, rsid: string): SNPData | undefined {
	return map.get(rsid.toLowerCase());
}

/**
 * Check if a GenotypeMap contains a specific SNP
 */
export function hasSNP(map: GenotypeMap, rsid: string): boolean {
	return map.has(rsid.toLowerCase());
}

/**
 * Get the genotype for a specific rsID, or null if not found
 */
export function getGenotype(map: GenotypeMap, rsid: string): string | null {
	const snp = getSNP(map, rsid);
	return snp?.genotype ?? null;
}
