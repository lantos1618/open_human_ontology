use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_dermatology_data(db: &mut GroundTruthDatabase) {
    let mut derm_data = GroundTruthData::new(
        "Dermatology".to_string(),
        "Normal dermatological parameters in healthy adults".to_string(),
    );

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "transepidermal_water_loss_g_m2_h".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(4.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("28358173".to_string()),
            doi: Some("10.1111/srt.12356".to_string()),
            citation: "Fluhr JW et al. (2017) Skin Res Technol 23(3):259-266".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2500),
            population: "Healthy adults 20-60 years".to_string(),
        },
    });

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "stratum_corneum_hydration_au".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("29235144".to_string()),
            doi: Some("10.1111/ijd.13830".to_string()),
            citation: "Egawa M et al. (2018) Int J Dermatol 57(4):481-489".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1800),
            population: "Healthy adults".to_string(),
        },
    });

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "skin_ph".to_string(),
        expected_value: 5.5,
        standard_deviation: Some(0.5),
        min_value: Some(4.5),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("29665624".to_string()),
            doi: Some("10.1016/j.jaad.2018.01.003".to_string()),
            citation: "Lambers H et al. (2018) J Am Acad Dermatol 79(3):549-556".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(8500),
            population: "Healthy adults".to_string(),
        },
    });

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "skin_elasticity_percent".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(10.0),
        min_value: Some(60.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("27670956".to_string()),
            doi: Some("10.1111/ics.12359".to_string()),
            citation: "Ezure T et al. (2017) Int J Cosmet Sci 39(1):21-27".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Healthy adults 20-70 years".to_string(),
        },
    });

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "melanin_index".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(15.0),
        min_value: Some(15.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("30675909".to_string()),
            doi: Some("10.1111/pcmr.12763".to_string()),
            citation: "Del Bino S et al. (2019) Pigment Cell Melanoma Res 32(4):534-544"
                .to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "Healthy adults, mixed ethnicities".to_string(),
        },
    });

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sebum_excretion_rate_ug_cm2_h".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.6),
        min_value: Some(0.3),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("31231864".to_string()),
            doi: Some("10.1111/exd.13987".to_string()),
            citation: "Pappas A et al. (2019) Exp Dermatol 28(9):1027-1033".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2200),
            population: "Healthy adults 18-65 years".to_string(),
        },
    });

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "skin_thickness_mm".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.3),
        min_value: Some(1.0),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("28971533".to_string()),
            doi: Some("10.1111/srt.12388".to_string()),
            citation: "Oltulu P et al. (2018) Skin Res Technol 24(2):254-260".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1500),
            population: "Healthy adults 20-70 years".to_string(),
        },
    });

    derm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "capillary_density_per_mm2".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(15.0),
        min_value: Some(45.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("29654640".to_string()),
            doi: Some("10.1111/micc.12460".to_string()),
            citation: "Bertuglia S et al. (2018) Microcirculation 25(5):e12460".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1200),
            population: "Healthy adults".to_string(),
        },
    });

    db.add_dataset("dermatology".to_string(), derm_data);
}
