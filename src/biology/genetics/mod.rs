pub mod addiction_genetics;
pub mod african_variants;
pub mod allele;
pub mod ancestry;
pub mod asian_variants;
pub mod athletic_performance;
pub mod autoimmune;
pub mod blood_type;
pub mod cancer_data_loader;
pub mod cancer_genetics;
pub mod cardiovascular_genetics;
pub mod chromosome;
pub mod comprehensive_risk_calculator;
pub mod dermatology;
pub mod dietary_genetics;
pub mod disease_association;
pub mod disease_susceptibility;
pub mod dna;
pub mod epistasis;
pub mod european_variants;
pub mod eye_genetics;
pub mod gene;
pub mod gene_catalog;
pub mod gene_environment;
pub mod gene_variants;
pub mod genome;
pub mod genotype;
pub mod hair_genetics;
pub mod hla_typing;
pub mod lactose_tolerance;
pub mod longevity_genetics;
pub mod markers;
pub mod mental_health_genetics;
pub mod metabolic_disease;
pub mod migraine_genetics;
pub mod mitochondrial;
pub mod native_american_variants;
pub mod neurological_genetics;
pub mod ophthalmology;
pub mod pain_genetics;
pub mod pharmacogenomics;
pub mod phenotype;
pub mod population;
pub mod population_traits;
pub mod respiratory_genetics;
pub mod skin_genetics;
pub mod sleep_genetics;
pub mod snp;
pub mod taste_smell;
pub mod trait_interactions;
pub mod variation;

// CRITICAL: Fix conflicts first
// Genotype: Use the main comprehensive struct from genotype.rs, alias the simple enum from population variants
pub use genotype::{Genotype, GenotypeRiskProfile, PhenotypeAssociation, AncestryComponent, DiseaseRisk, PharmacogeneticMarker, MetabolizerStatus, Severity};
pub use african_variants::{AfricanGeneticVariants, SickleCellStatus, G6PDStatus, LactasePersistence, SkinPigmentationGenotype, MalariaResistanceProfile, HypertensionRiskProfile, VitaminDMetabolism, APOL1Status};
pub use african_variants::Genotype as AfricanGenotype;
pub use native_american_variants::{NativeAmericanGeneticVariants, AlcoholMetabolismProfile, ALDH2Genotype, ADH1BGenotype, ADH1CGenotype, DiabetesRiskProfile, GallbladderRiskProfile, ACEInhibitorResponse, ACEResponse, APOEStatus};
pub use native_american_variants::Genotype as NativeAmericanGenotype;

pub use phenotype::{PhenotypeProfile, PhysicalTraits, MetabolicTraits, DiseaseSusceptibility, PharmacologicalTraits, EyeColor, HairColor, SkinPigmentation, EarwaxType, CaffeineMetabolism, AlcoholMetabolism, ALDH2Function, ADH1BVariant, VitaminDSynthesis, WarfarinSensitivity, OpioidMetabolism, SSRIResponse};

// HIGH PRIORITY: Core genetic types - resolve conflicts
// Allele: Use the comprehensive struct from allele.rs, alias the simple enum from gene_variants.rs
pub use allele::{Allele, AlleleType, AllelePair, Zygosity, CarrierStatus, AlleleFrequency};
pub use gene_variants::{GeneVariantCatalog, PharmacogeneticVariant, MetabolicVariant, DiseaseRiskVariant, TraitVariant, DrugResponse, DrugEffect, DosageAdjustment, PharmacogeneticPhenotype, MetabolicImpact, VariantClinicalSignificance, RiskAssessment, TraitEffect, EffectDirection};
pub use gene_variants::Allele as VariantAllele;
pub use gene_variants::Genotype as VariantGenotype;

// Gene: Use the comprehensive struct from gene.rs, alias the simple struct from genome.rs
pub use gene::{Gene, GeneType, Exon, Intron, Promoter, RegulatoryRegion, RegulatoryType};
pub use gene::Strand as GeneStrand;
pub use genome::{Genome, GenomicLocation, Variant};
pub use genome::Gene as GenomeGene;
pub use genome::Strand as GenomeStrand;

