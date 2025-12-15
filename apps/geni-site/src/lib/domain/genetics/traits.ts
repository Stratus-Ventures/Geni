import type { TraitDefinition, InsightResult, GenotypeMap } from './types';
import { getGenotype } from './parser';

const lactoseTolerance: TraitDefinition = {
	id: 'lactose-tolerance',
	title: 'Lactose Tolerance',
	rsid: 'rs4988235',
	category: 'diet',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'AA' || g === 'AG') {
			return {
				summary:
					'You likely produce lactase throughout adulthood, meaning you can digest dairy without issues.',
				confidence: 'high',
				confidenceNote: 'Well-studied genetic variant with strong scientific evidence.',
				whatToDoNext: [
					'Dairy products should be well-tolerated',
					'No genetic reason to avoid milk, cheese, or yogurt'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'GG') {
			return {
				summary:
					'You likely stop producing lactase after childhood. Dairy may cause digestive discomfort.',
				confidence: 'high',
				confidenceNote: 'Well-established genetic association.',
				whatToDoNext: [
					'Consider lactose-free alternatives',
					'Lactase supplements can help',
					'Fermented dairy may be better tolerated'
				],
				riskLevel: 'high'
			};
		}
		return null;
	}
};

const caffeineMetabolism: TraitDefinition = {
	id: 'caffeine-metabolism',
	title: 'Caffeine Metabolism',
	rsid: 'rs762551',
	category: 'diet',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'AA') {
			return {
				summary: 'You are a fast caffeine metabolizer. Coffee is cleared quickly from your system.',
				confidence: 'high',
				confidenceNote: 'CYP1A2 gene variant well-studied for caffeine processing.',
				whatToDoNext: [
					'Moderate coffee intake generally safe',
					'Less likely to experience jitters',
					'May need more caffeine for alertness effect'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AC' || g === 'CC') {
			return {
				summary: 'You are a slow caffeine metabolizer. Caffeine stays in your system longer.',
				confidence: 'high',
				confidenceNote: 'Strong evidence linking this variant to slower caffeine clearance.',
				whatToDoNext: [
					'Limit caffeine intake, especially afternoon',
					'May experience more anxiety from coffee',
					'Consider switching to half-caf or decaf'
				],
				riskLevel: g === 'CC' ? 'high' : 'low'
			};
		}
		return null;
	}
};

const alcoholFlush: TraitDefinition = {
	id: 'alcohol-flush',
	title: 'Alcohol Flush Reaction',
	rsid: 'rs671',
	category: 'diet',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'You have normal ALDH2 enzyme activity. Less likely to experience alcohol flush.',
				confidence: 'high',
				confidenceNote: 'ALDH2 variant well-documented in alcohol metabolism.',
				whatToDoNext: [
					'Normal alcohol metabolism expected',
					'Standard drinking guidelines apply',
					'Stay mindful of overall alcohol consumption'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG' || g === 'AA') {
			return {
				summary:
					'You may experience facial flushing when drinking alcohol due to reduced ALDH2 activity.',
				confidence: 'high',
				confidenceNote: 'Strong genetic association with Asian flush reaction.',
				whatToDoNext: [
					'Limit alcohol consumption',
					'Increased risk of esophageal issues with heavy drinking',
					'Consider avoiding alcohol or drinking very moderately'
				],
				riskLevel: g === 'AA' ? 'high' : 'low'
			};
		}
		return null;
	}
};

const bitterTaste: TraitDefinition = {
	id: 'bitter-taste',
	title: 'Bitter Taste Sensitivity',
	rsid: 'rs713598',
	category: 'diet',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'CC') {
			return {
				summary: 'You are likely a "supertaster" who perceives bitter compounds strongly.',
				confidence: 'moderate',
				confidenceNote: 'TAS2R38 gene influences perception of bitter compounds like PTC.',
				whatToDoNext: [
					'May dislike cruciferous vegetables naturally',
					'Try cooking methods that reduce bitterness',
					'Dark leafy greens may taste very bitter'
				],
				riskLevel: 'low'
			};
		}
		if (g === 'CG' || g === 'GG') {
			return {
				summary: 'You have reduced bitter taste sensitivity compared to supertasters.',
				confidence: 'moderate',
				confidenceNote: 'Associated with lower sensitivity to bitter compounds.',
				whatToDoNext: [
					'Bitter vegetables may be more palatable',
					'Can enjoy a wider range of foods',
					'May need to be mindful of sugar intake'
				],
				riskLevel: 'optimal'
			};
		}
		return null;
	}
};

