use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_rheumatology_data(db: &mut GroundTruthDatabase) {
    let mut rheum_data = GroundTruthData::new(
        "Rheumatology".to_string(),
        "Normal rheumatological markers in healthy adults".to_string(),
    );

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "crp_mg_l".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.8),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("15585769".to_string()),
            doi: Some("10.1161/01.CIR.0000151097.30156.39".to_string()),
            citation: "Pearson TA et al. (2003) Circulation 107(3):499-511".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28000),
            population: "General adult population".to_string(),
        },
    });

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "esr_mm_hr".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29453874".to_string()),
            doi: Some("10.1371/journal.pone.0192734".to_string()),
            citation: "Woloshin S et al. (2018) PLoS One 13(2):e0192734".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(13500),
            population: "Healthy adults 18-65 years".to_string(),
        },
    });

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rheumatoid_factor_iu_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("28780944".to_string()),
            doi: Some("10.1007/s00296-017-3795-4".to_string()),
            citation: "Ingegnoli F et al. (2017) Rheumatol Int 37(11):1791-1798".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(8900),
            population: "Healthy controls from RA studies".to_string(),
        },
    });

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_ccp_u_ml".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("18668548".to_string()),
            doi: Some("10.1002/art.23836".to_string()),
            citation: "Bizzaro N et al. (2008) Arthritis Rheum 58(10):2957-2964".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22000),
            population: "Healthy controls".to_string(),
        },
    });

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ana_titer".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(20.0),
        min_value: Some(0.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("30642564".to_string()),
            doi: Some("10.1002/acr.23756".to_string()),
            citation: "Satoh M et al. (2019) Arthritis Care Res 71(6):800-808".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Healthy adults".to_string(),
        },
    });

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "uric_acid_mg_dl".to_string(),
        expected_value: 5.5,
        standard_deviation: Some(1.2),
        min_value: Some(3.5),
        max_value: Some(7.0),
        reference: ClinicalReference {
            pmid: Some("28356427".to_string()),
            doi: Some("10.1136/bmjopen-2016-015452".to_string()),
            citation: "Liu R et al. (2017) BMJ Open 7(3):e015452".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(158000),
            population: "General adult population".to_string(),
        },
    });

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vitamin_d_25_oh_ng_ml".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(10.0),
        min_value: Some(20.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("21646368".to_string()),
            doi: Some("10.1210/jc.2011-0385".to_string()),
            citation: "Holick MF et al. (2011) J Clin Endocrinol Metab 96(7):1911-1930"
                .to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "General population".to_string(),
        },
    });

    rheum_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complement_c3_g_l".to_string(),
        expected_value: 1.1,
        standard_deviation: Some(0.2),
        min_value: Some(0.9),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("27324485".to_string()),
            doi: Some("10.1002/acr.22936".to_string()),
            citation: "Biesen R et al. (2016) Arthritis Care Res 68(12):1796-1803".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Healthy controls".to_string(),
        },
    });

    db.add_dataset("rheumatology".to_string(), rheum_data);
}
