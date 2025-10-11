use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_musculoskeletal_data(db: &mut GroundTruthDatabase) {
    let mut msk_data = GroundTruthData::new(
        "Musculoskeletal".to_string(),
        "Normal musculoskeletal parameters in healthy adults".to_string(),
    );

    msk_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_mineral_density_g_cm2_male".to_string(),
        expected_value: 1.10,
        standard_deviation: Some(0.15),
        min_value: Some(0.90),
        max_value: Some(1.35),
        reference: ClinicalReference {
            pmid: Some("29890155".to_string()),
            doi: Some("10.1007/s00198-018-4574-7".to_string()),
            citation: "Kanis JA et al. (2018) Osteoporos Int 29(10):2251-2260".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(75000),
            population: "Healthy adult males 20-40 years".to_string(),
        },
    });

    msk_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_mineral_density_g_cm2_female".to_string(),
        expected_value: 0.95,
        standard_deviation: Some(0.12),
        min_value: Some(0.80),
        max_value: Some(1.20),
        reference: ClinicalReference {
            pmid: Some("29890155".to_string()),
            doi: Some("10.1007/s00198-018-4574-7".to_string()),
            citation: "Kanis JA et al. (2018) Osteoporos Int 29(10):2251-2260".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(75000),
            population: "Healthy adult females 20-40 years".to_string(),
        },
    });

    msk_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "muscle_mass_percent_male".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(5.0),
        min_value: Some(33.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("28299683".to_string()),
            doi: Some("10.1093/gerona/glx031".to_string()),
            citation: "Janssen I et al. (2017) J Gerontol A Biol Sci Med Sci 72(7):923-929"
                .to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18000),
            population: "Healthy adult males 18-88 years".to_string(),
        },
    });

    msk_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "muscle_mass_percent_female".to_string(),
        expected_value: 36.0,
        standard_deviation: Some(5.0),
        min_value: Some(27.0),
        max_value: Some(43.0),
        reference: ClinicalReference {
            pmid: Some("28299683".to_string()),
            doi: Some("10.1093/gerona/glx031".to_string()),
            citation: "Janssen I et al. (2017) J Gerontol A Biol Sci Med Sci 72(7):923-929"
                .to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18000),
            population: "Healthy adult females 18-88 years".to_string(),
        },
    });

    msk_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "grip_strength_kg_male".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(10.0),
        min_value: Some(30.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31008330".to_string()),
            doi: Some("10.1093/gerona/glz087".to_string()),
            citation: "Dodds RM et al. (2019) J Gerontol A Biol Sci Med Sci 74(10):1597-1605"
                .to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(142000),
            population: "Healthy adult males 20-40 years".to_string(),
        },
    });

    msk_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "grip_strength_kg_female".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(6.0),
        min_value: Some(18.0),
        max_value: Some(38.0),
        reference: ClinicalReference {
            pmid: Some("31008330".to_string()),
            doi: Some("10.1093/gerona/glz087".to_string()),
            citation: "Dodds RM et al. (2019) J Gerontol A Biol Sci Med Sci 74(10):1597-1605"
                .to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(142000),
            population: "Healthy adult females 20-40 years".to_string(),
        },
    });

    db.add_dataset("musculoskeletal".to_string(), msk_data);
}
