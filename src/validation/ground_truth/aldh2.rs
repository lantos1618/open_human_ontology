use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_aldh2_data(db: &mut GroundTruthDatabase) {
    let mut aldh2_data = GroundTruthData::new(
        "ALDH2_Deficiency".to_string(),
        "Acetaldehyde metabolism in ALDH2*2 carriers".to_string(),
    );

    aldh2_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aldh2_activity_heterozygous".to_string(),
        expected_value: 0.12,
        standard_deviation: Some(0.05),
        min_value: Some(0.05),
        max_value: Some(0.20),
        reference: ClinicalReference {
            pmid: Some("19320537".to_string()),
            doi: Some("10.1371/journal.pmed.1000050".to_string()),
            citation: "Brooks PJ et al. (2009) PLoS Med 6(3):e50".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(44000),
            population: "East Asian populations".to_string(),
        },
    });

    aldh2_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acetaldehyde_peak_multiplier_aldh2_het".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.0),
        min_value: Some(2.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("19320537".to_string()),
            doi: Some("10.1371/journal.pmed.1000050".to_string()),
            citation: "Brooks PJ et al. (2009) PLoS Med 6(3):e50".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(44000),
            population: "East Asian populations".to_string(),
        },
    });

    aldh2_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "esophageal_cancer_risk_moderate_drinking".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(1.5),
        min_value: Some(3.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("19320537".to_string()),
            doi: Some("10.1371/journal.pmed.1000050".to_string()),
            citation: "Brooks PJ et al. (2009) PLoS Med 6(3):e50".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(44000),
            population: "East Asian ALDH2*1/*2 carriers".to_string(),
        },
    });

    aldh2_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aldh2_deficiency_frequency_east_asian".to_string(),
        expected_value: 0.36,
        standard_deviation: Some(0.05),
        min_value: Some(0.28),
        max_value: Some(0.45),
        reference: ClinicalReference {
            pmid: Some("30158283".to_string()),
            doi: Some("10.1038/s41436-018-0290-2".to_string()),
            citation: "Chen CH et al. (2018) Genet Med 21(8):1657-1659".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(550000000),
            population: "East Asian population".to_string(),
        },
    });

    db.add_dataset("aldh2".to_string(), aldh2_data);
}
