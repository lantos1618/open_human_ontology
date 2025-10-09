use human_biology::{
    Human,
    biology::genetics::*,
    pathology::neurological::*,
    pathology::metabolic,
    pathology::cardiovascular,
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║         COMPREHENSIVE DISEASE RISK ANALYSIS DEMONSTRATION                ║");
    println!("║                   Open Human Ontology Project                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝\n");

    demo_cardiovascular_risk_assessment();
    demo_metabolic_syndrome_diagnosis();
    demo_diabetes_management();
    demo_neurological_risk_profile();
    demo_genetic_risk_integration();
}

fn demo_cardiovascular_risk_assessment() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 1: Cardiovascular Risk Assessment                                 │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut cv_profile = cardiovascular::CardiovascularRiskProfile::new(
        55.0,
        cardiovascular::BiologicalSex::Male
    );

    cv_profile.systolic_bp_mmhg = 145.0;
    cv_profile.total_cholesterol_mg_dl = 240.0;
    cv_profile.hdl_cholesterol_mg_dl = 38.0;
    cv_profile.ldl_cholesterol_mg_dl = 160.0;
    cv_profile.smoker = true;
    cv_profile.diabetes = false;
    cv_profile.family_history_cad = true;
    cv_profile.bmi = 29.5;

    println!("  📊 Patient Profile:");
    println!("     Age: {} years", cv_profile.age);
    println!("     Sex: {:?}", cv_profile.biological_sex);
    println!("     Blood Pressure: {} mmHg systolic", cv_profile.systolic_bp_mmhg);
    println!("     Total Cholesterol: {} mg/dL", cv_profile.total_cholesterol_mg_dl);
    println!("     HDL Cholesterol: {} mg/dL", cv_profile.hdl_cholesterol_mg_dl);
    println!("     LDL Cholesterol: {} mg/dL", cv_profile.ldl_cholesterol_mg_dl);
    println!("     Smoker: {}", cv_profile.smoker);
    println!("     BMI: {:.1}", cv_profile.bmi);

    let framingham_risk = cv_profile.framingham_10_year_risk();
    let ascvd_risk = cv_profile.ascvd_10_year_risk();
    let lifetime_risk = cv_profile.lifetime_risk();
    let risk_category = cv_profile.risk_category();
    let needs_statin = cv_profile.requires_statin();

    println!("\n  🎯 Risk Assessment:");
    println!("     Framingham 10-Year Risk: {:.1}%", framingham_risk);
    println!("     ASCVD 10-Year Risk: {:.1}%", ascvd_risk);
    println!("     Lifetime Risk: {:.1}%", lifetime_risk);
    println!("     Risk Category: {:?}", risk_category);

    println!("\n  💊 Treatment Recommendations:");
    if needs_statin {
        println!("     ✓ Statin therapy INDICATED");
        println!("       - Consider moderate-to-high intensity statin");
        println!("       - Target LDL < 100 mg/dL");
    }
    println!("     ✓ Smoking cessation CRITICAL");
    println!("     ✓ Blood pressure management (target < 130/80 mmHg)");
    println!("     ✓ Weight loss (target BMI < 25)");
    println!("     ✓ Aerobic exercise 150 min/week");
}

fn demo_metabolic_syndrome_diagnosis() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 2: Metabolic Syndrome Diagnosis                                   │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let assessment = metabolic::MetabolicSyndromeAssessment {
        waist_circumference_cm: 108.0,
        triglycerides_mg_dl: 185.0,
        hdl_cholesterol_mg_dl: 36.0,
        systolic_bp_mmhg: 138.0,
        diastolic_bp_mmhg: 88.0,
        fasting_glucose_mg_dl: 108.0,
        biological_sex: metabolic::BiologicalSex::Male,
    };

    println!("  📏 Metabolic Measurements:");
    println!("     Waist Circumference: {} cm", assessment.waist_circumference_cm);
    println!("     Triglycerides: {} mg/dL", assessment.triglycerides_mg_dl);
    println!("     HDL Cholesterol: {} mg/dL", assessment.hdl_cholesterol_mg_dl);
    println!("     Blood Pressure: {}/{} mmHg", assessment.systolic_bp_mmhg, assessment.diastolic_bp_mmhg);
    println!("     Fasting Glucose: {} mg/dL", assessment.fasting_glucose_mg_dl);

    let has_metabolic_syndrome = assessment.has_metabolic_syndrome();
    let criteria = assessment.criteria_met();
    let cv_risk_multiplier = assessment.cardiovascular_risk_multiplier();

    println!("\n  🔬 Diagnosis:");
    if has_metabolic_syndrome {
        println!("     ⚠️  METABOLIC SYNDROME DIAGNOSED");
        println!("     Criteria met: {}/5", criteria.len());
        println!("\n  📋 Criteria Present:");
        for criterion in &criteria {
            println!("     • {}", criterion);
        }
        println!("\n  ⚕️  Clinical Implications:");
        println!("     • Cardiovascular risk increased by {:.1}x", cv_risk_multiplier);
        println!("     • Type 2 diabetes risk significantly elevated");
        println!("     • Consider comprehensive lifestyle intervention");
        println!("     • May require pharmacological intervention");
    } else {
        println!("     ✓ No metabolic syndrome");
    }
}

