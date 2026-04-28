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

// Canonical types — submodules each define their own variants; reach those
// directly via the submodule path. The cross-module aliases that used to live
// here were unused outside this file (verified) and have been removed.
pub use genotype::{
    AncestryComponent, DiseaseRisk, Genotype, GenotypeRiskProfile, MetabolizerStatus,
    PharmacogeneticMarker, PhenotypeAssociation, Severity,
};
pub use african_variants::{
    APOL1Status, AfricanGeneticVariants, G6PDStatus, HypertensionRiskProfile, LactasePersistence,
    MalariaResistanceProfile, SickleCellStatus, SkinPigmentationGenotype, VitaminDMetabolism,
};
pub use native_american_variants::{
    ACEInhibitorResponse, ACEResponse, ADH1BGenotype, ADH1CGenotype, ALDH2Genotype, APOEStatus,
    AlcoholMetabolismProfile, DiabetesRiskProfile, GallbladderRiskProfile,
    NativeAmericanGeneticVariants,
};
pub use phenotype::{
    ADH1BVariant, ALDH2Function, AlcoholMetabolism, CaffeineMetabolism, DiseaseSusceptibility,
    EarwaxType, EyeColor, HairColor, MetabolicTraits, OpioidMetabolism, PharmacologicalTraits,
    PhenotypeProfile, PhysicalTraits, SSRIResponse, SkinPigmentation, VitaminDSynthesis,
    WarfarinSensitivity,
};
pub use allele::{Allele, AlleleFrequency, AllelePair, AlleleType, CarrierStatus, Zygosity};
pub use gene_variants::{
    DiseaseRiskVariant, DosageAdjustment, DrugEffect, DrugResponse, EffectDirection,
    GeneVariantCatalog, MetabolicImpact, MetabolicVariant, PharmacogeneticPhenotype,
    PharmacogeneticVariant, RiskAssessment, TraitEffect, TraitVariant, VariantClinicalSignificance,
};
pub use gene::{Exon, Gene, GeneType, Intron, Promoter, RegulatoryRegion, RegulatoryType};
pub use genome::{Genome, GenomicLocation, Variant};
pub use chromosome::Chromosome;
pub use dna::{DNASequence, Nucleotide};

// Glob re-exports for modules with no name conflicts.
pub use addiction_genetics::*;
pub use ancestry::*;
pub use asian_variants::*;
pub use athletic_performance::*;
pub use autoimmune::{AutoimmuneGenesCatalog, HLAGenotype};
pub use blood_type::{
    ABOAllele, ABOGenotype, BloodType, BloodTypeCompatibility, PregnancyRiskAssessment, RhAllele,
    RhGenotype, TransfusionCompatibility,
};
pub use cancer_genetics::*;
pub use cardiovascular_genetics::*;
pub use comprehensive_risk_calculator::{
    ComprehensiveRiskCalculator, DiseaseRiskProfile, TraitPrediction,
};
pub use dermatology::{
    AcneRisk, FitzpatrickType, HairGenetics, SkinPigmentationGenetics, SkinTone,
};
pub use dietary_genetics::{
    ADORA2AGenotype, AlcoholMetabolismGenetics, BitterTasteSensitivity, CYP1A2Genotype,
    CaffeineSensitivity, DietaryGeneticProfile, FoodSensitivity, FolateMetabolism,
    GlutenSensitivity, IronAbsorption, LCTGenotype, MTHFRGenotype, MetabolismSpeed,
    NutrientMetabolism, NutritionPlan, Omega3Conversion, SensitivitySeverity, SensitivityType,
    TAS2R38Genotype, TasteGenetics, ToleranceLevel,
};
pub use disease_association::*;
pub use disease_susceptibility::*;
pub use epistasis::{
    EpistasisNetwork, EpistaticAdjustment, GeneContribution, PolygeneticRiskScore,
};
pub use european_variants::{
    CeliacDiseaseRisk, CysticFibrosisStatus, EuropeanEyeColorGenetics, EuropeanGeneticVariants,
    EuropeanHairColorGenetics, HemochromatosisStatus, MC1RRedHairStatus, ThrombophiliaRisk,
};
pub use eye_genetics::{
    ColorIntensity, ColorVisionDeficiency, ColorBlindnessSeverity, EyeDiseaseRisks,
    EyeGeneticProfile, GEYBEY2Genotype, HyperopiaRisk, NightVisionQuality, OCA2HERC2EyeGenotype,
    SLC24A4Genotype, TYREyeGenotype, VisionGenetics, predict_eye_color_from_parents,
};
pub use gene_catalog::*;
pub use gene_environment::{
    DietaryFactor, EnvironmentalFactor, ExerciseType, GeneEnvironmentProfile, InteractionDirection,
    LifestyleRecommendation, PollutionType, RecommendationPriority, SleepQuality,
};
pub use hair_genetics::{
    ARGeneCAGRepeats, BaldnessGenetics, BaldnessPrediction, BaldnessSeverity, EDARBaldnessVariant,
    EDARGenotype, GrayingTendency, HairDisorder, HairGeneticProfile, MC1RHairVariant,
    SLC45A2HairGenotype, TCHHGenotype, TYRHairGenotype, TYRP1Genotype,
    predict_hair_color_from_parents,
};
pub use hla_typing::{
    DrugHypersensitivityRisk, HLADiseaseAssociation, HLAGene, HLAProfile, HLAType,
    get_hla_disease_associations, get_hla_drug_hypersensitivities,
};
pub use lactose_tolerance::{
    LactoseToleranceGenetics, LactoseToleranceProfile, get_lct_variant_by_population,
    population_lactase_persistence_frequency,
};
pub use longevity_genetics::*;
pub use markers::*;
pub use mental_health_genetics::{
    MentalHealthCondition, MentalHealthGene, MentalHealthProfile, MentalHealthVariant,
    ResponseLevel, TreatmentResponse, get_common_mental_health_variants,
};
pub use metabolic_disease::*;
pub use migraine_genetics::*;
pub use mitochondrial::*;
pub use neurological_genetics::*;
pub use ophthalmology::{
    ColorVisionGenetics, EyeColorGenetics, GeneStatus, GlaucomaRisk, MyopiaGenetics, VisionGene,
};
pub use pain_genetics::*;
pub use pharmacogenomics::*;
pub use population::*;
pub use population_traits::*;
pub use respiratory_genetics::*;
pub use skin_genetics::{
    FrecklingTendency, MelanomaRiskProfile, OCA2HERC2Genotype, SLC24A5Genotype, SLC45A2Genotype,
    SkinGeneticProfile, SunSensitivity, TYRGenotype, TanningAbility, VitiligoRisk,
    predict_vitiligo_risk,
};
pub use sleep_genetics::*;
pub use snp::*;
pub use taste_smell::*;
pub use trait_interactions::{
    EpistaticEffect, TraitInteractionProfile, common_gene_environment_interactions,
    common_gene_gene_interactions, epistatic_interactions,
};
pub use variation::*;
