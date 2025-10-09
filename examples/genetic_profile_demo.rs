use human_biology::{Human, BiologicalSex};
use human_biology::biology::genetics::*;
use human_biology::diagnosis::ComprehensiveDiagnosticReport;

fn main() {
    println!("=== Comprehensive Human Genetic Profile Demo ===\n");

    demo_cardiovascular_genetics();
    println!("\n{}\n", "=".repeat(80));

    demo_neurological_genetics();
    println!("\n{}\n", "=".repeat(80));

    demo_cancer_genetics();
    println!("\n{}\n", "=".repeat(80));

    demo_respiratory_genetics();
    println!("\n{}\n", "=".repeat(80));

    demo_athletic_profile();
    println!("\n{}\n", "=".repeat(80));

    demo_comprehensive_assessment();
}

fn demo_cardiovascular_genetics() {
    println!("🫀 CARDIOVASCULAR GENETICS PROFILE");
    println!("{}", "-".repeat(80));

    let mut profile = CardiovascularGeneticProfile::default();

    println!("\n1. APOE E4/E4 Genotype (High Alzheimer's Risk):");
    profile.apoe = ApoeGenotype::e4_e4();
    println!("   - Alzheimer's Risk: {:.1}x", profile.apoe.alzheimers_relative_risk());
    println!("   - CVD Risk: {:.1}x", profile.apoe.cardiovascular_disease_relative_risk());
    println!("   - LDL Effect: {:+.0} mg/dL", profile.apoe.ldl_cholesterol_effect());

    println!("\n2. PCSK9 Loss-of-Function Variant (Cardioprotective):");
    profile.pcsk9 = Pcsk9Variant::LossOfFunction;
    println!("   - LDL Effect: {:+.0}%", profile.pcsk9.ldl_effect());
    println!("   - CVD Risk: {:.2}x", profile.pcsk9.cardiovascular_risk());

    println!("\n3. LDLR Heterozygous (Familial Hypercholesterolemia):");
    profile.ldlr = LdlrVariant::Heterozygous;
    println!("   - FH Risk: {:.0}x", profile.ldlr.familial_hypercholesterolemia_risk());
    println!("   - LDL Multiplier: {:.1}x", profile.ldlr.ldl_level_multiplier());

    println!("\n4. Factor V Leiden Heterozygous:");
    profile.factor5_leiden = Factor5LeidenStatus::Heterozygous;
    println!("   - Thrombosis Risk: {:.0}x", profile.factor5_leiden.thrombosis_risk());

    println!("\n5. MTHFR C677T TT Genotype:");
    profile.mthfr_677 = Mthfr677Variant::TT;
    println!("   - Homocysteine: {:.0}% of normal", profile.mthfr_677.homocysteine_level_multiplier() * 100.0);
    println!("   - CVD Risk: {:.1}x", profile.mthfr_677.cardiovascular_risk());
    println!("   - Folate Supplementation: {}", if profile.mthfr_677.requires_folate_supplementation() { "Required" } else { "Not required" });

    println!("\n6. CYP2C19 Poor Metabolizer:");
    profile.cyp2c19 = Cyp2c19Phenotype::Poor;
    println!("   - Clopidogrel: {}", profile.cyp2c19.clopidogrel_response());
    println!("   - PPI Metabolism: {}", profile.cyp2c19.ppi_metabolism());

    println!("\nComprehensive Cardiovascular Assessment:");
    println!("   - Overall CVD Risk: {:.2}x", profile.comprehensive_cardiovascular_risk());
    println!("   - Estimated LDL Effect: {:+.0} mg/dL", profile.estimated_ldl_effect());

    println!("\nStatin Considerations:");
    for consideration in profile.statin_considerations() {
        println!("   • {}", consideration);
    }

    println!("\nAntiplatelet Recommendations:");
    for rec in profile.antiplatelet_recommendations() {
        println!("   • {}", rec);
    }
}

