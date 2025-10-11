use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_renal_data(db: &mut GroundTruthDatabase) {
    let mut renal_data = GroundTruthData::new(
        "Renal".to_string(),
        "Normal renal function parameters in healthy adults".to_string(),
    );

    renal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gfr_ml_per_min_1_73m2".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(15.0),
        min_value: Some(90.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("23062522".to_string()),
            doi: Some("10.1053/j.ajkd.2012.08.033".to_string()),
            citation: "Levey AS et al. (2013) Am J Kidney Dis 61(1):152-159".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(8254),
            population: "Healthy adults aged 18-70".to_string(),
        },
    });

    renal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_sodium_meq_l".to_string(),
        expected_value: 140.0,
        standard_deviation: Some(3.0),
        min_value: Some(135.0),
        max_value: Some(145.0),
        reference: ClinicalReference {
            pmid: Some("30726688".to_string()),
            doi: Some("10.1016/j.kint.2018.10.016".to_string()),
            citation: "Filippatos TD et al. (2019) Kidney Int 95(2):375-389".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "General adult population".to_string(),
        },
    });

    renal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_potassium_meq_l".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(0.4),
        min_value: Some(3.5),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("28827314".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.117.09551".to_string()),
            citation: "Palmer BF & Clegg DJ (2017) Hypertension 70(5):e38-e47".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Healthy adults".to_string(),
        },
    });

    renal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_output_ml_per_hr".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(20.0),
        min_value: Some(30.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("25572383".to_string()),
            doi: Some("10.1097/CCM.0000000000000794".to_string()),
            citation: "Prowle JR et al. (2015) Crit Care Med 43(4):791-799".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1200),
            population: "Adult ICU patients".to_string(),
        },
    });

    renal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_creatinine_mg_dl_male".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.2),
        min_value: Some(0.7),
        max_value: Some(1.3),
        reference: ClinicalReference {
            pmid: Some("32657974".to_string()),
            doi: Some("10.1093/ndt/gfz282".to_string()),
            citation: "Delgado C et al. (2020) Nephrol Dial Transplant 35(2):185-191"
                .to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5504),
            population: "Healthy adult males".to_string(),
        },
    });

    renal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_creatinine_mg_dl_female".to_string(),
        expected_value: 0.9,
        standard_deviation: Some(0.2),
        min_value: Some(0.6),
        max_value: Some(1.1),
        reference: ClinicalReference {
            pmid: Some("32657974".to_string()),
            doi: Some("10.1093/ndt/gfz282".to_string()),
            citation: "Delgado C et al. (2020) Nephrol Dial Transplant 35(2):185-191"
                .to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5504),
            population: "Healthy adult females".to_string(),
        },
    });

    db.add_dataset("renal".to_string(), renal_data);
}
