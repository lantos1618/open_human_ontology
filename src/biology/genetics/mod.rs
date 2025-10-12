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
pub mod phenotype_predictor;
pub mod population;
pub mod population_genetics;
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

// CaffeineMetabolism: Use the comprehensive version from phenotype.rs, alias the simple enum from phenotype_predictor.rs
pub use phenotype::{PhenotypeProfile, PhysicalTraits, MetabolicTraits, DiseaseSusceptibility, PharmacologicalTraits, EyeColor, HairColor, SkinPigmentation, EarwaxType, CaffeineMetabolism, AlcoholMetabolism, ALDH2Function, ADH1BVariant, VitaminDSynthesis, WarfarinSensitivity, OpioidMetabolism, SSRIResponse};
pub use phenotype_predictor::{FullBodyPhenotypePredictor, PredictedTraits, PhysicalAppearance, PhysiologyTraits, PerformanceTraits, MetabolismTraits, SensoryTraits, BehaviorTraits, Probability};
pub use phenotype_predictor::CaffeineMetabolism as PredictorCaffeineMetabolism;

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

// PsoriasisRisk conflict: dermatology vs skin_genetics
pub use dermatology::PsoriasisRisk as DermatologyPsoriasisRisk;
pub use skin_genetics::PsoriasisRisk as SkinGeneticsPsoriasisRisk;

// Keep all modules as glob exports - conflicts resolved through aliases above
pub use addiction_genetics::*;
pub use ancestry::*;
pub use asian_variants::*;
pub use athletic_performance::*;
pub use autoimmune::*;
pub use blood_type::*;
pub use cancer_genetics::*;
pub use cardiovascular_genetics::*;
pub use comprehensive_risk_calculator::*;
pub use dermatology::*;
pub use dietary_genetics::*;
pub use disease_association::*;
pub use disease_susceptibility::*;
pub use epistasis::*;
pub use european_variants::*;
pub use eye_genetics::*;
pub use gene_catalog::*;
pub use gene_environment::*;
pub use hair_genetics::*;
pub use hla_typing::*;
pub use lactose_tolerance::*;
pub use longevity_genetics::*;
pub use markers::*;
pub use mental_health_genetics::*;
pub use metabolic_disease::*;
pub use migraine_genetics::*;
pub use mitochondrial::*;
pub use neurological_genetics::*;
pub use ophthalmology::*;
pub use pain_genetics::*;
pub use pharmacogenomics::*;
pub use population::*;
pub use population_genetics::*;
pub use population_traits::*;
pub use respiratory_genetics::*;
pub use skin_genetics::*;
pub use sleep_genetics::*;
pub use snp::*;
pub use taste_smell::*;
pub use trait_interactions::*;
pub use variation::*;