fn demo_diabetes_management() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 3: Type 2 Diabetes Management                                     │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut diabetes_profile = metabolic::DiabetesProfile::new(metabolic::DiabetesType::Type2);
    diabetes_profile.hba1c = 8.2;
    diabetes_profile.fasting_glucose_mg_dl = 165.0;
    diabetes_profile.time_since_diagnosis_years = 5.0;
    diabetes_profile.insulin_dependent = false;

    diabetes_profile.add_complication(metabolic::DiabeticComplication::Neuropathy);
    diabetes_profile.add_complication(metabolic::DiabeticComplication::Retinopathy);

    diabetes_profile.genetic_risk_factors.push("TCF7L2 risk variant".to_string());
    diabetes_profile.genetic_risk_factors.push("PPARG Pro12Ala".to_string());

    println!("  🩺 Diabetes Profile:");
    println!("     Type: {:?}", diabetes_profile.diabetes_type);
    println!("     HbA1c: {:.1}%", diabetes_profile.hba1c);
    println!("     Estimated Average Glucose: {:.0} mg/dL", diabetes_profile.estimated_glucose_mg_dl());
    println!("     Fasting Glucose: {:.0} mg/dL", diabetes_profile.fasting_glucose_mg_dl);
    println!("     Years Since Diagnosis: {:.1}", diabetes_profile.time_since_diagnosis_years);

    println!("\n  📊 Glycemic Control:");
    println!("     Status: {}", diabetes_profile.risk_category());
    if diabetes_profile.is_well_controlled() {
        println!("     ✓ Target achieved");
    } else {
        println!("     ⚠️  NOT at target (goal HbA1c < 7.0%)");
    }

    println!("\n  🧬 Genetic Risk Factors:");
    for factor in &diabetes_profile.genetic_risk_factors {
        println!("     • {}", factor);
    }

    println!("\n  ⚕️  Complications Present:");
    for complication in &diabetes_profile.complications {
        println!("     • {:?}", complication);
    }
    println!("     Total Complications: {}", diabetes_profile.complication_count());

    println!("\n  💊 Management Recommendations:");
    println!("     • Intensify glucose-lowering therapy");
    println!("     • Target HbA1c < 7.0%");
    println!("     • Annual ophthalmology exam (retinopathy present)");
    println!("     • Foot care and neuropathy screening");
    println!("     • Consider SGLT2 inhibitor or GLP-1 agonist");
    println!("     • Cardiovascular risk reduction (statin + ASA)");
}

fn demo_neurological_risk_profile() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 4: Neurological Risk Profile - Alzheimer's Disease                │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut neuro_profile = NeurologicalProfile::new();
    neuro_profile.add_genetic_risk("APOE ε4/ε4".to_string(), 12.0);
    neuro_profile.add_genetic_risk("TREM2 R47H".to_string(), 2.5);
    neuro_profile.add_genetic_risk("CLU".to_string(), 1.15);

    neuro_profile.add_symptom(NeurologicalSymptom::MemoryLoss);
    neuro_profile.add_symptom(NeurologicalSymptom::CognitiveImpairment);
    neuro_profile.quality_of_life_score = 75.0;

    let genetic_risk = GeneticNeurologicalRisk {
        alzheimers_apoe_status: APOEStatus::E4E4,
        parkinsons_lrrk2: false,
        parkinsons_snca: false,
        parkinsons_gba: false,
        migraine_susceptibility: vec![],
        epilepsy_susceptibility: vec![],
    };

    println!("  🧬 Genetic Risk Assessment:");
    println!("     APOE Status: {:?}", genetic_risk.alzheimers_apoe_status);
    println!("     Alzheimer's Risk: {:.1}x baseline", genetic_risk.alzheimers_apoe_status.alzheimers_risk());
    println!("     Lifetime Risk: {:.1}%", genetic_risk.alzheimers_lifetime_risk() * 100.0);

    if genetic_risk.alzheimers_apoe_status.is_high_risk() {
        println!("     ⚠️  HIGH GENETIC RISK - ε4/ε4 carrier");
    }

    println!("\n  📋 Additional Risk Factors:");
    for insight in neuro_profile.get_genetic_insights() {
        println!("     • {}", insight);
    }

    println!("\n  🧠 Current Symptoms:");
    for symptom in &neuro_profile.symptoms {
        println!("     • {:?}", symptom);
    }

    println!("\n  📊 Clinical Assessment:");
    println!("     Neurological Burden Score: {:.1}/100", neuro_profile.calculate_burden_score());
    println!("     Quality of Life Score: {:.1}/100", neuro_profile.quality_of_life_score);

    println!("\n  💡 Recommendations:");
    println!("     • Neuropsychological testing recommended");
    println!("     • Consider brain MRI with volumetric analysis");
    println!("     • CSF biomarkers (Aβ42, tau, p-tau) or amyloid PET");
    println!("     • Aggressive cardiovascular risk factor management");
    println!("     • Cognitive stimulation and exercise program");
    println!("     • Consider clinical trial participation");
    println!("     • Family counseling regarding genetic risk");
}

