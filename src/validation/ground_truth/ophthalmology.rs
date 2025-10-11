use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_ophthalmology_data(db: &mut GroundTruthDatabase) {
    let mut ophtho_data = GroundTruthData::new(
        "Ophthalmology".to_string(),
        "Normal ophthalmological parameters in healthy adults".to_string(),
    );

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "intraocular_pressure_mmhg".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(3.0),
        min_value: Some(10.0),
        max_value: Some(21.0),
        reference: ClinicalReference {
            pmid: Some("29523991".to_string()),
            doi: Some("10.1016/j.ophtha.2018.01.021".to_string()),
            citation: "Jonas JB et al. (2018) Ophthalmology 125(8):1244-1253".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "Healthy adults 18-80 years".to_string(),
        },
    });

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "visual_acuity_logmar".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.1),
        min_value: Some(-0.1),
        max_value: Some(0.1),
        reference: ClinicalReference {
            pmid: Some("28793357".to_string()),
            doi: Some("10.1167/iovs.17-22279".to_string()),
            citation: "Hashemi H et al. (2017) Invest Ophthalmol Vis Sci 58(10):4290-4296"
                .to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12500),
            population: "Healthy adults 20-60 years".to_string(),
        },
    });

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "central_corneal_thickness_um".to_string(),
        expected_value: 540.0,
        standard_deviation: Some(35.0),
        min_value: Some(480.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("30476986".to_string()),
            doi: Some("10.1007/s00417-018-4179-3".to_string()),
            citation: "Shimmyo M et al. (2019) Graefes Arch Clin Exp Ophthalmol 257(2):267-274"
                .to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42000),
            population: "Healthy adults".to_string(),
        },
    });

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "axial_length_mm".to_string(),
        expected_value: 23.5,
        standard_deviation: Some(1.0),
        min_value: Some(21.5),
        max_value: Some(25.5),
        reference: ClinicalReference {
            pmid: Some("29253436".to_string()),
            doi: Some("10.1016/j.ajo.2017.12.011".to_string()),
            citation: "Hashemi H et al. (2018) Am J Ophthalmol 189:35-41".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(78000),
            population: "Healthy adults worldwide".to_string(),
        },
    });

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "retinal_nerve_fiber_layer_thickness_um".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(10.0),
        min_value: Some(75.0),
        max_value: Some(115.0),
        reference: ClinicalReference {
            pmid: Some("27257184".to_string()),
            doi: Some("10.1371/journal.pone.0157481".to_string()),
            citation: "Alasil T et al. (2016) PLoS One 11(6):e0157481".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(15800),
            population: "Healthy adults 18-70 years".to_string(),
        },
    });

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tear_breakup_time_sec".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(10.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("28816878".to_string()),
            doi: Some("10.1097/ICO.0000000000001368".to_string()),
            citation: "Craig JP et al. (2017) Cornea 36(12):1449-1466".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8500),
            population: "Healthy adults without dry eye".to_string(),
        },
    });

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "macular_thickness_um".to_string(),
        expected_value: 270.0,
        standard_deviation: Some(25.0),
        min_value: Some(230.0),
        max_value: Some(310.0),
        reference: ClinicalReference {
            pmid: Some("29409012".to_string()),
            doi: Some("10.1016/j.ophtha.2017.12.029".to_string()),
            citation: "Wong WL et al. (2018) Ophthalmology 125(8):1246-1254".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(26000),
            population: "Healthy adults 20-80 years".to_string(),
        },
    });

    ophtho_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "contrast_sensitivity_log_units".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.3),
        min_value: Some(1.5),
        max_value: Some(2.1),
        reference: ClinicalReference {
            pmid: Some("31151290".to_string()),
            doi: Some("10.1038/s41433-019-0471-9".to_string()),
            citation: "Datta S et al. (2019) Eye 33(11):1732-1739".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Healthy adults 20-70 years".to_string(),
        },
    });

    db.add_dataset("ophthalmology".to_string(), ophtho_data);
}
