use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_pulmonary_data(db: &mut GroundTruthDatabase) {
    let mut pulmonary_data = GroundTruthData::new(
        "Pulmonary".to_string(),
        "Normal pulmonary function test parameters in healthy adults".to_string(),
    );

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fev1_percent_predicted".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(12.0),
        min_value: Some(80.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("23471469".to_string()),
            doi: Some("10.1183/09031936.00080312".to_string()),
            citation: "Quanjer PH et al. (2012) Eur Respir J 40(6):1324-1343".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(74187),
            population: "Global multi-ethnic reference values".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fvc_percent_predicted".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(12.0),
        min_value: Some(80.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("23471469".to_string()),
            doi: Some("10.1183/09031936.00080312".to_string()),
            citation: "Quanjer PH et al. (2012) Eur Respir J 40(6):1324-1343".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(74187),
            population: "Global multi-ethnic reference values".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fev1_fvc_ratio".to_string(),
        expected_value: 0.79,
        standard_deviation: Some(0.05),
        min_value: Some(0.70),
        max_value: Some(0.85),
        reference: ClinicalReference {
            pmid: Some("23471469".to_string()),
            doi: Some("10.1183/09031936.00080312".to_string()),
            citation: "Quanjer PH et al. (2012) Eur Respir J 40(6):1324-1343".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(74187),
            population: "Global multi-ethnic reference values".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dlco_percent_predicted".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(15.0),
        min_value: Some(75.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("28245654".to_string()),
            doi: Some("10.1513/AnnalsATS.201607-571FR".to_string()),
            citation: "Stanojevic S et al. (2017) Ann Am Thorac Soc 14(Suppl 1):S1-S11"
                .to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(15500),
            population: "Multi-ethnic adult population".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tlc_percent_predicted".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(12.0),
        min_value: Some(80.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("27872413".to_string()),
            doi: Some("10.1513/AnnalsATS.201605-387FR".to_string()),
            citation: "Stocks J et al. (2016) Ann Am Thorac Soc 13(Suppl 5):S364-S387"
                .to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12800),
            population: "Healthy adults".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rv_percent_predicted".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(18.0),
        min_value: Some(75.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("27872413".to_string()),
            doi: Some("10.1513/AnnalsATS.201605-387FR".to_string()),
            citation: "Stocks J et al. (2016) Ann Am Thorac Soc 13(Suppl 5):S364-S387"
                .to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12800),
            population: "Healthy adults".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peak_expiratory_flow_l_per_min".to_string(),
        expected_value: 500.0,
        standard_deviation: Some(80.0),
        min_value: Some(380.0),
        max_value: Some(650.0),
        reference: ClinicalReference {
            pmid: Some("23471469".to_string()),
            doi: Some("10.1183/09031936.00080312".to_string()),
            citation: "Quanjer PH et al. (2012) Eur Respir J 40(6):1324-1343".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(74187),
            population: "Global multi-ethnic reference values".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fef25_75_percent_predicted".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(20.0),
        min_value: Some(60.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("29382628".to_string()),
            doi: Some("10.1513/AnnalsATS.201707-555OC".to_string()),
            citation: "Bui DS et al. (2018) Ann Am Thorac Soc 15(10):1195-1201".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6400),
            population: "Healthy adults 25-75 years".to_string(),
        },
    });

    db.add_dataset("pulmonary".to_string(), pulmonary_data);
}
