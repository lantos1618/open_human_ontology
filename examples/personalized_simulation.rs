use human_biology::biology::genetics::{
    AncestryProfile, Ancestry, GeneEnvironmentProfile, EpistasisNetwork,
    PolygeneticRiskScore, GeneContribution, EnvironmentalFactor, ExerciseType,
    DietaryFactor,
};
use human_biology::personalized_medicine::{
    PersonalizedMedicineProfile, PersonalizedMedicineEngine, LifestyleIntervention,
    InterventionType, Priority, PreventiveStrategy, StrategyType,
};
use human_biology::pharmacology::pharmacogenomics::{
    PharmacogeneticProfile, PharmacogeneticGene, MetabolizerPhenotype,
};
use human_biology::human::BiologicalSex;

fn main() {
    println!("=== Personalized Medicine Simulation ===\n");

    let patient1 = create_east_asian_patient();
    println!("Patient 1: East Asian with ALDH2 deficiency");
    println!("Risk assessment:");
    for risk in &patient1.disease_risks {
        println!("  - {}: {:?} (combined risk: {:.2})",
            risk.condition, risk.risk_category, risk.combined_risk);
    }
    println!();

    let patient2 = create_european_patient();
    println!("Patient 2: European with CYP2D6 poor metabolizer");
    println!("Pharmacogenomic profile:");
    for (gene, phenotype) in &patient2.pharmacogenomics.phenotypes {
        println!("  - {:?}: {:?}", gene, phenotype);
    }
    println!();

    let patient3 = create_mixed_ancestry_patient();
    println!("Patient 3: Mixed ancestry (African + European)");
    println!("Lifestyle interventions:");
    for intervention in patient3.get_critical_interventions() {
        println!("  - {:?} for {} (Priority: {:?})",
            intervention.intervention_type,
            intervention.target_condition,
            intervention.priority);
    }
    println!();

    demonstrate_gene_environment_interactions();

    demonstrate_epistasis();

    demonstrate_personalized_drug_recommendations();

    println!("\n=== Simulation Complete ===");
}

fn create_east_asian_patient() -> PersonalizedMedicineProfile {
    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

    let mut profile = PersonalizedMedicineProfile::new(
        "PATIENT_EA_001".to_string(),
        ancestry,
    );

    profile.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::ALDH2,
        MetabolizerPhenotype::Poor,
    );

    let mut prs = PolygeneticRiskScore::new("Esophageal cancer".to_string());
    prs.add_gene_contribution(GeneContribution {
        gene: "ALDH2".to_string(),
        weight: 0.9,
        allele_score: 2.0,
        contribution: 1.8,
    });

    let mut gene_env = GeneEnvironmentProfile::new();
    gene_env.set_lifestyle_factor(EnvironmentalFactor::Alcohol, 0.8);

    profile.assess_disease_risk(
        "Esophageal cancer with alcohol consumption".to_string(),
        prs,
        &gene_env,
    );

    profile.add_lifestyle_intervention(LifestyleIntervention {
        intervention_type: InterventionType::AlcoholModeration,
        target_condition: "Esophageal cancer".to_string(),
        expected_risk_reduction: 0.8,
        priority: Priority::Critical,
        specific_recommendations: vec![
            "Avoid alcohol completely due to ALDH2 deficiency".to_string(),
            "10x increased cancer risk with alcohol consumption".to_string(),
        ],
        monitoring_parameters: vec![
            "Annual endoscopy if alcohol history".to_string(),
        ],
    });

    profile
}

fn create_european_patient() -> PersonalizedMedicineProfile {
    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::European, 85.0).unwrap();
    ancestry.add_component(Ancestry::Ashkenazi, 15.0).unwrap();

    let mut profile = PersonalizedMedicineProfile::new(
        "PATIENT_EU_001".to_string(),
        ancestry,
    );

    profile.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::CYP2D6,
        MetabolizerPhenotype::Poor,
    );

    profile.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::TPMT,
        MetabolizerPhenotype::Intermediate,
    );

    let mut prs = PolygeneticRiskScore::new("Cardiovascular disease".to_string());
    prs.add_gene_contribution(GeneContribution {
        gene: "APOE4".to_string(),
        weight: 0.7,
        allele_score: 1.5,
        contribution: 1.05,
    });

    let gene_env = GeneEnvironmentProfile::new();
    profile.assess_disease_risk(
        "Cardiovascular disease".to_string(),
        prs,
        &gene_env,
    );

    profile
}

