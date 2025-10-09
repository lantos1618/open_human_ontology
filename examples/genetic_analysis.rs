use human_biology::{Human, BiologicalSex};
use human_biology::biology::genetics::{
    AsianGeneticProfile, HLAGenotype, HLAAllele, MetabolicRiskProfile,
    GeneCatalog, AsianGeneticVariantsCatalog,
};

fn main() {
    println!("=== Human Genetic Analysis Examples ===\n");

    example_asian_ancestry_analysis();
    example_migraine_risk_assessment();
    example_metabolic_disease_screening();
    example_pharmacogenomic_recommendations();
    example_autoimmune_disease_risk();
}

fn example_asian_ancestry_analysis() {
    println!("--- Example 1: East Asian Ancestry Genetic Profile ---");

    let mut asian_profile = AsianGeneticProfile::new();
    asian_profile.has_aldh2_deficiency = true;
    asian_profile.has_edar_370a = true;
    asian_profile.has_dry_earwax = true;
    asian_profile.cyp2c19_poor_metabolizer = false;
    asian_profile.hla_b5801_positive = false;

    println!("ALDH2 Status: {}", if asian_profile.has_aldh2_deficiency {
        "Deficient (rs671 G>A present)"
    } else {
        "Normal"
    });
    println!("Alcohol Tolerance: {}", asian_profile.alcohol_tolerance());
    println!("EDAR 370A: {}", if asian_profile.has_edar_370a {
        "Present - Thick straight hair, shovel-shaped incisors"
    } else {
        "Absent"
    });
    println!("Earwax Type: {}", if asian_profile.has_dry_earwax {
        "Dry (reduced body odor)"
    } else {
        "Wet"
    });
    println!("Allopurinol Safety: {}", if asian_profile.allopurinol_safe() {
        "Safe to use"
    } else {
        "HIGH RISK - DO NOT USE (HLA-B*58:01 positive)"
    });
    println!("Clopidogrel Response: {}", asian_profile.clopidogrel_efficacy());
    println!();
}

fn example_migraine_risk_assessment() {
    println!("--- Example 2: Migraine Risk Assessment ---");

    let genes = GeneCatalog::get_neurological_genes();

    let cacna1a = genes.get("CACNA1A").unwrap();
    println!("Gene: {}", cacna1a.symbol);
    println!("Function: {:?}", cacna1a.function);

    for variant in &cacna1a.clinical_variants {
        println!("  Variant: {} ({})", variant.variant_name, variant.hgvs);
        println!("  Clinical Significance: {:?}", variant.clinical_significance);
        println!("  Associated Conditions:");
        for condition in &variant.associated_conditions {
            println!("    - {}", condition);
        }
        println!("  Population Frequency: {:.4}%", variant.population_frequency * 100.0);
    }

    let mthfr = genes.get("MTHFR").unwrap();
    println!("\nGene: {} (Migraine with Aura risk factor)", mthfr.symbol);
    println!("  C677T variant frequency: 25%");
    println!("  Associated with increased homocysteine and migraine with aura");
    println!();
}

fn example_metabolic_disease_screening() {
    println!("--- Example 3: Type 2 Diabetes and Metabolic Syndrome Screening ---");

    let human = Human::new_adult_male(
        "patient_001".to_string(),
        45.0,
        175.0,
        85.0,
    );

    let mut metabolic_profile = MetabolicRiskProfile::new();

    metabolic_profile.t2d_risk_score = 2.1;
    metabolic_profile.obesity_genetic_load = 1.4;
    metabolic_profile.fh_risk = false;

    let height_m = human.body_metrics.height_cm / 100.0;
    let bmi = human.body_metrics.weight_kg / (height_m * height_m);

    println!("Patient: {}", human.id);
    println!("Age: {} years", human.demographics.age_years);
    println!("BMI: {:.1}", bmi);
    println!("\nGenetic Risk Assessment:");
    println!("  TCF7L2 rs7903146: TT genotype (1.5x T2D risk)");
    println!("  FTO rs9939609: AA genotype (1.3x obesity risk)");
    println!("  Combined T2D Risk Score: {:.2}x", metabolic_profile.t2d_risk_score);
    println!("  Assessment: {}", metabolic_profile.assess_diabetes_risk());
    println!("\nRecommendations:");
    println!("  - HbA1c screening every 6 months");
    println!("  - Fasting glucose monitoring");
    println!("  - Weight management program");
    println!("  - Consider metformin if prediabetes develops");
    println!();
}

fn example_pharmacogenomic_recommendations() {
    println!("--- Example 4: Pharmacogenomic Recommendations ---");

    let genes = AsianGeneticVariantsCatalog::get_pharmacogenetic_variants();

    let cyp2c19 = genes.get("CYP2C19").unwrap();
    println!("Analyzing drug metabolism for patient...\n");
    println!("Gene: {}", cyp2c19.symbol);
    println!("Variants detected:");

    for variant in &cyp2c19.clinical_variants {
        println!("  {} ({})", variant.variant_name, variant.rs_id.as_ref().unwrap());
        println!("  Frequency in East Asians: {:.1}%", variant.population_frequency * 100.0);
        for condition in &variant.associated_conditions {
            println!("    - {}", condition);
        }
    }

    println!("\nDrug Recommendations:");
    println!("  Clopidogrel: Consider alternative (prasugrel or ticagrelor) if *2/*2 or *2/*3 genotype");
    println!("  Omeprazole: Standard dosing effective (may have enhanced response)");
    println!("  Voriconazole: Reduce dose if poor metabolizer");

    let hla_b5801 = genes.get("HLA-B*58:01").unwrap();
    println!("\nHLA-B*58:01 Status: Negative");
    println!("  Allopurinol: SAFE TO USE");
    println!("  (If positive: 100x increased risk of Stevens-Johnson syndrome)");
    println!();
}

fn example_autoimmune_disease_risk() {
    println!("--- Example 5: Autoimmune Disease Risk Assessment ---");

    let mut hla_genotype = HLAGenotype::new();
    hla_genotype.add_class_i(HLAAllele::B27);
    hla_genotype.add_class_ii(HLAAllele::DQ2);
    hla_genotype.add_class_ii(HLAAllele::DR4);

    println!("HLA Genotype Analysis:");
    println!("  Class I: HLA-B*27");
    println!("  Class II: HLA-DQ2, HLA-DR4");

    println!("\nAutoimmune Risk Assessment:");
    let risks = hla_genotype.autoimmune_risk_alleles();
    for (allele, condition) in &risks {
        println!("  {:?} → {}", allele, condition);
    }

    println!("\nSpecific Disease Risks:");
    println!("  Celiac Disease: {:.1}% (DQ2 present)", hla_genotype.celiac_risk() * 100.0);
    println!("  Type 1 Diabetes: {:.1}% (DR4 present)", hla_genotype.t1d_risk() * 100.0);
    println!("  Rheumatoid Arthritis: {:.1}% (DR4 present)", hla_genotype.ra_risk() * 100.0);
    println!("  Multiple Sclerosis: {:.2}% (DR15 absent)", hla_genotype.ms_risk() * 100.0);

    println!("\nClinical Recommendations:");
    println!("  - Celiac screening: Anti-TTG and anti-EMA antibodies");
    println!("  - Monitor for symptoms: chronic diarrhea, weight loss, fatigue");
    println!("  - Consider HLA typing for first-degree relatives");
    println!("  - Gluten-free diet if symptomatic or antibody positive");
    println!();
}
