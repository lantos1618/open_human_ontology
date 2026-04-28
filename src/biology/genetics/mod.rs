pub mod allele;
pub mod ancestry;
pub mod dietary_genetics;
pub mod dna;
pub mod genotype;
pub mod phenotype;
pub mod population_traits;
pub mod snp;

pub use allele::{Allele, AlleleFrequency, AllelePair, AlleleType, CarrierStatus, Zygosity};
pub use ancestry::{Ancestry, AncestryProfile};
pub use dietary_genetics::{
    ADH1BVariant, ADORA2AGenotype, AlcoholMetabolismGenetics, BitterTasteSensitivity,
    CYP1A2Genotype, CaffeineSensitivity, DietaryGeneticProfile, FoodSensitivity, FolateMetabolism,
    GlutenSensitivity, IronAbsorption, LCTGenotype, LactoseTolerance, MTHFRGenotype,
    MetabolismSpeed, NutrientMetabolism, NutritionPlan, Omega3Conversion, SensitivitySeverity,
    SensitivityType, TAS2R38Genotype, TasteGenetics, ToleranceLevel, VitaminDMetabolism,
};
pub use dna::{DNASequence, Nucleotide};
pub use genotype::{
    AncestryComponent, DiseaseRisk, Genotype, GenotypeRiskProfile, MetabolizerStatus,
    PharmacogeneticMarker, PhenotypeAssociation, Severity,
};
pub use phenotype::{
    ALDH2Function, AlcoholMetabolism, CaffeineMetabolism, DiseaseSusceptibility, EarwaxType,
    EyeColor, HairColor, MetabolicTraits, OpioidMetabolism, PharmacologicalTraits,
    PhenotypeProfile, PhysicalTraits, SSRIResponse, SkinPigmentation, VitaminDSynthesis,
    WarfarinSensitivity,
};
pub use population_traits::{AlcoholToleranceInfo, PopulationSpecificTraits};
pub use snp::SNP;
