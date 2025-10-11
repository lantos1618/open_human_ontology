use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_metabolic_data(db: &mut GroundTruthDatabase) {
    let mut metabolic_data = GroundTruthData::new(
        "Metabolic".to_string(),
        "Normal metabolic parameters in healthy adults".to_string(),
    );

    metabolic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fasting_glucose_mg_dl".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(10.0),
        min_value: Some(70.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32657974".to_string()),
            doi: Some("10.2337/dc20-S002".to_string()),
            citation: "American Diabetes Association (2020) Diabetes Care 43(Suppl 1):S14-S31"
                .to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "General adult population".to_string(),
        },
    });

    metabolic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bmi_healthy_range".to_string(),
        expected_value: 22.5,
        standard_deviation: Some(2.5),
        min_value: Some(18.5),
        max_value: Some(24.9),
        reference: ClinicalReference {
            pmid: Some("27216006".to_string()),
            doi: Some("10.1016/S0140-6736(16)30175-1".to_string()),
            citation: "GBD 2015 Obesity Collaborators (2017) Lancet 390(10113):2627-2642"
                .to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(19200000),
            population: "Global population".to_string(),
        },
    });

    db.add_dataset("metabolic".to_string(), metabolic_data);
}
