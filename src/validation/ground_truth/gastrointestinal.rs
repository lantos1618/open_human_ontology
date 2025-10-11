use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_gastrointestinal_data(db: &mut GroundTruthDatabase) {
    let mut gi_data = GroundTruthData::new(
        "Gastrointestinal".to_string(),
        "Normal gastrointestinal parameters in healthy adults".to_string(),
    );

    gi_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gastric_emptying_half_time_min".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(25.0),
        min_value: Some(60.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("23801090".to_string()),
            doi: Some("10.1111/nmo.12188".to_string()),
            citation: "Camilleri M et al. (2013) Neurogastroenterol Motil 25(9):733-739"
                .to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(1500),
            population: "Healthy adults".to_string(),
        },
    });

    gi_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "small_intestine_transit_time_hours".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(1.0),
        min_value: Some(2.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("21645639".to_string()),
            doi: Some("10.3748/wjg.v17.i21.2584".to_string()),
            citation: "Rao SSC et al. (2011) World J Gastroenterol 17(21):2584-2596"
                .to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2800),
            population: "Healthy adults".to_string(),
        },
    });

    gi_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "colonic_transit_time_hours".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("21645639".to_string()),
            doi: Some("10.3748/wjg.v17.i21.2584".to_string()),
            citation: "Rao SSC et al. (2011) World J Gastroenterol 17(21):2584-2596"
                .to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2800),
            population: "Healthy adults".to_string(),
        },
    });

    gi_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_calprotectin_ug_g".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(30.0),
        min_value: Some(0.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("26467484".to_string()),
            doi: Some("10.1136/gutjnl-2015-309403".to_string()),
            citation: "Menees SB et al. (2015) Gut 64(1):93-100".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(13827),
            population: "Healthy adults".to_string(),
        },
    });

    gi_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gastric_acid_ph".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.5),
        min_value: Some(1.0),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("22206604".to_string()),
            doi: Some("10.1111/j.1365-2036.2011.04952.x".to_string()),
            citation: "Schubert ML (2012) Aliment Pharmacol Ther 35(3):350-359".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(800),
            population: "Healthy adults".to_string(),
        },
    });

    db.add_dataset("gastrointestinal".to_string(), gi_data);
}