fn demo_neurological_genetics() {
    println!("🧠 NEUROLOGICAL GENETICS PROFILE");
    println!("{}", "-".repeat(80));

    let mut profile = NeurologicalGeneticProfile::default();

    println!("\n1. LRRK2 G2019S (Parkinson's Risk):");
    profile.lrrk2 = Lrrk2Status::G2019S;
    println!("   - PD Risk: {:.0}x", profile.lrrk2.parkinsons_risk());
    println!("   - Penetrance at age 70: {:.0}%", profile.lrrk2.penetrance(70.0) * 100.0);

    println!("\n2. GBA1 N370S (Gaucher Carrier & PD Risk):");
    profile.gba1 = Gba1Status::N370S;
    println!("   - PD Risk: {:.1}x", profile.gba1.parkinsons_risk());
    println!("   - Gaucher Carrier: {}", profile.gba1.gaucher_disease_carrier());
    println!("   - Cognitive Decline Risk: {:.1}x", profile.gba1.cognitive_decline_risk());

    println!("\n3. HLA-DRB1*15 (MS Risk):");
    profile.hla_drb1 = HlaDrb1Status::DR15Positive;
    println!("   - MS Risk: {:.1}x", profile.hla_drb1.multiple_sclerosis_risk());

    println!("\n4. BDNF Val66Met Genotype:");
    profile.bdnf = Bdnf66Variant::MetMet;
    println!("   - Memory: {}", profile.bdnf.memory_performance());
    println!("   - Depression Risk: {:.1}x", profile.bdnf.depression_risk());
    println!("   - Exercise Response: {}", profile.bdnf.response_to_exercise());

    println!("\n5. COMT Val158Met (Worrier vs Warrior):");
    profile.comt = Comt158Variant::MetMet;
    println!("   - Type: {}", profile.comt.dopamine_metabolism());
    println!("   - Stress: {}", profile.comt.stress_response());
    println!("   - Pain: {}", profile.comt.pain_sensitivity());
    println!("   - Opioid Requirement: {:.0}%", profile.comt.opioid_requirement_multiplier() * 100.0);

    println!("\n6. SLC6A4 (Serotonin Transporter):");
    profile.slc6a4 = Slc6a4Variant::SS;
    println!("   - Activity: {}", profile.slc6a4.serotonin_transporter_activity());
    println!("   - Depression Risk (with stress): {:.1}x", profile.slc6a4.depression_risk_with_stress());
    println!("   - Anxiety Risk: {:.1}x", profile.slc6a4.anxiety_risk());
    println!("   - SSRI Response: {}", profile.slc6a4.ssri_response());

    println!("\nComprehensive Neurological Assessment:");
    println!("   - Parkinson's Risk (age 60): {:.1}x", profile.parkinsons_risk(60.0));
    println!("   - MS Risk: {:.1}x", profile.multiple_sclerosis_risk());

    println!("\nCognitive Profile:");
    for item in profile.cognitive_profile() {
        println!("   • {}", item);
    }

    println!("\nPsychiatric Risk Factors:");
    for factor in profile.psychiatric_risk_factors() {
        println!("   • {}", factor);
    }

    println!("\nPain Management:");
    for item in profile.pain_management_profile() {
        println!("   • {}", item);
    }
}

fn demo_cancer_genetics() {
    println!("🎗️  CANCER GENETICS PROFILE");
    println!("{}", "-".repeat(80));

    let mut profile = CancerGeneticProfile::default();

    println!("\n1. BRCA1 185delAG (Ashkenazi Founder Mutation):");
    profile.brca1 = Brca1Status::Pathogenic185delAG;
    println!("   - Breast Cancer Risk (age 50): {:.0}%", profile.brca1.breast_cancer_risk_by_age(50.0) * 100.0);
    println!("   - Ovarian Cancer Lifetime Risk: {:.0}%", profile.brca1.ovarian_cancer_lifetime_risk() * 100.0);

    println!("\n   Screening Recommendations:");
    for rec in profile.brca1.screening_recommendations() {
        println!("      • {}", rec);
    }

    println!("\n2. TP53 Mutation (Li-Fraumeni Syndrome):");
    profile.tp53 = Tp53Status::Pathogenic;
    println!("   - Li-Fraumeni: {}", profile.tp53.li_fraumeni_syndrome());
    println!("   - Any Cancer Risk (age 30): {:.0}%", profile.tp53.cancer_risk_by_age(30.0) * 100.0);

    println!("\n3. MLH1 (Lynch Syndrome):");
    profile.mlh_msh = MlhMshStatus::Mlh1Pathogenic;
    println!("   - Lynch Syndrome: {}", profile.mlh_msh.lynch_syndrome());
    println!("   - Colorectal Cancer Risk: {:.0}%", profile.mlh_msh.colorectal_cancer_lifetime_risk() * 100.0);
    println!("   - Endometrial Cancer Risk: {:.0}%", profile.mlh_msh.endometrial_cancer_lifetime_risk() * 100.0);

    println!("\n4. APC (Familial Adenomatous Polyposis):");
    profile.apc = ApcStatus::Pathogenic;
    println!("   - FAP: {}", profile.apc.familial_adenomatous_polyposis());
    println!("   - CRC Risk: {:.0}%", profile.apc.colorectal_cancer_risk() * 100.0);
    println!("   - Polyp Burden: {}", profile.apc.polyp_burden());

    println!("\nHereditary Cancer Syndromes:");
    for syndrome in profile.hereditary_cancer_syndromes() {
        println!("   • {}", syndrome);
    }

    println!("\nLifetime Cancer Risks (Female, Age 40):");
    for (cancer_type, risk) in profile.lifetime_cancer_risk_summary(40.0, true) {
        println!("   • {}: {:.1}%", cancer_type, risk * 100.0);
    }

    println!("\nPARP Inhibitor Eligibility: {}", if profile.parp_inhibitor_eligibility() { "YES" } else { "NO" });

    println!("\nImmunotherapy Biomarkers:");
    let biomarkers = profile.immunotherapy_biomarkers();
    if biomarkers.is_empty() {
        println!("   • Standard biomarker testing");
    } else {
        for biomarker in biomarkers {
            println!("   • {}", biomarker);
        }
    }
}

