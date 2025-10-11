use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_immunology_data(db: &mut GroundTruthDatabase) {
    let mut immuno_data = GroundTruthData::new(
        "Immunology".to_string(),
        "Normal immunological parameters in healthy adults".to_string(),
    );

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cd4_count_per_ul".to_string(),
        expected_value: 900.0,
        standard_deviation: Some(300.0),
        min_value: Some(500.0),
        max_value: Some(1400.0),
        reference: ClinicalReference {
            pmid: Some("28475900".to_string()),
            doi: Some("10.1371/journal.pone.0177003".to_string()),
            citation: "Mandala WL et al. (2017) PLoS One 12(5):e0177003".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2800),
            population: "Healthy adults 18-60 years".to_string(),
        },
    });

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cd8_count_per_ul".to_string(),
        expected_value: 500.0,
        standard_deviation: Some(200.0),
        min_value: Some(200.0),
        max_value: Some(900.0),
        reference: ClinicalReference {
            pmid: Some("28475900".to_string()),
            doi: Some("10.1371/journal.pone.0177003".to_string()),
            citation: "Mandala WL et al. (2017) PLoS One 12(5):e0177003".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2800),
            population: "Healthy adults 18-60 years".to_string(),
        },
    });

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cd4_cd8_ratio".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.6),
        min_value: Some(1.0),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("28475900".to_string()),
            doi: Some("10.1371/journal.pone.0177003".to_string()),
            citation: "Mandala WL et al. (2017) PLoS One 12(5):e0177003".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2800),
            population: "Healthy adults 18-60 years".to_string(),
        },
    });

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "igg_g_l".to_string(),
        expected_value: 11.0,
        standard_deviation: Some(3.0),
        min_value: Some(7.0),
        max_value: Some(16.0),
        reference: ClinicalReference {
            pmid: Some("30554720".to_string()),
            doi: Some("10.1111/ijlh.12970".to_string()),
            citation: "Colantonio DA et al. (2019) Int J Lab Hematol 41(2):208-217".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Healthy adults 18-70 years".to_string(),
        },
    });

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "igm_g_l".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.5),
        min_value: Some(0.4),
        max_value: Some(2.3),
        reference: ClinicalReference {
            pmid: Some("30554720".to_string()),
            doi: Some("10.1111/ijlh.12970".to_string()),
            citation: "Colantonio DA et al. (2019) Int J Lab Hematol 41(2):208-217".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Healthy adults 18-70 years".to_string(),
        },
    });

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "iga_g_l".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.0),
        min_value: Some(0.7),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("30554720".to_string()),
            doi: Some("10.1111/ijlh.12970".to_string()),
            citation: "Colantonio DA et al. (2019) Int J Lab Hematol 41(2):208-217".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Healthy adults 18-70 years".to_string(),
        },
    });

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complement_c3_g_l".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.2),
        min_value: Some(0.9),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("26271151".to_string()),
            doi: Some("10.1111/vox.12309".to_string()),
            citation: "Steffensen R et al. (2015) Vox Sang 109(4):337-345".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Healthy adults 20-60 years".to_string(),
        },
    });

    immuno_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complement_c4_g_l".to_string(),
        expected_value: 0.25,
        standard_deviation: Some(0.08),
        min_value: Some(0.1),
        max_value: Some(0.4),
        reference: ClinicalReference {
            pmid: Some("26271151".to_string()),
            doi: Some("10.1111/vox.12309".to_string()),
            citation: "Steffensen R et al. (2015) Vox Sang 109(4):337-345".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Healthy adults 20-60 years".to_string(),
        },
    });

    db.add_dataset("immunology".to_string(), immuno_data);
}
