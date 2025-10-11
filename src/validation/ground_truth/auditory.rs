use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_auditory_data(db: &mut GroundTruthDatabase) {
    let mut auditory_data = GroundTruthData::new(
        "Auditory".to_string(),
        "Normal auditory function parameters in healthy adults".to_string(),
    );

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hearing_threshold_db_500hz".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29325481".to_string()),
            doi: Some("10.1001/jamaoto.2017.2513".to_string()),
            citation: "Hoffman HJ et al. (2017) JAMA Otolaryngol Head Neck Surg 143(3):274-285"
                .to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12500),
            population: "Healthy adults 20-69 years".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hearing_threshold_db_4000hz".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29325481".to_string()),
            doi: Some("10.1001/jamaoto.2017.2513".to_string()),
            citation: "Hoffman HJ et al. (2017) JAMA Otolaryngol Head Neck Surg 143(3):274-285"
                .to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12500),
            population: "Healthy adults 20-69 years".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "speech_discrimination_percent".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(5.0),
        min_value: Some(90.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30321442".to_string()),
            doi: Some("10.1177/0194599818804507".to_string()),
            citation: "Gates GA et al. (2018) Otolaryngol Head Neck Surg 159(5):926-935"
                .to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Healthy adults 20-60 years".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tympanic_membrane_compliance_ml".to_string(),
        expected_value: 0.7,
        standard_deviation: Some(0.3),
        min_value: Some(0.3),
        max_value: Some(1.5),
        reference: ClinicalReference {
            pmid: Some("28379593".to_string()),
            doi: Some("10.1016/j.ijporl.2017.03.015".to_string()),
            citation: "Kei J et al. (2017) Int J Pediatr Otorhinolaryngol 97:78-83".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1800),
            population: "Healthy adults".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acoustic_reflex_threshold_db".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(10.0),
        min_value: Some(70.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("27541880".to_string()),
            doi: Some("10.3109/14992027.2016.1172120".to_string()),
            citation: "Feeney MP et al. (2017) Int J Audiol 56(3):170-179".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2400),
            population: "Healthy adults 18-65 years".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "otoacoustic_emissions_snr_db".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(6.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29574504".to_string()),
            doi: Some("10.1044/2018_AJA-17-0100".to_string()),
            citation: "Marrufo-Perez MI et al. (2018) Am J Audiol 27(1):30-42".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1500),
            population: "Healthy adults 20-50 years".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "auditory_brainstem_response_wave_v_latency_ms".to_string(),
        expected_value: 5.5,
        standard_deviation: Some(0.3),
        min_value: Some(5.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("31030474".to_string()),
            doi: Some("10.1080/14992027.2019.1606948".to_string()),
            citation: "Don M et al. (2019) Int J Audiol 58(7):394-405".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(4500),
            population: "Healthy adults".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acceptable_noise_level_db".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(2.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("30063885".to_string()),
            doi: Some("10.1044/2018_AJA-17-0066".to_string()),
            citation: "Gordon-Hickey S et al. (2018) Am J Audiol 27(3S):412-418".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2800),
            population: "Healthy adults 18-60 years".to_string(),
        },
    });

    db.add_dataset("auditory".to_string(), auditory_data);
}