fn demo_respiratory_genetics() {
    println!("🫁 RESPIRATORY GENETICS PROFILE");
    println!("{}", "-".repeat(80));

    let mut profile = RespiratoryGeneticProfile::default();

    println!("\n1. CFTR ΔF508/ΔF508 (Cystic Fibrosis):");
    profile.cftr = CftrStatus::DeltaF508Homozygous;
    println!("   - CF: {}", profile.cftr.cystic_fibrosis());
    println!("   - Disease Severity: {}", profile.cftr.disease_severity());

    println!("\n   Treatment Options:");
    for treatment in profile.cftr.treatment_options() {
        println!("      • {}", treatment);
    }

    println!("\n2. SERPINA1 ZZ (Alpha-1 Antitrypsin Deficiency):");
    profile.serpina1 = SerpinA1Genotype::ZZ;
    println!("   - AAT Deficiency: {}", profile.serpina1.alpha1_antitrypsin_deficiency());
    println!("   - AAT Level: {:.0}% of normal", profile.serpina1.serum_aat_level_percent_normal());
    println!("   - COPD Risk: {:.0}x", profile.serpina1.copd_risk());
    println!("   - Liver Disease Risk: {:.0}x", profile.serpina1.liver_disease_risk());

    println!("\n3. ADRB2 Gly16Gly (Beta-Agonist Response):");
    profile.adrb2 = Adrb2Genotype::Gly16Gly;
    println!("   - Asthma Severity: {}", profile.adrb2.asthma_severity_modifier());
    println!("   - Beta-Agonist Response: {:.0}%", profile.adrb2.beta_agonist_response() * 100.0);

    println!("\n4. IL4RA Q576R (Asthma & Allergy):");
    profile.il4ra = Il4raVariant::Q576RHomozygous;
    println!("   - Asthma Risk: {:.1}x", profile.il4ra.asthma_risk());
    println!("   - IgE Levels: {:.0}% of baseline", profile.il4ra.ige_levels_modifier() * 100.0);

    println!("\n5. TNF-α -308 (Inflammation):");
    profile.tnfa = Tnfa308Variant::AA;
    println!("   - TNF Production: {}", profile.tnfa.tnf_alpha_production());
    println!("   - Asthma Severity Modifier: {:.1}x", profile.tnfa.asthma_severity_modifier());
    println!("   - COPD Risk: {:.1}x", profile.tnfa.copd_risk());

    println!("\nComprehensive Respiratory Assessment:");
    println!("   - Asthma Genetic Risk: {:.1}x", profile.asthma_genetic_risk());
    println!("   - COPD Risk (non-smoker): {:.1}x", profile.copd_genetic_risk(false));
    println!("   - COPD Risk (smoker): {:.1}x", profile.copd_genetic_risk(true));

    println!("\nAsthma Treatment Plan:");
    for item in profile.asthma_treatment_plan() {
        println!("   • {}", item);
    }

    println!("\nHigh-Priority Screening:");
    for screen in profile.high_priority_screening() {
        println!("   • {}", screen);
    }

    println!("\nCarrier Screening Implications:");
    for implication in profile.carrier_screening_implications() {
        println!("   • {}", implication);
    }
}