// Chromosome: Use the comprehensive enum from chromosome.rs (genome.rs should import this)
pub use chromosome::Chromosome;

// Nucleotide: Use the comprehensive version from dna.rs, alias the simple version from genome.rs
pub use dna::{Nucleotide, DNASequence};
pub use genome::Nucleotide as GenomeNucleotide;

// MEDIUM/LOW PRIORITY: Resolve conflicts using type aliases only, keep globs
// HLAAllele conflict: autoimmune vs hla_typing
pub use autoimmune::HLAAllele as AutoimmuneHLAAllele;
pub use hla_typing::HLAAllele as HLATypingAllele;

// RiskLevel conflict: blood_type vs mental_health_genetics vs hla_typing
pub use blood_type::RiskLevel as BloodRiskLevel;
pub use mental_health_genetics::RiskLevel as MentalHealthRiskLevel;
pub use hla_typing::RiskLevel as HLATypingRiskLevel;

// RiskCategory conflict: comprehensive_risk_calculator vs epistasis
pub use comprehensive_risk_calculator::RiskCategory as ComprehensiveRiskCategory;
pub use epistasis::RiskCategory as EpistaticRiskCategory;

// HairTexture conflict: dermatology vs hair_genetics
pub use dermatology::HairTexture as DermatologyHairTexture;
pub use hair_genetics::HairTexture as HairGeneticsTexture;

// HairColor conflict: phenotype vs hair_genetics
pub use hair_genetics::HairColor as HairGeneticsColor;

// HairThickness conflict: hair_genetics only (dermatology doesn't have it)
pub use hair_genetics::HairThickness as HairGeneticsThickness;

// PsoriasisRisk conflict: dermatology vs skin_genetics
pub use dermatology::PsoriasisRisk as DermatologyPsoriasisRisk;
pub use skin_genetics::PsoriasisRisk as SkinGeneticsPsoriasisRisk;

// EczemaRisk conflict: dermatology vs skin_genetics
pub use dermatology::EczemaRisk as DermatologyEczemaRisk;
pub use skin_genetics::EczemaRisk as SkinGeneticsEczemaRisk;

// LactoseTolerance conflict: dietary_genetics only (lactose_tolerance doesn't have it)
pub use dietary_genetics::LactoseTolerance as DietaryLactoseTolerance;

// MC1RVariant conflict: european_variants vs skin_genetics
pub use european_variants::MC1RVariant as EuropeanMC1RVariant;
pub use skin_genetics::MC1RVariant as SkinMC1RVariant;

// MyopiaRisk conflict: eye_genetics vs ophthalmology
pub use eye_genetics::MyopiaRisk as EyeGeneticsMyopiaRisk;
pub use ophthalmology::MyopiaRisk as OphthalmologyMyopiaRisk;

// ColorBlindnessType conflict: eye_genetics vs ophthalmology
pub use eye_genetics::ColorBlindnessType as EyeGeneticsColorBlindness;
pub use ophthalmology::ColorBlindnessType as OphthalmologyColorBlindness;

// GeneEnvironmentInteraction conflict: gene_environment vs trait_interactions
pub use gene_environment::GeneEnvironmentInteraction as GeneEnvInteraction;
pub use trait_interactions::GeneEnvironmentInteraction as TraitGeneEnvInteraction;

// Additional conflicts found during build:
// HairColor conflict: phenotype vs dermatology (already aliased above, HairGeneticsColor)
pub use dermatology::HairColor as DermatologyHairColor;

// ADH1BVariant conflict: phenotype vs dietary_genetics
pub use dietary_genetics::ADH1BVariant as DietaryADH1BVariant;

// VitaminDMetabolism conflict: african_variants vs dietary_genetics
pub use african_variants::VitaminDMetabolism as AfricanVitaminDMetabolism;
pub use dietary_genetics::VitaminDMetabolism as DietaryVitaminDMetabolism;

// LactasePersistence conflict: african_variants vs european_variants
pub use african_variants::LactasePersistence as AfricanLactasePersistence;
pub use european_variants::LactasePersistence as EuropeanLactasePersistence;