const vitaminB12: TraitDefinition = {
	id: 'vitamin-b12',
	title: 'Vitamin B12 Absorption',
	rsid: 'rs602662',
	category: 'diet',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'You likely have optimal B12 absorption from dietary sources.',
				confidence: 'moderate',
				confidenceNote: 'FUT2 gene affects B12 bioavailability.',
				whatToDoNext: [
					'Standard B12 intake should be sufficient',
					'Good sources: meat, fish, dairy, eggs',
					'Annual blood test can confirm levels'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG' || g === 'AA') {
			return {
				summary: 'You may have reduced B12 absorption from food sources.',
				confidence: 'moderate',
				confidenceNote: 'Associated with lower circulating B12 levels.',
				whatToDoNext: [
					'Consider B12 supplementation',
					'Get B12 levels tested regularly',
					'Sublingual or injectable B12 may be more effective'
				],
				riskLevel: g === 'AA' ? 'high' : 'low'
			};
		}
		return null;
	}
};

const muscleFiberType: TraitDefinition = {
	id: 'muscle-fiber-type',
	title: 'Muscle Fiber Type',
	rsid: 'rs1815739',
	category: 'fitness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'CC') {
			return {
				summary:
					'You likely have more fast-twitch muscle fibers, favoring power and sprint activities.',
				confidence: 'high',
				confidenceNote: 'ACTN3 R577X variant well-studied in athletic performance.',
				whatToDoNext: [
					'May excel at sprinting, jumping, weightlifting',
					'Focus on explosive power training',
					'Recovery between intense sets important'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'CT') {
			return {
				summary: 'You have a mix of muscle fiber types, suited for varied activities.',
				confidence: 'high',
				confidenceNote: 'Intermediate ACTN3 expression.',
				whatToDoNext: [
					'Versatile athletic potential',
					'Can train for both power and endurance',
					'Listen to your body for best results'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'TT') {
			return {
				summary: 'You likely have more slow-twitch fibers, favoring endurance activities.',
				confidence: 'high',
				confidenceNote: 'No functional alpha-actinin-3 protein.',
				whatToDoNext: [
					'May excel at running, cycling, swimming',
					'Endurance training may feel natural',
					'Building explosive power may require more focus'
				],
				riskLevel: 'optimal'
			};
		}
		return null;
	}
};

const endurancePotential: TraitDefinition = {
	id: 'endurance-potential',
	title: 'Endurance vs Power',
	rsid: 'rs7181866',
	category: 'fitness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'Your genetics suggest a tendency toward endurance-type performance.',
				confidence: 'moderate',
				confidenceNote: 'Associated with aerobic capacity markers.',
				whatToDoNext: [
					'Consider endurance sports like running or cycling',
					'Zone 2 training may be especially effective',
					'Build aerobic base before intensity'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG' || g === 'AA') {
			return {
				summary: 'Your genetics suggest balanced or power-leaning athletic traits.',
				confidence: 'moderate',
				confidenceNote: 'Mixed endurance/power genetic indicators.',
				whatToDoNext: [
					'Experiment with different training styles',
					'HIIT may work well for you',
					'Strength training can complement cardio'
				],
				riskLevel: 'optimal'
			};
		}
		return null;
	}
};

const recoverySpeed: TraitDefinition = {
	id: 'recovery-speed',
	title: 'Recovery Speed',
	rsid: 'rs1800012',
	category: 'fitness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'You likely have standard collagen production supporting normal recovery.',
				confidence: 'moderate',
				confidenceNote: 'COL1A1 gene affects connective tissue.',
				whatToDoNext: [
					'Standard rest between workouts sufficient',
					'Include mobility work in routine',
					'Adequate protein supports recovery'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG' || g === 'AA') {
			return {
				summary:
					'You may have slightly reduced collagen integrity, potentially affecting recovery.',
				confidence: 'moderate',
				confidenceNote: 'Associated with altered collagen structure.',
				whatToDoNext: [
					'Prioritize adequate rest between intense sessions',
					'Consider collagen supplementation',
					'Focus on proper warm-up and cool-down'
				],
				riskLevel: 'low'
			};
		}
		return null;
	}
};