fn demo_athletic_profile() {
    println!("🏃 ATHLETIC PERFORMANCE GENETICS");
    println!("{}", "-".repeat(80));

    println!("\n1. Power Athlete Profile:");
    let power_profile = AthleticPerformanceProfile {
        actn3: Actn3Genotype::RR,
        ace: AceGenotype::DD,
        ppargc1a: Ppargc1aVariant::SS,
        col5a1: Col5a1Genotype::CC,
        ampd1: AmpD1Genotype::CC,
    };

    println!("   - ACTN3: {}", power_profile.actn3.power_performance());
    println!("   - ACE: {}", power_profile.ace.endurance_performance());
    println!("   - Power/Endurance Score: {:.1}", power_profile.power_vs_endurance_score());
    println!("   - Profile: {}", power_profile.athletic_profile_summary());

    println!("\n   Optimal Sports:");
    for sport in power_profile.optimal_sports() {
        println!("      • {}", sport);
    }

    println!("\n2. Endurance Athlete Profile:");
    let endurance_profile = AthleticPerformanceProfile {
        actn3: Actn3Genotype::XX,
        ace: AceGenotype::II,
        ppargc1a: Ppargc1aVariant::GG,
        col5a1: Col5a1Genotype::CT,
        ampd1: AmpD1Genotype::TT,
    };

    println!("   - ACTN3: {}", endurance_profile.actn3.power_performance());
    println!("   - ACE: {}", endurance_profile.ace.endurance_performance());
    println!("   - PPARGC1A: {}", endurance_profile.ppargc1a.mitochondrial_biogenesis());
    println!("   - Power/Endurance Score: {:.1}", endurance_profile.power_vs_endurance_score());
    println!("   - Profile: {}", endurance_profile.athletic_profile_summary());

    println!("\n   Optimal Sports:");
    for sport in endurance_profile.optimal_sports() {
        println!("      • {}", sport);
    }

    println!("\n   VO2 Max Potential (baseline 50):");
    println!("      {:.1} ml/kg/min", endurance_profile.vo2_max_genetic_potential(50.0));

    println!("\n3. Injury Risk Profile:");
    let injury_profile = AthleticPerformanceProfile {
        actn3: Actn3Genotype::RX,
        ace: AceGenotype::ID,
        ppargc1a: Ppargc1aVariant::GS,
        col5a1: Col5a1Genotype::TT,
        ampd1: AmpD1Genotype::CC,
    };

    println!("   - COL5A1: {}", injury_profile.col5a1.tendon_injury_risk());
    println!("   - Injury Risk Multiplier: {:.1}x", injury_profile.col5a1.injury_risk_multiplier());

    println!("\n   Injury Prevention Priorities:");
    for priority in injury_profile.injury_prevention_priorities() {
        println!("      • {}", priority);
    }
}

fn demo_comprehensive_assessment() {
    println!("📋 COMPREHENSIVE DIAGNOSTIC ASSESSMENT");
    println!("{}", "-".repeat(80));

    let person = Human::new_adult_female("comprehensive_test".to_string(), 45.0, 165.0, 70.0);

    let report = person.comprehensive_diagnostic_assessment();

    println!("\nPatient: {}", report.patient_id);
    println!("Overall Health Score: {:.1}/100", report.overall_health_score);

    println!("\nGenetic Risk Profile:");
    println!("   Cardiovascular: {} ({:.1}x)",
        report.genetic_risk_profile.cardiovascular_risk.risk_category.as_str(),
        report.genetic_risk_profile.cardiovascular_risk.overall_risk_multiplier);
    println!("   Neurological: {} ({:.1}x)",
        report.genetic_risk_profile.neurological_risk.risk_category.as_str(),
        report.genetic_risk_profile.neurological_risk.overall_risk_multiplier);
    println!("   Respiratory: {} ({:.1}x)",
        report.genetic_risk_profile.respiratory_risk.risk_category.as_str(),
        report.genetic_risk_profile.respiratory_risk.overall_risk_multiplier);
    println!("   Metabolic: {} ({:.1}x)",
        report.genetic_risk_profile.metabolic_risk.risk_category.as_str(),
        report.genetic_risk_profile.metabolic_risk.overall_risk_multiplier);

    println!("\nCancer Risks:");
    for cancer_risk in &report.genetic_risk_profile.cancer_risk {
        println!("   • {}: {:.1}% ({}, screening at age {})",
            cancer_risk.cancer_type,
            cancer_risk.lifetime_risk * 100.0,
            cancer_risk.risk_category.as_str(),
            cancer_risk.screening_age.map(|a| a.to_string()).unwrap_or("N/A".to_string()));
    }

    println!("\nSystem Health Assessment:");
    println!("   Cardiovascular: {:.0}/100", report.system_health_assessment.cardiovascular_health);
    println!("   Respiratory: {:.0}/100", report.system_health_assessment.respiratory_health);
    println!("   Neurological: {:.0}/100", report.system_health_assessment.neurological_health);
    println!("   Metabolic: {:.0}/100", report.system_health_assessment.metabolic_health);
    println!("   Renal: {:.0}/100", report.system_health_assessment.renal_health);
    println!("   Immune: {:.0}/100", report.system_health_assessment.immune_health);

    if !report.urgent_alerts.is_empty() {
        println!("\n⚠️  URGENT ALERTS:");
        for alert in &report.urgent_alerts {
            println!("   {}", alert);
        }
    }

    println!("\nTop Screening Priorities:");
    for (i, screening) in report.screening_priorities.iter().take(5).enumerate() {
        println!("   {}. {} - {}", i+1, screening.test_name, screening.frequency);
        println!("      Rationale: {}", screening.rationale);
    }

    println!("\nLifestyle Recommendations (Top 5):");
    for (i, rec) in report.lifestyle_recommendations.iter().take(5).enumerate() {
        println!("   {}. {}", i+1, rec);
    }
}
