use human_biology::pathology::*;

fn main() {
    println!("=== Comprehensive Medical Assessment Example ===\n");

    dermatology_assessment();
    println!();

    ophthalmology_assessment();
    println!();

    psychiatric_assessment();
    println!();

    rheumatology_assessment();
    println!();

    toxicology_assessment();
    println!();

    allergy_assessment();
    println!();

    pulmonology_assessment();
    println!();

    gastroenterology_assessment();
}

fn dermatology_assessment() {
    println!("📋 DERMATOLOGY ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = DermatologyProfile::new(FitzpatrickType::TypeII);

    profile.add_lesion(SkinLesion {
        lesion_type: LesionType::Nevus,
        location: BodyRegion::Back,
        size_mm: 8.5,
        color: LesionColor::Variegated,
        border: BorderCharacteristic::Irregular,
        evolution: EvolutionPattern::Growing,
    });

    println!("Skin Type: {:?}", profile.skin_type);
    println!("Melanoma Relative Risk: {:.1}x", profile.skin_type.melanoma_relative_risk());
    println!("Sun Protection: {}", profile.skin_type.sun_protection_recommendation());

    if let Some(lesion) = profile.lesions.first() {
        let abcde = profile.assess_lesion_abcde(lesion);
        println!("\nABCDE Assessment:");
        println!("  - Border Irregularity: {}", abcde.border_irregularity);
        println!("  - Color Variation: {}", abcde.color_variation);
        println!("  - Diameter > 6mm: {}", abcde.diameter_gt_6mm);
        println!("  - Evolving: {}", abcde.evolving);

        let concerning = abcde.border_irregularity as u8 + abcde.color_variation as u8 +
                        abcde.diameter_gt_6mm as u8 + abcde.evolving as u8;
        println!("  ⚠️  Concerning features: {}/4", concerning);
    }

    println!("Overall Melanoma Risk: {:?}", profile.melanoma_risk.overall_risk);
}

fn ophthalmology_assessment() {
    println!("👁️  OPHTHALMOLOGY ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = OphthalmologyProfile::new();

    profile.visual_acuity.right_eye = SnellenAcuity { numerator: 20, denominator: 40 };
    profile.refractive_error.right_eye = Refraction {
        sphere: -3.0,
        cylinder: -1.5,
        axis: 90.0,
        error_type: RefractiveErrorType::Myopia,
    };

    profile.intraocular_pressure.right_eye_mmhg = 24.0;
    profile.retinal_health.cup_to_disc_ratio.right_eye = 0.65;

    println!("Right Eye Visual Acuity: 20/{}", profile.visual_acuity.right_eye.denominator);
    println!("  - Decimal: {:.2}", profile.visual_acuity.right_eye.decimal());
    println!("  - LogMAR: {:.2}", profile.visual_acuity.right_eye.logmar());

    let se = profile.refractive_error.right_eye.spherical_equivalent();
    println!("\nRefractive Error (RE):");
    println!("  - Spherical Equivalent: {:.2}D", se);
    println!("  - Type: {:?}", profile.refractive_error.right_eye.error_type);

    println!("\nIntraocular Pressure: {} mmHg", profile.intraocular_pressure.right_eye_mmhg);
    println!("Cup-to-Disc Ratio: {:.2}", profile.retinal_health.cup_to_disc_ratio.right_eye);
    println!("Glaucoma Risk: {:?}", profile.glaucoma_risk());
}

fn psychiatric_assessment() {
    println!("🧠 PSYCHIATRIC ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = PsychiatricProfile::new();

    profile.add_diagnosis(PsychiatricDiagnosis {
        disorder: DisorderType::MajorDepressiveDisorder,
        onset_age: Some(28.0),
        duration_months: Some(8),
        severity: PsychiatricSeverity::Moderate,
        episode_type: Some(EpisodeType::Depressive),
    });

    profile.symptom_severity.depression_score.phq9_score = 16;
    profile.symptom_severity.anxiety_score.gad7_score = 12;

    println!("Primary Diagnosis: Major Depressive Disorder");
    println!("Duration: 8 months");

    println!("\nSymptom Severity:");
    println!("  - PHQ-9 Score: {} ({})",
        profile.symptom_severity.depression_score.phq9_score,
        profile.symptom_severity.depression_score.severity()
    );
    println!("  - GAD-7 Score: {} ({})",
        profile.symptom_severity.anxiety_score.gad7_score,
        profile.symptom_severity.anxiety_score.severity()
    );

    let gaf = profile.overall_functioning();
    println!("\nGlobal Functioning: {:?}", gaf);
    println!("Suicide Risk: {:?}", profile.risk_assessments.suicide_risk);
}

fn rheumatology_assessment() {
    println!("🦴 RHEUMATOLOGY ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = RheumatologyProfile::new();

    profile.add_diagnosis(RheumatologicDiagnosis {
        condition: RheumatologicCondition::RheumatoidArthritis,
        disease_activity: DiseaseActivity::Moderate,
        onset_age: Some(42.0),
        duration_years: Some(3.0),
    });

    profile.update_joint(Joint::RightWrist, JointStatus {
        pain_score: 6,
        swelling: SwellingGrade::Moderate,
        tenderness: true,
        range_of_motion_percent: 70.0,
        deformity: None,
    });

    profile.update_joint(Joint::LeftWrist, JointStatus {
        pain_score: 5,
        swelling: SwellingGrade::Mild,
        tenderness: true,
        range_of_motion_percent: 75.0,
        deformity: None,
    });

    profile.inflammatory_markers.esr_mm_hr = 42.0;
    profile.inflammatory_markers.crp_mg_l = 18.0;

    profile.autoantibodies.rheumatoid_factor = AntibodyResult {
        positive: true,
        titer: Some(120.0),
    };

    println!("Diagnosis: Rheumatoid Arthritis (3 years)");

    if let Some(das28) = profile.das28_score() {
        println!("\nDAS28 Score: {:.2}", das28);
        let activity = if das28 > 5.1 { "High" } else if das28 > 3.2 { "Moderate" } else { "Low" };
        println!("Disease Activity: {}", activity);
    }

    println!("\nInflammatory Markers:");
    println!("  - ESR: {} mm/hr", profile.inflammatory_markers.esr_mm_hr);
    println!("  - CRP: {} mg/L", profile.inflammatory_markers.crp_mg_l);
    println!("  - Inflammatory Burden: {:?}", profile.inflammatory_burden());

    println!("\nAutoantibodies:");
    println!("  - RF: Positive (titer {})", profile.autoantibodies.rheumatoid_factor.titer.unwrap());

    let affected = profile.joint_status.len();
    println!("\nAffected Joints: {}", affected);
}

fn toxicology_assessment() {
    println!("☠️  TOXICOLOGY ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = ToxicologyProfile::new();

    profile.add_overdose(Overdose {
        substance: "Opioid - Oxycodone".to_string(),
        estimated_dose_mg: 120.0,
        time_since_ingestion_hours: 1.5,
        treatment: OverdoseTreatment {
            activated_charcoal_given: false,
            gastric_lavage: false,
            antidote: None,
            supportive_care: vec![],
        },
    });

    profile.add_exposure(ToxicExposure {
        substance: ToxicSubstance {
            name: "Oxycodone".to_string(),
            category: SubstanceCategory::Pharmaceutical,
            lethal_dose_50_mg_kg: Some(50.0),
        },
        route: ExposureRoute::Ingestion,
        dose_mg: Some(120.0),
        time_since_exposure_hours: 1.5,
        symptoms: vec![
            ToxicSymptom::Nausea,
            ToxicSymptom::Dizziness,
            ToxicSymptom::RespiratoryDepression,
        ],
    });

    println!("Overdose: Oxycodone 120mg (1.5 hours ago)");

    if let Some(antidote) = profile.requires_antidote() {
        println!("\n⚠️  ANTIDOTE REQUIRED: {:?}", antidote);
        println!("  - Route: {}", antidote.administration_route());
        println!("  - Mechanism: {}", antidote.mechanism());
    }

    let severity = profile.severity_score();
    println!("\nToxicity Severity: {:?}", severity);

    if let Some(exposure) = profile.exposures.first() {
        println!("Symptoms ({}):", exposure.symptoms.len());
        for symptom in &exposure.symptoms {
            println!("  - {:?}", symptom);
        }
    }
}

fn allergy_assessment() {
    println!("🤧 ALLERGY & IMMUNOLOGY ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = AllergyImmunologyProfile::new();

    profile.add_allergy(Allergy {
        allergen: Allergen {
            name: "Peanuts".to_string(),
            category: AllergenCategory::Food,
        },
        reaction_type: AllergyType::IgEMediated,
        severity: AllergySeverity::Anaphylactic,
        onset_age: Some(3.0),
        symptoms: vec![
            AllergySymptom::Urticaria,
            AllergySymptom::Angioedema,
            AllergySymptom::Dyspnea,
            AllergySymptom::Hypotension,
        ],
    });

    profile.ig_levels.ige_iu_ml = 320.0;

    println!("Primary Allergy: Peanuts (IgE-mediated)");
    println!("Severity: Anaphylactic");
    println!("Onset Age: 3 years");

    println!("\nAnaphylaxis Risk: {:?}", profile.anaphylaxis_risk());
    println!("EpiPen Required: {}", if profile.requires_epipen() { "YES" } else { "NO" });

    println!("\nImmunoglobulin Levels:");
    println!("  - IgA: {:.0} mg/dL", profile.ig_levels.iga_mg_dl);
    println!("  - IgG: {:.0} mg/dL", profile.ig_levels.igg_mg_dl);
    println!("  - IgM: {:.0} mg/dL", profile.ig_levels.igm_mg_dl);
    println!("  - IgE: {:.0} IU/mL ({})",
        profile.ig_levels.ige_iu_ml,
        if profile.ig_levels.has_elevated_ige() { "ELEVATED" } else { "Normal" }
    );
}

fn pulmonology_assessment() {
    println!("🫁 PULMONOLOGY ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = PulmonologyProfile::new();

    profile.add_condition(PulmonaryCondition {
        diagnosis: PulmonaryDiagnosis::COPD,
        severity: PulmonarySeverity::Moderate,
        exacerbations_per_year: 2,
        treatment_adherence: TreatmentAdherence::Good,
    });

    profile.pulmonary_function.fev1_liters = 2.0;
    profile.pulmonary_function.fvc_liters = 3.5;
    profile.pulmonary_function.fev1_fvc_ratio = 0.57;

    profile.gas_exchange.pao2_mmhg = 65.0;
    profile.gas_exchange.paco2_mmhg = 48.0;
    profile.gas_exchange.sao2_percent = 91.0;

    println!("Diagnosis: COPD (Moderate)");
    println!("Exacerbations: 2/year");

    println!("\nSpirometry:");
    println!("  - FEV1: {:.1}L ({:.0}% predicted)",
        profile.pulmonary_function.fev1_liters,
        profile.pulmonary_function.fev1_percent_predicted()
    );
    println!("  - FVC: {:.1}L", profile.pulmonary_function.fvc_liters);
    println!("  - FEV1/FVC: {:.2}", profile.pulmonary_function.fev1_fvc_ratio);
    println!("  - Pattern: {:?}", profile.spirometry_pattern());

    if let Some(gold) = profile.gold_copd_stage() {
        println!("\nGOLD Stage: {:?}", gold);
    }

    println!("\nGas Exchange:");
    println!("  - PaO2: {} mmHg", profile.gas_exchange.pao2_mmhg);
    println!("  - PaCO2: {} mmHg", profile.gas_exchange.paco2_mmhg);
    println!("  - SaO2: {}%", profile.gas_exchange.sao2_percent);

    if let Some(rf_type) = profile.respiratory_failure_type() {
        println!("  ⚠️  Respiratory Failure: {:?}", rf_type);
    }
}

fn gastroenterology_assessment() {
    println!("🫀 GASTROENTEROLOGY ASSESSMENT");
    println!("─────────────────────────");

    let mut profile = GastroenterologyProfile::new();

    profile.add_condition(GICondition {
        diagnosis: GIDiagnosis::Cirrhosis,
        severity: GISeverity::Moderate,
        location: GILocation::Liver,
        complications: vec![GIComplication::Bleeding],
    });

    profile.liver_function.alt_u_l = 85.0;
    profile.liver_function.ast_u_l = 120.0;
    profile.liver_function.bilirubin_total_mg_dl = 2.8;
    profile.liver_function.albumin_g_dl = 3.0;
    profile.liver_function.inr = 1.9;

    println!("Diagnosis: Cirrhosis (Moderate)");
    println!("Complication: Variceal Bleeding");

    println!("\nLiver Function Tests:");
    println!("  - ALT: {} U/L", profile.liver_function.alt_u_l);
    println!("  - AST: {} U/L", profile.liver_function.ast_u_l);
    println!("  - Total Bilirubin: {:.1} mg/dL", profile.liver_function.bilirubin_total_mg_dl);
    println!("  - Albumin: {:.1} g/dL", profile.liver_function.albumin_g_dl);
    println!("  - INR: {:.1}", profile.liver_function.inr);

    if profile.liver_function.hepatocellular_pattern() {
        println!("  - Pattern: Hepatocellular injury");
    }

    if let Some(child_pugh) = profile.child_pugh_score() {
        println!("\nChild-Pugh Score: {:?}", child_pugh);
        let mortality = match child_pugh {
            ChildPughScore::ClassA => "Good prognosis",
            ChildPughScore::ClassB => "Moderate prognosis, consider transplant evaluation",
            ChildPughScore::ClassC => "Poor prognosis, urgent transplant evaluation",
        };
        println!("  → {}", mortality);
    }

    println!("\nHepatic Encephalopathy Risk: {:?}", profile.hepatic_encephalopathy_risk());
}