const achillesInjuryRisk: TraitDefinition = {
	id: 'achilles-injury-risk',
	title: 'Achilles Tendon Risk',
	rsid: 'rs679620',
	category: 'fitness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'AA') {
			return {
				summary: 'You have a lower genetic risk for Achilles tendon injuries.',
				confidence: 'moderate',
				confidenceNote: 'MMP3 gene variant linked to tendon injury risk.',
				whatToDoNext: [
					'Standard injury prevention applies',
					'Still warm up thoroughly',
					'Eccentric exercises good for tendon health'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG' || g === 'GG') {
			return {
				summary: 'You may have elevated risk for Achilles tendon issues.',
				confidence: 'moderate',
				confidenceNote: 'Associated with altered matrix metalloproteinase activity.',
				whatToDoNext: [
					'Extra attention to calf stretching',
					'Gradual training load increases',
					'Consider heel drops for tendon strengthening'
				],
				riskLevel: g === 'GG' ? 'high' : 'low'
			};
		}
		return null;
	}
};

const vitaminDLevels: TraitDefinition = {
	id: 'vitamin-d-levels',
	title: 'Vitamin D Levels',
	rsid: 'rs2282679',
	category: 'wellness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'You likely maintain healthy vitamin D levels more easily.',
				confidence: 'high',
				confidenceNote: 'GC gene variant affects vitamin D binding protein.',
				whatToDoNext: [
					'Standard sun exposure and diet may suffice',
					'Still get levels tested annually',
					'Fatty fish and fortified foods are good sources'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'GT' || g === 'TT') {
			return {
				summary: 'You may have lower baseline vitamin D levels and need more attention to intake.',
				confidence: 'high',
				confidenceNote: 'Strong association with lower circulating vitamin D.',
				whatToDoNext: [
					'Get vitamin D levels tested',
					'Consider D3 supplementation (2000-4000 IU daily)',
					'More sun exposure may be needed'
				],
				riskLevel: g === 'TT' ? 'high' : 'low'
			};
		}
		return null;
	}
};

const sleepDepth: TraitDefinition = {
	id: 'sleep-depth',
	title: 'Sleep Quality',
	rsid: 'rs1801260',
	category: 'wellness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'AA') {
			return {
				summary: 'You likely have a natural tendency for earlier sleep-wake cycles.',
				confidence: 'moderate',
				confidenceNote: 'CLOCK gene influences circadian rhythm.',
				whatToDoNext: [
					'Morning person tendencies likely',
					'Align schedule with natural rhythm',
					'Avoid late-night activities when possible'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG' || g === 'GG') {
			return {
				summary: 'You may naturally lean toward later sleep times (night owl tendency).',
				confidence: 'moderate',
				confidenceNote: 'Associated with evening chronotype.',
				whatToDoNext: [
					'May function better with later schedule',
					'Light exposure in morning helps reset rhythm',
					'Consistent sleep schedule important'
				],
				riskLevel: 'optimal'
			};
		}
		return null;
	}
};

const stressResponse: TraitDefinition = {
	id: 'stress-response',
	title: 'Stress Response',
	rsid: 'rs6265',
	category: 'wellness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'CC') {
			return {
				summary: 'You have the Val/Val variant associated with better stress resilience.',
				confidence: 'moderate',
				confidenceNote: 'BDNF Val66Met variant affects brain plasticity.',
				whatToDoNext: [
					'Generally good stress coping',
					'Exercise enhances BDNF benefits',
					'Maintain healthy lifestyle for best function'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'CT' || g === 'TT') {
			return {
				summary: 'You carry the Met variant, which may affect stress resilience and memory.',
				confidence: 'moderate',
				confidenceNote: 'Associated with altered BDNF secretion.',
				whatToDoNext: [
					'Prioritize stress management techniques',
					'Regular exercise especially important',
					'Meditation and mindfulness beneficial'
				],
				riskLevel: g === 'TT' ? 'low' : 'optimal'
			};
		}
		return null;
	}
};

