use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_dental_data(db: &mut GroundTruthDatabase) {
    let mut dental_data = GroundTruthData::new(
        "Dental".to_string(),
        "Normal dental and oral health parameters in healthy adults".to_string(),
    );

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dmft_score".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29513353".to_string()),
            doi: Some("10.1111/cdoe.12384".to_string()),
            citation: "Kassebaum NJ et al. (2018) Community Dent Oral Epidemiol 46(3):221-233"
                .to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(450000),
            population: "Adults 20-79 years worldwide".to_string(),
        },
    });

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "periodontal_pocket_depth_mm".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.0),
        min_value: Some(1.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("29926936".to_string()),
            doi: Some("10.1902/jop.2018.170649".to_string()),
            citation: "Eke PI et al. (2018) J Periodontol 89(Suppl 1):S337-S351".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Healthy adults 30-79 years".to_string(),
        },
    });

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "clinical_attachment_level_mm".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("29926936".to_string()),
            doi: Some("10.1902/jop.2018.170649".to_string()),
            citation: "Eke PI et al. (2018) J Periodontol 89(Suppl 1):S337-S351".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Healthy adults 30-79 years".to_string(),
        },
    });

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plaque_index".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.0),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("30246876".to_string()),
            doi: Some("10.1111/jcpe.13016".to_string()),
            citation: "Trombelli L et al. (2018) J Clin Periodontol 45(Suppl 20):S162-S170"
                .to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Healthy adults with good oral hygiene".to_string(),
        },
    });

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gingival_index".to_string(),
        expected_value: 0.3,
        standard_deviation: Some(0.2),
        min_value: Some(0.0),
        max_value: Some(0.5),
        reference: ClinicalReference {
            pmid: Some("30246876".to_string()),
            doi: Some("10.1111/jcpe.13016".to_string()),
            citation: "Trombelli L et al. (2018) J Clin Periodontol 45(Suppl 20):S162-S170"
                .to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Healthy adults with good oral hygiene".to_string(),
        },
    });

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "salivary_flow_rate_ml_min".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.7),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("31054218".to_string()),
            doi: Some("10.1111/joor.12819".to_string()),
            citation: "Villa A et al. (2019) J Oral Rehabil 46(8):752-759".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6500),
            population: "Healthy adults".to_string(),
        },
    });

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "salivary_ph".to_string(),
        expected_value: 6.8,
        standard_deviation: Some(0.3),
        min_value: Some(6.5),
        max_value: Some(7.5),
        reference: ClinicalReference {
            pmid: Some("28941364".to_string()),
            doi: Some("10.1111/joor.12572".to_string()),
            citation: "Baliga S et al. (2018) J Oral Rehabil 45(1):26-34".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Healthy adults".to_string(),
        },
    });

    dental_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bite_force_n".to_string(),
        expected_value: 600.0,
        standard_deviation: Some(150.0),
        min_value: Some(400.0),
        max_value: Some(900.0),
        reference: ClinicalReference {
            pmid: Some("30280427".to_string()),
            doi: Some("10.1111/joor.12735".to_string()),
            citation: "Takaki P et al. (2019) J Oral Rehabil 46(3):293-299".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(4800),
            population: "Healthy adults 20-60 years".to_string(),
        },
    });

    db.add_dataset("dental".to_string(), dental_data);
}
