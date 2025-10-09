use human_biology::*;
use human_biology::biology::genetics::{Ancestry, AncestryProfile};
use human_biology::pharmacology::pharmacogenomics::{PharmacogeneticProfile, PharmacogeneticGene, MetabolizerPhenotype};
use human_biology::diagnosis::{DiagnosticEngine};
use human_biology::pathology::headache::{HeadacheProfile, HeadacheType, MigraineSubtype};

fn main() {
    println!("=== Human Biology Ontology - Comprehensive Analysis ===\n");

    let patient1 = create_asian_patient();
    analyze_patient(&patient1, "Patient 1: East Asian ancestry");

    println!("\n{}\n", "=".repeat(80));

    let patient2 = create_european_patient();
    analyze_patient(&patient2, "Patient 2: European ancestry");

    println!("\n{}\n", "=".repeat(80));

    let patient3 = create_mixed_ancestry_patient();
    analyze_patient(&patient3, "Patient 3: Mixed ancestry");
}

fn create_asian_patient() -> Human {
    let mut human = Human::new_adult_male("EA001".to_string(), 32.0, 170.0, 68.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::EastAsian, 85.0).unwrap();
    ancestry.add_component(Ancestry::SoutheastAsian, 15.0).unwrap();
    ancestry.neanderthal_percentage = 2.1;
    ancestry.haplogroup_maternal = Some("D4".to_string());
    ancestry.haplogroup_paternal = Some("O2".to_string());
    human.genetics.ancestry = ancestry;

    let mut pgx = PharmacogeneticProfile::new();
    pgx.add_phenotype(PharmacogeneticGene::ALDH2, MetabolizerPhenotype::Poor);
    pgx.add_genotype(PharmacogeneticGene::ALDH2, "*2/*2".to_string());
    pgx.add_phenotype(PharmacogeneticGene::CYP2C19, MetabolizerPhenotype::Intermediate);
    pgx.add_genotype(PharmacogeneticGene::CYP2C19, "*2/*1".to_string());
    human.pharmacogenomics = pgx;

    human.health_conditions.add_condition("Lactose intolerance".to_string());
    human.health_conditions.add_family_history("Gastric cancer".to_string());

    human
}

fn create_european_patient() -> Human {
    let mut human = Human::new_adult_female("EU001".to_string(), 45.0, 165.0, 72.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::NorthernEuropean, 60.0).unwrap();
    ancestry.add_component(Ancestry::WesternEuropean, 40.0).unwrap();
    ancestry.neanderthal_percentage = 2.6;
    ancestry.haplogroup_maternal = Some("H".to_string());
    ancestry.haplogroup_paternal = Some("R1b".to_string());
    human.genetics.ancestry = ancestry;

    let mut pgx = PharmacogeneticProfile::new();
    pgx.add_phenotype(PharmacogeneticGene::CYP2D6, MetabolizerPhenotype::UltraRapid);
    pgx.add_genotype(PharmacogeneticGene::CYP2D6, "*1/*2xN".to_string());
    pgx.add_phenotype(PharmacogeneticGene::CYP2C9, MetabolizerPhenotype::Poor);
    pgx.add_genotype(PharmacogeneticGene::CYP2C9, "*3/*3".to_string());
    human.pharmacogenomics = pgx;

    human.genetics.carrier_status.push("Factor V Leiden heterozygous".to_string());
    human.health_conditions.add_family_history("Breast cancer".to_string());
    human.health_conditions.add_condition("Mild hypercholesterolemia".to_string());

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.primary_diagnosis = Some(HeadacheType::Migraine(MigraineSubtype::WithoutAura));
    headache_profile.headache_days_per_month = 4.0;
    headache_profile.treatment_history.push("Sumatriptan".to_string());
    headache_profile.current_prophylaxis.push("Propranolol".to_string());
    human.health_conditions.headache_profile = Some(headache_profile);

    human
}

fn create_mixed_ancestry_patient() -> Human {
    let mut human = Human::new_adult_male("MX001".to_string(), 28.0, 178.0, 82.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::Ashkenazi, 50.0).unwrap();
    ancestry.add_component(Ancestry::SouthAsian, 30.0).unwrap();
    ancestry.add_component(Ancestry::European, 20.0).unwrap();
    ancestry.neanderthal_percentage = 2.3;
    human.genetics.ancestry = ancestry;

    let mut pgx = PharmacogeneticProfile::new();
    pgx.add_phenotype(PharmacogeneticGene::TPMT, MetabolizerPhenotype::Intermediate);
    pgx.add_genotype(PharmacogeneticGene::TPMT, "*1/*3A".to_string());
    pgx.add_phenotype(PharmacogeneticGene::CYP2C19, MetabolizerPhenotype::Poor);
    human.pharmacogenomics = pgx;

    human.genetics.carrier_status.push("Gaucher disease carrier".to_string());
    human.genetics.carrier_status.push("MTHFR C677T heterozygous".to_string());

    human.health_conditions.add_family_history("Tay-Sachs disease".to_string());
    human.health_conditions.add_family_history("Type 2 diabetes".to_string());

    human
}