// SkinPigmentation conflict: phenotype vs european_variants vs skin_genetics
pub use european_variants::SkinPigmentation as EuropeanSkinPigmentation;
pub use skin_genetics::SkinPigmentation as SkinGeneticsPigmentation;

// EyeColor conflict: phenotype vs european_variants vs eye_genetics vs ophthalmology
pub use european_variants::EyeColor as EuropeanEyeColor;
pub use eye_genetics::EyeColor as EyeGeneticsColor;
pub use ophthalmology::EyeColor as OphthalmologyEyeColor;

// AlcoholMetabolism conflict: phenotype vs european_variants
pub use european_variants::AlcoholMetabolism as EuropeanAlcoholMetabolism;

// ALDH2Genotype conflict: phenotype (ALDH2Function) vs european_variants
pub use european_variants::ALDH2Genotype as EuropeanALDH2Genotype;

// ADH1BGenotype conflict: european_variants vs native_american_variants
pub use european_variants::ADH1BGenotype as EuropeanADH1BGenotype;
pub use native_american_variants::ADH1BGenotype as NativeAmericanADH1BGenotype;

// Genotype conflict: european_variants vs others (already aliased above)
pub use european_variants::Genotype as EuropeanGenotype;

// GeneGeneInteraction conflict: epistasis vs trait_interactions
pub use epistasis::GeneGeneInteraction as EpistaticGeneGeneInteraction;
pub use trait_interactions::GeneGeneInteraction as TraitGeneGeneInteraction;

// InteractionType conflict: epistasis vs trait_interactions
pub use epistasis::InteractionType as EpistaticInteractionType;
pub use trait_interactions::InteractionType as TraitInteractionType;

// EvidenceLevel conflict: epistasis vs gene_environment
pub use epistasis::EvidenceLevel as EpistaticEvidenceLevel;
pub use gene_environment::EvidenceLevel as GeneEnvEvidenceLevel;

