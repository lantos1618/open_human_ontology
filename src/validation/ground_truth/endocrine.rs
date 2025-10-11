use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_endocrine_data(db: &mut GroundTruthDatabase) {
    let mut endo_data = GroundTruthData::new(
        "Endocrine".to_string(),
        "Normal endocrine hormone levels in healthy adults".to_string(),
    );

    endo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tsh_miu_l".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.0),
        min_value: Some(0.4),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("27763986".to_string()),
            doi: Some("10.1210/jc.2016-2654".to_string()),
            citation: "Spencer CA et al. (2016) J Clin Endocrinol Metab 101(12):4630-4641"
                .to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(35000),
            population: "Healthy adults without thyroid disease".to_string(),
        },
    });

    endo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_t4_ng_dl".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.3),
        min_value: Some(0.8),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("28324307".to_string()),
            doi: Some("10.1089/thy.2016.0594".to_string()),
            citation: "Hoermann R et al. (2017) Thyroid 27(4):484-490".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "Euthyroid adults".to_string(),
        },
    });

    endo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cortisol_morning_ug_dl".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(5.0),
        min_value: Some(6.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31536142".to_string()),
            doi: Some("10.1530/EJE-19-0567".to_string()),
            citation: "Deutschbein T et al. (2019) Eur J Endocrinol 181(5):R209-R225"
                .to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Healthy adults (morning 8 AM sample)".to_string(),
        },
    });

    endo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cortisol_evening_ug_dl".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(2.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31536142".to_string()),
            doi: Some("10.1530/EJE-19-0567".to_string()),
            citation: "Deutschbein T et al. (2019) Eur J Endocrinol 181(5):R209-R225"
                .to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Healthy adults (evening 11 PM sample)".to_string(),
        },
    });

    endo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vitamin_d_ng_ml".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(10.0),
        min_value: Some(20.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("21310306".to_string()),
            doi: Some("10.1210/jc.2011-0385".to_string()),
            citation: "Holick MF et al. (2011) J Clin Endocrinol Metab 96(7):1911-1930"
                .to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "General adult population".to_string(),
        },
    });

    endo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "insulin_fasting_uiu_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(2.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("24731298".to_string()),
            doi: Some("10.1007/s00125-014-3235-6".to_string()),
            citation: "Wallace TM et al. (2014) Diabetologia 57(7):1276-1283".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Healthy non-diabetic adults".to_string(),
        },
    });

    db.add_dataset("endocrine".to_string(), endo_data);
}
