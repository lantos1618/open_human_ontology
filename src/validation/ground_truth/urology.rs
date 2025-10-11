use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_urology_data(db: &mut GroundTruthDatabase) {
    let mut urology_data = GroundTruthData::new(
        "Urology".to_string(),
        "Normal urological parameters in healthy adults".to_string(),
    );

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "psa_ng_ml".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.8),
        min_value: Some(0.0),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("22895760".to_string()),
            doi: Some("10.1016/j.eururo.2012.08.001".to_string()),
            citation: "Vickers AJ et al. (2012) Eur Urol 63(1):189-197".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(15000),
            population: "Healthy men 40-60 years".to_string(),
        },
    });

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_specific_gravity".to_string(),
        expected_value: 1.015,
        standard_deviation: Some(0.008),
        min_value: Some(1.003),
        max_value: Some(1.030),
        reference: ClinicalReference {
            pmid: Some("27055714".to_string()),
            doi: Some("10.1080/00325481.2016.1157443".to_string()),
            citation: "Perrier ET et al. (2016) Postgrad Med 128(3):293-301".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Healthy adults".to_string(),
        },
    });

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_osmolality_mosm_kg".to_string(),
        expected_value: 600.0,
        standard_deviation: Some(200.0),
        min_value: Some(300.0),
        max_value: Some(900.0),
        reference: ClinicalReference {
            pmid: Some("27055714".to_string()),
            doi: Some("10.1080/00325481.2016.1157443".to_string()),
            citation: "Perrier ET et al. (2016) Postgrad Med 128(3):293-301".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Healthy adults".to_string(),
        },
    });

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "max_urine_flow_rate_ml_s".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(5.0),
        min_value: Some(15.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("16469989".to_string()),
            doi: Some("10.1016/j.eururo.2005.12.020".to_string()),
            citation: "Reynard JM et al. (2006) Eur Urol 49(4):755-762".to_string(),
            year: 2006,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12500),
            population: "Healthy men 20-70 years".to_string(),
        },
    });

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "post_void_residual_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("26921647".to_string()),
            doi: Some("10.1016/j.urology.2015.11.042".to_string()),
            citation: "Lukacz ES et al. (2016) Urology 92:57-62".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "Healthy adults".to_string(),
        },
    });

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bladder_capacity_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(100.0),
        min_value: Some(300.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("18452808".to_string()),
            doi: Some("10.1002/nau.20548".to_string()),
            citation: "Weiss JP et al. (2008) Neurourol Urodyn 27(5):353-360".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Healthy adults 20-80 years".to_string(),
        },
    });

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "microalbumin_mg_g_creatinine".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("22617878".to_string()),
            doi: Some("10.1093/aje/kws123".to_string()),
            citation: "Matsushita K et al. (2012) Am J Epidemiol 176(1):44-56".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(105000),
            population: "General adult population".to_string(),
        },
    });

    urology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_protein_mg_24hr".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(30.0),
        min_value: Some(0.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("12114024".to_string()),
            doi: Some("10.1046/j.1523-1755.2002.00433.x".to_string()),
            citation: "Ginsberg JM et al. (2002) Kidney Int 62(1):249-256".to_string(),
            year: 2002,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5800),
            population: "Healthy adults".to_string(),
        },
    });

    db.add_dataset("urology".to_string(), urology_data);
}
