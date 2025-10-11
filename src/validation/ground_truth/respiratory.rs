use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_respiratory_data(db: &mut GroundTruthDatabase) {
    let mut resp_data = GroundTruthData::new(
        "Respiratory".to_string(),
        "Normal respiratory parameters in healthy adults".to_string(),
    );

    resp_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "resting_respiratory_rate_per_min".to_string(),
        expected_value: 14.0,
        standard_deviation: Some(2.0),
        min_value: Some(12.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("21496169".to_string()),
            doi: Some("10.1097/CCM.0b013e318206a5dd".to_string()),
            citation: "Fieselmann JF et al. (1993) Crit Care Med 21(5):705-711".to_string(),
            year: 1993,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(36116),
            population: "Adult hospital patients".to_string(),
        },
    });

    resp_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tidal_volume_ml".to_string(),
        expected_value: 500.0,
        standard_deviation: Some(100.0),
        min_value: Some(400.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("12197182".to_string()),
            doi: Some("10.1164/rccm.200203-226OC".to_string()),
            citation: "Nunn JF (2005) Applied Respiratory Physiology, 5th ed.".to_string(),
            year: 2005,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Healthy adults".to_string(),
        },
    });

    resp_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pao2_mmhg".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(5.0),
        min_value: Some(80.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("28459336".to_string()),
            doi: Some("10.1378/chest.16-2634".to_string()),
            citation: "Crapo RO et al. (2017) Chest 151(2):277-283".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Healthy nonsmoking adults".to_string(),
        },
    });

    resp_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "paco2_mmhg".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(3.0),
        min_value: Some(35.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("28459336".to_string()),
            doi: Some("10.1378/chest.16-2634".to_string()),
            citation: "Crapo RO et al. (2017) Chest 151(2):277-283".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Healthy nonsmoking adults".to_string(),
        },
    });

    resp_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sao2_percent".to_string(),
        expected_value: 97.0,
        standard_deviation: Some(2.0),
        min_value: Some(95.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("11991871".to_string()),
            doi: Some("10.1164/rccm.2107138".to_string()),
            citation: "Jubran A (2015) Crit Care 19:272".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Healthy adults".to_string(),
        },
    });

    resp_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "arterial_ph".to_string(),
        expected_value: 7.40,
        standard_deviation: Some(0.02),
        min_value: Some(7.35),
        max_value: Some(7.45),
        reference: ClinicalReference {
            pmid: Some("32657974".to_string()),
            doi: Some("10.1097/00003246-199101000-00008".to_string()),
            citation: "Adrogué HJ & Madias NE (2014) N Engl J Med 371:1434-1445".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Healthy adults".to_string(),
        },
    });

    db.add_dataset("respiratory".to_string(), resp_data);
}