fn demo_genetic_risk_integration() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 5: Integrated Genetic Risk Assessment                             │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut person = Human::new_adult_male("HIGH_RISK_001".to_string(), 52.0, 178.0, 95.0);

    person.genetics.ancestry.add_component(Ancestry::European, 85.0).unwrap();
    person.genetics.ancestry.add_component(Ancestry::EastAsian, 15.0).unwrap();

    let cv_genetic_risk = cardiovascular::GeneticCardiovascularRisk {
        apoe_status: cardiovascular::APOEStatus::E4E4,
        factor_v_leiden: true,
        prothrombin_g20210a: false,
        mthfr_c677t: cardiovascular::MTHFRStatus::Heterozygous,
        apob_variants: vec![],
        pcsk9_variants: vec![],
        ldlr_variants: vec!["Pathogenic variant".to_string()],
    };

    println!("  👤 Patient: {}", person.id);
    println!("     Age: {} years", person.demographics.age_years);
    println!("     BMI: {:.1}", person.bmi());

    println!("\n  🧬 Genetic Risk Profile:");
    println!("\n     Cardiovascular Genetics:");
    println!("       • APOE: {:?} (CV risk: {:.1}x)",
        cv_genetic_risk.apoe_status,
        cv_genetic_risk.apoe_status.cardiovascular_risk()
    );

    if cv_genetic_risk.factor_v_leiden {
        println!("       • Factor V Leiden: POSITIVE");
        println!("         - Thrombosis risk: {:.1}x baseline", cv_genetic_risk.thrombosis_risk());
        println!("         - Avoid oral contraceptives and smoking");
        println!("         - Consider thromboprophylaxis for surgery");
    }

    if cv_genetic_risk.has_familial_hypercholesterolemia() {
        println!("       • Familial Hypercholesterolemia: SUSPECTED");
        println!("         - LDLR pathogenic variant detected");
        println!("         - High-intensity statin therapy required");
        println!("         - Consider ezetimibe or PCSK9 inhibitor");
        println!("         - Screen first-degree relatives");
    }

    println!("\n     Neurological Genetics:");
    println!("       • APOE ε4/ε4: 12x Alzheimer's risk");
    println!("       • Lifetime Alzheimer's risk: ~50-60%");

    println!("\n  ⚕️  Comprehensive Management Plan:");
    println!("\n     Cardiovascular:");
    println!("       1. High-intensity statin (rosuvastatin 40mg or atorvastatin 80mg)");
    println!("       2. Target LDL < 70 mg/dL (consider < 55 mg/dL)");
    println!("       3. Ezetimibe 10mg daily");
    println!("       4. Consider PCSK9 inhibitor if LDL remains elevated");
    println!("       5. Aspirin 81mg daily for primary prevention");
    println!("       6. Annual lipid panel and cardiovascular assessment");

    println!("\n     Thrombosis Prevention:");
    println!("       1. Avoid prolonged immobility");
    println!("       2. Thromboprophylaxis for high-risk situations");
    println!("       3. Genetic counseling for family members");

    println!("\n     Neuroprotection:");
    println!("       1. Optimize cardiovascular risk factors (crucial for brain health)");
    println!("       2. Mediterranean diet with emphasis on omega-3");
    println!("       3. Regular aerobic exercise (150 min/week)");
    println!("       4. Cognitive engagement activities");
    println!("       5. Consider baseline cognitive assessment");
    println!("       6. Monitor for early cognitive changes");

    println!("\n  📈 Follow-up Schedule:");
    println!("     • Lipid panel: Every 3 months initially, then every 6 months");
    println!("     • Cardiovascular assessment: Every 6 months");
    println!("     • Cognitive screening: Annually starting at age 60");
    println!("     • Genetic counseling: Arrange for first-degree relatives");
}
