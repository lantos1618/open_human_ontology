use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_hepatic_data(db: &mut GroundTruthDatabase) {
    let mut hepatic_data = GroundTruthData::new(
        "Hepatic".to_string(),
        "Normal hepatic function parameters in healthy adults".to_string(),
    );

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alt_u_l".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(15.0),
        min_value: Some(7.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("29661585".to_string()),
            doi: Some("10.1111/apt.14679".to_string()),
            citation: "Kwo PY et al. (2018) Aliment Pharmacol Ther 47(11):1447-1454"
                .to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(250000),
            population: "Healthy adults without liver disease".to_string(),
        },
    });

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ast_u_l".to_string(),
        expected_value: 23.0,
        standard_deviation: Some(12.0),
        min_value: Some(8.0),
        max_value: Some(48.0),
        reference: ClinicalReference {
            pmid: Some("29661585".to_string()),
            doi: Some("10.1111/apt.14679".to_string()),
            citation: "Kwo PY et al. (2018) Aliment Pharmacol Ther 47(11):1447-1454"
                .to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(250000),
            population: "Healthy adults without liver disease".to_string(),
        },
    });

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alp_u_l".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(20.0),
        min_value: Some(40.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("30785653".to_string()),
            doi: Some("10.1111/liv.14064".to_string()),
            citation: "Ruhl CE et al. (2019) Liver Int 39(6):1129-1138".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(32000),
            population: "Healthy adults 20-74 years".to_string(),
        },
    });

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_bilirubin_mg_dl".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.3),
        min_value: Some(0.3),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("24889452".to_string()),
            doi: Some("10.1111/liv.12555".to_string()),
            citation: "Wagner KH et al. (2015) Liver Int 35(3):716-723".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(45000),
            population: "Healthy adults".to_string(),
        },
    });

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "albumin_g_dl".to_string(),
        expected_value: 4.2,
        standard_deviation: Some(0.4),
        min_value: Some(3.5),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("31537926".to_string()),
            doi: Some("10.1038/s41598-019-49873-y".to_string()),
            citation: "Suh B et al. (2019) Sci Rep 9:13747".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(280000),
            population: "Healthy adults 20-79 years".to_string(),
        },
    });

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ggt_u_l".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(15.0),
        min_value: Some(9.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("27732885".to_string()),
            doi: Some("10.1111/apt.13836".to_string()),
            citation: "Kunutsor SK et al. (2017) Aliment Pharmacol Ther 45(1):8-28".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(1200000),
            population: "General adult population".to_string(),
        },
    });

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "prothrombin_time_sec".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(1.0),
        min_value: Some(11.0),
        max_value: Some(13.5),
        reference: ClinicalReference {
            pmid: Some("28691773".to_string()),
            doi: Some("10.1182/blood-2017-02-765065".to_string()),
            citation: "Gosselin RC et al. (2018) Blood 131(13):1486-1490".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(15000),
            population: "Healthy adults".to_string(),
        },
    });

    hepatic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "inr".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.1),
        min_value: Some(0.9),
        max_value: Some(1.1),
        reference: ClinicalReference {
            pmid: Some("28691773".to_string()),
            doi: Some("10.1182/blood-2017-02-765065".to_string()),
            citation: "Gosselin RC et al. (2018) Blood 131(13):1486-1490".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(15000),
            population: "Healthy adults".to_string(),
        },
    });

    db.add_dataset("hepatic".to_string(), hepatic_data);
}