// Keep all modules as glob exports except those with resolved conflicts
pub use addiction_genetics::*;
pub use ancestry::*;
pub use asian_variants::*;
pub use athletic_performance::*;
// autoimmune exports only non-conflicting items (HLAAllele aliased above)
pub use autoimmune::{HLAGenotype, AutoimmuneGenesCatalog};
// blood_type exports only non-conflicting items (RiskLevel aliased above)
pub use blood_type::{ABOAllele, RhAllele, ABOGenotype, RhGenotype, BloodType, BloodTypeCompatibility, TransfusionCompatibility, PregnancyRiskAssessment};
pub use cancer_genetics::*;
pub use cardiovascular_genetics::*;
// comprehensive_risk_calculator exports only non-conflicting items (RiskCategory aliased above)
pub use comprehensive_risk_calculator::{ComprehensiveRiskCalculator, DiseaseRiskProfile, TraitPrediction};
// dermatology exports only non-conflicting items (HairTexture, PsoriasisRisk, EczemaRisk, HairColor aliased above)
pub use dermatology::{SkinTone, FitzpatrickType, SkinPigmentationGenetics, HairGenetics, AcneRisk};
// dietary_genetics exports only non-conflicting items (LactoseTolerance, ADH1BVariant, VitaminDMetabolism aliased above)
pub use dietary_genetics::{DietaryGeneticProfile, LCTGenotype, ToleranceLevel, AlcoholMetabolismGenetics, ALDH2Variant, MetabolismSpeed, CaffeineSensitivity, CYP1A2Genotype, ADORA2AGenotype, GlutenSensitivity, TasteGenetics, TAS2R38Genotype, BitterTasteSensitivity, NutrientMetabolism, FolateMetabolism, MTHFRGenotype, IronAbsorption, Omega3Conversion, FoodSensitivity, SensitivityType, SensitivitySeverity, NutritionPlan};
pub use disease_association::*;
pub use disease_susceptibility::*;
// epistasis exports only non-conflicting items (RiskCategory, GeneGeneInteraction, InteractionType, EvidenceLevel aliased above)
pub use epistasis::{EpistasisNetwork, PolygeneticRiskScore, GeneContribution, EpistaticAdjustment};
// european_variants exports only non-conflicting items (MC1RVariant, LactasePersistence, SkinPigmentation, EyeColor, HairColor, AlcoholMetabolism, ALDH2Genotype, ADH1BGenotype, Genotype aliased above)
pub use european_variants::{EuropeanGeneticVariants, EuropeanEyeColorGenetics, EuropeanHairColorGenetics, MC1RRedHairStatus, HemochromatosisStatus, CysticFibrosisStatus, CeliacDiseaseRisk, ThrombophiliaRisk};
// eye_genetics exports only non-conflicting items (MyopiaRisk, ColorBlindnessType, EyeColor aliased above)
pub use eye_genetics::{EyeGeneticProfile, OCA2HERC2EyeGenotype, GEYBEY2Genotype, SLC24A4Genotype, TYREyeGenotype, ColorIntensity, VisionGenetics, HyperopiaRisk, NightVisionQuality, EyeDiseaseRisks, ColorVisionDeficiency, ColorBlindnessSeverity, predict_eye_color_from_parents};
pub use gene_catalog::*;
// gene_environment exports only non-conflicting items (GeneEnvironmentInteraction, EvidenceLevel aliased above)
pub use gene_environment::{EnvironmentalFactor, DietaryFactor, ExerciseType, SleepQuality, PollutionType, InteractionDirection, GeneEnvironmentProfile, LifestyleRecommendation, RecommendationPriority};
// hair_genetics exports only non-conflicting items (HairTexture, HairThickness, HairColor aliased above)
pub use hair_genetics::{HairGeneticProfile, MC1RHairVariant, SLC45A2HairGenotype, TYRHairGenotype, TYRP1Genotype, EDARGenotype, TCHHGenotype, BaldnessGenetics, ARGeneCAGRepeats, EDARBaldnessVariant, BaldnessSeverity, GrayingTendency, BaldnessPrediction, HairDisorder, predict_hair_color_from_parents};
// hla_typing exports only non-conflicting items (HLAAllele, RiskLevel aliased above)
pub use hla_typing::{HLAGene, HLAType, HLAProfile, HLADiseaseAssociation, DrugHypersensitivityRisk, get_hla_disease_associations, get_hla_drug_hypersensitivities};
// lactose_tolerance exports only non-conflicting items (LactasePersistence aliased above)
pub use lactose_tolerance::{LactoseToleranceGenetics, LactoseToleranceProfile, population_lactase_persistence_frequency, get_lct_variant_by_population};
pub use longevity_genetics::*;
pub use markers::*;
// mental_health_genetics exports only non-conflicting items (RiskLevel aliased above)
pub use mental_health_genetics::{MentalHealthCondition, MentalHealthGene, MentalHealthVariant, MentalHealthProfile, TreatmentResponse, ResponseLevel, get_common_mental_health_variants};
pub use metabolic_disease::*;
pub use migraine_genetics::*;
pub use mitochondrial::*;
pub use neurological_genetics::*;
// ophthalmology exports only non-conflicting items (MyopiaRisk, ColorBlindnessType, EyeColor aliased above)
pub use ophthalmology::{VisionGene, EyeColorGenetics, ColorVisionGenetics, GeneStatus, MyopiaGenetics, GlaucomaRisk};
pub use pain_genetics::*;
pub use pharmacogenomics::*;
pub use population::*;
pub use population_traits::*;
pub use respiratory_genetics::*;
// skin_genetics exports only non-conflicting items (PsoriasisRisk, EczemaRisk, MC1RVariant, SkinPigmentation aliased above)
pub use skin_genetics::{FrecklingTendency, TanningAbility, SkinGeneticProfile, SLC24A5Genotype, SLC45A2Genotype, TYRGenotype, OCA2HERC2Genotype, SunSensitivity, MelanomaRiskProfile, VitiligoRisk, predict_vitiligo_risk};
pub use sleep_genetics::*;
pub use snp::*;
pub use taste_smell::*;
// trait_interactions exports only non-conflicting items (GeneEnvironmentInteraction, GeneGeneInteraction, InteractionType aliased above)
pub use trait_interactions::{TraitInteractionProfile, EpistaticEffect, common_gene_gene_interactions, common_gene_environment_interactions, epistatic_interactions};
pub use variation::*;
