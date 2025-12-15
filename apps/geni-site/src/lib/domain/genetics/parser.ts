import type { SNPData, GenotypeMap, ParseResult, ParseDebugInfo, FileFormat } from './types';

const TARGET_SNPS = [
	'rs4988235',
	'rs762551',
	'rs671',
	'rs713598',
	'rs602662',
	'rs1815739',
	'rs7181866',
	'rs1800012',
	'rs679620',
	'rs2282679',
	'rs1801260',
	'rs6265',
	'rs1800795',
	'rs174546',
	'rs1801133',
	'rs1800562'
];

function normalizeGenotype(genotype: string): string {
	const upper = genotype.toUpperCase().trim();
	if (upper.length === 1) return upper + upper;
	if (upper.length === 2) return upper.split('').sort().join('');
	return upper;
}

function detectFormat(lines: string[]): FileFormat {
	for (const line of lines.slice(0, 50)) {
		if (line.includes('23andMe')) return '23andme';
		if (line.includes('AncestryDNA')) return 'ancestry';
	}
	return 'unknown';
}

function parseLine(line: string, _lineIndex: number): SNPData | null {
	if (!line || line.startsWith('#')) return null;

	let parts = line.split('\t');
	if (parts.length < 4) {
		parts = line.split(/\s+/);
		if (parts.length < 4) return null;
	}

	const [rsid, chromosome, positionStr, genotype] = parts;
	const rsidLower = rsid?.toLowerCase() || '';

	if (!rsidLower.startsWith('rs') && !rsidLower.startsWith('i')) return null;

	const position = parseInt(positionStr, 10);
	if (isNaN(position)) return null;

	if (genotype === '--' || genotype === '00' || !genotype) return null;

	return {
		rsid: rsidLower,
		chromosome: chromosome.toUpperCase(),
		position,
		genotype: normalizeGenotype(genotype)
	};
}

export function parseAncestry(fileContent: string): ParseResult {
	const data: GenotypeMap = new Map();
	const warnings: string[] = [];
	const lines = fileContent.split(/\r?\n/);

	const sampleLines: string[] = [];
	const sampleSNPs: string[] = [];
	const targetSNPsStatus: ParseDebugInfo['targetSNPsStatus'] = {};
	let skippedLines = 0;

	for (const snp of TARGET_SNPS) {
		targetSNPsStatus[snp] = { found: false, genotype: null, line: null };
	}

	for (let i = 0; i < lines.length; i++) {
		const line = lines[i].trim();

		if (!line || line.startsWith('#')) {
			skippedLines++;
			continue;
		}

		if (sampleLines.length < 5) sampleLines.push(line);

		const snpData = parseLine(line, i);
		if (!snpData) {
			skippedLines++;
			continue;
		}

		data.set(snpData.rsid, snpData);

		if (sampleSNPs.length < 10) sampleSNPs.push(snpData.rsid);

		if (TARGET_SNPS.includes(snpData.rsid)) {
			targetSNPsStatus[snpData.rsid] = {
				found: true,
				genotype: snpData.genotype,
				line: i + 1
			};
		}
	}

	return {
		data,
		snpCount: data.size,
		warnings,
		format: 'ancestry',
		debug: { sampleLines, sampleSNPs, targetSNPsStatus, totalLines: lines.length, skippedLines }
	};
}

export function parse23andMe(fileContent: string): ParseResult {
	const data: GenotypeMap = new Map();
	const warnings: string[] = [];
	const lines = fileContent.split(/\r?\n/);

	const sampleLines: string[] = [];
	const sampleSNPs: string[] = [];
	const targetSNPsStatus: ParseDebugInfo['targetSNPsStatus'] = {};
	let skippedLines = 0;
	let is23andMe = false;

	for (const snp of TARGET_SNPS) {
		targetSNPsStatus[snp] = { found: false, genotype: null, line: null };
	}

	for (let i = 0; i < lines.length; i++) {
		const line = lines[i].trim();

		if (!line) {
			skippedLines++;
			continue;
		}

		if (line.includes('23andMe')) {
			is23andMe = true;
			skippedLines++;
			continue;
		}

		if (line.startsWith('#')) {
			skippedLines++;
			continue;
		}

		if (sampleLines.length < 5) sampleLines.push(line);

		const snpData = parseLine(line, i);
		if (!snpData) {
			skippedLines++;
			continue;
		}

		data.set(snpData.rsid, snpData);

		if (sampleSNPs.length < 10) sampleSNPs.push(snpData.rsid);

		if (TARGET_SNPS.includes(snpData.rsid)) {
			targetSNPsStatus[snpData.rsid] = {
				found: true,
				genotype: snpData.genotype,
				line: i + 1
			};
		}
	}

	return {
		data,
		snpCount: data.size,
		warnings,
		format: is23andMe ? '23andme' : 'unknown',
		debug: { sampleLines, sampleSNPs, targetSNPsStatus, totalLines: lines.length, skippedLines }
	};
}

export function parseGeneticFile(fileContent: string): ParseResult {
	const lines = fileContent.split(/\r?\n/);
	const format = detectFormat(lines);

	if (format === 'ancestry') return parseAncestry(fileContent);
	return parse23andMe(fileContent);
}

export function getSNP(map: GenotypeMap, rsid: string): SNPData | undefined {
	return map.get(rsid.toLowerCase());
}

export function hasSNP(map: GenotypeMap, rsid: string): boolean {
	return map.has(rsid.toLowerCase());
}

export function getGenotype(map: GenotypeMap, rsid: string): string | null {
	const snp = getSNP(map, rsid);
	return snp?.genotype ?? null;
}

export function serializeGenotypes(
	map: GenotypeMap,
	rsids: string[]
): { rsid: string; genotype: string }[] {
	const result: { rsid: string; genotype: string }[] = [];
	for (const rsid of rsids) {
		const genotype = getGenotype(map, rsid);
		if (genotype) result.push({ rsid: rsid.toLowerCase(), genotype });
	}
	return result;
}

export function deserializeGenotypes(
	serialized: { rsid: string; genotype: string }[]
): GenotypeMap {
	const map: GenotypeMap = new Map();
	for (const { rsid, genotype } of serialized) {
		map.set(rsid, { rsid, chromosome: '', position: 0, genotype });
	}
	return map;
}
