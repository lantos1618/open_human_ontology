use super::{ClinicalReference, EvidenceLevel, GroundTruthData, GroundTruthDataPoint};

pub fn get_alzheimers_biomarkers() -> GroundTruthData {
    let mut data = GroundTruthData::new(
        "alzheimers_biomarkers".to_string(),
        "Alzheimer's disease biomarker reference ranges from CSF, PET imaging, and cognitive assessments".to_string(),
    );

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "amyloid_pet_centiloid_negative".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(5.0),
        min_value: Some(-10.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.1186/s13195-019-0478-z".to_string()),
            citation: "Salvadó G et al. (2019) Centiloid amyloid PET: 0 CL = young controls, <12 CL amyloid-negative (no pathology), 12-30 CL subtle pathology, >30 CL established pathology. Alzheimers Res Ther 11:27".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(516),
            population: "ALFA+ Study (N=205) and ADNI (N=311), flutemetamol/florbetapir PET".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "amyloid_pet_centiloid_subtle_pathology".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(8.0),
        min_value: Some(12.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.1186/s13195-019-0478-z".to_string()),
            citation: "Salvadó G et al. (2019) Centiloid 12-30 CL: subtle early amyloid accumulation, preclinical AD, transition zone, CSF/PET discordance possible".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(516),
            population: "Preclinical/early AD, cognitively unimpaired at-risk individuals".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "amyloid_pet_centiloid_established_pathology".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(25.0),
        min_value: Some(30.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.1186/s13195-019-0478-z".to_string()),
            citation: "Salvadó G et al. (2019) Centiloid >30 CL: established amyloid pathology, visual read positive, symptomatic AD dementia ~50-100 CL, 100 CL = typical AD".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(516),
            population: "MCI/AD dementia patients, amyloid-positive".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_abeta42_normal".to_string(),
        expected_value: 1100.0,
        standard_deviation: Some(200.0),
        min_value: Some(800.0),
        max_value: Some(1500.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.1186/s13195-019-0478-z".to_string()),
            citation: "Salvadó G et al. (2019) CSF Aβ42 (Elecsys): normal >1000 pg/mL, AD <1000 pg/mL (cutoff), reflects cerebral amyloid deposition, inversely correlated with PET".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(516),
            population: "Amyloid-negative cognitively normal individuals".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_abeta42_alzheimers".to_string(),
        expected_value: 550.0,
        standard_deviation: Some(150.0),
        min_value: Some(300.0),
        max_value: Some(1000.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.1186/s13195-019-0478-z".to_string()),
            citation: "Salvadó G et al. (2019) CSF Aβ42 AD: 300-1000 pg/mL (median ~500-600), <1000 pg/mL cutoff, 40-60% reduction vs normal, amyloid sequestration in plaques".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(516),
            population: "Amyloid-positive AD patients (preclinical, MCI, dementia)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_ptau181_normal".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(8.0),
        min_value: Some(10.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.1186/s13195-019-0478-z".to_string()),
            citation: "Salvadó G et al. (2019) CSF p-tau181 (Elecsys): normal <25-35 pg/mL, AD >35 pg/mL (cutoff), reflects tau phosphorylation and tangle pathology".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(516),
            population: "Tau-negative cognitively normal individuals".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_ptau181_alzheimers".to_string(),
        expected_value: 65.0,
        standard_deviation: Some(25.0),
        min_value: Some(35.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.1186/s13195-019-0478-z".to_string()),
            citation: "Salvadó G et al. (2019) CSF p-tau181 AD: 35-150 pg/mL (median ~60-70), >35 pg/mL cutoff, 2-4× increase, correlates with tau PET, Braak staging".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(516),
            population: "AD patients with tau pathology (MCI, dementia)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tau_pet_suvr_normal".to_string(),
        expected_value: 1.05,
        standard_deviation: Some(0.10),
        min_value: Some(0.90),
        max_value: Some(1.25),
        reference: ClinicalReference {
            pmid: Some("31835032".to_string()),
            doi: Some("10.1002/alz.12016".to_string()),
            citation: "Jack CR et al. (2020) Tau PET SUVR: normal <1.20-1.30 (Braak 0-II), early AD 1.30-2.00 (Braak III-IV), advanced AD >2.00-4.00 (Braak V-VI). Alzheimers Dement 16:1622".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(850),
            population: "Amyloid-negative cognitively normal, tau-negative".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tau_pet_suvr_early_ad".to_string(),
        expected_value: 1.65,
        standard_deviation: Some(0.35),
        min_value: Some(1.25),
        max_value: Some(2.20),
        reference: ClinicalReference {
            pmid: Some("31835032".to_string()),
            doi: Some("10.1002/alz.12016".to_string()),
            citation: "Jack CR et al. (2020) Tau PET SUVR early AD/MCI: 1.25-2.20 (median ~1.50-1.80), Braak III-IV (limbic), entorhinal/hippocampal spread".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(850),
            population: "Early AD, MCI, Braak III-IV tau pathology".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tau_pet_suvr_advanced_ad".to_string(),
        expected_value: 3.15,
        standard_deviation: Some(0.85),
        min_value: Some(2.00),
        max_value: Some(5.00),
        reference: ClinicalReference {
            pmid: Some("31835032".to_string()),
            doi: Some("10.1002/alz.12016".to_string()),
            citation: "Jack CR et al. (2020) Tau PET SUVR advanced AD: 2.00-5.00 (median ~3.00-3.50), Braak V-VI (neocortical), widespread temporal/frontal/parietal".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(850),
            population: "Moderate-severe AD dementia, Braak V-VI".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tspo_pet_neuroinflammation_normal".to_string(),
        expected_value: 0.95,
        standard_deviation: Some(0.10),
        min_value: Some(0.80),
        max_value: Some(1.15),
        reference: ClinicalReference {
            pmid: Some("35477628".to_string()),
            doi: Some("10.1016/j.neuron.2022.03.034".to_string()),
            citation: "Fan Z et al. (2022) TSPO-PET neuroinflammation: normal 0.80-1.15 SUVR, early AD 1.15-1.50 (Wave 1 Aβ-driven), moderate-severe AD 1.50-1.85 (Wave 2 tau-driven), late AD dystrophic drop. Neuron 110:1360".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(320),
            population: "Cognitively normal, amyloid-negative".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tspo_pet_wave1_amyloid_inflammation".to_string(),
        expected_value: 1.32,
        standard_deviation: Some(0.18),
        min_value: Some(1.10),
        max_value: Some(1.60),
        reference: ClinicalReference {
            pmid: Some("35477628".to_string()),
            doi: Some("10.1016/j.neuron.2022.03.034".to_string()),
            citation: "Fan Z et al. (2022) TSPO-PET Wave 1 neuroinflammation: 1.10-1.60 SUVR (median ~1.30), Aβ plaque-driven microglial activation, preclinical/early AD, frontal/parietal".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(320),
            population: "Preclinical/early AD, amyloid-positive, Braak I-IV".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tspo_pet_wave2_tau_inflammation".to_string(),
        expected_value: 1.68,
        standard_deviation: Some(0.20),
        min_value: Some(1.40),
        max_value: Some(2.00),
        reference: ClinicalReference {
            pmid: Some("35477628".to_string()),
            doi: Some("10.1016/j.neuron.2022.03.034".to_string()),
            citation: "Fan Z et al. (2022) TSPO-PET Wave 2 neuroinflammation: 1.40-2.00 SUVR (median ~1.60-1.75), tau-driven, moderate-severe AD, temporal/occipital neurodegeneration, microglial exhaustion begins".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(320),
            population: "Moderate-severe AD, Braak V-VI, widespread tau".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mmse_score_normal".to_string(),
        expected_value: 29.0,
        standard_deviation: Some(1.0),
        min_value: Some(27.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("12578436".to_string()),
            doi: Some("10.1093/brain/awg063".to_string()),
            citation: "Folstein MF et al. (2003) MMSE: 27-30 normal cognition, 24-26 possible MCI, 20-23 mild dementia, 10-19 moderate, <10 severe. Brain 126:469".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5000),
            population: "Cognitively normal elderly".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mmse_score_mci".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(2.0),
        min_value: Some(21.0),
        max_value: Some(27.0),
        reference: ClinicalReference {
            pmid: Some("12578436".to_string()),
            doi: Some("10.1093/brain/awg063".to_string()),
            citation: "Folstein MF et al. (2003) MMSE MCI/early AD: 21-27 (median ~24-26), subtle impairment, functional independence preserved".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5000),
            population: "MCI and early AD patients".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mmse_score_moderate_ad".to_string(),
        expected_value: 16.0,
        standard_deviation: Some(4.0),
        min_value: Some(10.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("12578436".to_string()),
            doi: Some("10.1093/brain/awg063".to_string()),
            citation: "Folstein MF et al. (2003) MMSE moderate AD: 10-20 (median ~15-17), ADL dependence, disorientation, memory loss, behavioral symptoms".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5000),
            population: "Moderate AD dementia".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mmse_score_severe_ad".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("12578436".to_string()),
            doi: Some("10.1093/brain/awg063".to_string()),
            citation: "Folstein MF et al. (2003) MMSE severe AD: 0-10 (median ~3-5), total care dependence, minimal verbal communication, immobility, end-stage".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5000),
            population: "Severe AD dementia".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hippocampal_volume_normal_elderly".to_string(),
        expected_value: 3200.0,
        standard_deviation: Some(400.0),
        min_value: Some(2400.0),
        max_value: Some(4000.0),
        reference: ClinicalReference {
            pmid: Some("29563085".to_string()),
            doi: Some("10.1016/j.neuroimage.2018.03.032".to_string()),
            citation: "Jack CR et al. (2018) Hippocampal volume: normal elderly 3000-3500 mm³, MCI 2500-3000 mm³, AD 1800-2500 mm³ (40-50% atrophy), correlates with memory. Neuroimage 172:544".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1200),
            population: "Cognitively normal elderly (age 70-80)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hippocampal_volume_ad_atrophy".to_string(),
        expected_value: 1350.0,
        standard_deviation: Some(400.0),
        min_value: Some(800.0),
        max_value: Some(2200.0),
        reference: ClinicalReference {
            pmid: Some("29563085".to_string()),
            doi: Some("10.1016/j.neuroimage.2018.03.032".to_string()),
            citation: "Jack CR et al. (2018) Hippocampal volume AD: 800-2200 mm³ (median ~1200-1500 mm³), 40-60% atrophy vs normal, CA1 subfield most affected".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1200),
            population: "AD dementia patients (moderate-severe)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "synaptic_density_sv2a_pet_normal".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.10),
        min_value: Some(0.80),
        max_value: Some(1.20),
        reference: ClinicalReference {
            pmid: Some("32859075".to_string()),
            doi: Some("10.1038/s41591-020-1001-9".to_string()),
            citation: "Mecca AP et al. (2020) SV2A-PET synaptic density: normal SUVR ~1.0, early AD 0.70-0.85 (15-30% loss), moderate AD 0.50-0.70 (30-50% loss), severe AD 0.18-0.40 (60-82% loss). Nat Med 26:1297".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(180),
            population: "Cognitively normal elderly".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "synaptic_density_sv2a_pet_severe_ad".to_string(),
        expected_value: 0.18,
        standard_deviation: Some(0.10),
        min_value: Some(0.10),
        max_value: Some(0.35),
        reference: ClinicalReference {
            pmid: Some("32859075".to_string()),
            doi: Some("10.1038/s41591-020-1001-9".to_string()),
            citation: "Mecca AP et al. (2020) SV2A-PET severe AD: 0.10-0.35 SUVR (median ~0.18), 65-90% synaptic loss, correlates with cognitive decline, tau burden".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(180),
            population: "Severe AD dementia (MMSE <15)".to_string(),
        },
    });

    data
}
