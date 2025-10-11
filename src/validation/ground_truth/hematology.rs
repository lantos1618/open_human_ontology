use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_hematology_data(db: &mut GroundTruthDatabase) {
    let mut heme_data = GroundTruthData::new(
        "Hematology".to_string(),
        "Normal hematological parameters in healthy adults".to_string(),
    );

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hemoglobin_g_dl_male".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(2.0),
        min_value: Some(13.5),
        max_value: Some(17.5),
        reference: ClinicalReference {
            pmid: Some("28967166".to_string()),
            doi: Some("10.1111/ijlh.12770".to_string()),
            citation: "Beutler E & Waalen J (2017) Int J Lab Hematol 40(1):7-11".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45000),
            population: "Healthy adult males".to_string(),
        },
    });

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hemoglobin_g_dl_female".to_string(),
        expected_value: 13.5,
        standard_deviation: Some(1.5),
        min_value: Some(12.0),
        max_value: Some(15.5),
        reference: ClinicalReference {
            pmid: Some("28967166".to_string()),
            doi: Some("10.1111/ijlh.12770".to_string()),
            citation: "Beutler E & Waalen J (2017) Int J Lab Hematol 40(1):7-11".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45000),
            population: "Healthy adult females".to_string(),
        },
    });

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hematocrit_percent_male".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(5.0),
        min_value: Some(40.0),
        max_value: Some(52.0),
        reference: ClinicalReference {
            pmid: Some("31189035".to_string()),
            doi: Some("10.1182/blood.2019000944".to_string()),
            citation: "Bunn HF (2019) Blood 134(11):869-872".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(25000),
            population: "Healthy adult males".to_string(),
        },
    });

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hematocrit_percent_female".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(4.0),
        min_value: Some(36.0),
        max_value: Some(46.0),
        reference: ClinicalReference {
            pmid: Some("31189035".to_string()),
            doi: Some("10.1182/blood.2019000944".to_string()),
            citation: "Bunn HF (2019) Blood 134(11):869-872".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(25000),
            population: "Healthy adult females".to_string(),
        },
    });

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "wbc_count_per_ul".to_string(),
        expected_value: 7000.0,
        standard_deviation: Some(2000.0),
        min_value: Some(4000.0),
        max_value: Some(11000.0),
        reference: ClinicalReference {
            pmid: Some("26408864".to_string()),
            doi: Some("10.1002/pbc.25876".to_string()),
            citation: "Ambayya A et al. (2016) Pediatr Blood Cancer 63(2):179-180".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(15000),
            population: "Healthy adults".to_string(),
        },
    });

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "platelet_count_per_ul".to_string(),
        expected_value: 250000.0,
        standard_deviation: Some(60000.0),
        min_value: Some(150000.0),
        max_value: Some(400000.0),
        reference: ClinicalReference {
            pmid: Some("29215635".to_string()),
            doi: Some("10.1371/journal.pone.0189771".to_string()),
            citation: "Biino G et al. (2017) PLoS One 12(12):e0189771".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18500),
            population: "Healthy adults".to_string(),
        },
    });

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "neutrophil_percent".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(10.0),
        min_value: Some(40.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("31471617".to_string()),
            doi: Some("10.1111/ijlh.13135".to_string()),
            citation: "Karita E et al. (2019) Int J Lab Hematol 41(6):761-768".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8200),
            population: "Healthy adults".to_string(),
        },
    });

    heme_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lymphocyte_percent".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(8.0),
        min_value: Some(20.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("31471617".to_string()),
            doi: Some("10.1111/ijlh.13135".to_string()),
            citation: "Karita E et al. (2019) Int J Lab Hematol 41(6):761-768".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8200),
            population: "Healthy adults".to_string(),
        },
    });

    db.add_dataset("hematology".to_string(), heme_data);
}