const inflammationTendency: TraitDefinition = {
	id: 'inflammation-tendency',
	title: 'Inflammation Response',
	rsid: 'rs1800795',
	category: 'wellness',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'You may have higher baseline IL-6 levels, associated with more inflammation.',
				confidence: 'moderate',
				confidenceNote: 'IL-6 gene promoter variant affects cytokine production.',
				whatToDoNext: [
					'Anti-inflammatory diet beneficial',
					'Regular exercise reduces inflammation',
					'Omega-3s and turmeric may help'
				],
				riskLevel: 'high'
			};
		}
		if (g === 'CG' || g === 'CC') {
			return {
				summary: 'You likely have lower baseline inflammatory markers.',
				confidence: 'moderate',
				confidenceNote: 'Associated with reduced IL-6 production.',
				whatToDoNext: [
					'Lower genetic inflammation risk',
					'Still maintain anti-inflammatory habits',
					'Regular exercise and good sleep help'
				],
				riskLevel: 'optimal'
			};
		}
		return null;
	}
};

const omega3Conversion: TraitDefinition = {
	id: 'omega3-conversion',
	title: 'Omega-3 Conversion',
	rsid: 'rs174546',
	category: 'supplements',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'CC') {
			return {
				summary: 'You efficiently convert plant omega-3s (ALA) to active forms (EPA/DHA).',
				confidence: 'high',
				confidenceNote: 'FADS1 gene variant affects fatty acid desaturase activity.',
				whatToDoNext: [
					'Plant sources like flax and chia beneficial',
					'Fish oil still helpful but less critical',
					'Balanced omega-6 to omega-3 ratio important'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'CT' || g === 'TT') {
			return {
				summary: 'You have reduced ability to convert plant omega-3s to EPA/DHA.',
				confidence: 'high',
				confidenceNote: 'Lower desaturase enzyme activity.',
				whatToDoNext: [
					'Direct EPA/DHA sources important (fish, algae oil)',
					'Consider fish oil or algae-based supplements',
					'Plant omega-3s alone may not suffice'
				],
				riskLevel: g === 'TT' ? 'high' : 'low'
			};
		}
		return null;
	}
};

const folateMetabolism: TraitDefinition = {
	id: 'folate-metabolism',
	title: 'Folate Metabolism (MTHFR)',
	rsid: 'rs1801133',
	category: 'supplements',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'You have normal MTHFR enzyme function for folate processing.',
				confidence: 'high',
				confidenceNote: 'C677T variant well-studied for methylation.',
				whatToDoNext: [
					'Standard folic acid supplementation works',
					'Leafy greens provide good folate',
					'No special methylfolate needed'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG') {
			return {
				summary: 'You have one copy of reduced MTHFR function (about 65% activity).',
				confidence: 'high',
				confidenceNote: 'Heterozygous C677T.',
				whatToDoNext: [
					'Methylfolate may be more effective than folic acid',
					'Eat plenty of leafy greens',
					'B12 also important for methylation'
				],
				riskLevel: 'low'
			};
		}
		if (g === 'AA') {
			return {
				summary: 'You have two copies of reduced MTHFR function (about 30% activity).',
				confidence: 'high',
				confidenceNote: 'Homozygous C677T significantly reduces enzyme activity.',
				whatToDoNext: [
					'Methylfolate (5-MTHF) recommended over folic acid',
					'Consider B-complex with active forms',
					'Get homocysteine levels tested'
				],
				riskLevel: 'high'
			};
		}
		return null;
	}
};

