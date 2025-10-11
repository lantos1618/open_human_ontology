use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_cardiovascular_data(db: &mut GroundTruthDatabase) {
    let mut cv_data = GroundTruthData::new(
        "Cardiovascular".to_string(),
        "Normal resting cardiovascular parameters in healthy adults".to_string(),
    );

    cv_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "resting_heart_rate_bpm".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(10.0),
        min_value: Some(60.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("25910639".to_string()),
            doi: Some("10.1161/JAHA.114.001377".to_string()),
            citation: "Reimers AK et al. (2015) J Am Heart Assoc 4(5):e001377".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(92757),
            population: "General adult population".to_string(),
        },
    });

    cv_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "systolic_bp_mmhg".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(10.0),
        min_value: Some(90.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("24222015".to_string()),
            doi: Some("10.1001/jama.2013.282543".to_string()),
            citation: "James PA et al. (2014) JAMA 311(5):507-520".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Adults >18 years".to_string(),
        },
    });

    cv_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "diastolic_bp_mmhg".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(8.0),
        min_value: Some(60.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("24222015".to_string()),
            doi: Some("10.1001/jama.2013.282543".to_string()),
            citation: "James PA et al. (2014) JAMA 311(5):507-520".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Adults >18 years".to_string(),
        },
    });

    db.add_dataset("cardiovascular".to_string(), cv_data);
}