fn create_mixed_ancestry_patient() -> PersonalizedMedicineProfile {
    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::SubSaharanAfrican, 60.0).unwrap();
    ancestry.add_component(Ancestry::European, 40.0).unwrap();

    let mut profile = PersonalizedMedicineProfile::new(
        "PATIENT_MX_001".to_string(),
        ancestry,
    );

    let mut prs = PolygeneticRiskScore::new("Type 2 Diabetes".to_string());
    prs.add_gene_contribution(GeneContribution {
        gene: "TCF7L2".to_string(),
        weight: 0.8,
        allele_score: 2.0,
        contribution: 1.6,
    });

    let mut gene_env = GeneEnvironmentProfile::new();
    gene_env.set_lifestyle_factor(
        EnvironmentalFactor::Exercise(ExerciseType::Sedentary),
        0.9,
    );
    gene_env.set_lifestyle_factor(
        EnvironmentalFactor::Diet(DietaryFactor::HighSugar),
        0.7,
    );

    profile.assess_disease_risk(
        "Type 2 Diabetes".to_string(),
        prs,
        &gene_env,
    );

    profile.add_lifestyle_intervention(LifestyleIntervention {
        intervention_type: InterventionType::Exercise,
        target_condition: "Type 2 Diabetes".to_string(),
        expected_risk_reduction: 0.4,
        priority: Priority::High,
        specific_recommendations: vec![
            "150 minutes moderate aerobic activity per week".to_string(),
            "Resistance training 2-3 times per week".to_string(),
            "Can reduce genetic diabetes risk by 40%".to_string(),
        ],
        monitoring_parameters: vec![
            "HbA1c every 3 months".to_string(),
            "Fasting glucose weekly".to_string(),
        ],
    });

    profile.add_lifestyle_intervention(LifestyleIntervention {
        intervention_type: InterventionType::Diet,
        target_condition: "Type 2 Diabetes".to_string(),
        expected_risk_reduction: 0.35,
        priority: Priority::High,
        specific_recommendations: vec![
            "Low glycemic index diet".to_string(),
            "Increase fiber intake to 25-30g daily".to_string(),
            "Limit refined sugars".to_string(),
        ],
        monitoring_parameters: vec![
            "Food diary".to_string(),
            "Weekly weight monitoring".to_string(),
        ],
    });

    profile
}

fn demonstrate_gene_environment_interactions() {
    println!("=== Gene-Environment Interactions ===");

    let mut gene_env_profile = GeneEnvironmentProfile::new();
    gene_env_profile.interactions = GeneEnvironmentProfile::load_known_interactions();

    gene_env_profile.set_lifestyle_factor(
        EnvironmentalFactor::Exercise(ExerciseType::Aerobic),
        0.8,
    );

    let fto_risk = gene_env_profile.calculate_gene_environment_risk("FTO", "Obesity risk");
    println!("FTO gene + High Exercise: Obesity risk modifier = {:.2}", fto_risk);
    println!("  (< 1.0 means protective effect)\n");

    gene_env_profile.set_lifestyle_factor(EnvironmentalFactor::Smoking, 0.9);
    let gst_risk = gene_env_profile.calculate_gene_environment_risk("GST", "Lung cancer");
    println!("GST gene + Smoking: Lung cancer risk modifier = {:.2}", gst_risk);
    println!("  (> 1.0 means increased risk)\n");

    let recommendations = gene_env_profile.get_personalized_recommendations("ALDH2");
    println!("Personalized recommendations for ALDH2 carriers:");
    for rec in &recommendations {
        println!("  - [{}] {}", format!("{:?}", rec.priority), rec.recommendation);
    }
    println!();
}

fn demonstrate_epistasis() {
    println!("=== Epistasis (Gene-Gene Interactions) ===");

    let network = EpistasisNetwork::load_known_interactions();

    println!("Known gene-gene interactions: {}", network.interactions.len());

    let effect = network.calculate_epistatic_effect(
        "APOE",
        "TOMM40",
        "Alzheimer's disease risk",
    );

    if let Some(effect) = effect {
        println!("\nAPOE × TOMM40 interaction:");
        println!("  Synergistic effect on Alzheimer's: {:.1}x", effect);
        println!("  Combined effect is greater than additive");
    }

    let effect2 = network.calculate_epistatic_effect(
        "ALDH2",
        "ADH1B",
        "Alcohol metabolism",
    );

    if let Some(effect) = effect2 {
        println!("\nALDH2 × ADH1B interaction:");
        println!("  Multiplicative effect on alcohol metabolism: {:.1}x", effect);
        println!("  Both genes work together in alcohol processing");
    }

    let hub_genes = network.get_hub_genes(1);
    println!("\nHub genes (connected to multiple other genes):");
    for gene in hub_genes.iter().take(5) {
        println!("  - {}", gene);
    }
    println!();
}

fn demonstrate_personalized_drug_recommendations() {
    println!("=== Personalized Drug Recommendations ===");

    let mut pharmacogenomics = PharmacogeneticProfile::new();

    pharmacogenomics.add_phenotype(
        PharmacogeneticGene::CYP2D6,
        MetabolizerPhenotype::Poor,
    );

    println!("Patient: CYP2D6 Poor Metabolizer\n");

    let drugs = vec!["Codeine", "Metoprolol", "Tramadol"];
    for drug in drugs {
        let response = pharmacogenomics.predict_drug_response(drug).unwrap();
        println!("Drug: {}", response.drug_name);
        println!("  Efficacy modifier: {:.2}x", response.efficacy_modifier);
        println!("  Toxicity risk: {:?}", response.toxicity_risk);
        println!("  Recommendation: {}", response.dosing_recommendation);
        if !response.warnings.is_empty() {
            println!("  Warnings:");
            for warning in &response.warnings {
                println!("    ⚠ {}", warning);
            }
        }
        println!();
    }

    let mut asian_profile = PharmacogeneticProfile::new();
    asian_profile.add_genotype(
        PharmacogeneticGene::HLA_B1502,
        "*15:02/*15:02".to_string(),
    );

    println!("Asian Patient with HLA-B*15:02\n");
    let response = asian_profile.predict_drug_response("Carbamazepine").unwrap();
    println!("Drug: {}", response.drug_name);
    println!("  Toxicity risk: {:?}", response.toxicity_risk);
    for warning in &response.warnings {
        println!("  ⚠ {}", warning);
    }
    println!();
}