fn analyze_patient(human: &Human, title: &str) {
    println!("### {} ###\n", title);

    println!("Demographics:");
    println!("  Age: {:.0} years", human.demographics.age_years);
    println!("  Sex: {:?}", human.demographics.biological_sex);
    println!("  Height: {:.1} cm", human.body_metrics.height_cm);
    println!("  Weight: {:.1} kg", human.body_metrics.weight_kg);
    println!("  BMI: {:.1}", human.bmi());
    println!();

    println!("Ancestry Composition:");
    for (ancestry, percentage) in &human.genetics.ancestry.components {
        println!("  {:?}: {:.1}%", ancestry, percentage);
    }
    if let Some(mat) = &human.genetics.ancestry.haplogroup_maternal {
        println!("  Maternal haplogroup: {}", mat);
    }
    if let Some(pat) = &human.genetics.ancestry.haplogroup_paternal {
        println!("  Paternal haplogroup: {}", pat);
    }
    println!("  Neanderthal DNA: {:.1}%", human.genetics.ancestry.neanderthal_percentage);
    println!();

    println!("Ancestry-Based Risk Factors:");
    let risks = human.ancestry_report();
    for risk in &risks {
        println!("  {}", risk);
    }
    println!();

    println!("Pharmacogenomics:");
    for (gene, phenotype) in &human.pharmacogenomics.phenotypes {
        println!("  {:?}: {:?} metabolizer", gene, phenotype);
    }
    println!();

    println!("Drug Compatibility Examples:");
    let drugs = vec!["Codeine", "Clopidogrel", "Warfarin", "Azathioprine"];
    for drug in drugs {
        let compat = human.drug_compatibility_check(drug);
        if !compat.is_empty() {
            println!("  {}:", drug);
            for note in compat {
                println!("    - {}", note);
            }
        }
    }
    println!();

    println!("Health Metrics:");
    let summary = human.health_summary();
    println!("  Cardiac output: {:.1} L/min", summary.cardiac_output);
    println!("  Respiratory rate: {:.1} breaths/min", summary.respiratory_rate);
    println!("  GFR: {:.1} mL/min", summary.gfr);
    println!("  Metabolic rate: {:.0} kcal/day", summary.metabolic_rate);
    println!();

    if !human.health_conditions.active_conditions.is_empty() {
        println!("Active Conditions:");
        for condition in &human.health_conditions.active_conditions {
            println!("  - {}", condition);
        }
        println!();
    }

    if !human.health_conditions.family_history.is_empty() {
        println!("Family History:");
        for condition in &human.health_conditions.family_history {
            println!("  - {}", condition);
        }
        println!();
    }

    if !human.genetics.carrier_status.is_empty() {
        println!("Carrier Status:");
        for carrier in &human.genetics.carrier_status {
            println!("  - {}", carrier);
        }
        println!();
    }

    if let Some(ref headache) = human.health_conditions.headache_profile {
        println!("Headache Profile:");
        if let Some(ref htype) = headache.primary_diagnosis {
            println!("  Type: {:?}", htype);
        }
        println!("  Headache days per month: {:.0}", headache.headache_days_per_month);
        if !headache.treatment_history.is_empty() {
            println!("  Treatment history: {}", headache.treatment_history.join(", "));
        }
        if !headache.current_prophylaxis.is_empty() {
            println!("  Current prophylaxis: {}", headache.current_prophylaxis.join(", "));
        }
        println!();
    }

    println!("Diagnostic Analysis:");
    let report = DiagnosticEngine::analyze(human);

    if !report.findings.is_empty() {
        println!("\n  Clinical Findings:");
        for finding in &report.findings {
            println!("    [{:?}] {:?}: {}", finding.category, finding.severity, finding.description);
        }
    }

    if !report.risk_factors.is_empty() {
        println!("\n  Risk Factors:");
        for risk in &report.risk_factors {
            println!("    {} ({:?} risk)", risk.condition, risk.risk_level);
            for factor in &risk.contributing_factors {
                println!("      - {}", factor);
            }
        }
    }

    if !report.genetic_insights.is_empty() {
        println!("\n  Genetic Insights:");
        for insight in &report.genetic_insights {
            println!("    {}: {}", insight.gene_or_variant, insight.clinical_relevance);
        }
    }

    if !report.pharmacogenetic_recommendations.is_empty() {
        println!("\n  Pharmacogenetic Recommendations:");
        for rec in &report.pharmacogenetic_recommendations {
            println!("    {} ({:?}): {}", rec.drug_class, rec.recommendation_type, rec.rationale);
        }
    }

    if !report.lifestyle_recommendations.is_empty() {
        println!("\n  Lifestyle Recommendations:");
        for rec in &report.lifestyle_recommendations {
            println!("    - {}", rec);
        }
    }

    if !report.follow_up_tests.is_empty() {
        println!("\n  Recommended Follow-up Tests:");
        for test in &report.follow_up_tests {
            println!("    - {}", test);
        }
    }
}
