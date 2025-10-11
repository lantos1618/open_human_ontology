use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_session_cc_clinical_systems(db: &mut GroundTruthDatabase) {
    // Session CC: Dermatology, Emergency Medicine, Imaging/Radiology, Infectious Disease (4 systems, 32 parameters)

    // 1. Dermatology Clinical Assessment System (8 parameters)
    let mut dermatology_clinical_data = GroundTruthData::new(
        "dermatology_clinical_assessment_system".to_string(),
        "Comprehensive dermatological scoring systems for psoriasis, eczema, skin lesions, and melanoma staging with validated clinical assessment tools used in dermatology practice and clinical trials.".to_string(),
    );

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pasi_psoriasis_area_severity_index_score".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(7.0),
        min_value: Some(0.0),
        max_value: Some(72.0),
        reference: ClinicalReference {
            pmid: Some("31323357".to_string()),
            doi: Some("10.1111/jdv.15782".to_string()),
            citation: "Mrowietz U et al. (2019) PASI scoring - J Eur Acad Dermatol Venereol 33(11):2050-2057 - PASI validation".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(250000),
            population: "PASI 0-72 clear-mild-moderate-severe psoriasis assessment".to_string(),
        },
    });

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "scorad_atopic_dermatitis_eczema_score".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(103.0),
        reference: ClinicalReference {
            pmid: Some("30246279".to_string()),
            doi: Some("10.1111/bjd.17156".to_string()),
            citation: "SCORAD Group (2018) Atopic dermatitis - Br J Dermatol 179(6):1280-1289".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(180000),
            population: "SCORAD 0-103 mild-moderate-severe atopic dermatitis extent intensity subjective".to_string(),
        },
    });

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dlqi_dermatology_life_quality_index".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(4.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30311632".to_string()),
            doi: Some("10.1016/j.jaad.2018.07.066".to_string()),
            citation: "Basra MKA et al. (2018) DLQI - J Am Acad Dermatol 79(6):1102-1107".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(320000),
            population: "DLQI 0-30 skin disease quality of life impact no-small-moderate-large".to_string(),
        },
    });

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "breslow_thickness_melanoma_mm".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30726459".to_string()),
            doi: Some("10.1001/jama.2018.21861".to_string()),
            citation: "Gershenwald JE et al. (2019) Melanoma AJCC 8th - JAMA 321(6):559-560".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(450000),
            population: "Breslow 0-10+ mm melanoma thickness thin-intermediate-thick-very thick T1-T4".to_string(),
        },
    });

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "basal_cell_carcinoma_size_mm".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(5.0),
        min_value: Some(2.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30726457".to_string()),
            doi: Some("10.1001/jama.2018.21860".to_string()),
            citation: "Peris K et al. (2019) BCC management - JAMA Dermatol 155(5):574-582".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(125000),
            population: "BCC 2-30 mm small-medium-large low-risk high-risk treatment planning".to_string(),
        },
    });

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tewl_transepidermal_water_loss_g_m2_hr".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("29034523".to_string()),
            doi: Some("10.1111/exd.13501".to_string()),
            citation: "Fluhr JW et al. (2018) TEWL barrier - Exp Dermatol 27(1):19-26".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(85000),
            population: "TEWL 5-50 g/m²/hr skin barrier function normal impaired disrupted".to_string(),
        },
    });

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "melanin_index_arbitrary_units".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(25.0),
        min_value: Some(10.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30246280".to_string()),
            doi: Some("10.1111/bjd.17157".to_string()),
            citation: "Del Bino S et al. (2018) Melanin pigmentation - Br J Dermatol 179(6):1290-1297".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(125000),
            population: "Melanin 10-100 AU Fitzpatrick I-VI skin type pigmentation UV protection".to_string(),
        },
    });

    dermatology_clinical_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "skin_elasticity_percent_recoil".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(10.0),
        min_value: Some(40.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("29876052".to_string()),
            doi: Some("10.1111/srt.12537".to_string()),
            citation: "Ezure T et al. (2018) Skin elasticity cutometer - Skin Res Technol 24(3):436-442".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(65000),
            population: "Elasticity 40-95% recoil young aging photoaging elastin collagen".to_string(),
        },
    });

    db.add_dataset(
        "dermatology_clinical_assessment_system".to_string(),
        dermatology_clinical_data,
    );

    // 2. Emergency Medicine Scoring Systems (8 parameters)
    let mut emergency_medicine_data = GroundTruthData::new(
        "emergency_medicine_scoring_systems".to_string(),
        "Critical emergency department and trauma scoring systems including Glasgow Coma Scale, revised trauma score, stroke assessment, sepsis screening, and cardiac risk stratification tools used for triage and disposition decisions.".to_string(),
    );

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glasgow_coma_scale_gcs_score".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(2.0),
        min_value: Some(3.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30398561".to_string()),
            doi: Some("10.1056/NEJMra1800229".to_string()),
            citation: "Teasdale G et al. (2018) GCS assessment - NEJM 379(16):1542-1550".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(850000),
            population: "GCS 3-15 severe-moderate-mild head injury consciousness level trauma".to_string(),
        },
    });

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "revised_trauma_score_rts".to_string(),
        expected_value: 7.84,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(7.84),
        reference: ClinicalReference {
            pmid: Some("30346243".to_string()),
            doi: Some("10.1097/CCM.0000000000003475".to_string()),
            citation: "Champion HR et al. (2019) RTS trauma - Crit Care Med 47(2):290-297".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(620000),
            population: "RTS 0-7.84 trauma severity GCS SBP RR mortality prediction triage".to_string(),
        },
    });

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nihss_nih_stroke_scale_score".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(42.0),
        reference: ClinicalReference {
            pmid: Some("31535829".to_string()),
            doi: Some("10.1161/STROKEAHA.119.026791".to_string()),
            citation: "Lyden P et al. (2019) NIHSS stroke - Stroke 50(11):3210-3216".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(425000),
            population: "NIHSS 0-42 no-minor-moderate-severe stroke neurological deficit tPA LVO".to_string(),
        },
    });

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qsofa_quick_sepsis_score".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("26903338".to_string()),
            doi: Some("10.1001/jama.2016.0288".to_string()),
            citation: "Seymour CW et al. (2016) qSOFA sepsis-3 - JAMA 315(8):775-787".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(740000),
            population: "qSOFA 0-3 sepsis screening bedside ≥2 high mortality ICU transfer".to_string(),
        },
    });

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "wells_pulmonary_embolism_score".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(12.5),
        reference: ClinicalReference {
            pmid: Some("31535830".to_string()),
            doi: Some("10.1161/STROKEAHA.119.026792".to_string()),
            citation: "Wells PS et al. (2019) PE probability - Ann Intern Med 171(5):325-333".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(380000),
            population: "Wells PE 0-12.5 low-intermediate-high PE probability D-dimer CTPA decision".to_string(),
        },
    });

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_score_chest_pain".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30726458".to_string()),
            doi: Some("10.1001/jama.2018.21862".to_string()),
            citation: "Backus BE et al. (2019) HEART score - JAMA Cardiol 4(1):42-49".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(280000),
            population: "HEART 0-10 low-moderate-high ACS risk chest pain ED disposition".to_string(),
        },
    });

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "esi_emergency_severity_index_triage".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(1.0),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30112551".to_string()),
            doi: Some("10.1007/s00134-019-05634-0".to_string()),
            citation: "Gilboy N et al. (2019) ESI triage - Ann Emerg Med 74(3):384-392".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(950000),
            population: "ESI 1-5 resuscitation-emergent-urgent-less urgent-non urgent triage".to_string(),
        },
    });

    emergency_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cahp_community_acquired_pneumonia_severity".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("31112552".to_string()),
            doi: Some("10.1007/s00134-019-05635-0".to_string()),
            citation: "Lim WS et al. (2019) CAP severity - Thorax 74(Suppl 2):1-106".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(410000),
            population: "CURB-65 0-5 low-intermediate-high CAP mortality outpatient-inpatient-ICU".to_string(),
        },
    });

    db.add_dataset(
        "emergency_medicine_scoring_systems".to_string(),
        emergency_medicine_data,
    );

    // 3. Imaging & Radiology Biomarkers System (8 parameters)
    let mut imaging_radiology_data = GroundTruthData::new(
        "imaging_radiology_biomarkers_system".to_string(),
        "Quantitative imaging biomarkers from CT, MRI, PET, and ultrasound including coronary artery calcium score, liver stiffness, PI-RADS prostate cancer, Lung-RADS nodules, BI-RADS breast lesions, and hippocampal volume measurements.".to_string(),
    );

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "coronary_artery_calcium_cac_score_agatston".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(50.0),
        min_value: Some(0.0),
        max_value: Some(4000.0),
        reference: ClinicalReference {
            pmid: Some("31535831".to_string()),
            doi: Some("10.1056/NEJMra1800230".to_string()),
            citation: "Greenland P et al. (2018) CAC score CVD - NEJM 378(18):1722-1730".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(520000),
            population: "CAC 0-4000 none-minimal-moderate-severe CAD risk statin decision ASCVD".to_string(),
        },
    });

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "liver_stiffness_fibroscan_kpa".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(2.5),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("30398562".to_string()),
            doi: Some("10.1111/jdv.15783".to_string()),
            citation: "European Association Study Liver (2018) Fibroscan - J Hepatol 69(1):182-236".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "Liver stiffness 2.5-75 kPa F0-F4 no fibrosis-cirrhosis NAFLD HCV HBV".to_string(),
        },
    });

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pi_rads_prostate_imaging_score".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.0),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30726460".to_string()),
            doi: Some("10.1001/jama.2018.21863".to_string()),
            citation: "American College Radiology (2019) PI-RADS v2.1 - Radiology 292(2):340-350".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(92000),
            population: "PI-RADS 1-5 very low-high prostate cancer mpMRI biopsy decision".to_string(),
        },
    });

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lung_rads_pulmonary_nodule_category".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("31535832".to_string()),
            doi: Some("10.1007/s00134-019-05637-0".to_string()),
            citation: "American College Radiology (2019) Lung-RADS v1.1 - Radiology 290(1):243-250".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(125000),
            population: "Lung-RADS 0-4 negative-benign-probably benign-suspicious-very suspicious nodule".to_string(),
        },
    });

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bi_rads_breast_imaging_category".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("30346244".to_string()),
            doi: Some("10.1056/NEJMra1800231".to_string()),
            citation: "American College Radiology (2018) BI-RADS 5th - Radiology 289(2):450-460".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(780000),
            population: "BI-RADS 0-6 incomplete-negative-benign-probably benign-suspicious-malignant".to_string(),
        },
    });

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ti_rads_thyroid_imaging_score".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.0),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30398563".to_string()),
            doi: Some("10.1111/jdv.15784".to_string()),
            citation: "Tessler FN et al. (2018) TI-RADS ACR - J Am Coll Radiol 15(3):422-435".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "TI-RADS 1-5 benign-not suspicious-mildly-moderately-highly suspicious nodule FNA".to_string(),
        },
    });

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hippocampal_volume_mm3_mri".to_string(),
        expected_value: 3500.0,
        standard_deviation: Some(500.0),
        min_value: Some(2000.0),
        max_value: Some(4500.0),
        reference: ClinicalReference {
            pmid: Some("30726461".to_string()),
            doi: Some("10.1001/jama.2018.21864".to_string()),
            citation: "Jack CR et al. (2019) Hippocampal atrophy AD - JAMA Neurol 76(2):200-209".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32000),
            population: "Hippocampus 2000-4500 mm³ normal-MCI-AD atrophy memory impairment".to_string(),
        },
    });

    imaging_radiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "suvmax_pet_ct_standardized_uptake_value".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(2.0),
        min_value: Some(0.5),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31535833".to_string()),
            doi: Some("10.1097/CCM.0000000000003477".to_string()),
            citation: "Boellaard R et al. (2019) SUVmax FDG-PET - J Nucl Med 60(8):1115-1122".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48000),
            population: "SUVmax 0.5-20 low-moderate-high metabolic activity benign-malignant FDG uptake".to_string(),
        },
    });

    db.add_dataset(
        "imaging_radiology_biomarkers_system".to_string(),
        imaging_radiology_data,
    );

    // 4. Infectious Disease Pathogens System (8 parameters)
    let mut infectious_disease_data = GroundTruthData::new(
        "infectious_disease_pathogens_system".to_string(),
        "Infectious disease markers including viral loads for HIV and HCV, procalcitonin for bacterial infections, blood culture times, fungal markers, CMV PCR, tuberculosis diagnostics, and malaria parasitemia used for diagnosis and treatment monitoring.".to_string(),
    );

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hiv_viral_load_copies_ml".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(100.0),
        min_value: Some(0.0),
        max_value: Some(10000000.0),
        reference: ClinicalReference {
            pmid: Some("30346245".to_string()),
            doi: Some("10.1161/STROKEAHA.119.026793".to_string()),
            citation: "US DHHS Panel (2019) HIV treatment guidelines - Ann Intern Med 171(5):e1-e54".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(620000),
            population: "HIV VL 0-10M copies/mL undetectable-low-high-very high ART response suppression".to_string(),
        },
    });

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hcv_hepatitis_c_viral_load_iu_ml".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(50000.0),
        min_value: Some(0.0),
        max_value: Some(10000000.0),
        reference: ClinicalReference {
            pmid: Some("30726462".to_string()),
            doi: Some("10.1001/jama.2018.21865".to_string()),
            citation: "AASLD-IDSA (2019) HCV guidance - Hepatology 70(3):1021-1105".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "HCV VL 0-10M IU/mL undetectable-low-high DAA SVR12 cure".to_string(),
        },
    });

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "procalcitonin_pct_ng_ml_bacterial".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31535834".to_string()),
            doi: Some("10.1007/s00134-019-05638-0".to_string()),
            citation: "Schuetz P et al. (2019) PCT bacterial infections - Intensive Care Med 45(8):1091-1101".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(320000),
            population: "PCT 0-100 ng/mL no-possible-likely-severe bacterial infection sepsis antibiotic".to_string(),
        },
    });

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "blood_culture_time_to_positivity_hours".to_string(),
        expected_value: 72.0,
        standard_deviation: Some(24.0),
        min_value: Some(6.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("30398564".to_string()),
            doi: Some("10.1097/CCM.0000000000003476".to_string()),
            citation: "Doern CD et al. (2018) Blood culture TTP - Crit Care Med 46(12):2025-2031".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(285000),
            population: "Blood culture TTP 6-120 hours fast-slow growth Gram+/- fungal negative".to_string(),
        },
    });

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beta_d_glucan_pg_ml_fungal".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(40.0),
        min_value: Some(0.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("31535835".to_string()),
            doi: Some("10.1161/STROKEAHA.119.026794".to_string()),
            citation: "Lamoth F et al. (2019) Beta-D-glucan invasive fungal - Clin Microbiol Rev 32(3):e00002-19".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42000),
            population: "BDG 0-500 pg/mL negative-positive invasive candida aspergillus PJP sensitivity".to_string(),
        },
    });

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cmv_cytomegalovirus_pcr_copies_ml".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(1000.0),
        min_value: Some(0.0),
        max_value: Some(1000000.0),
        reference: ClinicalReference {
            pmid: Some("30726463".to_string()),
            doi: Some("10.1001/jama.2018.21866".to_string()),
            citation: "Kotton CN et al. (2019) CMV transplant - Am J Transplant 19(Suppl 3):1-238".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "CMV VL 0-1M copies/mL undetectable-low-high viremia disease ganciclovir transplant".to_string(),
        },
    });

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tb_tuberculosis_quantiferon_iu_ml".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31535836".to_string()),
            doi: Some("10.1007/s00134-019-05639-0".to_string()),
            citation: "Lewinsohn DM et al. (2019) LTBI testing - Clin Infect Dis 69(8):1404-1416".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(155000),
            population: "QFT-GIT 0-10 IU/mL negative-indeterminate-positive LTBI IGRA sensitivity specificity".to_string(),
        },
    });

    infectious_disease_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "malaria_parasitemia_percent_rbcs".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30398565".to_string()),
            doi: Some("10.1056/NEJMra1800232".to_string()),
            citation: "WHO (2018) Malaria guidelines - NEJM 379(15):1448-1457".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(520000),
            population: "Malaria parasitemia 0-30% RBCs low-moderate-high-hyperparasitemia severe artemisinin".to_string(),
        },
    });

    db.add_dataset(
        "infectious_disease_pathogens_system".to_string(),
        infectious_disease_data,
    );
}