const ironAbsorption: TraitDefinition = {
	id: 'iron-absorption',
	title: 'Iron Absorption (HFE)',
	rsid: 'rs1800562',
	category: 'supplements',
	interpret: (genotype: string) => {
		const g = genotype.toUpperCase();
		if (g === 'GG') {
			return {
				summary: 'You have normal iron absorption regulation.',
				confidence: 'high',
				confidenceNote: 'HFE C282Y variant affects iron homeostasis.',
				whatToDoNext: [
					'Standard iron intake guidelines apply',
					'Get ferritin tested if fatigued',
					'Vitamin C enhances iron absorption'
				],
				riskLevel: 'optimal'
			};
		}
		if (g === 'AG') {
			return {
				summary: 'You carry one copy of a variant associated with increased iron absorption.',
				confidence: 'high',
				confidenceNote: 'Heterozygous for hemochromatosis variant.',
				whatToDoNext: [
					'Monitor ferritin levels periodically',
					'Avoid unnecessary iron supplements',
					'Blood donation can help if levels high'
				],
				riskLevel: 'low'
			};
		}
		if (g === 'AA') {
			return {
				summary: 'You have genetic risk for iron overload (hereditary hemochromatosis).',
				confidence: 'high',
				confidenceNote: 'Homozygous C282Y strongly associated with iron overload.',
				whatToDoNext: [
					'Get iron panel and ferritin tested',
					'Avoid iron supplements and fortified foods',
					'Regular blood donation often recommended',
					'Consult with doctor about monitoring'
				],
				riskLevel: 'high'
			};
		}
		return null;
	}
};

export const TRAITS: Record<string, TraitDefinition> = {
	'lactose-tolerance': lactoseTolerance,
	'caffeine-metabolism': caffeineMetabolism,
	'alcohol-flush': alcoholFlush,
	'bitter-taste': bitterTaste,
	'vitamin-b12': vitaminB12,
	'muscle-fiber-type': muscleFiberType,
	'endurance-potential': endurancePotential,
	'recovery-speed': recoverySpeed,
	'achilles-injury-risk': achillesInjuryRisk,
	'vitamin-d-levels': vitaminDLevels,
	'sleep-depth': sleepDepth,
	'stress-response': stressResponse,
	'inflammation-tendency': inflammationTendency,
	'omega3-conversion': omega3Conversion,
	'folate-metabolism': folateMetabolism,
	'iron-absorption': ironAbsorption
};

export const TRAIT_ORDER = [
	'lactose-tolerance',
	'caffeine-metabolism',
	'alcohol-flush',
	'bitter-taste',
	'vitamin-b12',
	'muscle-fiber-type',
	'endurance-potential',
	'recovery-speed',
	'achilles-injury-risk',
	'vitamin-d-levels',
	'sleep-depth',
	'stress-response',
	'inflammation-tendency',
	'omega3-conversion',
	'folate-metabolism',
	'iron-absorption'
];

export function analyzeTrait(genotypeMap: GenotypeMap, traitId: string): InsightResult | null {
	const trait = TRAITS[traitId];
	if (!trait) return null;

	const genotype = getGenotype(genotypeMap, trait.rsid);
	if (!genotype) return null;

	const interpretation = trait.interpret(genotype);
	if (!interpretation) return null;

	return {
		traitId: trait.id,
		title: trait.title,
		genotype,
		category: trait.category,
		...interpretation
	};
}

export function analyzeAllTraits(genotypeMap: GenotypeMap): InsightResult[] {
	const results: InsightResult[] = [];
	for (const traitId of TRAIT_ORDER) {
		const result = analyzeTrait(genotypeMap, traitId);
		if (result) results.push(result);
	}
	return results;
}

export function getTraitsByCategory(insights: InsightResult[]): Record<string, InsightResult[]> {
	const grouped: Record<string, InsightResult[]> = {
		diet: [],
		fitness: [],
		wellness: [],
		supplements: []
	};
	for (const insight of insights) {
		grouped[insight.category].push(insight);
	}
	return grouped;
}

export function getAvailableTraits(): string[] {
	return TRAIT_ORDER;
}

export function getTraitDefinition(traitId: string): TraitDefinition | undefined {
	return TRAITS[traitId];
}

export function getAllRsids(): string[] {
	return Object.values(TRAITS).map((t) => t.rsid);
}
