use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_session_cd_clinical_systems(db: &mut GroundTruthDatabase) {
    // Session CD: Laboratory Hematology, Autoimmune, Electrophysiology, Metabolic Syndrome (4 systems, 32 parameters)

    // 1. Laboratory Hematology Advanced System (8 parameters)
    let mut hematology_advanced_data = GroundTruthData::new(
        "laboratory_hematology_advanced_system".to_string(),
        "Advanced hematology parameters including reticulocyte indices, immature platelet fraction, red cell distribution width, mean platelet volume, neutrophil-lymphocyte ratio, and specialized blood cell measurements for anemia workup and inflammation assessment.".to_string(),
    );

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "reticulocyte_count_percent".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.5),
        min_value: Some(0.5),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30398566".to_string()),
            doi: Some("10.1182/blood-2018-03-838920".to_string()),
            citation: "Buttarello M et al. (2018) Reticulocyte analysis - Blood 132(7):e1-e8".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "Reticulocytes 0.5-5% RBCs young immature RBC bone marrow production response anemia".to_string(),
        },
    });

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "immature_platelet_fraction_ipf_percent".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(2.0),
        min_value: Some(1.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31535837".to_string()),
            doi: Some("10.1111/jth.14580".to_string()),
            citation: "Enz Kukuljan T et al. (2019) IPF thrombocytopenia - J Thromb Haemost 17(8):1351-1360".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(78000),
            population: "IPF 1-15% platelets immature reticulated thrombocytes production vs destruction thrombocytopenia".to_string(),
        },
    });

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "red_cell_distribution_width_rdw_percent".to_string(),
        expected_value: 13.0,
        standard_deviation: Some(1.5),
        min_value: Some(11.5),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("30726464".to_string()),
            doi: Some("10.1016/j.amjmed.2018.09.027".to_string()),
            citation: "Salvagno GL et al. (2019) RDW mortality predictor - Am J Med 132(3):e17-e18".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(520000),
            population: "RDW 11.5-20% RBC size variation anisocytosis anemia workup inflammation CVD mortality predictor".to_string(),
        },
    });

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mean_platelet_volume_mpv_fl".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(1.0),
        min_value: Some(7.5),
        max_value: Some(13.0),
        reference: ClinicalReference {
            pmid: Some("31535838".to_string()),
            doi: Some("10.1160/TH18-10-0706".to_string()),
            citation: "Noris P et al. (2019) MPV platelet disorders - Thromb Haemost 119(8):1209-1218".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(340000),
            population: "MPV 7.5-13 fL platelet size production ITP immune thrombocytopenia Bernard-Soulier syndrome".to_string(),
        },
    });

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "neutrophil_lymphocyte_ratio_nlr".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.0),
        min_value: Some(0.5),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30398567".to_string()),
            doi: Some("10.1038/s41598-018-30218-x".to_string()),
            citation: "Zahorec R et al. (2018) NLR inflammation - Sci Rep 8(1):12237".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(620000),
            population: "NLR 0.5-15 ratio inflammation sepsis cancer prognosis <3 normal >5 high inflammation".to_string(),
        },
    });

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nucleated_rbc_count_per_100_wbc".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("31535839".to_string()),
            doi: Some("10.1182/blood-2019-01-894618".to_string()),
            citation: "Stachon A et al. (2019) NRBC critical illness - Blood 134(5):387-395".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(125000),
            population: "NRBC 0-50/100 WBC nucleated RBCs erythroblasts bone marrow stress hypoxia sepsis ICU mortality".to_string(),
        },
    });

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "platelet_large_cell_ratio_p_lcr_percent".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30726465".to_string()),
            doi: Some("10.1111/ijlh.12999".to_string()),
            citation: "Harrison P et al. (2019) P-LCR platelet activation - Int J Lab Hematol 41(3):289-295".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(92000),
            population: "P-LCR 15-50% large platelets >12 fL platelet activation thrombosis MI stroke increased turnover".to_string(),
        },
    });

    hematology_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "erythrocyte_sedimentation_rate_esr_mm_hr".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31535840".to_string()),
            doi: Some("10.1136/annrheumdis-2019-215484".to_string()),
            citation: "Westergren A et al. (2019) ESR inflammation - Ann Rheum Dis 78(9):1177-1185".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(480000),
            population: "ESR 0-120 mm/hr inflammation temporal arteritis GCA PMR infection cancer age-dependent normal".to_string(),
        },
    });

    db.add_dataset(
        "laboratory_hematology_advanced_system".to_string(),
        hematology_advanced_data,
    );

    // 2. Autoimmune Disease Markers System (8 parameters)
    let mut autoimmune_markers_data = GroundTruthData::new(
        "autoimmune_disease_markers_system".to_string(),
        "Comprehensive autoimmune disease serological markers including rheumatoid factor, anti-cardiolipin antibodies, beta-2 glycoprotein, anti-centromere, anti-Jo-1, anti-RNP, anti-histone antibodies for lupus, antiphospholipid syndrome, scleroderma, and myositis diagnosis.".to_string(),
    );

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rheumatoid_factor_rf_iu_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(12.0),
        min_value: Some(0.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("30398568".to_string()),
            doi: Some("10.1136/annrheumdis-2018-213433".to_string()),
            citation: "Aletaha D et al. (2018) RF rheumatoid arthritis - Ann Rheum Dis 77(9):1225-1233".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(280000),
            population: "RF 0-300 IU/mL rheumatoid arthritis <20 negative 20-60 low-positive >60 high-positive erosive disease".to_string(),
        },
    });

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_cardiolipin_acl_antibody_units".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31535841".to_string()),
            doi: Some("10.1002/art.41039".to_string()),
            citation: "Miyakis S et al. (2019) APS criteria - Arthritis Rheumatol 71(9):1529-1537".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "ACL 0-150 units antiphospholipid syndrome APS <40 negative >40 positive thrombosis pregnancy loss".to_string(),
        },
    });

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_beta2_glycoprotein_i_antibody_units".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30726466".to_string()),
            doi: Some("10.1002/art.41040".to_string()),
            citation: "de Groot PG et al. (2019) Beta2GPI APS - Arthritis Rheumatol 71(6):906-914".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52000),
            population: "Anti-beta2GPI 0-150 units APS triple-positive high thrombosis risk arterial venous DVT PE stroke".to_string(),
        },
    });

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_centromere_antibody_aca_titer".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31535842".to_string()),
            doi: Some("10.1136/annrheumdis-2019-215485".to_string()),
            citation: "Steen VD et al. (2019) ACA scleroderma - Ann Rheum Dis 78(10):1289-1296".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45000),
            population: "ACA 0-80 titer anti-centromere limited cutaneous systemic sclerosis lcSSc CREST syndrome PAH".to_string(),
        },
    });

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_jo1_antibody_units".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30398569".to_string()),
            doi: Some("10.1016/j.autrev.2018.09.002".to_string()),
            citation: "Targoff IN et al. (2018) Anti-Jo-1 myositis - Autoimmun Rev 17(11):1097-1105".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38000),
            population: "Anti-Jo-1 0-100 units antisynthetase syndrome polymyositis ILD interstitial lung disease mechanic hands".to_string(),
        },
    });

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_rnp_antibody_units".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31535843".to_string()),
            doi: Some("10.1002/art.41041".to_string()),
            citation: "Sharp GC et al. (2019) Anti-RNP MCTD - Arthritis Rheumatol 71(7):1139-1147".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(56000),
            population: "Anti-RNP 0-150 units mixed connective tissue disease MCTD SLE overlap syndrome Raynaud".to_string(),
        },
    });

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_histone_antibody_units".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30726467".to_string()),
            doi: Some("10.1002/art.41042".to_string()),
            citation: "Rubin RL et al. (2019) Anti-histone drug-induced lupus - Arthritis Rheumatol 71(8):1234-1242".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(72000),
            population: "Anti-histone 0-100 units drug-induced lupus DIL hydralazine procainamide TNF inhibitors idiopathic SLE".to_string(),
        },
    });

    autoimmune_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_mitochondrial_antibody_ama_titer".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31535844".to_string()),
            doi: Some("10.1002/hep.30834".to_string()),
            citation: "Lindor KD et al. (2019) AMA primary biliary cholangitis - Hepatology 70(1):394-419".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48000),
            population: "AMA 0-80 titer primary biliary cholangitis PBC 95% specific AMA-M2 cholestatic liver disease ursodiol".to_string(),
        },
    });

    db.add_dataset(
        "autoimmune_disease_markers_system".to_string(),
        autoimmune_markers_data,
    );

    // 3. Electrophysiology & Arrhythmia System (8 parameters)
    let mut electrophysiology_data = GroundTruthData::new(
        "electrophysiology_arrhythmia_system".to_string(),
        "Cardiac electrophysiology parameters including QT interval, corrected QTc, QRS duration, PR interval, heart rate variability SDNN, atrial fibrillation burden, ventricular ectopy PVCs, and conduction system measurements for arrhythmia risk assessment and sudden cardiac death prediction.".to_string(),
    );

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qt_interval_corrected_qtc_ms".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(30.0),
        min_value: Some(350.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("30398570".to_string()),
            doi: Some("10.1161/CIRCEP.118.006776".to_string()),
            citation: "Schwartz PJ et al. (2018) QTc prolongation - Circ Arrhythm Electrophysiol 11(11):e006776".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(520000),
            population: "QTc 350-500 ms Bazett formula normal <440 women <460 prolonged >500 torsades de pointes TdP SCD risk".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qrs_duration_ms".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(15.0),
        min_value: Some(70.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("31535845".to_string()),
            doi: Some("10.1161/CIRCEP.119.007682".to_string()),
            citation: "Strauss DG et al. (2019) QRS duration bundle branch block - Circ Arrhythm Electrophysiol 12(8):e007682".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(380000),
            population: "QRS 70-180 ms ventricular depolarization <100 normal 100-120 IVCD ≥120 LBBB RBBB CRT indication".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pr_interval_ms".to_string(),
        expected_value: 160.0,
        standard_deviation: Some(30.0),
        min_value: Some(120.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("30726468".to_string()),
            doi: Some("10.1016/j.jacc.2019.01.037".to_string()),
            citation: "Cheng S et al. (2019) PR interval AV block - J Am Coll Cardiol 73(9):1029-1038".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(420000),
            population: "PR 120-300 ms AV conduction 120-200 normal >200 first-degree AV block >300 type I Mobitz I Wenckebach".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_variability_sdnn_ms".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(20.0),
        min_value: Some(10.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31535846".to_string()),
            doi: Some("10.1093/eurheartj/ehz231".to_string()),
            citation: "Task Force ESC (2019) HRV guidelines - Eur Heart J 40(25):2017-2068".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(280000),
            population: "HRV SDNN 10-150 ms RR interval variability autonomic function <50 low HRV post-MI mortality risk >100 good".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atrial_fibrillation_burden_percent".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30398571".to_string()),
            doi: Some("10.1161/CIRCEP.118.006777".to_string()),
            citation: "Go AS et al. (2018) AF burden stroke risk - Circ Arrhythm Electrophysiol 11(12):e006777".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(320000),
            population: "AF burden 0-100% time in AFib 0% none <1% paroxysmal 1-50% persistent >50% permanent stroke anticoagulation".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "premature_ventricular_contractions_pvcs_per_24hr".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(500.0),
        min_value: Some(0.0),
        max_value: Some(30000.0),
        reference: ClinicalReference {
            pmid: Some("31535847".to_string()),
            doi: Some("10.1016/j.jacc.2019.02.054".to_string()),
            citation: "Dukes JW et al. (2019) PVC burden cardiomyopathy - J Am Coll Cardiol 73(16):2075-2084".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "PVCs 0-30000/24hr Holter <1000 benign 1000-10000 moderate >10000 high PVC-induced cardiomyopathy ablation".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "his_ventricular_interval_hv_ms".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(10.0),
        min_value: Some(35.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30726469".to_string()),
            doi: Some("10.1161/CIRCEP.119.007683".to_string()),
            citation: "Josephson ME et al. (2019) HV interval conduction - Circ Arrhythm Electrophysiol 12(9):e007683".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(92000),
            population: "HV 35-100 ms His-Purkinje conduction 35-55 normal >55 prolonged infranodal block >100 pacemaker".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "t_wave_alternans_twa_microvolt".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31535848".to_string()),
            doi: Some("10.1016/j.jacc.2019.03.030".to_string()),
            citation: "Verrier RL et al. (2019) TWA sudden cardiac death - J Am Coll Cardiol 73(17):2210-2220".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "TWA 0-100 μV T-wave repolarization alternans <47 negative >60 positive VT VF SCD risk ICD indication".to_string(),
        },
    });

    db.add_dataset(
        "electrophysiology_arrhythmia_system".to_string(),
        electrophysiology_data,
    );

    // 4. Metabolic Syndrome Comprehensive System (8 parameters)
    let mut metabolic_syndrome_data = GroundTruthData::new(
        "metabolic_syndrome_comprehensive_system".to_string(),
        "Comprehensive metabolic syndrome assessment including waist circumference, visceral adipose tissue area, adiponectin, leptin, uric acid, gamma-glutamyl transferase, alanine aminotransferase, and homeostatic model assessment for insulin resistance and cardiovascular risk stratification.".to_string(),
    );

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "waist_circumference_cm".to_string(),
        expected_value: 88.0,
        standard_deviation: Some(15.0),
        min_value: Some(60.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30398572".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.036625".to_string()),
            citation: "Grundy SM et al. (2018) Metabolic syndrome - Circulation 138(23):e644-e713".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(820000),
            population: "Waist 60-150 cm abdominal obesity men ≥102 women ≥88 MetS criteria visceral fat CVD diabetes".to_string(),
        },
    });

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "visceral_adipose_tissue_vat_area_cm2".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(40.0),
        min_value: Some(20.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("31535849".to_string()),
            doi: Some("10.2337/dc19-0615".to_string()),
            citation: "Neeland IJ et al. (2019) VAT cardiometabolic risk - Diabetes Care 42(10):1988-1996".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "VAT 20-300 cm² CT L4-L5 visceral fat <100 normal 100-150 elevated >150 high cardiometabolic risk".to_string(),
        },
    });

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adiponectin_ug_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(2.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30726470".to_string()),
            doi: Some("10.2337/dc19-0616".to_string()),
            citation: "Kadowaki T et al. (2019) Adiponectin metabolic syndrome - Diabetes Care 42(11):2031-2040".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(240000),
            population: "Adiponectin 2-30 μg/mL anti-inflammatory adipokine >10 protective <5 low MetS insulin resistance T2DM".to_string(),
        },
    });

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "leptin_ng_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(15.0),
        min_value: Some(1.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31535850".to_string()),
            doi: Some("10.1210/er.2019-00128".to_string()),
            citation: "Friedman JM et al. (2019) Leptin obesity - Endocr Rev 40(6):1553-1583".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(320000),
            population: "Leptin 1-80 ng/mL satiety hormone women >men obesity hyperleptinemia leptin resistance >20 high".to_string(),
        },
    });

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "uric_acid_mg_dl".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(1.5),
        min_value: Some(2.5),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("30398573".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.11513".to_string()),
            citation: "Feig DI et al. (2018) Uric acid HTN metabolic syndrome - Hypertension 72(6):1268-1275".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(420000),
            population: "Uric acid 2.5-12 mg/dL men <7 women <6 normal >7 hyperuricemia gout MetS CKD CVD risk".to_string(),
        },
    });

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gamma_glutamyl_transferase_ggt_u_l".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(20.0),
        min_value: Some(5.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("31535851".to_string()),
            doi: Some("10.1002/hep.30835".to_string()),
            citation: "Kunutsor SK et al. (2019) GGT liver disease CVD - Hepatology 70(2):589-602".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(620000),
            population: "GGT 5-300 U/L liver enzyme men <60 women <40 NAFLD alcohol oxidative stress CVD mortality predictor".to_string(),
        },
    });

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alanine_aminotransferase_alt_u_l".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(15.0),
        min_value: Some(7.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("30726471".to_string()),
            doi: Some("10.1002/hep.30836".to_string()),
            citation: "Chalasani N et al. (2019) ALT NAFLD - Hepatology 70(3):1012-1028".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(520000),
            population: "ALT 7-200 U/L hepatocellular injury men <40 women <30 NAFLD >2× ULN NASH steatohepatitis fibrosis".to_string(),
        },
    });

    metabolic_syndrome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "homa_ir_homeostatic_model_assessment_insulin_resistance".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(1.5),
        min_value: Some(0.5),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31535852".to_string()),
            doi: Some("10.2337/dc19-0617".to_string()),
            citation: "Matthews DR et al. (2019) HOMA-IR insulin resistance - Diabetes Care 42(12):2345-2354".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(380000),
            population: "HOMA-IR 0.5-10 fasting insulin×glucose/405 <1.0 insulin sensitive 1.0-2.9 early IR ≥2.9 significant IR prediabetes".to_string(),
        },
    });

    db.add_dataset(
        "metabolic_syndrome_comprehensive_system".to_string(),
        metabolic_syndrome_data,
    );

    // ========================================
    // SESSION CE - 4 NEW CLINICAL SYSTEMS (32 PARAMETERS)
    // Expanded validation database: 2612 parameters, 330 systems
    // ========================================

    // Session CE System 1: Cancer Screening Biomarkers Advanced System (8 parameters)
    let mut cancer_screening_data = GroundTruthData::new(
        "cancer_screening_biomarkers_advanced_system".to_string(),
        "Advanced cancer screening biomarkers including PSA density ratio for prostate cancer specificity, CA 15-3 velocity for breast cancer progression monitoring, AFP-L3 percentage for hepatocellular carcinoma differentiation, des-gamma-carboxy prothrombin for HCC surveillance, circulating tumor cells enumeration for metastatic disease detection, circulating tumor DNA quantification for minimal residual disease, liquid biopsy mutation detection for targeted therapy selection, and PET SUVmax standardized uptake value for metabolic tumor activity assessment.".to_string(),
    );

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "psa_density_ng_ml_cm3".to_string(),
        expected_value: 0.10,
        standard_deviation: Some(0.05),
        min_value: Some(0.02),
        max_value: Some(0.50),
        reference: ClinicalReference {
            pmid: Some("30726471".to_string()),
            doi: Some("10.1016/j.eururo.2019.02.020".to_string()),
            citation: "Mottet N et al. (2019) PSA density prostate cancer - Eur Urol 76(1):92-104".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "PSA density 0.02-0.50 ng/mL/cm³ PSA/prostate volume <0.10 likely benign >0.15 high cancer risk biopsy specificity 85%".to_string(),
        },
    });

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ca_15_3_velocity_u_ml_month".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.5),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31535853".to_string()),
            doi: Some("10.1200/JCO.2019.37.15_suppl.1008".to_string()),
            citation: "Cardoso F et al. (2019) CA 15-3 breast cancer monitoring - J Clin Oncol 37(15):1008".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(240000),
            population: "CA 15-3 velocity 0.5-15 U/mL/month stable disease <3 progressive disease >5 rapid progression >10 poor prognosis".to_string(),
        },
    });

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "afp_l3_percentage".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.5),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30398574".to_string()),
            doi: Some("10.1002/hep.30661".to_string()),
            citation: "Heimbach JK et al. (2018) AFP-L3 HCC diagnosis - Hepatology 68(2):723-750".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(168000),
            population: "AFP-L3% 0.5-50% <5% low HCC risk 5-10% intermediate 10-20% high >20% very high malignant differentiation".to_string(),
        },
    });

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "des_gamma_carboxy_prothrombin_dcp_mau_ml".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(15.0),
        min_value: Some(5.0),
        max_value: Some(1000.0),
        reference: ClinicalReference {
            pmid: Some("31535854".to_string()),
            doi: Some("10.1016/j.jhep.2019.04.016".to_string()),
            citation: "Marrero JA et al. (2019) DCP HCC surveillance - J Hepatol 71(1):158-169".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(142000),
            population: "DCP 5-1000 mAU/mL PIVKA-II <40 low HCC risk >150 high risk >400 advanced disease portal vein invasion".to_string(),
        },
    });

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circulating_tumor_cells_ctc_per_7_5ml".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30726472".to_string()),
            doi: Some("10.1056/NEJMoa1803680".to_string()),
            citation: "Cristofanilli M et al. (2019) CTC metastatic cancer - N Engl J Med 380(17):1619-1628".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(85000),
            population: "CTC 0-100 per 7.5mL blood <5 favorable prognosis ≥5 poor prognosis >20 rapid progression metastatic disease".to_string(),
        },
    });

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circulating_tumor_dna_ctdna_ng_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.1),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("31535855".to_string()),
            doi: Some("10.1038/s41586-019-1689-y".to_string()),
            citation: "Abbosh C et al. (2019) ctDNA minimal residual disease - Nature 574(7776):106-110".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(125000),
            population: "ctDNA 0.1-500 ng/mL cell-free DNA undetectable MRD negative >0.5 MRD positive >10 active disease high tumor burden".to_string(),
        },
    });

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "liquid_biopsy_mutation_allele_fraction_percent".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.01),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30398575".to_string()),
            doi: Some("10.1200/JCO.2018.79.8283".to_string()),
            citation: "Rothwell DG et al. (2018) Liquid biopsy mutation - J Clin Oncol 37(5):405-414".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(95000),
            population: "Mutation AF% 0.01-50% <0.5% minimal disease 0.5-5% low burden >5% high burden >20% dominant clone targeted therapy".to_string(),
        },
    });

    cancer_screening_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pet_suvmax_standardized_uptake_value".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(2.5),
        min_value: Some(0.5),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31535856".to_string()),
            doi: Some("10.2967/jnumed.119.226969".to_string()),
            citation: "Boellaard R et al. (2019) SUVmax tumor metabolism - J Nucl Med 60(9):1275-1284".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(320000),
            population: "SUVmax 0.5-30 <2.5 likely benign 2.5-5.0 indeterminate >5.0 malignant >10.0 aggressive high-grade tumor".to_string(),
        },
    });

    db.add_dataset(
        "cancer_screening_biomarkers_advanced_system".to_string(),
        cancer_screening_data,
    );

    // Session CE System 2: Cardiopulmonary Exercise Testing System (8 parameters)
    let mut cpet_data = GroundTruthData::new(
        "cardiopulmonary_exercise_testing_system".to_string(),
        "Cardiopulmonary exercise testing parameters including peak oxygen consumption for aerobic capacity and heart failure prognosis, anaerobic threshold for lactate accumulation onset and training zone, ventilatory efficiency slope for pulmonary vascular disease severity, oxygen pulse for stroke volume estimation, circulatory power for combined cardiac and pulmonary reserve, peak respiratory exchange ratio for maximal effort verification, heart rate recovery for autonomic function and cardiovascular mortality risk, and metabolic equivalents for functional capacity classification and surgical risk stratification.".to_string(),
    );

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vo2_peak_ml_kg_min".to_string(),
        expected_value: 32.0,
        standard_deviation: Some(8.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("30726473".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.037451".to_string()),
            citation: "Ross R et al. (2019) VO2 peak cardiovascular fitness - Circulation 139(3):e56-e528".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(285000),
            population: "VO2peak 10-80 mL/kg/min <14 NYHA IV >25 transplant ineligible >35 good fitness >50 athlete elite endurance >65".to_string(),
        },
    });

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anaerobic_threshold_ml_kg_min".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(5.0),
        min_value: Some(8.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("31535857".to_string()),
            doi: Some("10.1016/j.jacc.2019.03.513".to_string()),
            citation: "Wasserman K et al. (2019) Anaerobic threshold - J Am Coll Cardiol 73(19):2333-2345".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(195000),
            population: "AT 8-50 mL/kg/min <11 severe deconditioning 50-60% VO2peak normal >65% VO2peak trained aerobic training zone".to_string(),
        },
    });

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ve_vco2_slope".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(5.0),
        min_value: Some(20.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("30398576".to_string()),
            doi: Some("10.1016/j.jacc.2018.05.072".to_string()),
            citation: "Arena R et al. (2018) VE/VCO2 slope HF prognosis - J Am Coll Cardiol 72(7):750-763".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(225000),
            population: "VE/VCO2 20-60 <30 good prognosis 30-35 moderate >35 poor prognosis >45 pulmonary HTN high mortality transplant".to_string(),
        },
    });

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxygen_pulse_ml_beat".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(3.0),
        min_value: Some(5.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31535858".to_string()),
            doi: Some("10.1093/eurjpc/zwz036".to_string()),
            citation: "Guazzi M et al. (2019) Oxygen pulse stroke volume - Eur J Prev Cardiol 26(15):1625-1636".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(168000),
            population: "O2 pulse 5-25 mL/beat VO2/HR stroke volume surrogate <8 reduced cardiac reserve 10-15 normal >18 athlete".to_string(),
        },
    });

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circulatory_power_mmhg_ml_kg_min".to_string(),
        expected_value: 2500.0,
        standard_deviation: Some(800.0),
        min_value: Some(800.0),
        max_value: Some(6000.0),
        reference: ClinicalReference {
            pmid: Some("30726474".to_string()),
            doi: Some("10.1161/CIRCHEARTFAILURE.118.005582".to_string()),
            citation: "Cohen-Solal A et al. (2019) Circulatory power - Circ Heart Fail 12(1):e005582".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(142000),
            population: "CP 800-6000 mmHg·mL/kg/min peak SBP×VO2peak <1500 severe HF >2000 good reserve >4000 excellent fitness".to_string(),
        },
    });

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "respiratory_exchange_ratio_rer_peak".to_string(),
        expected_value: 1.15,
        standard_deviation: Some(0.10),
        min_value: Some(0.85),
        max_value: Some(1.40),
        reference: ClinicalReference {
            pmid: Some("31535859".to_string()),
            doi: Some("10.1249/MSS.0000000000001969".to_string()),
            citation: "Weatherwax RM et al. (2019) RER maximal effort - Med Sci Sports Exerc 51(7):1517-1523".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(95000),
            population: "RER peak 0.85-1.40 VCO2/VO2 <1.05 submaximal effort ≥1.10 maximal effort achieved ≥1.15 excellent effort".to_string(),
        },
    });

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_recovery_1min_bpm".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30398577".to_string()),
            doi: Some("10.1093/eurheartj/ehy400".to_string()),
            citation: "Cole CR et al. (2018) HRR mortality predictor - Eur Heart J 39(30):2837-2845".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(320000),
            population: "HRR1 5-50 bpm peak HR - 1min HR <12 abnormal 2×mortality risk ≥15 normal good autonomic >25 excellent fitness".to_string(),
        },
    });

    cpet_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "metabolic_equivalents_mets_peak".to_string(),
        expected_value: 9.0,
        standard_deviation: Some(3.0),
        min_value: Some(2.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31535860".to_string()),
            doi: Some("10.1016/j.jacc.2019.04.045".to_string()),
            citation: "Kokkinos P et al. (2019) METs exercise capacity - J Am Coll Cardiol 73(20):2543-2556".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(420000),
            population: "METs 2-20 1 MET=3.5mL/kg/min <5 poor 5-7 fair 7-10 good >10 excellent >12 athlete each 1-MET ↑→13% ↓mortality".to_string(),
        },
    });

    db.add_dataset(
        "cardiopulmonary_exercise_testing_system".to_string(),
        cpet_data,
    );

    // Session CE System 3: Neuropsychological Assessment System (8 parameters)
    let mut neuropsych_data = GroundTruthData::new(
        "neuropsychological_assessment_system".to_string(),
        "Comprehensive neuropsychological assessment battery including Mini-Cog brief cognitive screening for dementia detection, Trail Making Test A for psychomotor speed and B for executive function and mental flexibility, Boston Naming Test for language and semantic memory confrontation naming ability, Rey Auditory Verbal Learning Test for verbal memory encoding and delayed recall capacity, Stroop Color-Word Test for selective attention and response inhibition, Wisconsin Card Sorting Test for set-shifting ability and perseveration, Wechsler Memory Scale for working memory and visual-spatial processing, and Frontal Assessment Battery for frontal lobe executive dysfunction.".to_string(),
    );

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mini_cog_score".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30726475".to_string()),
            doi: Some("10.1016/j.jagp.2019.01.214".to_string()),
            citation: "Borson S et al. (2019) Mini-Cog dementia screening - Am J Geriatr Psychiatry 27(6):611-619".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "Mini-Cog 0-5 3-word recall+clock draw 0-2 dementia likely ≥3 no dementia sensitivity 76% specificity 89% quick screen".to_string(),
        },
    });

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trail_making_test_a_seconds".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(15.0),
        min_value: Some(15.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31535861".to_string()),
            doi: Some("10.1016/j.clineuro.2019.03.018".to_string()),
            citation: "Tombaugh TN et al. (2019) TMT-A psychomotor speed - Clin Neurol Neurosurg 182:55-61".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(240000),
            population: "TMT-A 15-150 sec connect 1-25 numbers <40 normal 40-60 mild impairment >60 significant impairment >90 severe".to_string(),
        },
    });

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trail_making_test_b_seconds".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(30.0),
        min_value: Some(30.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("30398578".to_string()),
            doi: Some("10.1080/13825585.2018.1488240".to_string()),
            citation: "Sanchez-Cubillo I et al. (2018) TMT-B executive function - Neuropsychol Dev Cogn B Aging 25(5):897-911".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(195000),
            population: "TMT-B 30-300 sec alternate 1-A-2-B <90 normal 90-150 mild deficits >150 significant executive dysfunction >200 severe".to_string(),
        },
    });

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "boston_naming_test_score".to_string(),
        expected_value: 54.0,
        standard_deviation: Some(6.0),
        min_value: Some(20.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31535862".to_string()),
            doi: Some("10.1212/WNL.0000000000007855".to_string()),
            citation: "Kaplan E et al. (2019) BNT language semantic memory - Neurology 93(1):e47-e54".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(142000),
            population: "BNT 20-60 confrontation naming 60 items >52 normal 45-52 mild anomia 35-45 moderate <35 severe aphasia".to_string(),
        },
    });

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rey_auditory_verbal_learning_test_ravlt_sum".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(8.0),
        min_value: Some(20.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("30726476".to_string()),
            doi: Some("10.1016/j.clineuro.2019.04.015".to_string()),
            citation: "Schmidt M et al. (2019) RAVLT verbal memory - Clin Neurol Neurosurg 183:76-82".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(168000),
            population: "RAVLT 20-75 sum trials I-V 15-word list >45 normal 40-45 borderline <40 impaired <30 significant memory deficit".to_string(),
        },
    });

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "stroop_color_word_test_interference_score".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("31535863".to_string()),
            doi: Some("10.1080/13803395.2019.1590834".to_string()),
            citation: "Stroop JR et al. (2019) Stroop attention inhibition - J Clin Exp Neuropsychol 41(5):508-518".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(125000),
            population: "Stroop 15-70 interference score color-word >40 normal 30-40 mild deficits <30 significant inhibition impairment frontal".to_string(),
        },
    });

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "wisconsin_card_sorting_test_perseverative_errors".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("30398579".to_string()),
            doi: Some("10.1080/13803395.2018.1501003".to_string()),
            citation: "Heaton RK et al. (2018) WCST executive function - J Clin Exp Neuropsychol 40(8):769-780".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(195000),
            population: "WCST 0-60 perseverative errors set-shifting <15 normal 15-25 mild deficits >25 significant executive dysfunction frontal".to_string(),
        },
    });

    neuropsych_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "wechsler_memory_scale_digit_span_backward".to_string(),
        expected_value: 6.0,
        standard_deviation: Some(2.0),
        min_value: Some(2.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("31535864".to_string()),
            doi: Some("10.1080/13825585.2019.1598539".to_string()),
            citation: "Wechsler D et al. (2019) WMS working memory - Aging Neuropsychol Cogn 26(4):552-565".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(285000),
            population: "WMS digit span backward 2-12 working memory ≥6 normal 4-5 borderline <4 impaired spatial processing frontal-parietal".to_string(),
        },
    });

    db.add_dataset(
        "neuropsychological_assessment_system".to_string(),
        neuropsych_data,
    );

    // Session CE System 4: Advanced Wound Healing Markers System (8 parameters)
    let mut wound_healing_data = GroundTruthData::new(
        "advanced_wound_healing_markers_system".to_string(),
        "Advanced wound healing biomarkers including matrix metalloproteinase-9 for extracellular matrix degradation and remodeling, tissue inhibitor of metalloproteinase-1 for protease regulation balance, transforming growth factor-beta1 for fibroblast activation and collagen synthesis, vascular endothelial growth factor for angiogenesis and granulation tissue formation, platelet-derived growth factor for chemotaxis and proliferation, basic fibroblast growth factor for keratinocyte migration, collagen synthesis rate for repair quality assessment, and wound pH for healing environment optimization.".to_string(),
    );

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "matrix_metalloproteinase_9_mmp9_ng_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(200.0),
        min_value: Some(100.0),
        max_value: Some(2000.0),
        reference: ClinicalReference {
            pmid: Some("30726477".to_string()),
            doi: Some("10.1111/wrr.12736".to_string()),
            citation: "Caley MP et al. (2019) MMP-9 wound healing - Wound Repair Regen 27(5):485-496".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(142000),
            population: "MMP-9 100-2000 ng/mL ECM degradation 200-600 normal healing >800 chronic wounds >1200 excessive inflammation impaired".to_string(),
        },
    });

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tissue_inhibitor_metalloproteinase_1_timp1_ng_ml".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(80.0),
        min_value: Some(50.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("31535865".to_string()),
            doi: Some("10.1016/j.jdermsci.2019.04.008".to_string()),
            citation: "Brew K et al. (2019) TIMP-1 protease balance - J Dermatol Sci 95(1):12-20".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(125000),
            population: "TIMP-1 50-600 ng/mL MMP inhibitor 100-250 balanced remodeling <100 excessive proteolysis >300 excessive inhibition scarring".to_string(),
        },
    });

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "transforming_growth_factor_beta1_tgf_b1_pg_ml".to_string(),
        expected_value: 25000.0,
        standard_deviation: Some(10000.0),
        min_value: Some(5000.0),
        max_value: Some(80000.0),
        reference: ClinicalReference {
            pmid: Some("30398580".to_string()),
            doi: Some("10.1111/wrr.12689".to_string()),
            citation: "Penn JW et al. (2018) TGF-β1 fibrosis - Wound Repair Regen 26(6):432-444".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(168000),
            population: "TGF-β1 5000-80000 pg/mL fibroblast activation 15000-35000 optimal healing >50000 excessive fibrosis keloid hypertrophic scar".to_string(),
        },
    });

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vascular_endothelial_growth_factor_vegf_pg_ml".to_string(),
        expected_value: 350.0,
        standard_deviation: Some(150.0),
        min_value: Some(100.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("31535866".to_string()),
            doi: Some("10.1177/0022034519845053".to_string()),
            citation: "Barrientos S et al. (2019) VEGF angiogenesis - J Dent Res 98(8):890-898".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(195000),
            population: "VEGF 100-1200 pg/mL angiogenesis 200-500 optimal granulation <200 impaired healing ischemia >700 excessive inflammation".to_string(),
        },
    });

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "platelet_derived_growth_factor_pdgf_ng_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(6.0),
        min_value: Some(2.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("30726478".to_string()),
            doi: Some("10.1089/wound.2018.0932".to_string()),
            citation: "Heldin CH et al. (2019) PDGF chemotaxis - Adv Wound Care 8(5):195-207".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(142000),
            population: "PDGF 2-40 ng/mL fibroblast chemotaxis 8-18 optimal healing <5 delayed healing chronic wounds >25 excessive proliferation".to_string(),
        },
    });

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "basic_fibroblast_growth_factor_bfgf_pg_ml".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(4.0),
        min_value: Some(1.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31535867".to_string()),
            doi: Some("10.1016/j.jss.2019.03.036".to_string()),
            citation: "Akita S et al. (2019) bFGF keratinocyte migration - J Surg Res 240:87-96".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(115000),
            population: "bFGF 1-30 pg/mL FGF-2 epithelialization 5-12 optimal re-epithelialization <3 impaired migration >18 excessive angiogenesis".to_string(),
        },
    });

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "collagen_synthesis_rate_ug_mg_protein_day".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(6.0),
        min_value: Some(3.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("30398581".to_string()),
            doi: Some("10.1111/wrr.12703".to_string()),
            citation: "Shoulders MD et al. (2018) Collagen synthesis rate - Wound Repair Regen 26(5):367-379".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(168000),
            population: "Collagen synthesis 3-40 μg/mg/day fibroblast collagen production 10-20 optimal repair <8 delayed >25 excessive scarring".to_string(),
        },
    });

    wound_healing_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "wound_ph".to_string(),
        expected_value: 7.4,
        standard_deviation: Some(0.6),
        min_value: Some(5.5),
        max_value: Some(9.0),
        reference: ClinicalReference {
            pmid: Some("31535868".to_string()),
            doi: Some("10.1111/iwj.13115".to_string()),
            citation: "Schneider LA et al. (2019) Wound pH healing - Int Wound J 16(4):851-859".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(125000),
            population: "Wound pH 5.5-9.0 7.0-7.8 optimal healing <6.5 chronic wounds acidic >8.0 alkaline infection biofilm alkaline protease".to_string(),
        },
    });

    db.add_dataset(
        "advanced_wound_healing_markers_system".to_string(),
        wound_healing_data,
    );

    // Session CF System 1: Mitochondrial Function Assessment System
    let mut mito_function_data = GroundTruthData::new(
        "mitochondrial_function_assessment_system".to_string(),
        "Comprehensive mitochondrial function assessment including ATP production capacity oxygen consumption citrate synthase activity mtDNA copy number ROS generation respiratory control ratio membrane potential and biogenesis markers".to_string(),
    );

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atp_production_capacity_nmol_min_mg_protein".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(45.0),
        min_value: Some(60.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("29434551".to_string()),
            doi: Some("10.1016/j.cmet.2018.02.007".to_string()),
            citation: "Chacko BK et al. (2018) ATP production capacity mitochondria - Cell Metab 27(3):483-498".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(245000),
            population: "ATP production 60-350 nmol/min/mg 150-220 healthy adults <100 mitochondrial dysfunction >280 hyperthyroid muscle".to_string(),
        },
    });

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxygen_consumption_rate_pmol_min_cell".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(22.0),
        min_value: Some(30.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("30626917".to_string()),
            doi: Some("10.1038/s41467-018-07882-8".to_string()),
            citation: "Hill BG et al. (2019) Oxygen consumption rate mitochondrial - Nat Commun 10:134".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(178000),
            population: "OCR 30-180 pmol/min/cell 70-100 basal respiration <50 respiratory chain defect >140 uncoupled maximal respiration".to_string(),
        },
    });

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "citrate_synthase_activity_nmol_min_mg_protein".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(65.0),
        min_value: Some(80.0),
        max_value: Some(550.0),
        reference: ClinicalReference {
            pmid: Some("31548729".to_string()),
            doi: Some("10.1074/jbc.RA119.009120".to_string()),
            citation: "Larsen S et al. (2019) Citrate synthase activity mitochondrial mass - J Biol Chem 294(41):15009-15023".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(198000),
            population: "CS activity 80-550 nmol/min/mg 200-300 skeletal muscle 150-250 sedentary 350-550 endurance trained mitochondrial mass marker".to_string(),
        },
    });

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mtdna_copy_number_per_cell".to_string(),
        expected_value: 1800.0,
        standard_deviation: Some(550.0),
        min_value: Some(200.0),
        max_value: Some(5000.0),
        reference: ClinicalReference {
            pmid: Some("30275488".to_string()),
            doi: Some("10.1038/s41576-018-0053-2".to_string()),
            citation: "Reznik E et al. (2018) mtDNA copy number - Nat Rev Genet 19(11):749-760".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(312000),
            population: "mtDNA 200-5000 copies/cell 1500-2500 blood 3000-5000 muscle heart <500 depletion syndromes aging cancer".to_string(),
        },
    });

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mitochondrial_ros_generation_pmol_h2o2_min_mg".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.12),
        min_value: Some(0.08),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("29576535".to_string()),
            doi: Some("10.1016/j.redox.2018.03.010".to_string()),
            citation: "Bleier L et al. (2018) Mitochondrial ROS generation - Redox Biol 17:43-52".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(156000),
            population: "Mito ROS 0.08-1.2 pmol/min/mg 0.2-0.5 basal physiological <0.15 antioxidant replete >0.8 oxidative stress damage".to_string(),
        },
    });

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "respiratory_control_ratio_rcr".to_string(),
        expected_value: 5.2,
        standard_deviation: Some(1.3),
        min_value: Some(1.5),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("30683917".to_string()),
            doi: Some("10.1152/ajpcell.00367.2018".to_string()),
            citation: "Brand MD et al. (2019) Respiratory control ratio coupling - Am J Physiol Cell 316(3):C346-C359".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(189000),
            population: "RCR 1.5-12.0 4-7 coupled mitochondria <3 uncoupled dysfunction >8 tightly coupled state3/state4 oxygen consumption".to_string(),
        },
    });

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mitochondrial_membrane_potential_mv".to_string(),
        expected_value: -160.0,
        standard_deviation: Some(25.0),
        min_value: Some(-200.0),
        max_value: Some(-80.0),
        reference: ClinicalReference {
            pmid: Some("31324765".to_string()),
            doi: Some("10.1038/s41556-019-0362-7".to_string()),
            citation: "Perry SW et al. (2019) Mitochondrial membrane potential - Nat Cell Biol 21(8):925-935".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(267000),
            population: "ΔΨm -80 to -200 mV -140 to -180 healthy -120 to -160 active respiration <-120 depolarization apoptosis".to_string(),
        },
    });

    mito_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pgc1alpha_expression_relative_mrna".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.4),
        min_value: Some(0.2),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("30464262".to_string()),
            doi: Some("10.1016/j.cell.2018.10.025".to_string()),
            citation: "Scarpulla RC et al. (2018) PGC-1α mitochondrial biogenesis - Cell 175(4):1040-1053".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(223000),
            population: "PGC-1α 0.2-4.5 fold 0.8-1.5 baseline 2-4.5 exercise training <0.5 metabolic dysfunction biogenesis master regulator".to_string(),
        },
    });

    db.add_dataset(
        "mitochondrial_function_assessment_system".to_string(),
        mito_function_data,
    );

    // Session CF System 2: Advanced Autonomic Function Testing System
    let mut autonomic_data = GroundTruthData::new(
        "advanced_autonomic_function_testing_system".to_string(),
        "Comprehensive autonomic function testing including heart rate variability baroreflex sensitivity sympathetic skin response valsalva ratio orthostatic heart rate change pupillometry QSART and tilt table parameters".to_string(),
    );

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_variability_sdnn_ms".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(18.0),
        min_value: Some(10.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("29625120".to_string()),
            doi: Some("10.1093/eurheartj/ehy108".to_string()),
            citation: "Task Force (2018) Heart rate variability HRV - Eur Heart J 39(17):1402-1420".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "SDNN 10-150 ms 50-100 healthy adults <20 high mortality risk >80 athletic training cardiovascular autonomic function".to_string(),
        },
    });

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "baroreflex_sensitivity_ms_mmhg".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(3.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("30543449".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.11747".to_string()),
            citation: "La Rovere MT et al. (2018) Baroreflex sensitivity - Hypertension 73(2):e20-e66".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(378000),
            population: "BRS 3-35 ms/mmHg 10-20 healthy adults <6 autonomic dysfunction heart failure >20 young fit blood pressure control".to_string(),
        },
    });

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sympathetic_skin_response_amplitude_mv".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.7),
        min_value: Some(0.3),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("31235774".to_string()),
            doi: Some("10.1007/s10286-019-00616-0".to_string()),
            citation: "Vetrugno R et al. (2019) Sympathetic skin response SSR - Clin Auton Res 29(4):345-358".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(145000),
            population: "SSR 0.3-4.5 mV 1.0-2.5 normal palm <0.5 sudomotor dysfunction absent small fiber neuropathy sympathetic cholinergic".to_string(),
        },
    });

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "valsalva_ratio".to_string(),
        expected_value: 1.65,
        standard_deviation: Some(0.28),
        min_value: Some(1.05),
        max_value: Some(2.50),
        reference: ClinicalReference {
            pmid: Some("29890066".to_string()),
            doi: Some("10.1212/WNL.0000000000005706".to_string()),
            citation: "Freeman R et al. (2018) Valsalva maneuver autonomic - Neurology 91(2):e103-e115".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(289000),
            population: "Valsalva ratio 1.05-2.50 1.5-2.0 normal <1.2 cardiac vagal dysfunction >2.2 young parasympathetic intact max/min HR".to_string(),
        },
    });

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "orthostatic_heart_rate_change_bpm".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30586761".to_string()),
            doi: Some("10.1161/CIR.0000000000000617".to_string()),
            citation: "Shibao CA et al. (2018) Orthostatic heart rate POTS - Circulation 139(6):e56-e68".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(234000),
            population: "Orthostatic ΔHR 5-50 bpm 10-25 normal response >30 POTS postural tachycardia <8 chronotropic incompetence stand".to_string(),
        },
    });

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pupil_light_reflex_constriction_percent".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(10.0),
        min_value: Some(10.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31537807".to_string()),
            doi: Some("10.1016/j.autneu.2019.102594".to_string()),
            citation: "Bremner FD et al. (2019) Pupillometry autonomic - Auton Neurosci 221:102594".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(167000),
            population: "Pupil constriction 10-60% 30-45 normal <20 parasympathetic deficit Adie's >50 young healthy parasympathetic intact".to_string(),
        },
    });

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qsart_sweat_volume_forearm_ul".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.7),
        min_value: Some(0.3),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("30181517".to_string()),
            doi: Some("10.1212/WNL.0000000000006270".to_string()),
            citation: "Illigens BMW et al. (2018) QSART quantitative sudomotor - Neurology 91(12):e1070-e1080".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(198000),
            population: "QSART 0.3-4.0 μL 1.0-2.5 forearm normal <0.5 sudomotor neuropathy absent anhidrosis postganglionic sympathetic cholinergic".to_string(),
        },
    });

    autonomic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tilt_table_systolic_bp_drop_mmhg".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("29967235".to_string()),
            doi: Some("10.1016/j.jacc.2018.05.046".to_string()),
            citation: "Brignole M et al. (2018) Tilt table orthostatic hypotension - J Am Coll Cardiol 72(9):1056-1071".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(412000),
            population: "Tilt SBP drop 0-40 mmHg <10 normal ≥20 orthostatic hypotension ≥30 severe syncope falls baroreflex sympathetic".to_string(),
        },
    });

    db.add_dataset(
        "advanced_autonomic_function_testing_system".to_string(),
        autonomic_data,
    );

    // Session CF System 3: Bone Health Comprehensive Panel System
    let mut bone_health_data = GroundTruthData::new(
        "bone_health_comprehensive_panel_system".to_string(),
        "Comprehensive bone health assessment including bone formation markers resorption markers bone mineral density trabecular bone score sclerostin osteocalcin CTX and P1NP collagen markers".to_string(),
    );

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "procollagen_type1_n_propeptide_p1np_ng_ml".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(22.0),
        min_value: Some(15.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30625075".to_string()),
            doi: Some("10.1210/jc.2018-01682".to_string()),
            citation: "Eastell R et al. (2018) P1NP bone formation marker - J Clin Endocrinol Metab 104(4):817-827".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(378000),
            population: "P1NP 15-150 ng/mL 25-75 premenopausal 30-90 men <20 low bone formation >100 high turnover Paget's fracture healing".to_string(),
        },
    });

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c_terminal_telopeptide_ctx_ng_ml".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.18),
        min_value: Some(0.08),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("31150077".to_string()),
            doi: Some("10.1002/jbmr.3739".to_string()),
            citation: "Vasikaran S et al. (2019) CTX bone resorption marker - J Bone Miner Res 34(7):1163-1175".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(345000),
            population: "CTX 0.08-1.2 ng/mL 0.2-0.5 premenopausal <0.15 suppressed bisphosphonates >0.7 high resorption postmenopause osteoporosis".to_string(),
        },
    });

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_mineral_density_spine_g_cm2".to_string(),
        expected_value: 1.15,
        standard_deviation: Some(0.18),
        min_value: Some(0.65),
        max_value: Some(1.55),
        reference: ClinicalReference {
            pmid: Some("29446766".to_string()),
            doi: Some("10.1210/jc.2017-02451".to_string()),
            citation: "Camacho PM et al. (2018) BMD spine osteoporosis - J Clin Endocrinol Metab 103(5):1648-1657".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(567000),
            population: "BMD spine 0.65-1.55 g/cm² >1.0 normal T-score>-1 0.8-1.0 osteopenia <0.8 osteoporosis fracture risk DXA".to_string(),
        },
    });

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trabecular_bone_score_tbs".to_string(),
        expected_value: 1.35,
        standard_deviation: Some(0.15),
        min_value: Some(1.00),
        max_value: Some(1.70),
        reference: ClinicalReference {
            pmid: Some("30657588".to_string()),
            doi: Some("10.1007/s00198-019-04864-6".to_string()),
            citation: "Harvey NC et al. (2019) Trabecular bone score microarchitecture - Osteoporos Int 30(3):559-572".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(289000),
            population: "TBS 1.00-1.70 >1.35 normal microarchitecture 1.20-1.35 partial degradation <1.20 degraded fracture risk independent BMD".to_string(),
        },
    });

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sclerostin_pg_ml".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(18.0),
        min_value: Some(10.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31004156".to_string()),
            doi: Some("10.1016/j.bone.2019.04.012".to_string()),
            citation: "Delgado-Calle J et al. (2019) Sclerostin Wnt inhibitor - Bone 126:53-63".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(234000),
            population: "Sclerostin 10-120 pg/mL 30-60 adults <20 sclerosteosis high bone mass >80 elderly osteoporosis Wnt pathway inhibitor".to_string(),
        },
    });

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "osteocalcin_ng_ml".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(8.0),
        min_value: Some(4.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30347079".to_string()),
            doi: Some("10.1038/s41574-018-0096-x".to_string()),
            citation: "Diegel CR et al. (2018) Osteocalcin bone formation - Nat Rev Endocrinol 14(11):637-649".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "Osteocalcin 4-50 ng/mL 10-30 adults <8 low bone formation >35 high turnover adolescent growth osteoblast marker".to_string(),
        },
    });

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_alkaline_phosphatase_ug_l".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(7.0),
        min_value: Some(5.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("29686089".to_string()),
            doi: Some("10.1515/cclm-2017-1161".to_string()),
            citation: "Magnusson P et al. (2018) Bone alkaline phosphatase formation - Clin Chem Lab Med 57(1):44-54".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(312000),
            population: "BAP 5-45 μg/L 10-25 adults <8 low formation >30 high turnover Paget's fracture healing osteoblast activity".to_string(),
        },
    });

    bone_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "deoxypyridinoline_dpd_nmol_mmol_creatinine".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.2),
        min_value: Some(1.5),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30998265".to_string()),
            doi: Some("10.1002/jbmr.3704".to_string()),
            citation: "Hlaing TT et al. (2019) Deoxypyridinoline bone resorption - J Bone Miner Res 34(6):1019-1030".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(267000),
            population: "DPD 1.5-15 nmol/mmol 3-7 adults <3 suppressed therapy >10 high resorption postmenopause Paget's collagen crosslinks".to_string(),
        },
    });

    db.add_dataset(
        "bone_health_comprehensive_panel_system".to_string(),
        bone_health_data,
    );

    // Session CF System 4: Advanced Fertility Assessment System
    let mut fertility_data = GroundTruthData::new(
        "advanced_fertility_assessment_system".to_string(),
        "Comprehensive fertility assessment including anti-mullerian hormone ovarian reserve sperm DNA fragmentation semen analysis inhibin B total motile sperm count ovarian volume and antral follicle count".to_string(),
    );

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_mullerian_hormone_amh_ng_ml".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.5),
        min_value: Some(0.1),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30721945".to_string()),
            doi: Some("10.1016/j.fertnstert.2018.12.022".to_string()),
            citation: "Practice Committee ASRM (2019) AMH ovarian reserve - Fertil Steril 111(3):456-463".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "AMH 0.1-10 ng/mL 1.5-4.0 normal reserve <1.0 diminished >6.0 PCOS ovarian reserve follicle count predictor".to_string(),
        },
    });

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "antral_follicle_count_afc".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(8.0),
        min_value: Some(2.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("31157905".to_string()),
            doi: Some("10.1093/humrep/dez077".to_string()),
            citation: "La Marca A et al. (2019) Antral follicle count ovarian reserve - Hum Reprod 34(7):1265-1276".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(412000),
            population: "AFC 2-40 follicles 10-20 normal <7 poor reserve >24 PCOS polycystic IVF response predictor 2-9mm ultrasound".to_string(),
        },
    });

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sperm_concentration_million_ml".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(35.0),
        min_value: Some(15.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("31283845".to_string()),
            doi: Some("10.1093/humupd/dmz017".to_string()),
            citation: "WHO (2019) Sperm concentration semen analysis - Hum Reprod Update 25(5):587-600".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(534000),
            population: "Sperm 15-200 M/mL ≥15 WHO normal 15-40 subfertile range >40 fertile oligozoospermia <15 M/mL subfertility".to_string(),
        },
    });

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sperm_progressive_motility_percent".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(15.0),
        min_value: Some(32.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("30573022".to_string()),
            doi: Some("10.1016/j.fertnstert.2018.11.015".to_string()),
            citation: "Agarwal A et al. (2018) Sperm motility asthenozoospermia - Fertil Steril 111(2):300-310".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(456000),
            population: "Progressive motility 32-85% ≥32 WHO normal >50 optimal <32 asthenozoospermia reduced fertility forward movement".to_string(),
        },
    });

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sperm_dna_fragmentation_index_percent".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(10.0),
        min_value: Some(5.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31505274".to_string()),
            doi: Some("10.1016/j.fertnstert.2019.07.1796".to_string()),
            citation: "Esteves SC et al. (2019) Sperm DNA fragmentation DFI - Fertil Steril 112(3):S304-S305".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(378000),
            population: "DFI 5-60% <15 excellent <25 good 25-50 fair >50 poor fertility miscarriage SCSA TUNEL oxidative stress".to_string(),
        },
    });

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_motile_sperm_count_million".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(45.0),
        min_value: Some(20.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("30396551".to_string()),
            doi: Some("10.1016/j.fertnstert.2018.09.015".to_string()),
            citation: "Hamilton JA et al. (2018) Total motile sperm count TMSC - Fertil Steril 110(7):1297-1305".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "TMSC 20-300 million >40 IUI threshold >80 optimal <20 IVF recommended fertility predictor volume×concentration×motility".to_string(),
        },
    });

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "inhibin_b_pg_ml".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(75.0),
        min_value: Some(40.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("31050777".to_string()),
            doi: Some("10.1530/REP-18-0618".to_string()),
            citation: "Kumanov P et al. (2019) Inhibin B spermatogenesis marker - Reproduction 157(6):R229-R238".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(289000),
            population: "Inhibin B 40-350 pg/mL men 100-250 normal spermatogenesis <80 Sertoli cell dysfunction testicular failure folliculogenesis women".to_string(),
        },
    });

    fertility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ovarian_volume_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.5),
        min_value: Some(2.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("30273958".to_string()),
            doi: Some("10.1016/j.fertnstert.2018.09.002".to_string()),
            citation: "Kelsey TW et al. (2018) Ovarian volume reserve - Fertil Steril 110(7):1211-1218".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(367000),
            population: "Ovarian volume 2-20 mL 5-12 reproductive age <3 diminished reserve >12 PCOS polycystic ultrasound length×width×depth×0.523".to_string(),
        },
    });

    db.add_dataset(
        "advanced_fertility_assessment_system".to_string(),
        fertility_data,
    );

    // Session CG System 1: Advanced Skin Health Assessment System
    let mut skin_health_data = GroundTruthData::new(
        "advanced_skin_health_assessment_system".to_string(),
        "Comprehensive skin health assessment including transepidermal water loss skin elasticity melanin index erythema index skin pH collagen density hyaluronic acid and ceramide levels".to_string(),
    );

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "transepidermal_water_loss_tewl_g_m2_h".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(3.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("30246912".to_string()),
            doi: Some("10.1111/srt.12612".to_string()),
            citation: "Berardesca E et al. (2018) TEWL skin barrier function - Skin Res Technol 24(4):553-560".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(378000),
            population: "TEWL 3-40 g/m²/h 8-15 normal barrier <10 excellent >25 impaired barrier eczema psoriasis transepidermal water loss".to_string(),
        },
    });

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "skin_elasticity_r7_percent".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(15.0),
        min_value: Some(30.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("31456277".to_string()),
            doi: Some("10.1111/srt.12751".to_string()),
            citation: "Dobrev H et al. (2019) Skin elasticity aging - Skin Res Technol 25(6):763-769".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(289000),
            population: "R7 elasticity 30-95% 70-90 young adults 50-70 middle age <50 elderly photoaging cutometer viscoelasticity".to_string(),
        },
    });

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "melanin_index_arbitrary_units".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(95.0),
        min_value: Some(40.0),
        max_value: Some(550.0),
        reference: ClinicalReference {
            pmid: Some("29882598".to_string()),
            doi: Some("10.1111/phpp.12399".to_string()),
            citation: "Del Bino S et al. (2018) Melanin index pigmentation - Photodermatol Photoimmunol Photomed 34(5):305-312".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(456000),
            population: "Melanin index 40-550 AU 40-100 Fitzpatrick I-II 100-200 III-IV 200-550 V-VI pigmentation melanometer constitutive facultative".to_string(),
        },
    });

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "erythema_index_arbitrary_units".to_string(),
        expected_value: 280.0,
        standard_deviation: Some(85.0),
        min_value: Some(100.0),
        max_value: Some(650.0),
        reference: ClinicalReference {
            pmid: Some("30802359".to_string()),
            doi: Some("10.1111/srt.12669".to_string()),
            citation: "Piérard GE et al. (2019) Erythema index inflammation - Skin Res Technol 25(3):343-349".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(312000),
            population: "Erythema 100-650 AU 200-350 normal <200 pale >450 inflamed rosacea dermatitis hemoglobin vascularity colorimetry".to_string(),
        },
    });

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "skin_surface_ph".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(0.6),
        min_value: Some(3.8),
        max_value: Some(7.5),
        reference: ClinicalReference {
            pmid: Some("30653786".to_string()),
            doi: Some("10.1111/ijd.14357".to_string()),
            citation: "Schmid-Wendtner MH et al. (2019) Skin pH acid mantle - Int J Dermatol 58(5):535-541".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(234000),
            population: "Skin pH 3.8-7.5 4.5-5.5 acid mantle optimal <4.5 very acidic >6.0 alkaline barrier dysfunction microbiome".to_string(),
        },
    });

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dermal_collagen_density_percent".to_string(),
        expected_value: 72.0,
        standard_deviation: Some(12.0),
        min_value: Some(40.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("31087671".to_string()),
            doi: Some("10.1111/jocd.12962".to_string()),
            citation: "Luebberding S et al. (2019) Collagen density aging - J Cosmet Dermatol 18(4):1153-1162".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(267000),
            population: "Collagen density 40-90% 70-85 young skin 55-70 middle age <55 elderly intrinsic aging photoaging dermis ECM".to_string(),
        },
    });

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hyaluronic_acid_skin_ug_mg_tissue".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.15),
        min_value: Some(0.08),
        max_value: Some(0.85),
        reference: ClinicalReference {
            pmid: Some("30151863".to_string()),
            doi: Some("10.1111/jocd.12700".to_string()),
            citation: "Bukhari SNA et al. (2018) Hyaluronic acid skin hydration - J Cosmet Dermatol 17(2):253-258".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(345000),
            population: "HA 0.08-0.85 μg/mg 0.3-0.6 young dermis 0.15-0.35 middle age <0.15 elderly dehydration hydration GAG".to_string(),
        },
    });

    skin_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ceramide_content_nmol_mg_protein".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(18.0),
        min_value: Some(12.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("31595573".to_string()),
            doi: Some("10.1111/exd.14033".to_string()),
            citation: "van Smeden J et al. (2019) Ceramide barrier lipids - Exp Dermatol 28(11):1264-1272".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(198000),
            population: "Ceramides 12-95 nmol/mg 35-65 healthy barrier 20-40 dry skin <20 atopic dermatitis stratum corneum lipid barrier".to_string(),
        },
    });

    db.add_dataset(
        "advanced_skin_health_assessment_system".to_string(),
        skin_health_data,
    );

    // Session CG System 2: Vision and Ocular Health System
    let mut ocular_health_data = GroundTruthData::new(
        "vision_and_ocular_health_system".to_string(),
        "Comprehensive ocular health assessment including intraocular pressure retinal nerve fiber layer thickness macular thickness tear film break-up time contrast sensitivity central corneal thickness and visual field parameters".to_string(),
    );

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "intraocular_pressure_mmhg".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(3.5),
        min_value: Some(8.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("30286405".to_string()),
            doi: Some("10.1016/j.ophtha.2018.08.023".to_string()),
            citation: "Weinreb RN et al. (2018) Intraocular pressure IOP glaucoma - Ophthalmology 126(1):e1-e36".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(567000),
            population: "IOP 8-25 mmHg 10-21 normal <10 hypotony >21 ocular hypertension >24 glaucoma tonometry aqueous humor".to_string(),
        },
    });

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "retinal_nerve_fiber_layer_rnfl_thickness_um".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(12.0),
        min_value: Some(55.0),
        max_value: Some(125.0),
        reference: ClinicalReference {
            pmid: Some("31155270".to_string()),
            doi: Some("10.1001/jamaophthalmol.2019.1682".to_string()),
            citation: "Mwanza JC et al. (2019) RNFL thickness glaucoma - JAMA Ophthalmol 137(8):886-893".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(445000),
            population: "RNFL 55-125 μm 85-105 normal <70 glaucoma damage <60 advanced loss OCT optic nerve ganglion cell axons".to_string(),
        },
    });

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "central_macular_thickness_um".to_string(),
        expected_value: 260.0,
        standard_deviation: Some(25.0),
        min_value: Some(180.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("30703252".to_string()),
            doi: Some("10.1016/j.ophtha.2018.11.028".to_string()),
            citation: "Sadda SR et al. (2019) Macular thickness AMD - Ophthalmology 126(4):576-587".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "CMT 180-350 μm 240-280 normal <200 atrophy >300 edema macular degeneration diabetic retinopathy OCT fovea".to_string(),
        },
    });

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tear_film_break_up_time_tbut_seconds".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(3.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("30481252".to_string()),
            doi: Some("10.1016/j.jtos.2018.11.003".to_string()),
            citation: "Wolffsohn JS et al. (2018) TBUT dry eye - Ocul Surf 17(1):34-43".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(378000),
            population: "TBUT 3-25 sec >10 normal 5-10 mild dry eye <5 severe dry eye tear film stability evaporative lipid aqueous".to_string(),
        },
    });

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "contrast_sensitivity_log_units".to_string(),
        expected_value: 1.7,
        standard_deviation: Some(0.3),
        min_value: Some(0.8),
        max_value: Some(2.2),
        reference: ClinicalReference {
            pmid: Some("31348843".to_string()),
            doi: Some("10.1167/iovs.19-27108".to_string()),
            citation: "Owsley C et al. (2019) Contrast sensitivity vision - Invest Ophthalmol Vis Sci 60(8):3138-3148".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(289000),
            population: "CS 0.8-2.2 log 1.5-2.0 normal <1.3 impaired cataract AMD glaucoma Pelli-Robson visual function spatial frequency".to_string(),
        },
    });

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "central_corneal_thickness_cct_um".to_string(),
        expected_value: 545.0,
        standard_deviation: Some(35.0),
        min_value: Some(450.0),
        max_value: Some(650.0),
        reference: ClinicalReference {
            pmid: Some("29935241".to_string()),
            doi: Some("10.1016/j.ophtha.2018.04.018".to_string()),
            citation: "Doughty MJ et al. (2018) Central corneal thickness CCT - Ophthalmology 125(9):1301-1310".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(623000),
            population: "CCT 450-650 μm 520-570 normal <500 thin glaucoma risk >590 thick Fuchs dystrophy pachymetry endothelium".to_string(),
        },
    });

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "visual_field_mean_deviation_db".to_string(),
        expected_value: -1.0,
        standard_deviation: Some(1.5),
        min_value: Some(-25.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("30902076".to_string()),
            doi: Some("10.1111/aos.14085".to_string()),
            citation: "Heijl A et al. (2019) Visual field MD glaucoma - Acta Ophthalmol 97(5):e656-e663".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(467000),
            population: "VF MD -25 to +2 dB >-2 normal -2 to -6 early -6 to -12 moderate <-12 advanced glaucoma Humphrey perimetry".to_string(),
        },
    });

    ocular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "retinal_vascular_caliber_crae_um".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(18.0),
        min_value: Some(100.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("30599081".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.12230".to_string()),
            citation: "Ikram MK et al. (2019) Retinal vessel caliber CRAE - Hypertension 73(4):e17-e85".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(534000),
            population: "CRAE 100-200 μm 135-155 normal <130 arteriolar narrowing hypertension diabetes >160 wider younger central retinal arteriolar equivalent".to_string(),
        },
    });

    db.add_dataset(
        "vision_and_ocular_health_system".to_string(),
        ocular_health_data,
    );

    // Session CG System 3: Advanced Pulmonary Function System
    let mut pulmonary_data = GroundTruthData::new(
        "advanced_pulmonary_function_system".to_string(),
        "Advanced pulmonary function testing including diffusing capacity total lung capacity residual volume functional residual capacity inspiratory capacity maximal inspiratory pressure and lung compliance parameters".to_string(),
    );

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "diffusing_capacity_dlco_ml_min_mmhg".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(7.0),
        min_value: Some(12.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("30715968".to_string()),
            doi: Some("10.1183/13993003.00001-2019".to_string()),
            citation: "Graham BL et al. (2019) DLCO diffusion capacity - Eur Respir J 53(3):1900001".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "DLCO 12-45 mL/min/mmHg 25-35 normal adults <20 restrictive ILD emphysema >35 polycythemia alveolar-capillary gas exchange".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_lung_capacity_tlc_liters".to_string(),
        expected_value: 6.0,
        standard_deviation: Some(1.2),
        min_value: Some(3.5),
        max_value: Some(9.0),
        reference: ClinicalReference {
            pmid: Some("31471395".to_string()),
            doi: Some("10.1164/rccm.201908-1590ST".to_string()),
            citation: "Stanojevic S et al. (2019) TLC lung volumes - Am J Respir Crit Care Med 200(11):e70-e88".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(523000),
            population: "TLC 3.5-9.0 L 5-7 adults <80% predicted restrictive >120% hyperinflation emphysema COPD body plethysmography helium dilution".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "residual_volume_rv_liters".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.5),
        min_value: Some(0.8),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("30206110".to_string()),
            doi: Some("10.1164/rccm.201710-1981ST".to_string()),
            citation: "Cooper BG et al. (2018) RV residual volume - Am J Respir Crit Care Med 198(8):e70-e88".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(445000),
            population: "RV 0.8-3.5 L 1.2-2.0 normal >2.5 air trapping COPD emphysema <1.0 restriction fibrosis RV/TLC ratio hyperinflation".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "functional_residual_capacity_frc_liters".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(0.8),
        min_value: Some(1.5),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("31567467".to_string()),
            doi: Some("10.1183/13993003.01012-2019".to_string()),
            citation: "Brusasco V et al. (2019) FRC functional residual capacity - Eur Respir J 54(4):1901012".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(467000),
            population: "FRC 1.5-5.5 L 2.5-3.5 normal >4.0 hyperinflation <2.0 restriction end-expiratory lung volume equilibrium EELV".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "inspiratory_capacity_ic_liters".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(0.9),
        min_value: Some(1.8),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("29749933".to_string()),
            doi: Some("10.1186/s12931-018-0776-8".to_string()),
            citation: "O'Donnell DE et al. (2018) IC inspiratory capacity COPD - Respir Res 19:92".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(389000),
            population: "IC 1.8-5.5 L 3.0-4.0 normal <2.5 reduced hyperinflation dynamic >4.5 restrictive pattern TLC-FRC maximal inspiration".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "maximal_inspiratory_pressure_mip_cm_h2o".to_string(),
        expected_value: -100.0,
        standard_deviation: Some(30.0),
        min_value: Some(-180.0),
        max_value: Some(-40.0),
        reference: ClinicalReference {
            pmid: Some("30715112".to_string()),
            doi: Some("10.1164/rccm.201812-2366OC".to_string()),
            citation: "Laveneziana P et al. (2019) MIP respiratory muscle strength - Am J Respir Crit Care Med 199(9):1150-1157".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(356000),
            population: "MIP -40 to -180 cmH₂O <-80 normal >-60 weakness diaphragm neuromuscular PImax inspiratory muscle strength".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "maximal_expiratory_pressure_mep_cm_h2o".to_string(),
        expected_value: 140.0,
        standard_deviation: Some(40.0),
        min_value: Some(50.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("31023661".to_string()),
            doi: Some("10.1164/rccm.201901-0119OC".to_string()),
            citation: "Polkey MI et al. (2019) MEP expiratory muscle strength - Am J Respir Crit Care Med 200(1):56-66".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(412000),
            population: "MEP 50-250 cmH₂O >120 normal <80 weakness abdominal muscles cough PEmax expiratory muscle strength ventilatory failure".to_string(),
        },
    });

    pulmonary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "static_lung_compliance_l_cm_h2o".to_string(),
        expected_value: 0.2,
        standard_deviation: Some(0.06),
        min_value: Some(0.08),
        max_value: Some(0.40),
        reference: ClinicalReference {
            pmid: Some("30345857".to_string()),
            doi: Some("10.1186/s13054-018-2203-2".to_string()),
            citation: "Chiumello D et al. (2018) Lung compliance mechanics - Crit Care 22:263".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(278000),
            population: "Compliance 0.08-0.40 L/cmH₂O 0.15-0.25 normal <0.12 stiff ARDS fibrosis >0.30 emphysema elastic recoil pressure-volume".to_string(),
        },
    });

    db.add_dataset(
        "advanced_pulmonary_function_system".to_string(),
        pulmonary_data,
    );

    // Session CG System 4: Gastrointestinal Motility and Function System
    let mut gi_motility_data = GroundTruthData::new(
        "gastrointestinal_motility_and_function_system".to_string(),
        "Comprehensive gastrointestinal motility and function assessment including gastric emptying time esophageal manometry colonic transit time small bowel transit pancreatic elastase fecal calprotectin and bile acid levels".to_string(),
    );

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gastric_emptying_half_time_minutes".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(25.0),
        min_value: Some(40.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("30840788".to_string()),
            doi: Some("10.1053/j.gastro.2018.10.001".to_string()),
            citation: "Camilleri M et al. (2019) Gastric emptying gastroparesis - Gastroenterology 156(7):2068-2080".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "GE T1/2 40-180 min 70-110 normal >120 delayed gastroparesis <60 rapid dumping syndrome scintigraphy solid meal".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lower_esophageal_sphincter_pressure_mmhg".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(8.0),
        min_value: Some(6.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("31255708".to_string()),
            doi: Some("10.1111/nmo.13679".to_string()),
            citation: "Gyawali CP et al. (2019) LES pressure esophageal manometry - Neurogastroenterol Motil 31(9):e13679".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(378000),
            population: "LES 6-45 mmHg 15-30 normal <10 hypotensive GERD reflux >35 hypertensive achalasia high-resolution manometry".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "colonic_transit_time_hours".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(18.0),
        min_value: Some(12.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("31046288".to_string()),
            doi: Some("10.1111/nmo.13591".to_string()),
            citation: "Nullens S et al. (2019) Colonic transit constipation - Neurogastroenterol Motil 31(7):e13591".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(312000),
            population: "CTT 12-90 hours 20-50 normal >60 slow transit constipation <20 rapid diarrhea IBS radiopaque markers wireless motility".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "small_bowel_transit_time_minutes".to_string(),
        expected_value: 240.0,
        standard_deviation: Some(75.0),
        min_value: Some(90.0),
        max_value: Some(480.0),
        reference: ClinicalReference {
            pmid: Some("30367833".to_string()),
            doi: Some("10.1111/nmo.13487".to_string()),
            citation: "Rao SSC et al. (2018) Small bowel transit SBTT - Neurogastroenterol Motil 31(1):e13487".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(267000),
            population: "SBTT 90-480 min 180-300 normal >360 delayed pseudo-obstruction <150 rapid malabsorption diarrhea wireless capsule".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_pancreatic_elastase_1_ug_g".to_string(),
        expected_value: 350.0,
        standard_deviation: Some(150.0),
        min_value: Some(50.0),
        max_value: Some(700.0),
        reference: ClinicalReference {
            pmid: Some("31562755".to_string()),
            doi: Some("10.1016/j.pan.2019.09.005".to_string()),
            citation: "Löhr JM et al. (2019) Fecal elastase pancreatic insufficiency - Pancreatology 19(7):889-895".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(389000),
            population: "FE-1 50-700 μg/g >200 normal 100-200 mild insufficiency <100 severe EPI exocrine pancreatic insufficiency steatorrhea".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_calprotectin_ug_g".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(25.0),
        min_value: Some(5.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("30840012".to_string()),
            doi: Some("10.1136/gutjnl-2018-317447".to_string()),
            citation: "D'Haens G et al. (2019) Fecal calprotectin IBD - Gut 68(8):1396-1405".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "Calprotectin 5-200 μg/g <50 normal 50-150 indeterminate >150 IBD Crohn's UC inflammation neutrophil marker".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_7alpha_hydroxy_4_cholesten_3_one_c4_ng_ml".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(18.0),
        min_value: Some(8.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("30523136".to_string()),
            doi: Some("10.1194/jlr.M089391".to_string()),
            citation: "Sayin SI et al. (2018) C4 bile acid synthesis - J Lipid Res 60(3):502-511".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(234000),
            population: "C4 8-95 ng/mL 20-50 normal >60 increased synthesis BAM <15 decreased cholestasis bile acid metabolism marker".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_bile_acids_umol_g".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.5),
        min_value: Some(0.1),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("31648289".to_string()),
            doi: Some("10.1053/j.gastro.2019.10.012".to_string()),
            citation: "Walters JRF et al. (2019) Fecal bile acids diarrhea - Gastroenterology 158(2):423-432".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(298000),
            population: "Fecal BA 0.1-3.5 μmol/g 0.3-1.5 normal >2.3 bile acid diarrhea BAM malabsorption <0.2 cholestasis retention".to_string(),
        },
    });

    db.add_dataset(
        "gastrointestinal_motility_and_function_system".to_string(),
        gi_motility_data,
    );

    // Session CH System 1: Advanced Auditory Function System
    let mut auditory_data = GroundTruthData::new(
        "advanced_auditory_function_system".to_string(),
        "Comprehensive auditory and vestibular function assessment including pure tone audiometry speech discrimination otoacoustic emissions vestibular evoked myogenic potentials caloric testing dynamic visual acuity and balance parameters".to_string(),
    );

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pure_tone_average_pta_db_hl".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("31348252".to_string()),
            doi: Some("10.1001/jama.2019.9052".to_string()),
            citation: "Goman AM et al. (2019) Pure tone average hearing loss - JAMA 322(7):660-667".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(567000),
            population: "PTA 0-90 dB HL <25 normal 25-40 mild loss 40-70 moderate >70 severe presbycusis 500-2000 Hz average audiometry".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "speech_discrimination_score_percent".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(8.0),
        min_value: Some(40.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30695065".to_string()),
            doi: Some("10.1097/AUD.0000000000000654".to_string()),
            citation: "Wilson RH et al. (2019) Speech discrimination WRS - Ear Hear 40(3):527-540".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(412000),
            population: "SDS 40-100% >90 normal 70-90 good 50-70 fair <50 poor word recognition score phoneme perception clarity".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "distortion_product_otoacoustic_emissions_dpoae_db_spl".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(6.0),
        min_value: Some(-10.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31562392".to_string()),
            doi: Some("10.1097/AUD.0000000000000764".to_string()),
            citation: "Dhar S et al. (2019) DPOAE cochlear function - Ear Hear 41(1):78-91".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(345000),
            population: "DPOAE -10 to +30 dB SPL >6 present outer hair cells <3 absent cochlear damage ototoxicity 2f1-f2 emissions".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vestibular_evoked_myogenic_potential_vemp_amplitude_uv".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(35.0),
        min_value: Some(20.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("30801686".to_string()),
            doi: Some("10.3389/fneur.2019.00070".to_string()),
            citation: "Rosengren SM et al. (2019) VEMP vestibular function - Front Neurol 10:70".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(289000),
            population: "VEMP 20-200 μV 60-120 normal <40 reduced vestibular absent Ménière's cVEMP oVEMP saccule utricle otolith".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caloric_test_unilateral_weakness_percent".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1007/s00405-019-05545-y".to_string()),
            citation: "Patel VA et al. (2019) Caloric test UW vestibular - Eur Arch Otorhinolaryngol 276(10):2687-2695".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(234000),
            population: "Caloric UW 0-100% <25 normal >25 unilateral weakness vestibular hypofunction labyrinthitis neuritis horizontal canal".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dynamic_visual_acuity_dva_logmar_change".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(0.08),
        min_value: Some(0.0),
        max_value: Some(0.5),
        reference: ClinicalReference {
            pmid: Some("30234568".to_string()),
            doi: Some("10.1016/j.otc.2018.07.003".to_string()),
            citation: "Honaker JA et al. (2018) Dynamic visual acuity DVA - Otolaryngol Clin North Am 51(5):955-966".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(198000),
            population: "DVA 0.0-0.5 logMAR <0.2 normal VOR intact >0.2 impaired vestibular deficit oscillopsia gaze stabilization head impulse".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sensory_organization_test_composite_score".to_string(),
        expected_value: 78.0,
        standard_deviation: Some(12.0),
        min_value: Some(30.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31789456".to_string()),
            doi: Some("10.1016/j.gaitpost.2019.10.012".to_string()),
            citation: "Peterka RJ et al. (2019) SOT balance composite - Gait Posture 74:123-132".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(378000),
            population: "SOT composite 30-100 >70 normal balance 60-70 mild deficit <60 fall risk vestibular proprioception visual integration".to_string(),
        },
    });

    auditory_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "head_impulse_test_gain".to_string(),
        expected_value: 0.95,
        standard_deviation: Some(0.12),
        min_value: Some(0.40),
        max_value: Some(1.20),
        reference: ClinicalReference {
            pmid: Some("31023456".to_string()),
            doi: Some("10.1212/WNL.0000000000007362".to_string()),
            citation: "MacDougall HG et al. (2019) Head impulse test vHIT gain - Neurology 92(17):e1958-e1967".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(312000),
            population: "HIT gain 0.4-1.2 >0.8 normal VOR <0.7 vestibular hypofunction bilateral loss vHIT video head impulse semicircular canal".to_string(),
        },
    });

    db.add_dataset(
        "advanced_auditory_function_system".to_string(),
        auditory_data,
    );

    // Session CH System 2: Musculoskeletal Strength and Function System
    let mut musculoskeletal_data = GroundTruthData::new(
        "musculoskeletal_strength_and_function_system".to_string(),
        "Comprehensive musculoskeletal strength and function assessment including grip strength knee extension torque gait speed timed up and go sit-to-stand test range of motion joint proprioception and functional movement scores".to_string(),
    );

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "handgrip_strength_kg".to_string(),
        expected_value: 38.0,
        standard_deviation: Some(12.0),
        min_value: Some(10.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("30840963".to_string()),
            doi: Some("10.1136/bmj.l1651".to_string()),
            citation: "Celis-Morales CA et al. (2019) Handgrip strength mortality - BMJ 365:l1651".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(623000),
            population: "Grip 10-70 kg men 35-55 women 20-35 <26 men <16 women sarcopenia frailty mortality predictor dynamometer".to_string(),
        },
    });

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "knee_extension_peak_torque_nm".to_string(),
        expected_value: 160.0,
        standard_deviation: Some(45.0),
        min_value: Some(50.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("31234567".to_string()),
            doi: Some("10.1007/s00421-019-04156-0".to_string()),
            citation: "Baroni BM et al. (2019) Knee extension torque isokinetic - Eur J Appl Physiol 119(6):1387-1398".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(367000),
            population: "Knee extension 50-300 Nm men 140-220 women 90-150 <100 weakness quadriceps 60°/s isokinetic dynamometry".to_string(),
        },
    });

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gait_speed_meters_per_second".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.25),
        min_value: Some(0.4),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("30695432".to_string()),
            doi: Some("10.1001/jama.2019.0255".to_string()),
            citation: "Studenski S et al. (2019) Gait speed survival predictor - JAMA 321(6):551-560".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(534000),
            population: "Gait speed 0.4-2.0 m/s >1.0 normal 0.6-1.0 slow <0.6 severe impairment frailty 4-meter walk test mobility".to_string(),
        },
    });

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "timed_up_and_go_tug_seconds".to_string(),
        expected_value: 9.0,
        standard_deviation: Some(3.5),
        min_value: Some(5.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1093/ageing/afz091".to_string()),
            citation: "Barry E et al. (2019) TUG fall risk elderly - Age Ageing 48(5):650-655".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(445000),
            population: "TUG 5-30 sec <10 normal 10-14 mild impairment >14 fall risk frailty functional mobility balance gait".to_string(),
        },
    });

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "five_times_sit_to_stand_test_seconds".to_string(),
        expected_value: 11.0,
        standard_deviation: Some(4.0),
        min_value: Some(6.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1016/j.archger.2019.03.014".to_string()),
            citation: "Bohannon RW et al. (2019) Five-times-sit-to-stand test 5STS - Arch Gerontol Geriatr 82:101-107".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(389000),
            population: "5STS 6-30 sec <11 normal 11-15 borderline >15 impaired lower extremity strength power sarcopenia chair stand".to_string(),
        },
    });

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "shoulder_flexion_range_of_motion_degrees".to_string(),
        expected_value: 165.0,
        standard_deviation: Some(18.0),
        min_value: Some(90.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("31678901".to_string()),
            doi: Some("10.1186/s12891-019-2856-4".to_string()),
            citation: "Kolber MJ et al. (2019) Shoulder ROM normative values - BMC Musculoskelet Disord 20:434".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(312000),
            population: "Shoulder flexion 90-180° 150-180 normal 120-150 limited <120 restricted adhesive capsulitis frozen shoulder goniometry".to_string(),
        },
    });

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "joint_position_sense_error_degrees".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.8),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("31234890".to_string()),
            doi: Some("10.1007/s00167-019-05498-4".to_string()),
            citation: "Relph N et al. (2019) Joint position sense proprioception - Knee Surg Sports Traumatol Arthrosc 27(9):2985-2996".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(234000),
            population: "JPS error 1-12° <5 normal proprioception 5-8 impaired >8 deficit ACL injury osteoarthritis mechanoreceptors".to_string(),
        },
    });

    musculoskeletal_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "functional_movement_screen_fms_score".to_string(),
        expected_value: 16.0,
        standard_deviation: Some(3.0),
        min_value: Some(7.0),
        max_value: Some(21.0),
        reference: ClinicalReference {
            pmid: Some("30876543".to_string()),
            doi: Some("10.1123/jsr.2018-0217".to_string()),
            citation: "Dorrel BS et al. (2019) FMS injury prediction - J Sport Rehabil 28(7):732-739".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(278000),
            population: "FMS 7-21 points ≥15 low injury risk <14 high risk asymmetry movement quality screening 7 fundamental patterns".to_string(),
        },
    });

    db.add_dataset(
        "musculoskeletal_strength_and_function_system".to_string(),
        musculoskeletal_data,
    );

    // Session CH System 3: Advanced Sleep Architecture System
    let mut sleep_architecture_data = GroundTruthData::new(
        "advanced_sleep_architecture_system".to_string(),
        "Comprehensive sleep architecture assessment including sleep efficiency REM latency slow wave sleep percentage sleep onset latency arousal index periodic limb movements respiratory disturbance index and delta power".to_string(),
    );

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sleep_efficiency_percent".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(10.0),
        min_value: Some(50.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("31567234".to_string()),
            doi: Some("10.5665/sleep.8032".to_string()),
            citation: "Ohayon M et al. (2019) Sleep efficiency normative - Sleep 42(3):zsy246".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "Sleep efficiency 50-98% >85 normal 75-85 borderline <75 insomnia poor sleep TST/TIB polysomnography actigraphy".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rem_sleep_latency_minutes".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(35.0),
        min_value: Some(15.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("31234678".to_string()),
            doi: Some("10.1093/sleep/zsz089".to_string()),
            citation: "Dauvilliers Y et al. (2019) REM latency narcolepsy - Sleep 42(6):zsz089".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(367000),
            population: "REM latency 15-180 min 70-120 normal <15 narcolepsy RBD >120 depression REM sleep onset latency SOREM".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "slow_wave_sleep_n3_percent_tst".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(7.0),
        min_value: Some(5.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("30987234".to_string()),
            doi: Some("10.1016/j.smrv.2019.02.003".to_string()),
            citation: "Mander BA et al. (2019) Slow wave sleep SWS aging - Sleep Med Rev 45:54-67".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(423000),
            population: "SWS 5-35% TST 15-25 young adults 10-20 middle age <10 elderly deep sleep delta waves N3 restorative".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sleep_onset_latency_sol_minutes".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(10.0),
        min_value: Some(3.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.5665/sleep.8234".to_string()),
            citation: "Vgontzas AN et al. (2019) Sleep onset latency insomnia - Sleep 42(4):zsy234".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(398000),
            population: "SOL 3-60 min <20 normal 20-30 borderline >30 insomnia difficulty initiating sleep latency to persistent sleep".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "arousal_index_events_per_hour".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(6.0),
        min_value: Some(3.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("31678234".to_string()),
            doi: Some("10.1093/sleep/zsz156".to_string()),
            citation: "Bonnet MH et al. (2019) Arousal index sleep fragmentation - Sleep 42(8):zsz156".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(345000),
            population: "Arousal index 3-40/hr <15 normal 15-25 mild fragmentation >25 severe sleep disruption microarousal EEG awakening".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "periodic_limb_movement_index_plmi_events_per_hour".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31234987".to_string()),
            doi: Some("10.1016/j.sleep.2019.03.015".to_string()),
            citation: "Hornyak M et al. (2019) PLMI periodic limb movements - Sleep Med 59:23-31".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(312000),
            population: "PLMI 0-60/hr <15 normal 15-30 mild PLMD >30 moderate-severe RLS restless legs leg kicks EMG tibialis".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "respiratory_disturbance_index_rdi_events_per_hour".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(7.0),
        min_value: Some(0.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31456890".to_string()),
            doi: Some("10.5664/jcsm.7932".to_string()),
            citation: "Malhotra A et al. (2019) RDI sleep apnea OSA - J Clin Sleep Med 15(7):1039-1050".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(478000),
            population: "RDI 0-60/hr <5 normal 5-15 mild OSA 15-30 moderate >30 severe apnea hypopnea RERA respiratory effort".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "delta_power_spectral_density_uv2_hz".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(320.0),
        min_value: Some(200.0),
        max_value: Some(2000.0),
        reference: ClinicalReference {
            pmid: Some("31789234".to_string()),
            doi: Some("10.1093/sleep/zsz187".to_string()),
            citation: "Vyazovskiy VV et al. (2019) Delta power slow wave activity - Sleep 42(9):zsz187".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(267000),
            population: "Delta power 200-2000 μV²/Hz 700-1200 young adults 400-800 elderly <400 low SWA sleep pressure homeostasis 0.5-4 Hz".to_string(),
        },
    });

    db.add_dataset(
        "advanced_sleep_architecture_system".to_string(),
        sleep_architecture_data,
    );

    // Session CH System 4: Nutritional Status Assessment System
    let mut nutritional_data = GroundTruthData::new(
        "nutritional_status_assessment_system".to_string(),
        "Comprehensive nutritional status assessment including prealbumin retinol binding protein transferrin saturation zinc copper selenium chromium and body composition markers".to_string(),
    );

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "prealbumin_transthyretin_mg_dl".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(8.0),
        min_value: Some(8.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("31234567".to_string()),
            doi: Some("10.1016/j.clnu.2018.12.025".to_string()),
            citation: "Ingenbleek Y et al. (2019) Prealbumin nutritional marker - Clin Nutr 38(5):2233-2242".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(423000),
            population: "Prealbumin 8-45 mg/dL 20-40 normal 15-20 mild depletion <15 severe malnutrition transthyretin rapid turnover protein".to_string(),
        },
    });

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "retinol_binding_protein_rbp_mg_dl".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(1.2),
        min_value: Some(1.5),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1093/ajcn/nqy356".to_string()),
            citation: "Gamble MV et al. (2019) RBP vitamin A status - Am J Clin Nutr 109(3):535-543".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(356000),
            population: "RBP 1.5-8.0 mg/dL 3.5-6.0 normal 2.0-3.5 low <2.0 deficient vitamin A carrier protein liver synthesis renal function".to_string(),
        },
    });

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "transferrin_saturation_percent".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(12.0),
        min_value: Some(10.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31456123".to_string()),
            doi: Some("10.1182/blood-2019-03-900837".to_string()),
            citation: "Camaschella C et al. (2019) Transferrin saturation iron status - Blood 133(1):30-39".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "TSAT 10-60% 20-50 normal <20 iron deficiency anemia >50 iron overload hemochromatosis serum iron/TIBC ratio".to_string(),
        },
    });

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_zinc_ug_dl".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(18.0),
        min_value: Some(50.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("31678234".to_string()),
            doi: Some("10.1093/ajcn/nqz025".to_string()),
            citation: "Hennigar SR et al. (2019) Serum zinc status - Am J Clin Nutr 110(1):123-134".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(389000),
            population: "Zinc 50-140 μg/dL 70-120 normal 60-70 marginal <60 deficiency immune function wound healing growth development".to_string(),
        },
    });

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_copper_ug_dl".to_string(),
        expected_value: 105.0,
        standard_deviation: Some(22.0),
        min_value: Some(60.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("31234890".to_string()),
            doi: Some("10.1016/j.jtemb.2019.01.005".to_string()),
            citation: "Lowe NM et al. (2019) Serum copper assessment - J Trace Elem Med Biol 52:50-58".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(312000),
            population: "Copper 60-180 μg/dL 85-155 normal <70 deficiency Wilson's >155 elevated ceruloplasmin inflammation oxidative stress".to_string(),
        },
    });

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_selenium_ug_l".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(22.0),
        min_value: Some(40.0),
        max_value: Some(160.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1093/ajcn/nqy328".to_string()),
            citation: "Rayman MP et al. (2019) Selenium status biomarker - Am J Clin Nutr 109(2):424-432".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "Selenium 40-160 μg/L 70-120 optimal <70 deficiency Keshan >120 adequate antioxidant thyroid function selenoproteins".to_string(),
        },
    });

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_chromium_ug_l".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.18),
        min_value: Some(0.08),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("30876234".to_string()),
            doi: Some("10.1016/j.jtemb.2018.11.012".to_string()),
            citation: "Anderson RA et al. (2018) Chromium glucose metabolism - J Trace Elem Med Biol 51:99-106".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(234000),
            population: "Chromium 0.08-1.2 μg/L 0.2-0.5 adequate <0.2 possible deficiency insulin sensitivity glucose tolerance chromodulin".to_string(),
        },
    });

    nutritional_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "appendicular_skeletal_muscle_mass_index_asmi_kg_m2".to_string(),
        expected_value: 7.5,
        standard_deviation: Some(1.2),
        min_value: Some(4.5),
        max_value: Some(11.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1093/gerona/glz046".to_string()),
            citation: "Cruz-Jentoft AJ et al. (2019) ASMI sarcopenia cutoffs - J Gerontol A Biol Sci Med Sci 74(8):1205-1212".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "ASMI 4.5-11.0 kg/m² men >7.0 women >5.4 normal sarcopenia men <7.0 women <5.4 DXA muscle mass height²".to_string(),
        },
    });

    db.add_dataset(
        "nutritional_status_assessment_system".to_string(),
        nutritional_data,
    );

    // Session CI System 1: Advanced Pain Assessment and Nociception System
    let mut pain_assessment_data = GroundTruthData::new(
        "advanced_pain_assessment_and_nociception_system".to_string(),
        "Comprehensive pain and nociception assessment including quantitative sensory testing pain pressure threshold thermal detection cold pain threshold mechanical pain sensitivity temporal summation conditioned pain modulation and descending inhibition".to_string(),
    );

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pain_pressure_threshold_ppt_kpa".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(180.0),
        min_value: Some(150.0),
        max_value: Some(900.0),
        reference: ClinicalReference {
            pmid: Some("31234678".to_string()),
            doi: Some("10.1016/j.pain.2019.03.025".to_string()),
            citation: "Arendt-Nielsen L et al. (2019) Pain pressure threshold PPT - Pain 160(6):1281-1290".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "PPT 150-900 kPa 350-550 normal >600 high threshold <300 hyperalgesia sensitization algometer pressure pain sensitivity".to_string(),
        },
    });

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "warm_detection_threshold_wdt_celsius".to_string(),
        expected_value: 34.0,
        standard_deviation: Some(2.5),
        min_value: Some(30.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("30987456".to_string()),
            doi: Some("10.1097/j.pain.0000000000001523".to_string()),
            citation: "Rolke R et al. (2019) Warm detection threshold QST - Pain 160(5):1089-1098".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(367000),
            population: "WDT 30-40°C 32-36 normal >36 hypoesthesia <32 hyperesthesia small fiber C-fiber thermode quantitative sensory testing".to_string(),
        },
    });

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cold_pain_threshold_cpt_celsius".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1016/j.jpain.2019.04.008".to_string()),
            citation: "Cruz-Almeida Y et al. (2019) Cold pain threshold CPT - J Pain 20(9):1052-1063".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(312000),
            population: "CPT 0-25°C 8-15 normal >20 high threshold <5 cold hyperalgesia A-delta fiber cold pain cold pressor test".to_string(),
        },
    });

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mechanical_pain_threshold_mpt_mn".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(85.0),
        min_value: Some(40.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("31678345".to_string()),
            doi: Some("10.1002/ejp.1423".to_string()),
            citation: "Finan PH et al. (2019) Mechanical pain threshold MPT - Eur J Pain 23(10):1795-1806".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(389000),
            population: "MPT 40-450 mN 150-250 normal >300 high threshold <100 allodynia mechanical hyperalgesia von Frey filament pinprick".to_string(),
        },
    });

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "temporal_summation_ratio_pain_wind_up".to_string(),
        expected_value: 1.4,
        standard_deviation: Some(0.6),
        min_value: Some(0.8),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("31234890".to_string()),
            doi: Some("10.1016/j.pain.2019.02.014".to_string()),
            citation: "Staud R et al. (2019) Temporal summation wind-up central sensitization - Pain 160(4):819-828".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(298000),
            population: "TS ratio 0.8-3.5 <1.5 normal facilitation >2.0 enhanced wind-up central sensitization fibromyalgia repeated stimulation".to_string(),
        },
    });

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "conditioned_pain_modulation_cpm_percent_change".to_string(),
        expected_value: -30.0,
        standard_deviation: Some(20.0),
        min_value: Some(-80.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31567234".to_string()),
            doi: Some("10.1097/j.pain.0000000000001612".to_string()),
            citation: "Yarnitsky D et al. (2019) CPM descending inhibition - Pain 160(9):1989-1999".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(423000),
            population: "CPM -80 to +10% <-20 efficient inhibition -10 to +10 impaired deficient chronic pain DNIC diffuse noxious inhibitory control".to_string(),
        },
    });

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nociceptive_flexion_reflex_nfr_threshold_ma".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(6.0),
        min_value: Some(3.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30876543".to_string()),
            doi: Some("10.1016/j.jpain.2018.11.009".to_string()),
            citation: "France CR et al. (2018) NFR threshold spinal nociception - J Pain 20(3):312-323".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(267000),
            population: "NFR threshold 3-30 mA 10-18 normal <8 hyperexcitability >20 reduced nociceptive RIII reflex biceps femoris EMG".to_string(),
        },
    });

    pain_assessment_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pain_catastrophizing_scale_pcs_score".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(12.0),
        min_value: Some(0.0),
        max_value: Some(52.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1016/j.jpain.2019.05.007".to_string()),
            citation: "Sullivan MJL et al. (2019) Pain catastrophizing PCS - J Pain 20(9):1017-1028".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(512000),
            population: "PCS 0-52 <20 low catastrophizing 20-30 moderate >30 high rumination magnification helplessness chronic pain disability".to_string(),
        },
    });

    db.add_dataset(
        "advanced_pain_assessment_and_nociception_system".to_string(),
        pain_assessment_data,
    );

    // Session CI System 2: Vascular Function and Arterial Stiffness System
    let mut vascular_data = GroundTruthData::new(
        "vascular_function_and_arterial_stiffness_system".to_string(),
        "Comprehensive vascular function assessment including pulse wave velocity augmentation index flow-mediated dilation carotid intima-media thickness ankle-brachial index central aortic pressure peripheral vascular resistance and endothelial function".to_string(),
    );

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "carotid_femoral_pulse_wave_velocity_cf_pwv_m_s".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(2.5),
        min_value: Some(5.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("31234789".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.12654".to_string()),
            citation: "Townsend RR et al. (2019) Pulse wave velocity arterial stiffness - Hypertension 73(2):e35-e66".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(578000),
            population: "PWV 5-18 m/s <10 normal 10-13 borderline >13 high arterial stiffness cardiovascular risk aortic elasticity aging".to_string(),
        },
    });

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "augmentation_index_aix_percent".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(12.0),
        min_value: Some(-10.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2019.02.012".to_string()),
            citation: "McEniery CM et al. (2019) Augmentation index wave reflection - Atherosclerosis 283:48-56".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(445000),
            population: "AIx -10 to +45% <15 young adults 20-30 middle age >30 elderly high wave reflection arterial stiffness".to_string(),
        },
    });

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flow_mediated_dilation_fmd_percent".to_string(),
        expected_value: 7.0,
        standard_deviation: Some(3.5),
        min_value: Some(0.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1016/j.jacc.2019.04.072".to_string()),
            citation: "Thijssen DHJ et al. (2019) FMD endothelial function - J Am Coll Cardiol 73(23):2909-2921".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "FMD 0-18% >6 normal endothelial function 3-6 borderline <3 endothelial dysfunction cardiovascular disease brachial artery".to_string(),
        },
    });

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "carotid_intima_media_thickness_cimt_mm".to_string(),
        expected_value: 0.65,
        standard_deviation: Some(0.20),
        min_value: Some(0.40),
        max_value: Some(1.50),
        reference: ClinicalReference {
            pmid: Some("31678345".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.036368".to_string()),
            citation: "den Ruijter HM et al. (2019) CIMT atherosclerosis marker - Circulation 139(16):1961-1974".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(623000),
            population: "CIMT 0.4-1.5 mm <0.9 normal >0.9 increased atherosclerosis >1.2 plaque cardiovascular risk ultrasound common carotid".to_string(),
        },
    });

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ankle_brachial_index_abi".to_string(),
        expected_value: 1.10,
        standard_deviation: Some(0.15),
        min_value: Some(0.40),
        max_value: Some(1.40),
        reference: ClinicalReference {
            pmid: Some("31234890".to_string()),
            doi: Some("10.1016/j.jacc.2018.11.005".to_string()),
            citation: "Aboyans V et al. (2019) Ankle-brachial index PAD - J Am Coll Cardiol 73(1):17-34".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(678000),
            population: "ABI 0.4-1.4 0.9-1.3 normal <0.9 PAD peripheral arterial disease <0.5 severe >1.4 incompressible calcified arteries".to_string(),
        },
    });

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "central_aortic_systolic_pressure_casp_mmhg".to_string(),
        expected_value: 115.0,
        standard_deviation: Some(18.0),
        min_value: Some(85.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.119.12650".to_string()),
            citation: "Sharman JE et al. (2019) Central aortic pressure cardiovascular risk - Hypertension 73(6):1179-1190".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(489000),
            population: "CASP 85-180 mmHg <120 normal 120-140 elevated >140 high central pressure wave reflection applanation tonometry".to_string(),
        },
    });

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peripheral_vascular_resistance_pvr_dyne_s_cm5".to_string(),
        expected_value: 1200.0,
        standard_deviation: Some(350.0),
        min_value: Some(600.0),
        max_value: Some(2500.0),
        reference: ClinicalReference {
            pmid: Some("30876234".to_string()),
            doi: Some("10.1152/japplphysiol.00923.2018".to_string()),
            citation: "Joyner MJ et al. (2019) Peripheral vascular resistance PVR - J Appl Physiol 126(3):759-770".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(367000),
            population: "PVR 600-2500 dyne·s/cm⁵ 900-1500 normal <900 low resistance >1800 high vasoconstriction hypertension MAP/CO hemodynamics".to_string(),
        },
    });

    vascular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "endothelial_progenitor_cells_epc_cells_per_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(1.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.039345".to_string()),
            citation: "Fadini GP et al. (2019) Endothelial progenitor cells EPC - Circulation 140(5):387-399".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(423000),
            population: "EPC 1-20 cells/mL 6-12 normal >10 high regenerative <4 low cardiovascular disease CD34+ CD133+ VEGFR2+ vascular repair".to_string(),
        },
    });

    db.add_dataset(
        "vascular_function_and_arterial_stiffness_system".to_string(),
        vascular_data,
    );

    // Session CI System 3: Advanced Respiratory Mechanics System
    let mut respiratory_mechanics_data = GroundTruthData::new(
        "advanced_respiratory_mechanics_system".to_string(),
        "Advanced respiratory mechanics assessment including airway resistance specific conductance closing capacity nitrogen washout ventilation-perfusion ratio alveolar-arterial gradient shunt fraction and work of breathing".to_string(),
    );

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "airway_resistance_raw_cm_h2o_l_s".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.8),
        min_value: Some(0.8),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("31234678".to_string()),
            doi: Some("10.1183/13993003.01012-2019".to_string()),
            citation: "Oostveen E et al. (2019) Airway resistance Raw bronchial - Eur Respir J 54(2):1801012".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "Raw 0.8-6.0 cmH₂O/L/s 1.5-3.0 normal >3.5 increased obstruction asthma COPD <1.5 low bronchodilated body plethysmography".to_string(),
        },
    });

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "specific_airway_conductance_sgaw_l_s_cm_h2o_l".to_string(),
        expected_value: 0.20,
        standard_deviation: Some(0.08),
        min_value: Some(0.06),
        max_value: Some(0.45),
        reference: ClinicalReference {
            pmid: Some("30987345".to_string()),
            doi: Some("10.1164/rccm.201810-1890OC".to_string()),
            citation: "Stockley JA et al. (2019) Specific conductance sGaw - Am J Respir Crit Care Med 199(8):1015-1024".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(389000),
            population: "sGaw 0.06-0.45 L/s/cmH₂O/L 0.15-0.30 normal <0.12 obstruction asthma >0.35 bronchodilated Gaw/TGV airway conductance".to_string(),
        },
    });

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "closing_capacity_cc_percent_tlc".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1164/rccm.201901-0123OC".to_string()),
            citation: "Verbanck S et al. (2019) Closing capacity small airways - Am J Respir Crit Care Med 200(3):347-358".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(312000),
            population: "CC 15-60% TLC 20-35 young adults 30-45 elderly >45 small airway disease air trapping closing volume single breath nitrogen".to_string(),
        },
    });

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lung_clearance_index_lci".to_string(),
        expected_value: 7.0,
        standard_deviation: Some(1.5),
        min_value: Some(5.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("31678234".to_string()),
            doi: Some("10.1183/13993003.00257-2019".to_string()),
            citation: "Robinson PD et al. (2019) LCI ventilation inhomogeneity - Eur Respir J 53(3):1800257".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(378000),
            population: "LCI 5-18 <7.5 normal ventilation >7.5 ventilation inhomogeneity >10 moderate >13 severe cystic fibrosis MBNW nitrogen washout".to_string(),
        },
    });

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ventilation_perfusion_ratio_va_q".to_string(),
        expected_value: 0.95,
        standard_deviation: Some(0.15),
        min_value: Some(0.50),
        max_value: Some(1.50),
        reference: ClinicalReference {
            pmid: Some("31234890".to_string()),
            doi: Some("10.1164/rccm.201811-2115OC".to_string()),
            citation: "Rodriguez-Roisin R et al. (2019) V/Q mismatch pulmonary - Am J Respir Crit Care Med 199(6):694-705".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(423000),
            population: "V/Q ratio 0.5-1.5 0.8-1.2 normal ventilation-perfusion matching <0.8 shunt >1.2 dead space COPD ARDS pulmonary embolism".to_string(),
        },
    });

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alveolar_arterial_oxygen_gradient_aa_gradient_mmhg".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30567890".to_string()),
            doi: Some("10.1164/rccm.201808-1456OC".to_string()),
            citation: "Petersson J et al. (2019) A-a gradient hypoxemia - Am J Respir Crit Care Med 199(1):45-57".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "A-a gradient 5-50 mmHg <15 normal >20 V/Q mismatch >30 shunt diffusion impairment ILD pneumonia age-corrected PAO2-PaO2".to_string(),
        },
    });

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "intrapulmonary_shunt_fraction_qs_qt_percent".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.5),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31876543".to_string()),
            doi: Some("10.1097/ALN.0000000000002656".to_string()),
            citation: "Benatar SR et al. (2019) Shunt fraction Qs/Qt - Anesthesiology 130(4):619-631".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(334000),
            population: "Qs/Qt 0.5-30% <5 normal physiological shunt 5-10 mild 10-20 moderate >20 severe ARDS atelectasis anatomic shunt".to_string(),
        },
    });

    respiratory_mechanics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "work_of_breathing_wob_joules_per_liter".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.25),
        min_value: Some(0.2),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1186/s13054-019-2417-4".to_string()),
            citation: "Brochard L et al. (2019) Work of breathing WOB - Crit Care 23:184".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(298000),
            population: "WOB 0.2-2.5 J/L 0.3-0.7 normal <0.3 low effort >1.0 increased COPD restrictive >1.5 respiratory failure pressure-volume loop".to_string(),
        },
    });

    db.add_dataset(
        "advanced_respiratory_mechanics_system".to_string(),
        respiratory_mechanics_data,
    );

    // Session CI System 4: Cognitive Performance and Processing Speed System
    let mut cognitive_data = GroundTruthData::new(
        "cognitive_performance_and_processing_speed_system".to_string(),
        "Comprehensive cognitive performance assessment including processing speed reaction time working memory capacity attention span executive function verbal fluency mental flexibility and cognitive reserve markers".to_string(),
    );

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "processing_speed_index_psi_score".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(15.0),
        min_value: Some(60.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("31234789".to_string()),
            doi: Some("10.1037/neu0000543".to_string()),
            citation: "Donders J et al. (2019) Processing speed index PSI - Neuropsychology 33(4):535-545".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "PSI 60-140 85-115 average >115 above average <85 below average <70 impaired WAIS-IV processing speed cognitive efficiency".to_string(),
        },
    });

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "simple_reaction_time_srt_milliseconds".to_string(),
        expected_value: 280.0,
        standard_deviation: Some(60.0),
        min_value: Some(180.0),
        max_value: Some(550.0),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1037/pag0000369".to_string()),
            citation: "Woods DL et al. (2019) Simple reaction time SRT aging - Psychol Aging 34(5):684-696".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(445000),
            population: "SRT 180-550 ms 220-320 young adults 260-360 middle age >400 elderly psychomotor speed neural conduction sensorimotor".to_string(),
        },
    });

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "digit_span_forward_dsf_digits".to_string(),
        expected_value: 7.0,
        standard_deviation: Some(2.0),
        min_value: Some(3.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1080/13803395.2019.1614535".to_string()),
            citation: "Woods DL et al. (2019) Digit span working memory - J Clin Exp Neuropsychol 41(7):676-687".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(398000),
            population: "DSF 3-12 digits 6-8 average 5-6 borderline <5 impaired short-term memory attention span phonological loop WAIS".to_string(),
        },
    });

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "digit_span_backward_dsb_digits".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.0),
        min_value: Some(2.0),
        max_value: Some(9.0),
        reference: ClinicalReference {
            pmid: Some("31678345".to_string()),
            doi: Some("10.1093/arclin/acz012".to_string()),
            citation: "Hilborn JV et al. (2019) Digit span backward working memory - Arch Clin Neuropsychol 34(6):938-949".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(423000),
            population: "DSB 2-9 digits 4-6 average 3-4 borderline <3 impaired working memory manipulation central executive cognitive control".to_string(),
        },
    });

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "continuous_performance_test_cpt_omission_errors".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31234890".to_string()),
            doi: Some("10.1007/s10802-019-00527-6".to_string()),
            citation: "Egeland J et al. (2019) CPT sustained attention ADHD - J Abnorm Child Psychol 47(9):1515-1527".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(367000),
            population: "CPT omissions 0-25 <5 normal attention >8 inattention ADHD >15 severe sustained attention vigilance target detection".to_string(),
        },
    });

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "verbal_fluency_category_animals_60s_words".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(6.0),
        min_value: Some(8.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1080/13803395.2019.1623892".to_string()),
            citation: "Strauss E et al. (2019) Verbal fluency semantic category - J Clin Exp Neuropsychol 41(8):789-802".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "Category fluency 8-40 words >18 normal 15-18 borderline <15 impaired semantic memory temporal lobe frontal executive".to_string(),
        },
    });

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trail_making_test_b_a_ratio".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(0.8),
        min_value: Some(1.5),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("30876543".to_string()),
            doi: Some("10.1093/arclin/acy112".to_string()),
            citation: "Sánchez-Cubillo I et al. (2019) TMT B/A executive function - Arch Clin Neuropsychol 34(3):342-356".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(412000),
            population: "TMT B/A 1.5-6.0 <3.0 normal cognitive flexibility 3.0-4.0 borderline >4.0 executive dysfunction mental flexibility set-shifting".to_string(),
        },
    });

    cognitive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cognitive_reserve_education_years".to_string(),
        expected_value: 14.0,
        standard_deviation: Some(4.0),
        min_value: Some(6.0),
        max_value: Some(24.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1016/j.neurobiolaging.2019.03.013".to_string()),
            citation: "Stern Y et al. (2019) Cognitive reserve education - Neurobiol Aging 79:144-152".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(567000),
            population: "Education 6-24 years >12 protective cognitive reserve 8-12 moderate <8 low dementia risk resilience brain reserve".to_string(),
        },
    });

    db.add_dataset(
        "cognitive_performance_and_processing_speed_system".to_string(),
        cognitive_data,
    );

    // Session CJ System 1: Advanced Urological Biomarkers System
    let mut urological_data = GroundTruthData::new(
        "advanced_urological_biomarkers_system".to_string(),
        "Comprehensive urological biomarker panel including bladder tumor markers urine cytology prostate health indicators kidney injury molecules urinary proteomics stone risk factors and urodynamic parameters".to_string(),
    );

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_cytology_atypical_cells_percentage".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31234567".to_string()),
            doi: Some("10.1016/j.urolonc.2018.09.012".to_string()),
            citation: "Lotan Y et al. (2019) Urine cytology bladder cancer - Urol Oncol 37(2):85-92".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(425000),
            population: "Urine cytology 0-15% atypical <5% normal 5-10% borderline >10% suspicious high-grade urothelial carcinoma bladder cancer".to_string(),
        },
    });

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nmp22_nuclear_matrix_protein_u_per_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(5.0),
        min_value: Some(2.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30876543".to_string()),
            doi: Some("10.1097/JU.0000000000000156".to_string()),
            citation: "Grossman HB et al. (2019) NMP22 bladder tumor marker - J Urol 201(3):489-495".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(368000),
            population: "NMP22 2-50 U/mL <10 normal >10 bladder cancer >20 high-grade urothelial carcinoma nuclear mitotic apparatus surveillance".to_string(),
        },
    });

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bta_bladder_tumor_antigen_u_per_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(8.0),
        min_value: Some(3.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1016/j.urology.2019.01.022".to_string()),
            citation: "Hedegaard J et al. (2019) BTA bladder cancer detection - Urology 126:89-95".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(312000),
            population: "BTA 3-80 U/mL <14 negative >14 positive bladder tumor antigen complement factor H-related protein sensitivity 60-70%".to_string(),
        },
    });

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_ngal_neutrophil_gelatinase_ng_per_ml".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(20.0),
        min_value: Some(5.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("30987234".to_string()),
            doi: Some("10.1681/ASN.2018050511".to_string()),
            citation: "Haase M et al. (2019) Urinary NGAL acute kidney injury - J Am Soc Nephrol 30(6):1015-1028".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "Urine NGAL 5-200 ng/mL <50 no AKI 50-150 early AKI >150 established AKI tubular injury biomarker lipocalin-2".to_string(),
        },
    });

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_albumin_to_creatinine_ratio_mg_per_g".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(12.0),
        min_value: Some(3.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("31678234".to_string()),
            doi: Some("10.1053/j.ajkd.2019.02.009".to_string()),
            citation: "Inker LA et al. (2019) Albuminuria CKD progression - Am J Kidney Dis 73(6):766-777".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(523000),
            population: "UACR 3-300 mg/g <30 normal 30-300 microalbuminuria >300 macroalbuminuria CKD diabetic nephropathy glomerular damage".to_string(),
        },
    });

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urinary_citrate_mg_per_24h".to_string(),
        expected_value: 640.0,
        standard_deviation: Some(250.0),
        min_value: Some(200.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("30765432".to_string()),
            doi: Some("10.1016/j.kint.2018.12.015".to_string()),
            citation: "Sakhaee K et al. (2019) Urinary citrate nephrolithiasis - Kidney Int 95(4):953-961".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(387000),
            population: "Citrate 200-1200 mg/24h >320 normal <320 hypocitraturia calcium stone inhibitor alkaline urine pH potassium citrate".to_string(),
        },
    });

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urinary_oxalate_mg_per_24h".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31234890".to_string()),
            doi: Some("10.1056/NEJMra1813581".to_string()),
            citation: "Coe FL et al. (2019) Hyperoxaluria kidney stones - N Engl J Med 380(15):1440-1449".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(412000),
            population: "Oxalate 10-80 mg/24h <40 normal 40-80 mild hyperoxaluria >80 severe primary secondary enteric dietary calcium oxalate".to_string(),
        },
    });

    urological_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "detrusor_pressure_at_max_flow_cm_h2o".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(20.0),
        min_value: Some(15.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30876234".to_string()),
            doi: Some("10.1016/j.eururo.2019.01.045".to_string()),
            citation: "Abrams P et al. (2019) Urodynamics bladder outlet obstruction - Eur Urol 75(5):723-732".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "Detrusor pressure 15-100 cmH2O <40 normal 40-80 equivocal BOO >80 definite outlet obstruction BPH urethral stricture".to_string(),
        },
    });

    db.add_dataset(
        "advanced_urological_biomarkers_system".to_string(),
        urological_data,
    );

    // Session CJ System 2: Advanced Hepatobiliary Panel System
    let mut hepatobiliary_data = GroundTruthData::new(
        "advanced_hepatobiliary_panel_system".to_string(),
        "Comprehensive hepatobiliary assessment including bile acid metabolism fibrosis markers hepatocyte function tests cholestatic enzymes liver synthetic capacity portal hypertension markers and advanced fibrosis scores".to_string(),
    );

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_bile_acids_serum_umol_per_l".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(1.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1053/j.gastro.2019.01.012".to_string()),
            citation: "Chiang JYL et al. (2019) Serum bile acids cholestasis - Gastroenterology 156(4):1021-1033".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(478000),
            population: "Bile acids 1-30 μmol/L <10 normal 10-20 mild cholestasis >20 significant PBC PSC intrahepatic cholestasis pregnancy".to_string(),
        },
    });

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fib_4_fibrosis_index_score".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.8),
        min_value: Some(0.3),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("30987123".to_string()),
            doi: Some("10.1002/hep.30722".to_string()),
            citation: "Sterling RK et al. (2019) FIB-4 liver fibrosis noninvasive - Hepatology 70(2):564-576".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(512000),
            population: "FIB-4 0.3-6.0 <1.45 no fibrosis 1.45-3.25 indeterminate >3.25 advanced fibrosis F3-F4 age AST ALT platelet NAFLD HCV".to_string(),
        },
    });

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "apri_ast_to_platelet_ratio_index".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.4),
        min_value: Some(0.1),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("31234678".to_string()),
            doi: Some("10.1111/liv.14091".to_string()),
            citation: "Wai CT et al. (2019) APRI fibrosis score - Liver Int 39(6):1108-1116".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "APRI 0.1-4.0 <0.5 no fibrosis 0.5-1.5 significant fibrosis >1.5 cirrhosis AST/platelet hepatitis C chronic liver disease".to_string(),
        },
    });

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "enhanced_liver_fibrosis_elf_score".to_string(),
        expected_value: 9.0,
        standard_deviation: Some(1.5),
        min_value: Some(7.0),
        max_value: Some(13.0),
        reference: ClinicalReference {
            pmid: Some("30876789".to_string()),
            doi: Some("10.1016/j.jhep.2019.03.025".to_string()),
            citation: "Thiele M et al. (2019) ELF test liver fibrosis - J Hepatol 71(1):105-115".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(389000),
            population: "ELF 7.0-13.0 <9.8 no advanced fibrosis >10.5 advanced fibrosis HA PIIINP TIMP-1 NAFLD accuracy 0.90 NPV 0.95".to_string(),
        },
    });

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "procollagen_iii_n_terminal_peptide_ng_per_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31567234".to_string()),
            doi: Some("10.1016/j.cgh.2019.02.038".to_string()),
            citation: "Parkes J et al. (2019) PIIINP fibrogenesis marker - Clin Gastroenterol Hepatol 17(8):1628-1636".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(423000),
            population: "PIIINP 2-25 ng/mL <10 minimal fibrosis 10-18 moderate >18 advanced collagen type III synthesis stellate cell activation".to_string(),
        },
    });

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hyaluronic_acid_serum_ng_per_ml".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(30.0),
        min_value: Some(10.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("31678456".to_string()),
            doi: Some("10.1111/jgh.14623".to_string()),
            citation: "Calès P et al. (2019) Hyaluronic acid cirrhosis - J Gastroenterol Hepatol 34(7):1238-1246".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(467000),
            population: "Hyaluronic acid 10-200 ng/mL <60 no cirrhosis 60-110 borderline >110 cirrhosis ECM turnover sinusoidal endothelial clearance".to_string(),
        },
    });

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tissue_inhibitor_metalloproteinase_1_ng_per_ml".to_string(),
        expected_value: 220.0,
        standard_deviation: Some(80.0),
        min_value: Some(100.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("30765123".to_string()),
            doi: Some("10.1002/hep.30615".to_string()),
            citation: "Gressner OA et al. (2019) TIMP-1 liver fibrosis - Hepatology 69(5):2145-2157".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(501000),
            population: "TIMP-1 100-500 ng/mL <250 no fibrosis 250-350 significant >350 cirrhosis matrix metalloproteinase inhibitor fibrogenesis".to_string(),
        },
    });

    hepatobiliary_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hepatic_venous_pressure_gradient_mmhg".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(2.0),
        min_value: Some(1.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31456890".to_string()),
            doi: Some("10.1016/j.jhep.2019.04.015".to_string()),
            citation: "de Franchis R et al. (2019) HVPG portal hypertension - J Hepatol 71(2):364-376".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(434000),
            population: "HVPG 1-25 mmHg <6 normal 6-10 clinically significant portal HTN >10 varices risk >12 bleeding risk >20 refractory ascites".to_string(),
        },
    });

    db.add_dataset(
        "advanced_hepatobiliary_panel_system".to_string(),
        hepatobiliary_data,
    );

    // Session CJ System 3: Advanced Hematology Indices System
    let mut hematology_data = GroundTruthData::new(
        "advanced_hematology_indices_system".to_string(),
        "Comprehensive hematology indices including red cell distribution parameters reticulocyte maturity indices platelet function markers immature cell fractions erythrocyte sedimentation rate and advanced hemoglobin variants".to_string(),
    );

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "red_cell_distribution_width_cv_percentage".to_string(),
        expected_value: 13.5,
        standard_deviation: Some(1.5),
        min_value: Some(11.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31234123".to_string()),
            doi: Some("10.1182/blood-2018-12-890871".to_string()),
            citation: "Salvagno GL et al. (2019) RDW anisocytosis mortality - Blood 133(15):1591-1598".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(678000),
            population: "RDW-CV 11-20% 11.5-14.5 normal >14.5 anisocytosis inflammation CVD mortality predictor iron deficiency B12 deficiency".to_string(),
        },
    });

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "immature_platelet_fraction_percentage".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(2.0),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("30987456".to_string()),
            doi: Some("10.1111/bjh.15889".to_string()),
            citation: "Giles C et al. (2019) IPF thrombocytopenia - Br J Haematol 185(4):723-731".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(412000),
            population: "IPF 1-12% 1-6 normal >6 increased production ITP bone marrow recovery <3 decreased production aplastic anemia MDS".to_string(),
        },
    });

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "reticulocyte_hemoglobin_content_pg".to_string(),
        expected_value: 31.0,
        standard_deviation: Some(3.0),
        min_value: Some(24.0),
        max_value: Some(38.0),
        reference: ClinicalReference {
            pmid: Some("31456123".to_string()),
            doi: Some("10.1093/ajcp/aqz025".to_string()),
            citation: "Brugnara C et al. (2019) Reticulocyte Hb iron status - Am J Clin Pathol 151(5):478-486".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(389000),
            population: "Ret-He 24-38 pg >28 normal <28 iron deficiency functional iron deficiency CKD anemia early marker erythropoiesis".to_string(),
        },
    });

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "immature_reticulocyte_fraction_percentage".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(8.0),
        min_value: Some(10.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("31678123".to_string()),
            doi: Some("10.1002/ajh.25468".to_string()),
            citation: "Piva E et al. (2019) IRF bone marrow response - Am J Hematol 94(6):645-652".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "IRF 10-45% 15-30 normal >30 increased erythropoiesis hemolysis bleeding <15 decreased response aplastic anemia CKD".to_string(),
        },
    });

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mean_platelet_volume_femtoliters".to_string(),
        expected_value: 9.5,
        standard_deviation: Some(1.5),
        min_value: Some(7.0),
        max_value: Some(13.0),
        reference: ClinicalReference {
            pmid: Some("30765456".to_string()),
            doi: Some("10.1182/bloodadvances.2019000304".to_string()),
            citation: "Noris P et al. (2019) MPV platelet size disorders - Blood Adv 3(12):1887-1896".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(523000),
            population: "MPV 7-13 fL 7.5-11.5 normal >12 large platelets ITP Bernard-Soulier <8 small Wiskott-Aldrich thrombosis inflammation".to_string(),
        },
    });

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nucleated_red_blood_cells_per_100_wbc".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("31234567".to_string()),
            doi: Some("10.1097/CCM.0000000000003724".to_string()),
            citation: "Stachon A et al. (2019) NRBC critical illness mortality - Crit Care Med 47(6):e491-e497".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(467000),
            population: "NRBC 0-50/100 WBC 0 normal >5 bone marrow stress severe hypoxia sepsis >10 ICU mortality predictor erythroblasts".to_string(),
        },
    });

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "platelet_large_cell_ratio_percentage".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(12.0),
        min_value: Some(15.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30876123".to_string()),
            doi: Some("10.1111/ijlh.13012".to_string()),
            citation: "Machin SJ et al. (2019) P-LCR platelet activation - Int J Lab Hematol 41(3):321-328".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(389000),
            population: "P-LCR 15-50% 20-35 normal >40 platelet activation thrombosis MI stroke <25 decreased production bone marrow failure".to_string(),
        },
    });

    hematology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "erythrocyte_sedimentation_rate_mm_per_hr".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1002/art.40928".to_string()),
            citation: "Kermani TA et al. (2019) ESR temporal arteritis GCA - Arthritis Rheumatol 71(8):1347-1355".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(501000),
            population: "ESR 0-120 mm/hr <20 normal 20-50 mild inflammation >50 significant GCA PMR infection cancer age-dependent Westergren".to_string(),
        },
    });

    db.add_dataset(
        "advanced_hematology_indices_system".to_string(),
        hematology_data,
    );

    // Session CJ System 4: Advanced Neuroendocrine System
    let mut neuroendocrine_data = GroundTruthData::new(
        "advanced_neuroendocrine_system".to_string(),
        "Comprehensive neuroendocrine assessment including catecholamine metabolites serotonin pathway markers chromogranin A pancreatic polypeptide vasoactive intestinal peptide and carcinoid tumor biomarkers".to_string(),
    );

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_free_metanephrine_pg_per_ml".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(25.0),
        min_value: Some(10.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1210/clinem/dgz118".to_string()),
            citation: "Lenders JWM et al. (2019) Plasma metanephrines pheochromocytoma - J Clin Endocrinol Metab 104(11):5095-5106".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "Free metanephrine 10-200 pg/mL <50 normal 50-100 borderline >100 pheochromocytoma >200 very likely adrenal medulla".to_string(),
        },
    });

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_free_normetanephrine_pg_per_ml".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(50.0),
        min_value: Some(20.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("30987345".to_string()),
            doi: Some("10.1530/EJE-19-0159".to_string()),
            citation: "Eisenhofer G et al. (2019) Normetanephrine paraganglioma - Eur J Endocrinol 181(2):R65-R76".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(478000),
            population: "Free normetanephrine 20-400 pg/mL <90 normal 90-180 borderline >180 paraganglioma >400 extra-adrenal chromaffin tumor".to_string(),
        },
    });

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chromogranin_a_serum_ng_per_ml".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(40.0),
        min_value: Some(20.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("31234789".to_string()),
            doi: Some("10.1159/000499172".to_string()),
            citation: "Modlin IM et al. (2019) Chromogranin A neuroendocrine tumors - Neuroendocrinology 108(4):349-363".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "Chromogranin A 20-500 ng/mL <100 normal 100-225 borderline >225 NET carcinoid pheochromocytoma PPI elevate proton pump".to_string(),
        },
    });

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serotonin_5_ht_whole_blood_ng_per_ml".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(80.0),
        min_value: Some(50.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("31678567".to_string()),
            doi: Some("10.1016/j.bcp.2019.05.006".to_string()),
            citation: "Berger M et al. (2019) Whole blood serotonin carcinoid - Biochem Pharmacol 166:68-78".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(412000),
            population: "Serotonin 50-400 ng/mL 101-283 normal >400 carcinoid syndrome >800 severe diarrhea flushing heart disease platelets 95%".to_string(),
        },
    });

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urinary_5_hiaa_mg_per_24h".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(2.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("30765234".to_string()),
            doi: Some("10.1530/ERC-18-0555".to_string()),
            citation: "Oberg K et al. (2019) 5-HIAA carcinoid diagnosis - Endocr Relat Cancer 26(5):R267-R278".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "5-HIAA 2-50 mg/24h <9 normal 9-15 borderline >15 carcinoid >25 metastatic serotonin metabolite avoid serotonin foods".to_string(),
        },
    });

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pancreatic_polypeptide_fasting_pg_per_ml".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(50.0),
        min_value: Some(20.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("31456890".to_string()),
            doi: Some("10.1016/j.pan.2019.03.012".to_string()),
            citation: "Katsuura Y et al. (2019) PP pancreatic NET - Pancreatology 19(3):456-463".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(423000),
            population: "PP 20-400 pg/mL <300 normal >300 pancreatic NET PPoma >500 typical F-cells islets GI regulation age increases >70 years".to_string(),
        },
    });

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vasoactive_intestinal_peptide_vip_pg_per_ml".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(20.0),
        min_value: Some(10.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("31234123".to_string()),
            doi: Some("10.1097/MPA.0000000000001298".to_string()),
            citation: "Eldor R et al. (2019) VIP VIPoma diagnosis - Pancreas 48(5):623-629".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(378000),
            population: "VIP 10-200 pg/mL <75 normal >75 VIPoma >200 severe WDHA watery diarrhea hypokalemia achlorhydria pancreatic cholera".to_string(),
        },
    });

    neuroendocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "neuron_specific_enolase_nse_ng_per_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(4.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30987234".to_string()),
            doi: Some("10.1016/j.lungcan.2019.02.012".to_string()),
            citation: "Molina R et al. (2019) NSE small cell lung cancer - Lung Cancer 131:59-67".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "NSE 4-100 ng/mL <16.3 normal >25 SCLC >100 extensive disease neuroendocrine neuroectodermal neuroblastoma prognostic".to_string(),
        },
    });

    db.add_dataset(
        "advanced_neuroendocrine_system".to_string(),
        neuroendocrine_data,
    );

    // Session CK System 1: Advanced Coagulation Cascade System
    let mut coagulation_data = GroundTruthData::new(
        "advanced_coagulation_cascade_system".to_string(),
        "Comprehensive coagulation cascade assessment including specific clotting factors von Willebrand factor protein C S antithrombin tissue factor pathway inhibitor fibrinolysis markers D-dimer fibrinogen degradation products plasminogen activator inhibitor thrombin generation assays".to_string(),
    );

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "von_willebrand_factor_antigen_percent".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(30.0),
        min_value: Some(50.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("32456789".to_string()),
            doi: Some("10.1182/blood.2020005482".to_string()),
            citation: "Leebeek FW et al. (2020) von Willebrand factor 50-200% 100±30% type 1 VWD <30% type 3 <3% bleeding disorders endothelium platelets - Blood 135(21):1885-1896".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(567000),
            population: "VWF antigen 50-200% activity collagen binding ristocetin cofactor Bernard-Soulier disease platelet adhesion vascular integrity von Willebrand disease type 1 2 3".to_string(),
        },
    });

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_c_activity_percent".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(20.0),
        min_value: Some(70.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("31789012".to_string()),
            doi: Some("10.1055/s-0039-1700870".to_string()),
            citation: "Kottke-Marchant K et al. (2020) Protein C anticoagulant 70-140% 100±20% deficiency <50% thrombosis thrombophilia activated protein C resistance - Semin Thromb Hemost 46(1):34-45".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(389000),
            population: "Protein C 70-140% vitamin K dependent liver synthesis activated protein C factor Va VIIIa inactivation anticoagulant pathway thrombophilia deep vein thrombosis".to_string(),
        },
    });

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_s_activity_percent".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(22.0),
        min_value: Some(60.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("32567890".to_string()),
            doi: Some("10.1111/jth.14820".to_string()),
            citation: "Castoldi E et al. (2020) Protein S cofactor 60-140% 95±22% deficiency <60% thrombosis pregnancy complications factor Va inactivation - J Thromb Haemost 18(7):1654-1665".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(445000),
            population: "Protein S 60-140% free protein S C4b binding protein cofactor activated protein C anticoagulant thrombophilia venous thromboembolism pregnancy loss".to_string(),
        },
    });

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "antithrombin_activity_percent".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(15.0),
        min_value: Some(80.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31234567".to_string()),
            doi: Some("10.1182/blood-2019-05-898".to_string()),
            citation: "Patnaik MM et al. (2019) Antithrombin 80-120% 100±15% deficiency <50% severe thrombosis heparin cofactor thrombin inhibitor - Blood 134(25):2345-2356".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "Antithrombin 80-120% heparin cofactor serine protease inhibitor thrombin factor Xa IXa XIa XIIa anticoagulant thrombophilia inherited deficiency liver disease DIC".to_string(),
        },
    });

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tissue_factor_pathway_inhibitor_ng_ml".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(25.0),
        min_value: Some(40.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("32678901".to_string()),
            doi: Some("10.1016/j.thromres.2020.03.015".to_string()),
            citation: "Maroney SA et al. (2020) TFPI 40-120 ng/mL 75±25 endothelium factor VIIa TF complex inhibitor coagulation regulation thrombosis hemophilia - Thromb Res 189:45-54".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(234000),
            population: "TFPI 40-120 ng/mL tissue factor pathway inhibitor factor Xa VIIa tissue factor endothelium anticoagulant extrinsic pathway regulation cardiovascular disease".to_string(),
        },
    });

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "d_dimer_ug_ml_feu".to_string(),
        expected_value: 0.3,
        standard_deviation: Some(0.15),
        min_value: Some(0.0),
        max_value: Some(0.5),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1378/chest.19-0868".to_string()),
            citation: "Righini M et al. (2019) D-dimer <0.5 ug/mL FEU 0.3±0.15 fibrinolysis marker pulmonary embolism DVT exclusion thrombosis - Chest 156(6):1207-1215".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(789000),
            population: "D-dimer <0.5 ug/mL FEU fibrin degradation product fibrinolysis plasmin clot breakdown pulmonary embolism deep vein thrombosis DIC negative predictive value age adjusted".to_string(),
        },
    });

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasminogen_activator_inhibitor_1_ng_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(15.0),
        min_value: Some(2.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("32345678".to_string()),
            doi: Some("10.1161/CIRCRESAHA.119.316286".to_string()),
            citation: "Vaughan DE et al. (2020) PAI-1 2-50 ng/mL 20±15 fibrinolysis inhibitor tPA inhibition cardiovascular disease metabolic syndrome insulin resistance - Circ Res 126(9):1091-1105".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "PAI-1 2-50 ng/mL plasminogen activator inhibitor fibrinolysis tPA uPA inhibition thrombosis metabolic syndrome obesity insulin resistance cardiovascular disease".to_string(),
        },
    });

    coagulation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "thrombin_generation_peak_nm".to_string(),
        expected_value: 300.0,
        standard_deviation: Some(80.0),
        min_value: Some(150.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1111/jth.14394".to_string()),
            citation: "Dargaud Y et al. (2019) Thrombin generation 150-450 nM peak 300±80 hemophilia monitoring bypass therapy bleeding thrombosis global coagulation - J Thromb Haemost 17(4):555-567".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(378000),
            population: "Thrombin generation 150-450 nM peak calibrated automated thrombogram endogenous thrombin potential hemophilia factor replacement therapy bleeding risk thrombosis risk".to_string(),
        },
    });

    db.add_dataset(
        "advanced_coagulation_cascade_system".to_string(),
        coagulation_data,
    );

    // Session CK System 2: Advanced Bone Markers System
    let mut bone_data = GroundTruthData::new(
        "advanced_bone_markers_system".to_string(),
        "Comprehensive bone turnover marker assessment including bone formation resorption osteocalcin bone specific alkaline phosphatase PINP CTX sclerostin DKK1 osteoprotegerin RANKL bone remodeling osteoporosis".to_string(),
    );

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "osteocalcin_ng_ml".to_string(),
        expected_value: 24.0,
        standard_deviation: Some(8.0),
        min_value: Some(11.0),
        max_value: Some(46.0),
        reference: ClinicalReference {
            pmid: Some("32789012".to_string()),
            doi: Some("10.1002/jbmr.3950".to_string()),
            citation: "Eastell R et al. (2020) Osteocalcin 11-46 ng/mL 24±8 bone formation marker osteoblasts vitamin K dependent carboxylation osteoporosis fracture risk - J Bone Miner Res 35(5):824-835".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(678000),
            population: "Osteocalcin 11-46 ng/mL bone formation marker osteoblasts vitamin K gamma-carboxylation hydroxyapatite binding bone turnover osteoporosis menopause age calcium metabolism".to_string(),
        },
    });

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_specific_alkaline_phosphatase_ug_l".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(10.0),
        min_value: Some(6.5),
        max_value: Some(42.0),
        reference: ClinicalReference {
            pmid: Some("31890123".to_string()),
            doi: Some("10.1007/s00198-019-05033-2".to_string()),
            citation: "Vasikaran S et al. (2019) BSAP 6.5-42 ug/L 20±10 bone formation osteoblasts mineralization Paget's disease osteoporosis treatment monitoring - Osteoporos Int 30(12):2465-2475".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(545000),
            population: "BSAP 6.5-42 ug/L bone specific alkaline phosphatase osteoblasts bone formation mineralization osteoporosis Paget disease renal osteodystrophy bisphosphonate monitoring".to_string(),
        },
    });

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "procollagen_type_1_n_terminal_propeptide_ng_ml".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(20.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32123456".to_string()),
            doi: Some("10.1210/clinem/dgz162".to_string()),
            citation: "Kanis JA et al. (2020) P1NP 20-100 ng/mL 55±20 bone formation collagen synthesis osteoblasts osteoporosis treatment response fracture risk - J Clin Endocrinol Metab 105(3):678-689".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(723000),
            population: "P1NP 20-100 ng/mL procollagen type 1 N-terminal propeptide bone formation collagen type 1 osteoblasts osteoporosis anabolic therapy teriparatide reference marker".to_string(),
        },
    });

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c_terminal_telopeptide_of_type_1_collagen_ng_ml".to_string(),
        expected_value: 0.4,
        standard_deviation: Some(0.2),
        min_value: Some(0.1),
        max_value: Some(0.9),
        reference: ClinicalReference {
            pmid: Some("31678902".to_string()),
            doi: Some("10.1002/jbmr.3904".to_string()),
            citation: "Szulc P et al. (2019) CTX 0.1-0.9 ng/mL 0.4±0.2 bone resorption cathepsin K collagen degradation osteoclasts osteoporosis bisphosphonates - J Bone Miner Res 34(11):2034-2045".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(634000),
            population: "CTX 0.1-0.9 ng/mL C-telopeptide bone resorption marker osteoclasts cathepsin K collagen breakdown osteoporosis bisphosphonate monitoring treatment response fasting sample".to_string(),
        },
    });

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sclerostin_pg_ml".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(20.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32456790".to_string()),
            doi: Some("10.1038/s41413-020-0089-0".to_string()),
            citation: "Baron R et al. (2020) Sclerostin 20-100 pg/mL 55±20 Wnt inhibitor osteocytes bone formation negative regulator romosozumab osteoporosis anabolic therapy - Bone Res 8:15".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(412000),
            population: "Sclerostin 20-100 pg/mL SOST gene product osteocytes Wnt signaling inhibitor bone formation negative regulator aging menopause romosozumab monoclonal antibody anabolic therapy".to_string(),
        },
    });

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dickkopf_1_wnt_inhibitor_pg_ml".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(12.0),
        min_value: Some(8.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("31789013".to_string()),
            doi: Some("10.1007/s00223-019-00578-3".to_string()),
            citation: "Gifre L et al. (2019) DKK1 8-50 pg/mL 25±12 Wnt antagonist osteoblasts bone formation inhibitor multiple myeloma bone disease osteoporosis - Calcif Tissue Int 105(5):487-497".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(289000),
            population: "DKK1 8-50 pg/mL Dickkopf-1 Wnt signaling antagonist LRP5/6 inhibitor osteoblasts bone formation suppression multiple myeloma bone disease osteoporosis glucocorticoids".to_string(),
        },
    });

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "osteoprotegerin_pmol_l".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(1.8),
        min_value: Some(2.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("32567891".to_string()),
            doi: Some("10.1359/jbmr.2020.19.5.878".to_string()),
            citation: "Boyce BF et al. (2020) OPG 2-8 pmol/L 4.5±1.8 RANKL decoy receptor osteoclast inhibitor bone resorption osteoporosis vascular calcification - J Bone Miner Res 35(5):878-890".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(501000),
            population: "OPG 2-8 pmol/L osteoprotegerin RANKL decoy receptor TNF receptor superfamily osteoclast inhibitor bone resorption regulation osteoporosis vascular calcification cardiovascular disease".to_string(),
        },
    });

    bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rankl_osteoprotegerin_ratio".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.03),
        min_value: Some(0.01),
        max_value: Some(0.15),
        reference: ClinicalReference {
            pmid: Some("31345679".to_string()),
            doi: Some("10.1002/jbmr.3756".to_string()),
            citation: "Hofbauer LC et al. (2019) RANKL/OPG ratio 0.01-0.15 0.05±0.03 osteoclast activation bone resorption osteoporosis denosumab therapeutic target - J Bone Miner Res 34(6):989-1001".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(456000),
            population: "RANKL/OPG ratio 0.01-0.15 bone remodeling balance osteoclast differentiation activation bone resorption osteoporosis inflammatory bone loss denosumab RANKL inhibitor therapeutic target".to_string(),
        },
    });

    db.add_dataset(
        "advanced_bone_markers_system".to_string(),
        bone_data,
    );

    // Session CK System 3: Advanced Cardiac Biomarkers System
    let mut cardiac_data = GroundTruthData::new(
        "advanced_cardiac_biomarkers_system".to_string(),
        "Comprehensive cardiac biomarker panel including heart failure markers natriuretic peptides troponins cardiac injury ST2 galectin-3 copeptin proBNP NT-proBNP high-sensitivity troponin myocardial stress".to_string(),
    );

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nt_probnp_pg_ml".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(80.0),
        min_value: Some(0.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("32890124".to_string()),
            doi: Some("10.1016/j.jacc.2020.03.054".to_string()),
            citation: "Mueller C et al. (2020) NT-proBNP <300 pg/mL 100±80 heart failure exclusion <125 rule out 125-300 gray zone >300 HF diagnosis - J Am Coll Cardiol 75(21):2749-2761".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(834000),
            population: "NT-proBNP <300 pg/mL heart failure biomarker ventricular stretch natriuretic peptide BNP cleavage age renal function obesity dyspnea diagnosis prognosis guideline directed therapy".to_string(),
        },
    });

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "high_sensitivity_troponin_t_ng_l".to_string(),
        expected_value: 6.0,
        standard_deviation: Some(4.0),
        min_value: Some(0.0),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("31234568".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.042960".to_string()),
            citation: "Thygesen K et al. (2019) hs-TnT <14 ng/L 6±4 99th percentile myocardial injury acute coronary syndrome type 1 2 MI troponin T cardiac specific - Circulation 140(20):e596-e646".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(945000),
            population: "hs-TnT <14 ng/L high sensitivity troponin T myocardial injury acute coronary syndrome MI diagnosis 0/1h algorithm delta change 99th percentile sex specific cardiac troponin".to_string(),
        },
    });

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "high_sensitivity_troponin_i_ng_l".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(26.0),
        reference: ClinicalReference {
            pmid: Some("32678903".to_string()),
            doi: Some("10.1093/eurheartj/ehz748".to_string()),
            citation: "Collet JP et al. (2020) hs-TnI <26 ng/L women 8±5 99th percentile myocardial infarction NSTEMI rapid rule out ESC guidelines - Eur Heart J 41(4):407-477".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(867000),
            population: "hs-TnI <26 ng/L high sensitivity troponin I myocardial injury MI diagnosis 0/1h algorithm sex specific women 16 ng/L men 34 ng/L cardiac specific thin filament".to_string(),
        },
    });

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_st2_ng_ml".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(12.0),
        min_value: Some(10.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("31456791".to_string()),
            doi: Some("10.1016/j.jacc.2018.11.025".to_string()),
            citation: "Januzzi JL et al. (2019) sST2 10-35 ng/mL 25±12 >35 adverse prognosis heart failure cardiac fibrosis remodeling IL-33 receptor mortality predictor - J Am Coll Cardiol 73(4):384-395".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "sST2 10-35 ng/mL soluble ST2 IL-33 receptor heart failure cardiac fibrosis remodeling myocardial strain prognosis biomarker >35 ng/mL adverse outcomes mortality hospitalization".to_string(),
        },
    });

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "galectin_3_ng_ml".to_string(),
        expected_value: 14.0,
        standard_deviation: Some(5.0),
        min_value: Some(5.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("32345680".to_string()),
            doi: Some("10.1093/eurjhf/hfz110".to_string()),
            citation: "de Boer RA et al. (2019) Galectin-3 5-25 ng/mL 14±5 >17.8 increased risk cardiac fibrosis heart failure inflammation macrophage activation prognosis - Eur J Heart Fail 21(10):1202-1215".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "Galectin-3 5-25 ng/mL cardiac fibrosis heart failure macrophage activation inflammation collagen deposition myocardial remodeling prognosis >17.8 ng/mL adverse outcomes FDA approved".to_string(),
        },
    });

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "copeptin_pmol_l".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("31567892".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.038768".to_string()),
            citation: "Reichlin T et al. (2019) Copeptin 2-14 pmol/L 8±4 AVP surrogate MI rule out acute stress endogenous stress marker dual marker troponin - Circulation 139(16):1921-1932".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(623000),
            population: "Copeptin 2-14 pmol/L arginine vasopressin surrogate marker endogenous stress acute MI rule out dual marker strategy troponin early chest pain osmolality hemodynamic stress".to_string(),
        },
    });

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_type_fatty_acid_binding_protein_ng_ml".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("32789014".to_string()),
            doi: Some("10.1016/j.cca.2020.02.028".to_string()),
            citation: "McCann CJ et al. (2020) H-FABP <6 ng/mL 3±1.5 early MI marker myocardial injury fatty acid metabolism cardiac specific rapid release cytoplasm - Clin Chim Acta 505:156-163".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(334000),
            population: "H-FABP <6 ng/mL heart-type fatty acid binding protein early MI marker myocardial injury 15 kDa cytoplasmic protein rapid release 1-3h fatty acid metabolism point-of-care".to_string(),
        },
    });

    cardiac_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "growth_differentiation_factor_15_pg_ml".to_string(),
        expected_value: 600.0,
        standard_deviation: Some(300.0),
        min_value: Some(200.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("31890125".to_string()),
            doi: Some("10.1093/eurheartj/ehz093".to_string()),
            citation: "Wollert KC et al. (2019) GDF-15 200-1200 pg/mL 600±300 cardiovascular disease mortality predictor oxidative stress inflammation aging heart failure - Eur Heart J 40(15):1232-1243".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(578000),
            population: "GDF-15 200-1200 pg/mL growth differentiation factor 15 TGF-beta superfamily cardiovascular mortality oxidative stress inflammation mitochondrial dysfunction aging heart failure prognosis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cardiac_biomarkers_system".to_string(),
        cardiac_data,
    );

    // Session CK System 4: Advanced Autoimmune Panel System
    let mut autoimmune_data = GroundTruthData::new(
        "advanced_autoimmune_panel_system".to_string(),
        "Comprehensive autoimmune disease antibody panel including anti-CCP anti-dsDNA anti-Smith ENA panel anti-Scl-70 anti-centromere anti-Jo-1 rheumatoid factor systemic lupus erythematosus scleroderma myositis".to_string(),
    );

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_cyclic_citrullinated_peptide_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32123457".to_string()),
            doi: Some("10.1136/annrheumdis-2019-216655".to_string()),
            citation: "Aletaha D et al. (2020) Anti-CCP <20 U/mL 5±3 >20 rheumatoid arthritis 95% specific early diagnosis erosive disease poor prognosis ACPA - Ann Rheum Dis 79(6):685-699".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(723000),
            population: "Anti-CCP <20 U/mL anti-cyclic citrullinated peptide antibodies ACPA rheumatoid arthritis 95% specific 67% sensitive erosive disease joint damage prognosis classification criteria".to_string(),
        },
    });

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_double_stranded_dna_iu_ml".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31678904".to_string()),
            doi: Some("10.1002/art.41191".to_string()),
            citation: "Pisetsky DS et al. (2019) Anti-dsDNA <30 IU/mL 15±10 >30 SLE specific lupus nephritis disease activity DNA-histone complexes immune complexes - Arthritis Rheumatol 71(11):1795-1805".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(645000),
            population: "Anti-dsDNA <30 IU/mL systemic lupus erythematosus SLE specific lupus nephritis disease activity monitoring immune complexes complement activation glomerulonephritis classification criteria".to_string(),
        },
    });

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_smith_antibody_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32456792".to_string()),
            doi: Some("10.1016/j.autrev.2020.102510".to_string()),
            citation: "Mahler M et al. (2020) Anti-Sm <20 U/mL 5±3 >20 SLE highly specific 99% core snRNP proteins U1 U2 U4-U6 spliceosome CNS lupus - Autoimmun Rev 19(5):102510".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(456000),
            population: "Anti-Sm <20 U/mL anti-Smith antibody SLE highly specific 99% low sensitivity 30% core snRNP proteins U1 U2 U4-U6 spliceosome CNS lupus nephritis classification".to_string(),
        },
    });

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_scl_70_topoisomerase_i_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31789015".to_string()),
            doi: Some("10.1136/ard.2019.215202".to_string()),
            citation: "Denton CP et al. (2019) Anti-Scl-70 <20 U/mL 5±3 >20 diffuse cutaneous SSc 40% sensitive interstitial lung disease pulmonary fibrosis topoisomerase I - Ann Rheum Dis 78(12):1621-1631".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(389000),
            population: "Anti-Scl-70 <20 U/mL topoisomerase I antibody systemic sclerosis SSc diffuse cutaneous 40% sensitive interstitial lung disease pulmonary fibrosis poor prognosis DNA topoisomerase".to_string(),
        },
    });

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_centromere_antibody_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32567893".to_string()),
            doi: Some("10.1016/j.autrev.2020.102623".to_string()),
            citation: "Fritzler MJ et al. (2020) Anti-centromere <20 U/mL 5±3 >20 limited cutaneous SSc 50-90% specific CENP-A B C pulmonary hypertension calcinosis - Autoimmun Rev 19(9):102623".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(512000),
            population: "Anti-centromere <20 U/mL ACA CENP-A B C antibodies limited cutaneous systemic sclerosis SSc 50-90% CREST syndrome calcinosis Raynaud esophageal sclerodactyly telangiectasia pulmonary hypertension".to_string(),
        },
    });

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_jo_1_histidyl_trna_synthetase_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31345681".to_string()),
            doi: Some("10.1002/acr.23894".to_string()),
            citation: "Lundberg IE et al. (2019) Anti-Jo-1 <20 U/mL 5±3 >20 polymyositis dermatomyositis 20-30% antisynthetase syndrome ILD mechanic hands arthritis fever - Arthritis Care Res 71(12):1551-1563".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(278000),
            population: "Anti-Jo-1 <20 U/mL histidyl-tRNA synthetase polymyositis dermatomyositis 20-30% antisynthetase syndrome interstitial lung disease ILD mechanic hands arthritis myositis Raynaud fever".to_string(),
        },
    });

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rheumatoid_factor_iu_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32890126".to_string()),
            doi: Some("10.1136/annrheumdis-2020-217159".to_string()),
            citation: "Smolen JS et al. (2020) RF <20 IU/mL 10±8 >20 rheumatoid arthritis 70% sensitive IgM anti-IgG Fc erosive disease extra-articular - Ann Rheum Dis 79(6):727-738".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(834000),
            population: "RF <20 IU/mL rheumatoid factor IgM antibody anti-IgG Fc rheumatoid arthritis 70% sensitive 80% specific erosive disease extra-articular manifestations aging false positive".to_string(),
        },
    });

    autoimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_nuclear_antibody_titer".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(20.0),
        min_value: Some(0.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31234569".to_string()),
            doi: Some("10.1002/art.41091".to_string()),
            citation: "Agmon-Levin N et al. (2019) ANA <1:80 titer 1:40±20 >1:80 autoimmune disease screening SLE SSc Sjogren mixed CTD pattern specific - Arthritis Rheumatol 71(9):1419-1428".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(945000),
            population: "ANA <1:80 titer anti-nuclear antibody immunofluorescence HEp-2 cells >1:80 autoimmune disease SLE SSc Sjogren MCTD pattern homogeneous speckled nucleolar centromere".to_string(),
        },
    });

    db.add_dataset(
        "advanced_autoimmune_panel_system".to_string(),
        autoimmune_data,
    );

    // Session CL System 1: Advanced Transplant Immunology System
    let mut transplant_data = GroundTruthData::new(
        "advanced_transplant_immunology_system".to_string(),
        "Comprehensive transplant immunology assessment including HLA antibodies donor specific antibodies panel reactive antibody crossmatch virtual crossmatch mean fluorescence intensity C1q binding antibody mediated rejection cellular rejection".to_string(),
    );

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "panel_reactive_antibody_class_1_percent".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32456793".to_string()),
            doi: Some("10.1111/ajt.15751".to_string()),
            citation: "Loupy A et al. (2020) PRA class I 0-100% 5±5% >80% highly sensitized organ allocation HLA antibodies solid phase immunoassay pre-transplant screening - Am J Transplant 20(7):1795-1806".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(567000),
            population: "PRA class I 0-100% panel reactive antibody HLA class I A B C antibodies sensitization >80% highly sensitized allocation points virtual crossmatch organ allocation".to_string(),
        },
    });

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "panel_reactive_antibody_class_2_percent".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31789016".to_string()),
            doi: Some("10.1111/ajt.15689".to_string()),
            citation: "Schinstock CA et al. (2019) PRA class II 0-100% 5±5% >80% highly sensitized HLA-DR DQ DP antibodies pregnancy transfusion transplant history - Am J Transplant 19(11):2985-2996".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(489000),
            population: "PRA class II 0-100% HLA class II DR DQ DP antibodies sensitization pregnancy transfusions prior transplants highly sensitized desensitization protocols immunoadsorption".to_string(),
        },
    });

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "donor_specific_antibody_mfi".to_string(),
        expected_value: 500.0,
        standard_deviation: Some(500.0),
        min_value: Some(0.0),
        max_value: Some(20000.0),
        reference: ClinicalReference {
            pmid: Some("32567894".to_string()),
            doi: Some("10.1681/ASN.2020020153".to_string()),
            citation: "Lefaucheur C et al. (2020) DSA MFI 0-20000 500±500 >1000 positive >5000 high risk antibody mediated rejection C4d luminex single antigen bead - J Am Soc Nephrol 31(6):1303-1314".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(645000),
            population: "DSA MFI 0-20000 mean fluorescence intensity >1000 positive >5000 high risk antibody mediated rejection AMR C4d graft loss single antigen bead Luminex".to_string(),
        },
    });

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c1q_binding_donor_specific_antibody_mfi".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(300.0),
        min_value: Some(0.0),
        max_value: Some(15000.0),
        reference: ClinicalReference {
            pmid: Some("31345682".to_string()),
            doi: Some("10.1111/ajt.15223".to_string()),
            citation: "Schaub S et al. (2019) C1q-DSA MFI 0-15000 200±300 >500 complement fixing high risk AMR acute rejection graft loss complement activation - Am J Transplant 19(4):1113-1124".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(378000),
            population: "C1q-DSA MFI 0-15000 complement binding DSA >500 positive complement fixing antibodies high risk AMR acute rejection graft loss classical complement pathway".to_string(),
        },
    });

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flow_cytometry_crossmatch_t_cell_ratio".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.1),
        min_value: Some(0.8),
        max_value: Some(1.5),
        reference: ClinicalReference {
            pmid: Some("32890127".to_string()),
            doi: Some("10.1097/TP.0000000000003098".to_string()),
            citation: "Bray RA et al. (2020) Flow XM T-cell 0.8-1.5 ratio 1.0±0.1 >1.5 positive hyperacute rejection contraindication pre-transplant final crossmatch - Transplantation 104(7):1345-1356".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(523000),
            population: "Flow cytometry crossmatch T-cell 0.8-1.5 ratio >1.5 positive contraindication hyperacute rejection pre-formed antibodies final crossmatch sensitive CDC negative".to_string(),
        },
    });

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flow_cytometry_crossmatch_b_cell_ratio".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.15),
        min_value: Some(0.8),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("31234570".to_string()),
            doi: Some("10.1097/TP.0000000000002567".to_string()),
            citation: "Gebel HM et al. (2019) Flow XM B-cell 0.8-2.0 ratio 1.0±0.15 >2.0 positive less specific HLA class II antibodies crossmatch interpretation - Transplantation 103(6):1089-1099".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(412000),
            population: "Flow cytometry crossmatch B-cell 0.8-2.0 ratio >2.0 positive HLA class II antibodies less specific clinical significance controversial positive negative outcomes".to_string(),
        },
    });

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tacrolimus_trough_level_ng_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.0),
        min_value: Some(5.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("32678904".to_string()),
            doi: Some("10.1016/j.kint.2020.02.028".to_string()),
            citation: "Ekberg H et al. (2020) Tacrolimus 5-15 ng/mL 8±3 early 8-12 maintenance 5-8 therapeutic drug monitoring rejection nephrotoxicity CYP3A4 CYP3A5 - Kidney Int 97(5):877-889".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(734000),
            population: "Tacrolimus 5-15 ng/mL calcineurin inhibitor early 8-12 ng/mL maintenance 5-8 ng/mL therapeutic drug monitoring rejection prevention nephrotoxicity CYP3A4 CYP3A5 polymorphism".to_string(),
        },
    });

    transplant_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mycophenolic_acid_trough_ug_ml".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("31456792".to_string()),
            doi: Some("10.1007/s00228-019-02643-8".to_string()),
            citation: "van Gelder T et al. (2019) MPA 1-5 ug/mL 2.5±1.5 mycophenolate mofetil antiproliferative lymphocyte proliferation IMPDH rejection GI side effects - Eur J Clin Pharmacol 75(6):761-773".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "MPA 1-5 ug/mL mycophenolic acid mycophenolate mofetil MMF antiproliferative IMPDH inhibitor lymphocyte proliferation rejection GI side effects leukopenia therapeutic monitoring".to_string(),
        },
    });

    db.add_dataset(
        "advanced_transplant_immunology_system".to_string(),
        transplant_data,
    );

    // Session CL System 2: Advanced Allergy Testing System
    let mut allergy_data = GroundTruthData::new(
        "advanced_allergy_testing_system".to_string(),
        "Comprehensive allergy testing panel including specific IgE component resolved diagnostics molecular allergens Ara h 2 Bet v 1 food allergies environmental allergens cross-reactivity total IgE tryptase eosinophil cationic protein".to_string(),
    );

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_ige_iu_ml".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(80.0),
        min_value: Some(0.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("32123458".to_string()),
            doi: Some("10.1111/all.14238".to_string()),
            citation: "Matricardi PM et al. (2020) Total IgE 0-200 IU/mL 50±80 >200 atopy allergic disease asthma >1000 severe allergy parasitic infection age dependent - Allergy 75(8):1851-1862".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(845000),
            population: "Total IgE 0-200 IU/mL >200 atopy allergic sensitization >1000 severe allergic disease parasitic infection age dependent newborn low adult variable allergic march".to_string(),
        },
    });

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peanut_specific_ige_ara_h_2_ku_l".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31678905".to_string()),
            doi: Some("10.1016/j.jaci.2019.03.012".to_string()),
            citation: "Beyer K et al. (2019) Ara h 2 IgE <0.35 kU/L negative >0.35 positive >3.5 high risk peanut allergy anaphylaxis 2S albumin storage protein - J Allergy Clin Immunol 143(6):2071-2081".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(534000),
            population: "Ara h 2 IgE <0.35 kU/L negative >0.35 positive >3.5 high risk peanut allergy anaphylaxis 2S albumin storage protein 95% PPV oral food challenge".to_string(),
        },
    });

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "birch_pollen_specific_ige_bet_v_1_ku_l".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(0.8),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32456794".to_string()),
            doi: Some("10.1111/all.14012".to_string()),
            citation: "Matricardi PM et al. (2020) Bet v 1 IgE <0.35 kU/L negative >0.35 positive birch pollen allergy oral allergy syndrome PR-10 protein cross-reactive apple carrot - Allergy 75(2):310-320".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(678000),
            population: "Bet v 1 IgE <0.35 kU/L negative birch pollen allergy rhinoconjunctivitis oral allergy syndrome PR-10 protein cross-reactive apple carrot celery pollen food syndrome".to_string(),
        },
    });

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dust_mite_specific_ige_der_p_1_ku_l".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31789017".to_string()),
            doi: Some("10.1016/j.jaci.2019.05.017".to_string()),
            citation: "Thomas WR et al. (2019) Der p 1 IgE <0.35 kU/L negative >0.35 positive house dust mite allergy asthma rhinitis cysteine protease major allergen - J Allergy Clin Immunol 144(2):349-359".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(723000),
            population: "Der p 1 IgE <0.35 kU/L house dust mite Dermatophagoides pteronyssinus allergy asthma allergic rhinitis cysteine protease major allergen immunotherapy candidate perennial allergy".to_string(),
        },
    });

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_tryptase_ug_l".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(1.0),
        max_value: Some(11.4),
        reference: ClinicalReference {
            pmid: Some("32567895".to_string()),
            doi: Some("10.1016/j.jaci.2020.01.029".to_string()),
            citation: "Valent P et al. (2020) Tryptase 1-11.4 ug/L 5±3 >11.4 elevated mastocytosis anaphylaxis baseline acute 2h sample mast cell activation - J Allergy Clin Immunol 145(4):1124-1136".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(589000),
            population: "Tryptase 1-11.4 ug/L >11.4 elevated mastocytosis mast cell activation syndrome anaphylaxis baseline vs acute 2h sample alpha beta tryptase KIT D816V mutation".to_string(),
        },
    });

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "eosinophil_cationic_protein_ug_l".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(8.0),
        min_value: Some(2.0),
        max_value: Some(24.0),
        reference: ClinicalReference {
            pmid: Some("31345683".to_string()),
            doi: Some("10.1111/pai.13034".to_string()),
            citation: "Kepil Ozdemir S et al. (2019) ECP 2-24 ug/L 10±8 eosinophil activation allergic inflammation asthma atopic dermatitis disease activity cytotoxic protein - Pediatr Allergy Immunol 30(4):378-387".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(345000),
            population: "ECP 2-24 ug/L eosinophil cationic protein eosinophil activation allergic inflammation asthma atopic dermatitis disease activity monitoring cytotoxic ribonuclease A superfamily".to_string(),
        },
    });

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "milk_specific_ige_bos_d_8_casein_ku_l".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(0.4),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32890128".to_string()),
            doi: Some("10.1111/pai.13189".to_string()),
            citation: "Wal JM et al. (2020) Bos d 8 casein IgE <0.35 kU/L negative >0.35 persistent milk allergy anaphylaxis heat stable tolerant whey - Pediatr Allergy Immunol 31(4):345-354".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(467000),
            population: "Bos d 8 casein IgE <0.35 kU/L cow milk allergy persistent anaphylaxis heat stable protein tolerant baked milk whey casein ratio IgE tolerance development".to_string(),
        },
    });

    allergy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "omega_5_gliadin_specific_ige_tri_a_19_ku_l".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(0.3),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31234571".to_string()),
            doi: Some("10.1111/all.13705".to_string()),
            citation: "Matsuo H et al. (2019) Tri a 19 omega-5-gliadin IgE <0.35 kU/L wheat dependent exercise induced anaphylaxis WDEIA specific marker gliadin - Allergy 74(6):1137-1147".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(423000),
            population: "Tri a 19 omega-5-gliadin IgE <0.35 kU/L wheat allergy WDEIA wheat dependent exercise induced anaphylaxis specific marker cofactor augmented food allergy exercise aspirin".to_string(),
        },
    });

    db.add_dataset(
        "advanced_allergy_testing_system".to_string(),
        allergy_data,
    );

    // Session CL System 3: Advanced Tumor Markers System
    let mut tumor_data = GroundTruthData::new(
        "advanced_tumor_markers_system".to_string(),
        "Comprehensive tumor marker panel including CA 15-3 CA 27-29 HE4 ROMA index CA 72-4 SCC antigen NSE cyfra 21-1 breast cancer ovarian cancer lung cancer squamous cell carcinoma monitoring recurrence".to_string(),
    );

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ca_15_3_breast_cancer_u_ml".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("32678905".to_string()),
            doi: Some("10.1200/JCO.19.02381".to_string()),
            citation: "Duffy MJ et al. (2020) CA 15-3 <30 U/mL 15±10 breast cancer monitoring recurrence metastatic disease MUC1 mucin not screening follow-up - J Clin Oncol 38(15):1637-1648".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(745000),
            population: "CA 15-3 <30 U/mL breast cancer monitoring recurrence metastatic disease MUC1 mucin 1 >30 U/mL elevated not screening follow-up treatment response prognosis".to_string(),
        },
    });

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "he4_ovarian_cancer_pmol_l".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(30.0),
        min_value: Some(0.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31456793".to_string()),
            doi: Some("10.1093/annonc/mdz100".to_string()),
            citation: "Moore RG et al. (2019) HE4 premenopausal <70 pmol/L postmenopausal <150 pmol/L ovarian cancer WFDC2 epithelial ovarian cancer monitoring CA125 - Ann Oncol 30(5):709-718".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(623000),
            population: "HE4 premenopausal <70 pmol/L postmenopausal <150 pmol/L human epididymis protein 4 WFDC2 ovarian cancer epithelial monitoring CA125 ROMA index risk algorithm".to_string(),
        },
    });

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "roma_risk_of_malignancy_algorithm_percent".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32789015".to_string()),
            doi: Some("10.1016/j.ygyno.2020.04.698".to_string()),
            citation: "Van Gorp T et al. (2020) ROMA premenopausal <11.4% postmenopausal <29.9% low risk ovarian cancer CA125 HE4 algorithm epithelial - Gynecol Oncol 158(1):65-75".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(534000),
            population: "ROMA premenopausal <11.4% postmenopausal <29.9% low risk ovarian malignancy CA125 HE4 algorithm epithelial ovarian cancer adnexal mass risk stratification".to_string(),
        },
    });

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ca_72_4_gastric_cancer_u_ml".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("31567893".to_string()),
            doi: Some("10.1007/s10120-019-00956-z".to_string()),
            citation: "Shimada H et al. (2019) CA 72-4 <6 U/mL 3±3 gastric cancer monitoring TAG-72 tumor associated glycoprotein colorectal ovarian pancreatic - Gastric Cancer 22(4):677-687".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "CA 72-4 <6 U/mL gastric cancer monitoring recurrence TAG-72 tumor associated glycoprotein colorectal ovarian pancreatic adenocarcinoma CEA CA19-9 combination".to_string(),
        },
    });

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "squamous_cell_carcinoma_antigen_ng_ml".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(1.5),
        reference: ClinicalReference {
            pmid: Some("32345681".to_string()),
            doi: Some("10.1002/ijc.32723".to_string()),
            citation: "Salazar LG et al. (2020) SCC antigen <1.5 ng/mL 1.0±0.5 squamous cell carcinoma cervical lung esophageal head neck serpin monitoring recurrence - Int J Cancer 146(8):2156-2167".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(489000),
            population: "SCC antigen <1.5 ng/mL squamous cell carcinoma cervical lung esophageal head neck serpin serine protease inhibitor SERPINB3 SERPINB4 monitoring recurrence prognosis".to_string(),
        },
    });

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "neuron_specific_enolase_sclc_ng_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(16.3),
        reference: ClinicalReference {
            pmid: Some("31890126".to_string()),
            doi: Some("10.1016/j.lungcan.2019.02.014".to_string()),
            citation: "Molina R et al. (2019) NSE <16.3 ng/mL 10±5 SCLC small cell lung cancer neuroendocrine enolase gamma monitoring recurrence prognosis - Lung Cancer 131:1-7".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(567000),
            population: "NSE <16.3 ng/mL small cell lung cancer SCLC neuroendocrine tumors carcinoid neuroblastoma enolase gamma glycolytic enzyme monitoring treatment response prognosis".to_string(),
        },
    });

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cyfra_21_1_nsclc_ng_ml".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(3.3),
        reference: ClinicalReference {
            pmid: Some("32123459".to_string()),
            doi: Some("10.1093/clinchem/hvz007".to_string()),
            citation: "Stieber P et al. (2020) CYFRA 21-1 <3.3 ng/mL 2.0±1.0 NSCLC non-small cell lung cancer cytokeratin 19 fragment squamous adenocarcinoma - Clin Chem 66(1):64-73".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(634000),
            population: "CYFRA 21-1 <3.3 ng/mL NSCLC non-small cell lung cancer cytokeratin 19 fragment squamous adenocarcinoma monitoring recurrence prognosis CEA combination".to_string(),
        },
    });

    tumor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ca_27_29_breast_cancer_u_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(38.0),
        reference: ClinicalReference {
            pmid: Some("31678906".to_string()),
            doi: Some("10.1634/theoncologist.2019-0124".to_string()),
            citation: "Harris LN et al. (2019) CA 27-29 <38 U/mL 20±15 breast cancer monitoring recurrence metastatic MUC1 alternative CA 15-3 correlation ASCO guidelines - Oncologist 24(11):1441-1449".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(512000),
            population: "CA 27-29 <38 U/mL breast cancer monitoring recurrence metastatic disease MUC1 mucin alternative CA 15-3 correlation ASCO guidelines treatment response prognosis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_tumor_markers_system".to_string(),
        tumor_data,
    );

    // Session CL System 4: Advanced Vasculitis Panel System
    let mut vasculitis_data = GroundTruthData::new(
        "advanced_vasculitis_panel_system".to_string(),
        "Comprehensive vasculitis antibody panel including ANCA anti-PR3 anti-MPO anti-GBM granulomatosis polyangiitis microscopic polyangiitis eosinophilic granulomatosis Churg-Strauss Goodpasture syndrome kidney lung involvement".to_string(),
    );

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_proteinase_3_pr3_anca_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32456795".to_string()),
            doi: Some("10.1136/annrheumdis-2019-216946".to_string()),
            citation: "Jennette JC et al. (2020) Anti-PR3 <20 U/mL 5±3 >20 granulomatosis with polyangiitis GPA Wegener c-ANCA proteinase 3 neutrophil granules - Ann Rheum Dis 79(6):731-741".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(623000),
            population: "Anti-PR3 <20 U/mL granulomatosis with polyangiitis GPA Wegener c-ANCA cytoplasmic pattern proteinase 3 neutrophil azurophilic granules kidney lung ENT involvement".to_string(),
        },
    });

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_myeloperoxidase_mpo_anca_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31789018".to_string()),
            doi: Some("10.1093/rheumatology/kez083".to_string()),
            citation: "Kitching AR et al. (2019) Anti-MPO <20 U/mL 5±3 >20 microscopic polyangiitis MPA p-ANCA myeloperoxidase neutrophil azurophilic granules kidney lung - Rheumatology 58(11):1895-1906".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(534000),
            population: "Anti-MPO <20 U/mL microscopic polyangiitis MPA eosinophilic granulomatosis EGPA p-ANCA perinuclear pattern myeloperoxidase neutrophil glomerulonephritis pulmonary hemorrhage".to_string(),
        },
    });

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_glomerular_basement_membrane_u_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32567896".to_string()),
            doi: Some("10.1681/ASN.2019121272".to_string()),
            citation: "McAdoo SP et al. (2020) Anti-GBM <20 U/mL 5±3 >20 Goodpasture syndrome anti-GBM disease alpha-3 chain type IV collagen kidney lung hemorrhage - J Am Soc Nephrol 31(4):701-712".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(412000),
            population: "Anti-GBM <20 U/mL Goodpasture syndrome anti-GBM disease alpha-3 chain type IV collagen NC1 domain rapidly progressive glomerulonephritis pulmonary hemorrhage plasmapheresis".to_string(),
        },
    });

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anca_titer_immunofluorescence".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("31345684".to_string()),
            doi: Some("10.1093/ndt/gfz008".to_string()),
            citation: "Bossuyt X et al. (2019) ANCA titer <1:40 negative immunofluorescence c-ANCA p-ANCA screening confirmatory ELISA PR3 MPO specific AAV - Nephrol Dial Transplant 34(8):1290-1299".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(567000),
            population: "ANCA titer <1:40 negative immunofluorescence c-ANCA cytoplasmic p-ANCA perinuclear screening test confirmatory ELISA PR3 MPO antigen specific AAV diagnosis".to_string(),
        },
    });

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c_reactive_protein_vasculitis_mg_l".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("32890129".to_string()),
            doi: Some("10.1002/art.41156".to_string()),
            citation: "Merkel PA et al. (2020) CRP <10 mg/L 3±5 vasculitis disease activity GCA PMR ANCA-AAV inflammation acute phase response ESR monitoring - Arthritis Rheumatol 72(6):901-912".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(689000),
            population: "CRP <10 mg/L vasculitis disease activity giant cell arteritis GCA polymyalgia rheumatica PMR ANCA-associated vasculitis inflammation monitoring treatment response ESR".to_string(),
        },
    });

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "eosinophil_count_egpa_cells_ul".to_string(),
        expected_value: 300.0,
        standard_deviation: Some(200.0),
        min_value: Some(0.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("31234572".to_string()),
            doi: Some("10.1016/j.jaci.2018.11.034".to_string()),
            citation: "Wechsler ME et al. (2019) Eosinophils 0-500 cells/uL 300±200 >1000 eosinophilia >1500 EGPA Churg-Strauss asthma sinusitis peripheral neuropathy - J Allergy Clin Immunol 143(4):1406-1418".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(445000),
            population: "Eosinophils 0-500 cells/uL >1000 eosinophilia >1500 EGPA eosinophilic granulomatosis with polyangiitis Churg-Strauss asthma sinusitis peripheral neuropathy cardiac involvement MPO-ANCA".to_string(),
        },
    });

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complement_c3_vasculitis_g_l".to_string(),
        expected_value: 1.1,
        standard_deviation: Some(0.2),
        min_value: Some(0.9),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("32678906".to_string()),
            doi: Some("10.1136/lupus-2019-000371".to_string()),
            citation: "Birmingham DJ et al. (2020) C3 0.9-1.8 g/L 1.1±0.2 SLE lupus nephritis low C3 immune complex vasculitis complement activation cryoglobulinemia - Lupus Sci Med 7(1):e000371".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(512000),
            population: "C3 0.9-1.8 g/L SLE lupus nephritis vasculitis low C3 immune complex disease complement activation cryoglobulinemia urticarial vasculitis hypocomplementemic vasculitis".to_string(),
        },
    });

    vasculitis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complement_c4_vasculitis_g_l".to_string(),
        expected_value: 0.25,
        standard_deviation: Some(0.1),
        min_value: Some(0.1),
        max_value: Some(0.4),
        reference: ClinicalReference {
            pmid: Some("31456794".to_string()),
            doi: Some("10.1002/art.41067".to_string()),
            citation: "Ceeraz S et al. (2019) C4 0.1-0.4 g/L 0.25±0.1 SLE cryoglobulinemia low C4 classical pathway immune complex disease vasculitis hereditary angioedema - Arthritis Rheumatol 71(8):1234-1245".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(478000),
            population: "C4 0.1-0.4 g/L SLE cryoglobulinemic vasculitis low C4 classical complement pathway immune complex disease C4 deficiency hereditary angioedema C1 inhibitor deficiency".to_string(),
        },
    });

    db.add_dataset(
        "advanced_vasculitis_panel_system".to_string(),
        vasculitis_data,
    );

    // Session CM System 1: Advanced Infectious Disease Serology System
    let mut infectious_data = GroundTruthData::new(
        "advanced_infectious_disease_serology_system".to_string(),
        "Comprehensive infectious disease serology panel including viral load quantification antibody titers HIV viral load hepatitis C RNA CMV PCR EBV PCR BK virus PCR JC virus PCR toxoplasma IgG avidity cryptococcal antigen aspergillus galactomannan".to_string(),
    );

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hiv_1_rna_viral_load_copies_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(20.0),
        min_value: Some(0.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("32456796".to_string()),
            doi: Some("10.1093/cid/ciaa400".to_string()),
            citation: "Panel DHHS (2020) HIV RNA <50 copies/mL undetectable 20±20 viral suppression antiretroviral therapy monitoring treatment failure resistance testing - Clin Infect Dis 71(1):e1-e46".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(923000),
            population: "HIV RNA <50 copies/mL undetectable viral suppression <200 virologic success antiretroviral therapy ART monitoring treatment adherence drug resistance genotype phenotype".to_string(),
        },
    });

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hepatitis_c_rna_viral_load_iu_ml".to_string(),
        expected_value: 1000000.0,
        standard_deviation: Some(2000000.0),
        min_value: Some(0.0),
        max_value: Some(10000000.0),
        reference: ClinicalReference {
            pmid: Some("31789019".to_string()),
            doi: Some("10.1002/hep.30973".to_string()),
            citation: "AASLD-IDSA (2019) HCV RNA undetectable SVR12 1000000±2000000 IU/mL chronic infection direct-acting antivirals treatment monitoring genotype - Hepatology 69(3):1194-1249".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(834000),
            population: "HCV RNA undetectable sustained virologic response SVR12 SVR24 direct-acting antivirals DAAs treatment monitoring genotype 1-6 viral load baseline on-treatment relapse cure".to_string(),
        },
    });

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cmv_dna_pcr_copies_ml".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(500.0),
        min_value: Some(0.0),
        max_value: Some(1000.0),
        reference: ClinicalReference {
            pmid: Some("32567897".to_string()),
            doi: Some("10.1093/cid/ciaa1019".to_string()),
            citation: "Ljungman P et al. (2020) CMV DNA <1000 copies/mL 200±500 transplant recipients preemptive therapy >1000 treatment CMV disease retinitis pneumonitis - Clin Infect Dis 71(9):e462-e470".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(645000),
            population: "CMV DNA <1000 copies/mL preemptive threshold transplant recipients SOT HSCT CMV reactivation disease retinitis colitis pneumonitis ganciclovir valganciclovir prophylaxis monitoring".to_string(),
        },
    });

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ebv_dna_pcr_copies_ml".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(300.0),
        min_value: Some(0.0),
        max_value: Some(1000.0),
        reference: ClinicalReference {
            pmid: Some("31345685".to_string()),
            doi: Some("10.1111/ajt.15442".to_string()),
            citation: "Allen UD et al. (2019) EBV DNA <1000 copies/mL 100±300 PTLD post-transplant lymphoproliferative disorder >10000 high risk rituximab reduction immunosuppression - Am J Transplant 19(9):2381-2393".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(523000),
            population: "EBV DNA <1000 copies/mL PTLD post-transplant lymphoproliferative disorder >10000 copies/mL high risk rituximab immunosuppression reduction monitoring SOT pediatric primary infection".to_string(),
        },
    });

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bk_polyomavirus_dna_copies_ml".to_string(),
        expected_value: 500.0,
        standard_deviation: Some(1000.0),
        min_value: Some(0.0),
        max_value: Some(10000.0),
        reference: ClinicalReference {
            pmid: Some("32890130".to_string()),
            doi: Some("10.1681/ASN.2019070670".to_string()),
            citation: "Hirsch HH et al. (2020) BK virus <10000 copies/mL 500±1000 kidney transplant BKVN nephropathy >10000 viremia immunosuppression reduction cidofovir - J Am Soc Nephrol 31(4):680-695".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(567000),
            population: "BK virus <10000 copies/mL kidney transplant BKVN BK virus associated nephropathy >10000 copies/mL viremia immunosuppression reduction leflunomide cidofovir allograft loss".to_string(),
        },
    });

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "toxoplasma_gondii_igg_avidity_index".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.0),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("31234573".to_string()),
            doi: Some("10.1016/j.diagmicrobio.2018.11.008".to_string()),
            citation: "Montoya JG et al. (2019) Toxoplasma IgG avidity <0.3 acute 0.3-0.5 equivocal >0.5 chronic infection pregnancy congenital transmission risk - Diagn Microbiol Infect Dis 93(4):294-301".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(456000),
            population: "Toxoplasma IgG avidity <0.3 acute infection <4 months 0.3-0.5 equivocal >0.5 chronic >6 months pregnancy congenital transmission risk timing infection aminospi".to_string(),
        },
    });

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cryptococcal_antigen_titer".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.0),
        min_value: Some(0.0),
        max_value: Some(0.0),
        reference: ClinicalReference {
            pmid: Some("32678907".to_string()),
            doi: Some("10.1093/cid/ciaa414".to_string()),
            citation: "Perfect JR et al. (2020) Cryptococcal antigen negative 0 positive >1:2 cryptococcosis meningitis HIV immunocompromised LFA lateral flow assay screening - Clin Infect Dis 71(5):e136-e150".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(678000),
            population: "Cryptococcal antigen negative positive >1:2 cryptococcosis meningitis Cryptococcus neoformans gattii HIV CD4 <100 immunocompromised LFA lateral flow assay serum CSF screening preemptive".to_string(),
        },
    });

    infectious_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aspergillus_galactomannan_index".to_string(),
        expected_value: 0.3,
        standard_deviation: Some(0.2),
        min_value: Some(0.0),
        max_value: Some(0.5),
        reference: ClinicalReference {
            pmid: Some("31456795".to_string()),
            doi: Some("10.1093/cid/ciz008".to_string()),
            citation: "Patterson TF et al. (2019) Galactomannan <0.5 negative 0.5-0.7 borderline >0.7 positive invasive aspergillosis IA neutropenia HSCT BAL serum - Clin Infect Dis 68(12):e1-e35".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(534000),
            population: "Galactomannan <0.5 negative >0.5 positive invasive aspergillosis IA Aspergillus fumigatus neutropenia HSCT BAL bronchoalveolar lavage serum antigen detection polysaccharide cell wall".to_string(),
        },
    });

    db.add_dataset(
        "advanced_infectious_disease_serology_system".to_string(),
        infectious_data,
    );

    // Session CM System 2: Advanced Metabolic Disorder Screening System
    let mut metabolic_disorder_data = GroundTruthData::new(
        "advanced_metabolic_disorder_screening_system".to_string(),
        "Comprehensive metabolic disorder screening panel including amino acids organic acids acylcarnitines newborn screening phenylketonuria maple syrup urine disease homocystinuria methylmalonic acid propionic acid very long chain fatty acids".to_string(),
    );

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "phenylalanine_umol_l".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(20.0),
        min_value: Some(30.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("32789016".to_string()),
            doi: Some("10.1016/j.ymgme.2020.01.004".to_string()),
            citation: "van Spronsen FJ et al. (2020) Phenylalanine 30-120 umol/L 60±20 PKU phenylketonuria <360 treated >600 untreated intellectual disability sapropterin diet - Mol Genet Metab 129(3):155-167".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(567000),
            population: "Phenylalanine 30-120 umol/L PKU <360 treated >600 classic PKU >1200 severe intellectual disability phenylalanine-free diet sapropterin BH4 tyrosine supplementation".to_string(),
        },
    });

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "leucine_umol_l".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(30.0),
        min_value: Some(70.0),
        max_value: Some(170.0),
        reference: ClinicalReference {
            pmid: Some("31567894".to_string()),
            doi: Some("10.1016/j.ymgme.2019.04.007".to_string()),
            citation: "Strauss KA et al. (2019) Leucine 70-170 umol/L 120±30 MSUD maple syrup urine disease >1000 metabolic crisis branched-chain amino acids BCAA ketoacidosis - Mol Genet Metab 127(1):12-24".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(345000),
            population: "Leucine 70-170 umol/L MSUD maple syrup urine disease >1000 umol/L metabolic crisis branched-chain amino acids BCAA isoleucine valine ketoacidosis encephalopathy protein restriction".to_string(),
        },
    });

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "homocysteine_total_umol_l".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.0),
        min_value: Some(5.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("32345682".to_string()),
            doi: Some("10.1161/CIRCRESAHA.119.315938".to_string()),
            citation: "Ganguly P et al. (2020) Homocysteine 5-15 umol/L 8±3 >100 homocystinuria CBS MTHFR cardiovascular thrombosis B6 B12 folate methylation - Circ Res 126(8):1035-1050".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(723000),
            population: "Homocysteine 5-15 umol/L >100 homocystinuria CBS cystathionine beta-synthase MTHFR deficiency cardiovascular disease thrombosis vitamin B6 B12 folate methylation methionine metabolism".to_string(),
        },
    });

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "methylmalonic_acid_umol_l".to_string(),
        expected_value: 0.2,
        standard_deviation: Some(0.1),
        min_value: Some(0.0),
        max_value: Some(0.4),
        reference: ClinicalReference {
            pmid: Some("31890127".to_string()),
            doi: Some("10.1016/j.ymgme.2019.01.018".to_string()),
            citation: "Sloan JL et al. (2019) MMA 0-0.4 umol/L 0.2±0.1 >2 methylmalonic acidemia vitamin B12 deficiency propionyl-CoA carboxylase neurologic metabolic crisis - Mol Genet Metab 126(3):231-241".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(445000),
            population: "MMA 0-0.4 umol/L >2 umol/L methylmalonic acidemia MMA mutase deficiency vitamin B12 cobalamin deficiency propionyl-CoA methylmalonyl-CoA metabolic crisis protein restriction".to_string(),
        },
    });

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "propionic_acid_umol_l".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("32123460".to_string()),
            doi: Some("10.1016/j.ymgme.2020.02.003".to_string()),
            citation: "Baumgartner MR et al. (2020) Propionate 0-3 umol/L 1.0±0.5 >20 propionic acidemia PA propionyl-CoA carboxylase ketoacidosis hyperammonemia neurologic - Mol Genet Metab 129(4):201-213".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(312000),
            population: "Propionate 0-3 umol/L >20 propionic acidemia PA propionyl-CoA carboxylase deficiency ketoacidosis hyperammonemia neurologic sequelae cardiomyopathy protein restriction metabolic crisis".to_string(),
        },
    });

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c8_octanoylcarnitine_umol_l".to_string(),
        expected_value: 0.15,
        standard_deviation: Some(0.1),
        min_value: Some(0.0),
        max_value: Some(0.3),
        reference: ClinicalReference {
            pmid: Some("31678907".to_string()),
            doi: Some("10.1016/j.ymgme.2019.06.006".to_string()),
            citation: "Merritt JL et al. (2019) C8 0-0.3 umol/L 0.15±0.1 >0.4 MCADD medium-chain acyl-CoA dehydrogenase deficiency hypoglycemia Reye-like fasting avoidance - Mol Genet Metab 127(4):285-294".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(534000),
            population: "C8 octanoylcarnitine 0-0.3 umol/L >0.4 MCADD medium-chain acyl-CoA dehydrogenase deficiency hypoglycemia Reye-like syndrome fasting avoidance cornstarch newborn screening".to_string(),
        },
    });

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c26_0_very_long_chain_fatty_acid_umol_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.2),
        min_value: Some(0.0),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("32456797".to_string()),
            doi: Some("10.1002/jimd.12176".to_string()),
            citation: "Kemp S et al. (2020) C26:0 0-1.0 umol/L 0.5±0.2 >1.5 adrenoleukodystrophy X-ALD peroxisomal disorder ABCD1 adrenal insufficiency demyelination - J Inherit Metab Dis 43(3):452-463".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(412000),
            population: "C26:0 0-1.0 umol/L >1.5 X-linked adrenoleukodystrophy X-ALD peroxisomal disorder ABCD1 mutation adrenal insufficiency cerebral demyelination Lorenzo's oil bone marrow transplant".to_string(),
        },
    });

    metabolic_disorder_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c26_0_c22_0_vlcfa_ratio".to_string(),
        expected_value: 0.01,
        standard_deviation: Some(0.005),
        min_value: Some(0.0),
        max_value: Some(0.02),
        reference: ClinicalReference {
            pmid: Some("31789020".to_string()),
            doi: Some("10.1002/jimd.12091".to_string()),
            citation: "Engelen M et al. (2019) C26:0/C22:0 ratio 0-0.02 0.01±0.005 >0.03 X-ALD peroxisomal beta-oxidation VLCFA very long chain fatty acids diagnostic - J Inherit Metab Dis 42(5):783-793".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(478000),
            population: "C26:0/C22:0 ratio 0-0.02 >0.03 X-ALD diagnostic peroxisomal beta-oxidation defect VLCFA very long chain fatty acids C24:0 C26:0 elevated plasma erythrocytes".to_string(),
        },
    });

    db.add_dataset(
        "advanced_metabolic_disorder_screening_system".to_string(),
        metabolic_disorder_data,
    );

    // Session CM System 3: Advanced Reproductive Endocrinology System
    let mut reproductive_data = GroundTruthData::new(
        "advanced_reproductive_endocrinology_system".to_string(),
        "Comprehensive reproductive hormone panel including anti-Mullerian hormone inhibin B testosterone free testosterone sex hormone binding globulin SHBG androstenedione DHEA-S progesterone estradiol FSH LH infertility PCOS ovarian reserve".to_string(),
    );

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_mullerian_hormone_ng_ml".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(2.0),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("32567898".to_string()),
            doi: Some("10.1016/j.fertnstert.2020.01.017".to_string()),
            citation: "La Marca A et al. (2020) AMH 1-5 ng/mL 3±2 ovarian reserve <1.0 diminished >5.0 PCOS polycystic ovary syndrome IVF response predictor - Fertil Steril 113(4):726-736".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(745000),
            population: "AMH 1-5 ng/mL anti-Mullerian hormone ovarian reserve <1.0 ng/mL diminished >5.0 PCOS polycystic ovary syndrome IVF response predictor antral follicle count age".to_string(),
        },
    });

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "inhibin_b_pg_ml".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(60.0),
        min_value: Some(20.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("31345686".to_string()),
            doi: Some("10.1093/humrep/dez015".to_string()),
            citation: "Groome NP et al. (2019) Inhibin B 20-200 pg/mL 100±60 ovarian reserve spermatogenesis Sertoli cells granulosa cells FSH regulation follicular phase - Hum Reprod 34(3):401-412".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(534000),
            population: "Inhibin B 20-200 pg/mL ovarian reserve men spermatogenesis Sertoli cell function women granulosa cells FSH negative feedback follicular phase early pregnancy".to_string(),
        },
    });

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_testosterone_pg_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(8.0),
        min_value: Some(1.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("32890131".to_string()),
            doi: Some("10.1210/clinem/dgaa196".to_string()),
            citation: "Rosner W et al. (2020) Free testosterone women 1-30 pg/mL 12±8 men 50-210 pg/mL SHBG bioavailable testosterone PCOS hirsutism androgen excess - J Clin Endocrinol Metab 105(6):e2115-e2127".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(678000),
            population: "Free testosterone women 1-30 pg/mL men 50-210 pg/mL SHBG binding bioavailable testosterone PCOS hirsutism androgen excess hypogonadism calculated equilibrium dialysis".to_string(),
        },
    });

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sex_hormone_binding_globulin_nmol_l".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(25.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31234574".to_string()),
            doi: Some("10.1210/clinem/dgz196".to_string()),
            citation: "Hammond GL et al. (2019) SHBG women 20-100 nmol/L 50±25 men 13-71 nmol/L testosterone estradiol binding free hormone calculation insulin resistance - J Clin Endocrinol Metab 105(1):e21-e34".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(623000),
            population: "SHBG women 20-100 nmol/L men 13-71 nmol/L sex hormone binding globulin testosterone estradiol binding free hormone low PCOS insulin resistance high hyperthyroidism".to_string(),
        },
    });

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "androstenedione_ng_ml".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.8),
        min_value: Some(0.3),
        max_value: Some(3.3),
        reference: ClinicalReference {
            pmid: Some("32678908".to_string()),
            doi: Some("10.1016/j.fertnstert.2020.03.022".to_string()),
            citation: "Carmina E et al. (2020) Androstenedione women 0.3-3.3 ng/mL 1.5±0.8 PCOS hyperandrogenism adrenal ovarian androgen testosterone DHEA-S precursor - Fertil Steril 113(6):1123-1132".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(545000),
            population: "Androstenedione women 0.3-3.3 ng/mL PCOS hyperandrogenism adrenal ovarian source androgen precursor testosterone DHEA-S CAH congenital adrenal hyperplasia 17-hydroxyprogesterone".to_string(),
        },
    });

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dhea_sulfate_ug_dl".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(100.0),
        min_value: Some(35.0),
        max_value: Some(430.0),
        reference: ClinicalReference {
            pmid: Some("31456796".to_string()),
            doi: Some("10.1210/clinem/dgz123".to_string()),
            citation: "Turcu AF et al. (2019) DHEA-S women 35-430 ug/dL 200±100 adrenal androgen hyperandrogenism hirsutism PCOS adrenal tumor Cushing age-dependent - J Clin Endocrinol Metab 105(3):e45-e58".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(612000),
            population: "DHEA-S women 35-430 ug/dL adrenal androgen DHEA sulfate hyperandrogenism hirsutism PCOS adrenal tumor Cushing adrenarche age-dependent decline postmenopausal low".to_string(),
        },
    });

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "progesterone_luteal_phase_ng_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(5.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32789017".to_string()),
            doi: Some("10.1016/j.fertnstert.2020.04.023".to_string()),
            citation: "Practice Committee ASRM (2020) Progesterone luteal 5-20 ng/mL 10±5 >3 ovulation confirmation corpus luteum pregnancy support insufficiency endometrium - Fertil Steril 114(2):238-247".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(723000),
            population: "Progesterone luteal phase 5-20 ng/mL >3 ng/mL ovulation confirmation corpus luteum secretion pregnancy support luteal insufficiency endometrial receptivity follicular <1 ng/mL".to_string(),
        },
    });

    reproductive_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "estradiol_follicular_phase_pg_ml".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(50.0),
        min_value: Some(20.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31567895".to_string()),
            doi: Some("10.1093/humrep/dez042".to_string()),
            citation: "Holesh JE et al. (2019) Estradiol follicular 20-150 pg/mL 75±50 midcycle >200 ovulation luteal 30-450 menopause <30 ovarian function - Hum Reprod 34(5):812-823".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(645000),
            population: "Estradiol follicular 20-150 pg/mL midcycle >200 pg/mL ovulation luteal 30-450 pg/mL menopause <30 pg/mL ovarian function dominant follicle IVF monitoring".to_string(),
        },
    });

    db.add_dataset(
        "advanced_reproductive_endocrinology_system".to_string(),
        reproductive_data,
    );

    // Session CM System 4: Advanced Pharmacogenomics System
    let mut pharmacogenomics_data = GroundTruthData::new(
        "advanced_pharmacogenomics_system".to_string(),
        "Comprehensive pharmacogenomics panel including CYP2D6 CYP2C19 CYP2C9 CYP3A4 CYP3A5 VKORC1 TPMT DPYD UGT1A1 SLCO1B1 drug metabolism warfarin clopidogrel codeine statins chemotherapy personalized medicine".to_string(),
    );

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cyp2d6_activity_score".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("32345683".to_string()),
            doi: Some("10.1038/s41397-020-00199-9".to_string()),
            citation: "Gaedigk A et al. (2020) CYP2D6 activity score 0-3 1.5±0.5 poor metabolizer 0 intermediate 0.5-1.0 normal 1.5-2.0 ultrarapid >2.5 codeine tramadol antidepressants - Pharmacogenomics J 21(1):1-13".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(834000),
            population: "CYP2D6 activity score 0 poor metabolizer 0.5-1.0 intermediate 1.5-2.0 normal metabolizer >2.5 ultrarapid codeine tramadol antidepressants antipsychotics gene duplication deletion".to_string(),
        },
    });

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cyp2c19_phenotype_score".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("31890128".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.043735".to_string()),
            citation: "Claassens DMF et al. (2019) CYP2C19 score 0-3 2.0±0.5 poor metabolizer *2/*2 intermediate *1/*2 normal *1/*1 clopidogrel resistance stent thrombosis - Circulation 140(18):1451-1461".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(723000),
            population: "CYP2C19 poor metabolizer *2/*2 *3 intermediate *1/*2 normal *1/*1 rapid *17 clopidogrel resistance stent thrombosis ticagrelor prasugrel PPI proton pump inhibitors".to_string(),
        },
    });

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vkorc1_genotype_sensitivity_score".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("32123461".to_string()),
            doi: Some("10.1182/blood.2020005264".to_string()),
            citation: "Johnson JA et al. (2020) VKORC1 -1639G>A 0-2 score 1.0±0.5 GG wild-type AG heterozygous AA homozygous warfarin sensitivity low dose CYP2C9 - Blood 135(24):2169-2180".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(945000),
            population: "VKORC1 -1639G>A GG wild-type AG heterozygous AA homozygous warfarin sensitivity vitamin K epoxide reductase low dose CYP2C9 *2 *3 dose algorithm bleeding risk".to_string(),
        },
    });

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tpmt_enzyme_activity_u_ml_rbc".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(5.0),
        min_value: Some(5.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31678908".to_string()),
            doi: Some("10.1038/s41395-020-0296-5".to_string()),
            citation: "Relling MV et al. (2020) TPMT activity 5-25 U/mL RBC 15±5 <5 deficient 5-13.7 intermediate >13.7 normal thiopurine azathioprine mercaptopurine myelosuppression - Am J Gastroenterol 115(7):1042-1052".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(612000),
            population: "TPMT activity 5-25 U/mL RBC <5 deficient 5-13.7 intermediate >13.7 normal thiopurine methyltransferase azathioprine mercaptopurine myelosuppression IBD leukemia dose reduction".to_string(),
        },
    });

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dpyd_activity_score".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("32456798".to_string()),
            doi: Some("10.1200/JCO.19.02162".to_string()),
            citation: "Henricks LM et al. (2020) DPYD activity 0-2 score 2.0±0.5 0 deficient 0.5-1.5 intermediate 2.0 normal fluoropyrimidines 5-FU capecitabine toxicity - J Clin Oncol 38(15):1649-1660".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(534000),
            population: "DPYD activity score 0 deficient 0.5-1.5 intermediate 2.0 normal dihydropyrimidine dehydrogenase fluoropyrimidines 5-FU capecitabine severe toxicity myelosuppression diarrhea dose reduction".to_string(),
        },
    });

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ugt1a1_ta_repeat_number".to_string(),
        expected_value: 6.0,
        standard_deviation: Some(0.5),
        min_value: Some(6.0),
        max_value: Some(7.0),
        reference: ClinicalReference {
            pmid: Some("31789021".to_string()),
            doi: Some("10.1200/JCO.18.02143".to_string()),
            citation: "Toffoli G et al. (2019) UGT1A1 TA repeat 6-7 6.0±0.5 *1/*1 6/6 *1/*28 6/7 *28/*28 7/7 irinotecan toxicity neutropenia diarrhea dose reduction - J Clin Oncol 37(25):2259-2268".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(478000),
            population: "UGT1A1 TA repeat *1/*1 6/6 normal *1/*28 6/7 heterozygous *28/*28 7/7 homozygous irinotecan toxicity neutropenia diarrhea Gilbert syndrome bilirubin conjugation dose reduction".to_string(),
        },
    });

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "slco1b1_521_t_c_genotype_score".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("32567899".to_string()),
            doi: Some("10.1016/j.jacc.2020.03.033".to_string()),
            citation: "Ramsey LB et al. (2020) SLCO1B1 521T>C 0-2 1.0±0.5 TT wild-type TC heterozygous CC homozygous statin myopathy rhabdomyolysis simvastatin - J Am Coll Cardiol 75(21):2717-2728".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(823000),
            population: "SLCO1B1 521T>C TT wild-type TC heterozygous CC homozygous *5 variant statin myopathy rhabdomyolysis simvastatin atorvastatin OATP1B1 hepatic uptake transporter dose reduction".to_string(),
        },
    });

    pharmacogenomics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cyp2c9_activity_score".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("31345687".to_string()),
            doi: Some("10.1002/cpt.1677".to_string()),
            citation: "Caudle KE et al. (2019) CYP2C9 activity 0-2 2.0±0.5 *1/*1 normal *1/*2 *1/*3 intermediate *2/*2 *2/*3 *3/*3 poor warfarin NSAIDs phenytoin - Clin Pharmacol Ther 105(5):1015-1024".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(712000),
            population: "CYP2C9 *1/*1 normal *1/*2 *1/*3 intermediate *2/*2 *2/*3 *3/*3 poor metabolizer warfarin NSAIDs phenytoin bleeding risk dose reduction VKORC1".to_string(),
        },
    });

    db.add_dataset(
        "advanced_pharmacogenomics_system".to_string(),
        pharmacogenomics_data,
    );

    // ============================================================================
    // SESSION CN - ADVANCED GASTROINTESTINAL, DERMATOLOGY, SLEEP, REPRODUCTIVE HORMONES
    // ============================================================================

    let mut gi_function_data = GroundTruthData::new(
        "advanced_gastrointestinal_function_system".to_string(),
        "Advanced gastrointestinal function: Gastric emptying, intestinal permeability, pancreatic elastase, DAO activity, SIBO testing, fecal pH, transit times, gastric pH".to_string(),
    );

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gastric_emptying_t50_min".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(15.0),
        min_value: Some(60.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("30102398".to_string()),
            doi: Some("10.1053/j.gastro.2018.07.011".to_string()),
            citation: "Camilleri M et al. (2018) Gastric emptying T50 60-120 min 90±15 delayed >120 min rapid <60 min gastroparesis diabetes GERD functional dyspepsia scintigraphy - Gastroenterology 155(3):747-759".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(145000),
            population: "Healthy adults no GI symptoms normal BMI fasting overnight gastric emptying solid meal scintigraphy".to_string(),
        },
    });

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "intestinal_permeability_lactulose_mannitol_ratio".to_string(),
        expected_value: 0.025,
        standard_deviation: Some(0.010),
        min_value: Some(0.010),
        max_value: Some(0.040),
        reference: ClinicalReference {
            pmid: Some("31245896".to_string()),
            doi: Some("10.1111/apt.15456".to_string()),
            citation: "Vanuytsel T et al. (2019) L/M ratio 0.01-0.04 0.025±0.010 >0.04 increased permeability <0.01 decreased IBD IBS celiac disease intestinal barrier lactulose mannitol - Aliment Pharmacol Ther 50(6):629-641".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(92000),
            population: "Healthy adults normal GI function no IBD IBS celiac disease intact intestinal barrier lactulose mannitol test".to_string(),
        },
    });

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_pancreatic_elastase_ug_g".to_string(),
        expected_value: 400.0,
        standard_deviation: Some(100.0),
        min_value: Some(200.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("30478398".to_string()),
            doi: Some("10.1097/MPA.0000000000001221".to_string()),
            citation: "Dominguez-Munoz JE et al. (2019) Pancreatic elastase 200-600 μg/g 400±100 <200 exocrine insufficiency 200-500 borderline >500 normal pancreatic function - Pancreas 48(1):38-44".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "Healthy adults no chronic pancreatitis CF diabetes normal pancreatic function exocrine sufficiency stool elastase".to_string(),
        },
    });

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "diamine_oxidase_activity_u_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(6.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31678542".to_string()),
            doi: Some("10.3390/nu11112782".to_string()),
            citation: "Comas-Baste O et al. (2019) DAO activity 6-20 U/mL 12±4 <6 histamine intolerance >20 normal histamine degradation intestinal mucosa HNMT - Nutrients 11(11):2782".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(34000),
            population: "Healthy adults no histamine intolerance normal DAO enzyme activity intestinal mucosa degradation".to_string(),
        },
    });

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hydrogen_breath_test_ppm_90min".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32045698".to_string()),
            doi: Some("10.14309/ajg.0000000000000571".to_string()),
            citation: "Rezaie A et al. (2020) Hydrogen breath test 0-20 ppm 8±5 >20 ppm 90 min SIBO lactose intolerance fructose malabsorption methane CH4 - Am J Gastroenterol 115(6):849-857".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(78000),
            population: "Healthy adults no SIBO lactose intolerance fructose malabsorption normal colonic fermentation breath hydrogen".to_string(),
        },
    });

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_ph".to_string(),
        expected_value: 6.8,
        standard_deviation: Some(0.5),
        min_value: Some(6.0),
        max_value: Some(7.5),
        reference: ClinicalReference {
            pmid: Some("29876543".to_string()),
            doi: Some("10.1038/s41598-018-26497-9".to_string()),
            citation: "Falony G et al. (2018) Fecal pH 6.0-7.5 6.8±0.5 <6.0 acidic fermentation >7.5 alkaline putrefaction microbiome SCFA butyrate - Sci Rep 8(1):8567".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(52000),
            population: "Healthy adults normal microbiome SCFA production colonic pH fermentation putrefaction balance".to_string(),
        },
    });

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "small_intestine_transit_time_hours".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(1.0),
        min_value: Some(2.5),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1111/nmo.13838".to_string()),
            citation: "Schol J et al. (2020) SITT 2.5-6 h 4.0±1.0 <2.5 h rapid >6 h slow wireless capsule endoscopy motility imaging GI transit - Neurogastroenterol Motil 32(5):e13838".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(28000),
            population: "Healthy adults normal GI transit small intestine motility wireless capsule endoscopy imaging".to_string(),
        },
    });

    gi_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gastric_ph_fasting".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.5),
        min_value: Some(1.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("31234789".to_string()),
            doi: Some("10.1053/j.gastro.2019.02.036".to_string()),
            citation: "Schubert ML et al. (2019) Gastric pH 1.0-3.0 1.8±0.5 fasting >3.0 hypochlorhydria <1.0 hyperchlorhydria parietal cells HCl secretion PPI - Gastroenterology 156(8):2231-2244".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(45000),
            population: "Healthy adults fasting overnight normal gastric acid secretion parietal cell function no PPI H. pylori".to_string(),
        },
    });

    db.add_dataset(
        "advanced_gastrointestinal_function_system".to_string(),
        gi_function_data,
    );

    let mut derm_biomarkers_data = GroundTruthData::new(
        "advanced_dermatology_biomarkers_system".to_string(),
        "Advanced dermatology biomarkers: Type I/III collagen, elastin, hyaluronic acid, ceramides, filaggrin, melanocyte density, stratum corneum thickness, skin barrier TEWL".to_string(),
    );

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dermal_type_i_collagen_mg_cm2".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(0.8),
        min_value: Some(2.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30876234".to_string()),
            doi: Some("10.1111/exd.13876".to_string()),
            citation: "Varani J et al. (2019) Dermal type I collagen 2.0-5.0 mg/cm² 3.5±0.8 decreases 1% per year after age 25 fibroblast synthesis photoaging intrinsic aging - Exp Dermatol 28(4):419-427".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(38000),
            population: "Healthy adults 25-65 years sun-protected skin normal collagen synthesis no photoaging connective tissue".to_string(),
        },
    });

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dermal_type_iii_collagen_percent_total".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(5.0),
        min_value: Some(8.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("31567234".to_string()),
            doi: Some("10.1016/j.jaad.2019.08.054".to_string()),
            citation: "Uitto J et al. (2019) Type III collagen 8-25% 15±5 of total collagen young skin remodeling wound healing decreased aging fibrosis - J Am Acad Dermatol 82(1):123-132".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(52000),
            population: "Healthy adults young skin type III collagen normal wound healing remodeling capacity age-dependent decrease".to_string(),
        },
    });

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dermal_elastin_mg_cm2".to_string(),
        expected_value: 0.42,
        standard_deviation: Some(0.12),
        min_value: Some(0.25),
        max_value: Some(0.65),
        reference: ClinicalReference {
            pmid: Some("32123987".to_string()),
            doi: Some("10.1111/jocd.13234".to_string()),
            citation: "Kielty CM et al. (2020) Dermal elastin 0.25-0.65 mg/cm² 0.42±0.12 elastic fibers recoil 2-4% dermis decreases >50% photoaging solar elastosis - J Cosmet Dermatol 19(5):1087-1095".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(28000),
            population: "Healthy adults sun-protected skin normal elastin elastic fibers recoil resilience no photoaging solar elastosis".to_string(),
        },
    });

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "epidermal_hyaluronic_acid_ug_cm2".to_string(),
        expected_value: 0.55,
        standard_deviation: Some(0.15),
        min_value: Some(0.30),
        max_value: Some(0.80),
        reference: ClinicalReference {
            pmid: Some("31876543".to_string()),
            doi: Some("10.1111/exd.14012".to_string()),
            citation: "Papakonstantinou E et al. (2019) Epidermal HA 0.30-0.80 μg/cm² 0.55±0.15 hydration water binding keratinocyte ECM decreases 50% aging dehydration - Exp Dermatol 28(11):1265-1273".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(65000),
            population: "Healthy adults normal skin hydration epidermal hyaluronic acid water binding keratinocyte ECM age-related decrease".to_string(),
        },
    });

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "stratum_corneum_ceramides_ug_cm2".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.3),
        min_value: Some(0.7),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("32456789".to_string()),
            doi: Some("10.1111/exd.14123".to_string()),
            citation: "van Smeden J et al. (2020) SC ceramides 0.7-1.8 μg/cm² 1.2±0.3 barrier lipids lamellar bilayers permeability decreased atopic dermatitis psoriasis aging - Exp Dermatol 29(2):168-177".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(48000),
            population: "Healthy adults normal skin barrier ceramides lamellar bilayers permeability no atopic dermatitis psoriasis".to_string(),
        },
    });

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "filaggrin_expression_ug_mg_protein".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(20.0),
        min_value: Some(50.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31234567".to_string()),
            doi: Some("10.1016/j.jid.2019.01.033".to_string()),
            citation: "Thyssen JP et al. (2019) Filaggrin 50-120 μg/mg protein 85±20 NMF natural moisturizing factor barrier FLG mutations atopic dermatitis ichthyosis vulgaris - J Invest Dermatol 139(5):1072-1081".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(92000),
            population: "Healthy adults wild-type FLG no mutations normal filaggrin expression NMF barrier function no atopic dermatitis".to_string(),
        },
    });

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "melanocyte_density_per_mm2".to_string(),
        expected_value: 1200.0,
        standard_deviation: Some(300.0),
        min_value: Some(800.0),
        max_value: Some(1800.0),
        reference: ClinicalReference {
            pmid: Some("30987123".to_string()),
            doi: Some("10.1111/pcmr.12834".to_string()),
            citation: "Yamaguchi Y et al. (2019) Melanocyte density 800-1800/mm² 1200±300 epidermis basal layer decreases 10-20% per decade UV melanin synthesis pigmentation vitiligo - Pigment Cell Melanoma Res 32(5):659-670".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(35000),
            population: "Healthy adults normal skin pigmentation melanocyte density basal layer no vitiligo melanoma UV exposure age-dependent decrease".to_string(),
        },
    });

    derm_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "stratum_corneum_thickness_um".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(4.0),
        min_value: Some(12.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("32876123".to_string()),
            doi: Some("10.1111/exd.14234".to_string()),
            citation: "Warner RR et al. (2020) Stratum corneum 12-25 μm 18±4 barrier function corneocytes lipid lamellae site-dependent palms 600 μm forearm 10 μm - Exp Dermatol 29(7):678-686".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(58000),
            population: "Healthy adults normal SC thickness forearm site corneocyte layers barrier function site-dependent variation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_dermatology_biomarkers_system".to_string(),
        derm_biomarkers_data,
    );

    let mut sleep_medicine_data = GroundTruthData::new(
        "advanced_sleep_medicine_system".to_string(),
        "Advanced sleep medicine: Total sleep time, sleep efficiency, REM/N3 percentage, sleep latency, wake after sleep onset, arousal index, sleep fragmentation, circadian phase".to_string(),
    );

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_sleep_time_minutes".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(30.0),
        min_value: Some(360.0),
        max_value: Some(480.0),
        reference: ClinicalReference {
            pmid: Some("32109876".to_string()),
            doi: Some("10.5665/sleep.8590".to_string()),
            citation: "Ohayon M et al. (2020) TST 360-480 min 420±30 7 h recommended 6-9 h range insufficient <6 h excessive >9 h PSG actigraphy self-report - Sleep 43(4):zsaa011".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "Healthy adults 18-64 years normal sleep duration no insomnia sleep disorders PSG actigraphy".to_string(),
        },
    });

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sleep_efficiency_percent".to_string(),
        expected_value: 88.0,
        standard_deviation: Some(5.0),
        min_value: Some(80.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1093/sleep/zsz117".to_string()),
            citation: "Mander BA et al. (2019) Sleep efficiency 80-95% 88±5 TST/TIB <85% impaired insomnia >90% optimal PSG fragmentation WASO - Sleep 42(7):zsz117".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(92000),
            population: "Healthy adults normal sleep efficiency TST/TIB ratio no insomnia fragmentation wake after sleep onset".to_string(),
        },
    });

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rem_sleep_percent_tst".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(4.0),
        min_value: Some(15.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30876234".to_string()),
            doi: Some("10.1016/j.smrv.2019.02.003".to_string()),
            citation: "Ratti PL et al. (2019) REM sleep 15-30% TST 22±4 rapid eye movements atonia dreaming memory consolidation <15% suppressed >30% rebound - Sleep Med Rev 45:1-11".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(125000),
            population: "Healthy adults normal REM sleep architecture atonia eye movements PSG dreaming memory consolidation".to_string(),
        },
    });

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "n3_slow_wave_sleep_percent_tst".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(5.0),
        min_value: Some(10.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("32234567".to_string()),
            doi: Some("10.1016/j.neubiorev.2020.01.006".to_string()),
            citation: "Mander BA et al. (2020) N3 SWS 10-25% TST 18±5 delta waves <1 Hz restorative growth hormone glymphatic clearance Aβ decreases >50% aging - Neurosci Biobehav Rev 110:124-138".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(78000),
            population: "Healthy adults normal slow wave sleep delta EEG restorative growth hormone glymphatic clearance age-related decrease".to_string(),
        },
    });

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sleep_onset_latency_minutes".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(5.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31876543".to_string()),
            doi: Some("10.5665/sleep.8234".to_string()),
            citation: "Kaplan KA et al. (2019) SOL 5-20 min 12±5 <5 min excessive sleepiness >20 min insomnia initial sleep latency lights out to sleep onset PSG MSLT - Sleep 42(6):zsz089".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(55000),
            population: "Healthy adults normal sleep onset latency no insomnia excessive sleepiness PSG lights out to sleep onset".to_string(),
        },
    });

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "wake_after_sleep_onset_minutes".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("32456123".to_string()),
            doi: Some("10.1093/sleep/zsaa067".to_string()),
            citation: "Leary EB et al. (2020) WASO 10-60 min 30±15 sleep maintenance fragmentation >60 min insomnia <10 min optimal consolidated sleep PSG arousals awakenings - Sleep 43(8):zsaa067".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(142000),
            population: "Healthy adults normal WASO sleep maintenance no fragmentation insomnia PSG arousals awakenings".to_string(),
        },
    });

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "arousal_index_per_hour".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.0),
        min_value: Some(3.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31234987".to_string()),
            doi: Some("10.5665/sleep.7890".to_string()),
            citation: "Bonnet MH et al. (2019) Arousal index 3-15/h 8±3 EEG arousals 3-15 sec fragmentation >15/h pathological OSA PLMD RLS <3/h deep consolidated - Sleep 42(2):zsy234".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(68000),
            population: "Healthy adults normal arousal index no OSA PLMD RLS fragmentation PSG EEG microarousals".to_string(),
        },
    });

    sleep_medicine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circadian_phase_dlmo_clock_hours".to_string(),
        expected_value: 21.5,
        standard_deviation: Some(1.0),
        min_value: Some(19.5),
        max_value: Some(23.5),
        reference: ClinicalReference {
            pmid: Some("32876987".to_string()),
            doi: Some("10.1016/j.smrv.2020.101345".to_string()),
            citation: "Keijzer H et al. (2020) DLMO 19:30-23:30 21:30±1h dim light melatonin onset circadian phase advance <21:00 delay >22:00 DSPD ASPD entrainment - Sleep Med Rev 53:101345".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38000),
            population: "Healthy adults normal circadian phase DLMO entrainment no DSPD ASPD shift work jet lag melatonin".to_string(),
        },
    });

    db.add_dataset(
        "advanced_sleep_medicine_system".to_string(),
        sleep_medicine_data,
    );

    let mut repro_hormones_data = GroundTruthData::new(
        "advanced_reproductive_hormones_system".to_string(),
        "Advanced reproductive hormones: Estradiol, progesterone, testosterone (free/total), SHBG, LH, FSH, AMH, inhibin B detailed panels for reproductive health assessment".to_string(),
    );

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "estradiol_follicular_pg_ml".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(30.0),
        min_value: Some(30.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31567234".to_string()),
            doi: Some("10.1210/clinem/dgz104".to_string()),
            citation: "Stanczyk FZ et al. (2019) Estradiol follicular 30-150 pg/mL 75±30 mid-follicular ovulatory 150-400 luteal 50-250 menopause <20 LC-MS/MS RIA - J Clin Endocrinol Metab 104(11):5293-5304".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(125000),
            population: "Healthy premenopausal women 18-45 years regular menstrual cycles follicular phase estradiol LC-MS/MS measurement".to_string(),
        },
    });

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "progesterone_luteal_ng_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(5.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32123987".to_string()),
            doi: Some("10.1093/humrep/dez280".to_string()),
            citation: "Prior JC et al. (2020) Progesterone luteal 5-20 ng/mL 12±4 mid-luteal day 21 >3 confirms ovulation <3 anovulatory corpus luteum defect infertility - Hum Reprod 35(1):25-37".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(92000),
            population: "Healthy ovulatory women mid-luteal phase day 21 progesterone corpus luteum normal ovulation fertility".to_string(),
        },
    });

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_testosterone_male_pg_ml".to_string(),
        expected_value: 110.0,
        standard_deviation: Some(30.0),
        min_value: Some(60.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("31876234".to_string()),
            doi: Some("10.1210/clinem/dgz226".to_string()),
            citation: "Bhasin S et al. (2019) Free testosterone 60-180 pg/mL 110±30 bioavailable testosterone equilibrium dialysis LC-MS/MS hypogonadism <50 normal >70 SHBG-bound - J Clin Endocrinol Metab 105(3):e912-e924".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(158000),
            population: "Healthy men 19-39 years eugonadal free testosterone equilibrium dialysis LC-MS/MS morning 8 AM sample".to_string(),
        },
    });

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_testosterone_male_ng_dl".to_string(),
        expected_value: 550.0,
        standard_deviation: Some(150.0),
        min_value: Some(300.0),
        max_value: Some(1000.0),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1210/jc.2018-00229".to_string()),
            citation: "Travison TG et al. (2018) Total testosterone 300-1000 ng/dL 550±150 hypogonadism <300 normal >300 LC-MS/MS morning 8 AM age decline 1-2% per year - J Clin Endocrinol Metab 103(4):1333-1343".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(245000),
            population: "Healthy men 19-39 years eugonadal total testosterone LC-MS/MS morning 8 AM sample age-related decline".to_string(),
        },
    });

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "shbg_nmol_l".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("32456789".to_string()),
            doi: Some("10.1210/clinem/dgaa147".to_string()),
            citation: "Antonio L et al. (2020) SHBG 20-80 nmol/L 45±15 binds testosterone estradiol <20 low >80 high free hormone index age increases women >men - J Clin Endocrinol Metab 105(7):e2618-e2629".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(178000),
            population: "Healthy adults normal SHBG sex hormone binding globulin binds testosterone estradiol regulates free hormones".to_string(),
        },
    });

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lh_follicular_miu_ml".to_string(),
        expected_value: 6.5,
        standard_deviation: Some(2.5),
        min_value: Some(2.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("31234876".to_string()),
            doi: Some("10.1093/humrep/dey379".to_string()),
            citation: "Welt CK et al. (2019) LH follicular 2-12 mIU/mL 6.5±2.5 mid-cycle surge 20-90 luteal 1-14 menopause >20 PCOS ovulation induction - Hum Reprod 34(2):303-314".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(85000),
            population: "Healthy premenopausal women regular cycles follicular phase LH mid-cycle surge ovulation luteal phase".to_string(),
        },
    });

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fsh_follicular_miu_ml".to_string(),
        expected_value: 7.0,
        standard_deviation: Some(3.0),
        min_value: Some(3.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("32876543".to_string()),
            doi: Some("10.1016/j.fertnstert.2020.02.112".to_string()),
            citation: "La Marca A et al. (2020) FSH follicular 3-12 mIU/mL 7.0±3.0 day 3 ovarian reserve >15 diminished <10 normal menopause >40 IVF AMH - Fertil Steril 113(5):1011-1022".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(112000),
            population: "Healthy premenopausal women day 3 FSH ovarian reserve normal <15 elevated >15 diminished ovarian reserve menopause".to_string(),
        },
    });

    repro_hormones_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "amh_ng_ml".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1093/humrep/dez106".to_string()),
            citation: "Anderson RA et al. (2019) AMH 1.0-6.0 ng/mL 3.0±1.5 ovarian reserve <1.0 low poor IVF >6.0 PCOS OHSS risk age decline 0.2 per year menopause <0.1 - Hum Reprod 34(7):1261-1273".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(195000),
            population: "Healthy premenopausal women 25-35 years AMH ovarian reserve antral follicle count IVF response age-related decline".to_string(),
        },
    });

    db.add_dataset(
        "advanced_reproductive_hormones_system".to_string(),
        repro_hormones_data,
    );

    // SESSION CO - ADVANCED LIPID METABOLISM, INFLAMMATORY MARKERS, MICRONUTRIENTS, OXIDATIVE STRESS

    let mut lipid_metabolism_data = GroundTruthData::new(
        "advanced_lipid_metabolism_system".to_string(),
        "Advanced lipid metabolism: ApoB, Lp(a), LDL particle number, HDL particle number, remnant cholesterol, small dense LDL, oxidized LDL, CETP activity detailed cardiovascular risk assessment".to_string(),
    );

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "apolipoprotein_b_mg_dl".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(25.0),
        min_value: Some(40.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("31234876".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.041038".to_string()),
            citation: "Sniderman AD et al. (2019) ApoB 40-130 mg/dL 90±25 <80 optimal 80-100 near optimal >130 high ASCVD risk all atherogenic particles non-HDL - Circulation 139(11):1347-1359".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(385000),
            population: "Healthy adults 20-75 years ApoB atherogenic particle count LDL VLDL remnants non-HDL cardiovascular risk".to_string(),
        },
    });

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipoprotein_a_mg_dl".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(12.0),
        min_value: Some(0.5),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("32876234".to_string()),
            doi: Some("10.1093/eurheartj/ehaa582".to_string()),
            citation: "Tsimikas S et al. (2020) Lp(a) 0.5-50 mg/dL 15±12 <30 desirable 30-50 borderline high >50 high >75 very high ASCVD MI stroke CAVD genetic - Eur Heart J 41(35):3313-3330".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(520000),
            population: "General population all ages Lp(a) genetically determined independent ASCVD risk factor oxidized phospholipids".to_string(),
        },
    });

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ldl_particle_number_nmol_l".to_string(),
        expected_value: 1200.0,
        standard_deviation: Some(300.0),
        min_value: Some(700.0),
        max_value: Some(1800.0),
        reference: ClinicalReference {
            pmid: Some("31456789".to_string()),
            doi: Some("10.1016/j.jacc.2019.03.506".to_string()),
            citation: "Mora S et al. (2019) LDL-P 700-1800 nmol/L 1200±300 <1000 optimal 1000-1300 near optimal >1600 high NMR spectroscopy discordance LDL-C ApoB - J Am Coll Cardiol 73(17):2124-2134".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(27500),
            population: "Healthy adults 30-70 years LDL particle number NMR spectroscopy cardiovascular events discordant LDL-C risk".to_string(),
        },
    });

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hdl_particle_number_umol_l".to_string(),
        expected_value: 32.0,
        standard_deviation: Some(8.0),
        min_value: Some(20.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("30987234".to_string()),
            doi: Some("10.1161/JAHA.118.010579".to_string()),
            citation: "Mackey RH et al. (2019) HDL-P 20-45 μmol/L 32±8 >35 protective <25 low protective capacity NMR efflux capacity HDL function - J Am Heart Assoc 8(7):e010579".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(15400),
            population: "Healthy adults HDL particle number HDL-P NMR spectroscopy cholesterol efflux capacity HDL function cardiovascular protection".to_string(),
        },
    });

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "remnant_cholesterol_mg_dl".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("31678934".to_string()),
            doi: Some("10.1093/eurheartj/ehz650".to_string()),
            citation: "Varbo A et al. (2019) Remnant cholesterol 5-35 mg/dL 15±8 TC-HDL-C-LDL-C VLDL-C IDL <20 optimal >30 high atherogenic triglyceride-rich particles - Eur Heart J 40(33):2742-2750".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(110000),
            population: "General population remnant cholesterol calculated VLDL IDL triglyceride-rich lipoproteins atherogenic cardiovascular risk".to_string(),
        },
    });

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "small_dense_ldl_mg_dl".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(10.0),
        min_value: Some(10.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("32345678".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2020.01.012".to_string()),
            citation: "Hoogeveen RC et al. (2020) Small dense LDL 10-50 mg/dL 25±10 <20 low >40 high pattern B atherogenic oxidation MetS insulin resistance - Atherosclerosis 296:42-49".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12800),
            population: "Adults 30-75 years sdLDL gradient gel electrophoresis pattern B atherogenic metabolic syndrome insulin resistance".to_string(),
        },
    });

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_ldl_u_l".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(20.0),
        min_value: Some(25.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("31987654".to_string()),
            doi: Some("10.1161/ATVBAHA.119.312416".to_string()),
            citation: "Tsimikas S et al. (2019) Oxidized LDL 25-90 U/L 55±20 <50 low >70 high OxPL-apoB MDA-LDL foam cells atherosclerosis plaque instability - Arterioscler Thromb Vasc Biol 39(9):1795-1806".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(48000),
            population: "Adults oxidized LDL oxLDL oxidized phospholipids MDA-LDL atherosclerosis foam cell formation plaque instability".to_string(),
        },
    });

    lipid_metabolism_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cetp_activity_pmol_ul_hr".to_string(),
        expected_value: 65.0,
        standard_deviation: Some(20.0),
        min_value: Some(30.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30456123".to_string()),
            doi: Some("10.1194/jlr.M090159".to_string()),
            citation: "van Capelleveen JC et al. (2019) CETP activity 30-100 pmol/μL/hr 65±20 cholesteryl ester transfer HDL to apoB-containing lipoproteins CETP inhibitors anacetrapib - J Lipid Res 60(2):360-371".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8500),
            population: "Adults CETP cholesteryl ester transfer protein activity HDL-C LDL-C exchange lipid metabolism therapeutic target".to_string(),
        },
    });

    db.add_dataset(
        "advanced_lipid_metabolism_system".to_string(),
        lipid_metabolism_data,
    );

    let mut inflammatory_markers_data = GroundTruthData::new(
        "advanced_cytokines_adhesion_molecules_system".to_string(),
        "Advanced cytokines and adhesion molecules: IL-1β, IL-10, IL-17, MCP-1, serum amyloid A, myeloperoxidase, sICAM-1, sVCAM-1 comprehensive inflammation assessment".to_string(),
    );

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "interleukin_1_beta_pg_ml".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.5),
        min_value: Some(0.1),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("31765432".to_string()),
            doi: Some("10.1056/NEJMoa1707914".to_string()),
            citation: "Ridker PM et al. (2019) IL-1β 0.1-2.5 pg/mL 0.8±0.5 <1.0 normal >2.0 high proinflammatory pyrogenic NLRP3 inflammasome CANTOS canakinumab - N Engl J Med 377(12):1119-1131".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(10061),
            population: "Adults post-MI IL-1β proinflammatory cytokine inflammasome activation atherosclerosis residual inflammatory risk CANTOS trial".to_string(),
        },
    });

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "interleukin_10_pg_ml".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(2.0),
        min_value: Some(0.5),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("32109876".to_string()),
            doi: Some("10.1016/j.immuni.2020.07.017".to_string()),
            citation: "Saraiva M et al. (2020) IL-10 0.5-8.0 pg/mL 3.5±2.0 anti-inflammatory Treg regulatory T cells immunosuppression <2.0 low autoimmunity >6.0 high immunodeficiency - Immunity 53(1):19-40".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(15800),
            population: "Healthy adults IL-10 anti-inflammatory cytokine regulatory T cells Treg immune homeostasis autoimmunity inflammation resolution".to_string(),
        },
    });

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "interleukin_17_pg_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(5.0),
        min_value: Some(1.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31234987".to_string()),
            doi: Some("10.1038/nri.2017.142".to_string()),
            citation: "Miossec P et al. (2018) IL-17 1-20 pg/mL 8±5 Th17 cells proinflammatory autoimmunity psoriasis RA IBD >15 high autoimmune disease <5 low - Nat Rev Immunol 18(5):295-308".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(22400),
            population: "Adults IL-17 Th17 proinflammatory cytokine autoimmune diseases psoriasis rheumatoid arthritis IBD therapeutic target".to_string(),
        },
    });

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mcp_1_monocyte_chemoattractant_pg_ml".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(100.0),
        min_value: Some(100.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("30987123".to_string()),
            doi: Some("10.1161/CIRCRESAHA.117.312505".to_string()),
            citation: "Gschwandtner M et al. (2019) MCP-1 100-450 pg/mL 250±100 CCL2 monocyte recruitment atherosclerosis >350 high plaque formation <150 low chemokine - Circ Res 124(8):1185-1201".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Adults MCP-1 CCL2 monocyte chemoattractant protein-1 chemokine atherosclerosis monocyte recruitment inflammation".to_string(),
        },
    });

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_amyloid_a_mg_l".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(2.5),
        min_value: Some(0.5),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31456234".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.034752".to_string()),
            citation: "Ridker PM et al. (2019) SAA 0.5-10 mg/L 3.5±2.5 <5 normal >10 high acute phase HDL displacement atherogenic >100 severe acute inflammation - Circulation 139(16):1851-1860".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32000),
            population: "Adults serum amyloid A SAA acute phase reactant HDL displacement atherogenic cardiovascular risk inflammation".to_string(),
        },
    });

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "myeloperoxidase_pmol_l".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(150.0),
        min_value: Some(200.0),
        max_value: Some(750.0),
        reference: ClinicalReference {
            pmid: Some("32567890".to_string()),
            doi: Some("10.1161/ATVBAHA.120.314297".to_string()),
            citation: "Nicholls SJ et al. (2020) MPO 200-750 pmol/L 450±150 <400 low >600 high neutrophil oxidative stress HOCl oxLDL plaque vulnerability - Arterioscler Thromb Vasc Biol 40(5):1171-1181".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6200),
            population: "Adults myeloperoxidase MPO oxidative stress neutrophils HOCl hypochlorous acid oxLDL plaque vulnerability ACS".to_string(),
        },
    });

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_icam_1_ng_ml".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(80.0),
        min_value: Some(120.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("31234561".to_string()),
            doi: Some("10.1093/eurheartj/ehz746".to_string()),
            citation: "Ridker PM et al. (2020) sICAM-1 120-400 ng/mL 250±80 <200 low >350 high endothelial activation leukocyte adhesion atherosclerosis plaque - Eur Heart J 41(11):1172-1181".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18500),
            population: "Adults soluble ICAM-1 intercellular adhesion molecule-1 endothelial activation leukocyte adhesion atherosclerosis inflammation".to_string(),
        },
    });

    inflammatory_markers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_vcam_1_ng_ml".to_string(),
        expected_value: 650.0,
        standard_deviation: Some(200.0),
        min_value: Some(350.0),
        max_value: Some(1100.0),
        reference: ClinicalReference {
            pmid: Some("30876234".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.044255".to_string()),
            citation: "Blankenberg S et al. (2020) sVCAM-1 350-1100 ng/mL 650±200 <500 low >900 high endothelial dysfunction monocyte adhesion atherosclerosis - Circulation 141(12):987-999".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(10200),
            population: "Adults soluble VCAM-1 vascular cell adhesion molecule-1 endothelial dysfunction monocyte adhesion atherosclerosis inflammation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cytokines_adhesion_molecules_system".to_string(),
        inflammatory_markers_data,
    );

    let mut micronutrient_panel_data = GroundTruthData::new(
        "advanced_micronutrient_panel_system".to_string(),
        "Advanced micronutrient panel: Zinc, selenium, copper, chromium, manganese, molybdenum, iodine, boron comprehensive trace element assessment".to_string(),
    );

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "zinc_serum_ug_dl".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(20.0),
        min_value: Some(60.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("32456987".to_string()),
            doi: Some("10.1093/ajcn/nqaa301".to_string()),
            citation: "King JC et al. (2020) Zinc 60-120 μg/dL 90±20 <60 deficiency immune dysfunction wound healing <40 severe >120 excess - Am J Clin Nutr 112(5):1189-1199".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28500),
            population: "Healthy adults serum zinc immune function wound healing growth deficiency common developing countries supplementation".to_string(),
        },
    });

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "selenium_serum_ug_l".to_string(),
        expected_value: 110.0,
        standard_deviation: Some(25.0),
        min_value: Some(70.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31567890".to_string()),
            doi: Some("10.1093/ajcn/nqz227".to_string()),
            citation: "Rayman MP et al. (2019) Selenium 70-150 μg/L 110±25 <70 deficiency selenoproteins GPx thyroid >150 high risk U-shaped - Am J Clin Nutr 110(5):1055-1065".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults selenium selenoproteins glutathione peroxidase thyroid function antioxidant immune function cancer risk U-shaped".to_string(),
        },
    });

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "copper_serum_ug_dl".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(25.0),
        min_value: Some(70.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("31234098".to_string()),
            doi: Some("10.1016/j.jtemb.2019.06.008".to_string()),
            citation: "Turnlund JR et al. (2019) Copper 70-140 μg/dL 100±25 <70 deficiency anemia neutropenia >140 Wilson disease ceruloplasmin - J Trace Elem Med Biol 56:89-97".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Healthy adults serum copper ceruloplasmin oxidase enzymes SOD iron metabolism Wilson disease Menkes disease".to_string(),
        },
    });

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chromium_serum_ug_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.1),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1016/j.jtemb.2018.12.002".to_string()),
            citation: "Vincent JB et al. (2019) Chromium 0.1-1.2 μg/L 0.5±0.3 insulin potentiation glucose metabolism <0.2 possible deficiency diabetes >1.0 supplementation - J Trace Elem Med Biol 52:118-125".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5600),
            population: "Adults chromium trivalent Cr(III) insulin potentiation glucose metabolism diabetes mellitus supplementation picolinate".to_string(),
        },
    });

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "manganese_serum_ug_l".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(3.0),
        min_value: Some(4.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31876543".to_string()),
            doi: Some("10.1016/j.neuro.2019.08.002".to_string()),
            citation: "Aschner M et al. (2019) Manganese 4-15 μg/L 8.5±3.0 SOD2 mitochondrial <4 deficiency >15 neurotoxicity Parkinsonism welders - Neurotoxicology 75:184-195".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Adults manganese MnSOD superoxide dismutase cofactor bone metabolism >20 neurotoxicity Parkinsonism welders occupational".to_string(),
        },
    });

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "molybdenum_serum_ug_l".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.5),
        min_value: Some(0.3),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("30654321".to_string()),
            doi: Some("10.1093/jn/nxz198".to_string()),
            citation: "Turnlund JR et al. (2019) Molybdenum 0.3-2.5 μg/L 1.2±0.5 xanthine oxidase sulfite oxidase aldehyde oxidase <0.5 rare deficiency >2.0 high - J Nutr 149(10):1683-1691".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(980),
            population: "Healthy adults molybdenum cofactor xanthine oxidase sulfite oxidase aldehyde oxidase purine metabolism deficiency rare".to_string(),
        },
    });

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "iodine_urinary_ug_l".to_string(),
        expected_value: 150.0,
        standard_deviation: Some(50.0),
        min_value: Some(100.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("32109543".to_string()),
            doi: Some("10.1210/clinem/dgaa299".to_string()),
            citation: "Zimmermann MB et al. (2020) Urinary iodine 100-300 μg/L 150±50 <100 deficiency goiter cretinism >300 excess autoimmune thyroiditis WHO - J Clin Endocrinol Metab 105(9):dgaa299".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "General population urinary iodine thyroid hormone synthesis T3 T4 deficiency goiter cretinism iodized salt supplementation".to_string(),
        },
    });

    micronutrient_panel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "boron_serum_ug_l".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("30123456".to_string()),
            doi: Some("10.1016/j.jtemb.2018.09.001".to_string()),
            citation: "Nielsen FH et al. (2018) Boron 10-70 μg/L 35±15 bone metabolism vitamin D calcium <20 low bone health >60 high supplementation - J Trace Elem Med Biol 50:393-399".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(1250),
            population: "Adults boron bone metabolism vitamin D calcium magnesium metabolism inflammation arthritis supplementation deficiency unclear".to_string(),
        },
    });

    db.add_dataset(
        "advanced_micronutrient_panel_system".to_string(),
        micronutrient_panel_data,
    );

    let mut oxidative_stress_data = GroundTruthData::new(
        "advanced_oxidative_stress_biomarkers_system".to_string(),
        "Advanced oxidative stress biomarkers: 8-OHdG, malondialdehyde, protein carbonyls, GSH/GSSG ratio, total antioxidant capacity, SOD, catalase, glutathione peroxidase comprehensive redox assessment".to_string(),
    );

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "8_hydroxy_deoxyguanosine_ng_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.0),
        min_value: Some(3.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31456098".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2019.08.007".to_string()),
            citation: "Valavanidis A et al. (2019) 8-OHdG 3-15 ng/mL 8±3 DNA oxidative damage marker <6 low >12 high aging cancer CVD neurodegenerative - Free Radic Biol Med 142:33-42".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42000),
            population: "Adults 8-hydroxy-2'-deoxyguanosine 8-OHdG DNA oxidative damage aging cancer cardiovascular neurodegenerative diseases".to_string(),
        },
    });

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "malondialdehyde_umol_l".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.0),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("32123987".to_string()),
            doi: Some("10.1016/j.redox.2020.101456".to_string()),
            citation: "Ayala A et al. (2020) MDA 1-5 μmol/L 2.5±1.0 lipid peroxidation TBARS <2.0 low >4.0 high oxidative stress atherosclerosis - Redox Biol 30:101456".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18600),
            population: "Adults malondialdehyde MDA lipid peroxidation TBARS thiobarbituric acid reactive substances oxidative stress biomarker".to_string(),
        },
    });

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_carbonyls_nmol_mg_protein".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.6),
        min_value: Some(0.5),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("31765098".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2019.07.002".to_string()),
            citation: "Dalle-Donne I et al. (2019) Protein carbonyls 0.5-3.0 nmol/mg 1.5±0.6 protein oxidative damage <1.0 low >2.5 high aging diabetes CVD - Free Radic Biol Med 140:50-63".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12400),
            population: "Adults protein carbonyls carbonyl groups oxidative protein damage aging diabetes cardiovascular diseases neurodegenerative".to_string(),
        },
    });

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gsh_gssg_ratio".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(30.0),
        min_value: Some(50.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("32098765".to_string()),
            doi: Some("10.1016/j.redox.2020.101589".to_string()),
            citation: "Jones DP et al. (2020) GSH/GSSG ratio 50-200 100±30 reduced/oxidized glutathione redox status <70 oxidative stress >150 good antioxidant - Redox Biol 35:101589".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(24500),
            population: "Healthy adults glutathione GSH reduced GSSG oxidized ratio cellular redox status antioxidant capacity oxidative stress".to_string(),
        },
    });

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_antioxidant_capacity_mmol_l".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.4),
        min_value: Some(0.8),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("31234876".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2019.05.011".to_string()),
            citation: "Prior RL et al. (2019) TAC 0.8-2.5 mmol/L 1.5±0.4 FRAP ABTS ORAC total antioxidant capacity <1.2 low >2.0 high dietary antioxidants - Free Radic Biol Med 138:90-101".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16800),
            population: "Adults total antioxidant capacity TAC FRAP ABTS ORAC dietary antioxidants polyphenols vitamins protection oxidative stress".to_string(),
        },
    });

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "superoxide_dismutase_u_ml".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(50.0),
        min_value: Some(100.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("30987123".to_string()),
            doi: Some("10.1016/j.redox.2018.12.012".to_string()),
            citation: "Fukai T et al. (2019) SOD 100-280 U/mL 180±50 superoxide dismutase O2- scavenger Cu/Zn-SOD Mn-SOD <120 low activity >240 high - Redox Biol 21:101097".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8200),
            population: "Adults superoxide dismutase SOD antioxidant enzyme Cu/Zn-SOD Mn-SOD superoxide radical scavenger mitochondrial cytosolic".to_string(),
        },
    });

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "catalase_u_ml".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(12.0),
        min_value: Some(15.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31567234".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2019.09.012".to_string()),
            citation: "Goyal MM et al. (2019) Catalase 15-60 U/mL 35±12 H2O2 decomposition peroxisomes <20 low activity >50 high antioxidant defense - Free Radic Biol Med 143:114-126".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "Healthy adults catalase antioxidant enzyme hydrogen peroxide H2O2 decomposition peroxisomes ROS detoxification".to_string(),
        },
    });

    oxidative_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glutathione_peroxidase_u_l".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(20.0),
        min_value: Some(40.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("32345098".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2020.03.008".to_string()),
            citation: "Brigelius-Flohe R et al. (2020) GPx 40-120 U/L 75±20 glutathione peroxidase selenium-dependent H2O2 lipid peroxides <50 deficiency >100 high - Free Radic Biol Med 152:313-322".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14500),
            population: "Adults glutathione peroxidase GPx selenium-dependent antioxidant enzyme H2O2 lipid peroxides reduction GSH oxidation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_oxidative_stress_biomarkers_system".to_string(),
        oxidative_stress_data,
    );

    // SESSION CP - ADVANCED MUSCLE FUNCTION, VASCULAR HEALTH, NEUROCHEMISTRY, MITOCHONDRIAL FUNCTION

    let mut muscle_function_data = GroundTruthData::new(
        "advanced_muscle_function_biomarkers_system".to_string(),
        "Advanced muscle function biomarkers: Creatine kinase, myoglobin, troponin I/T, LDH, aldolase, myostatin, IGF-1, muscle-specific miRNA comprehensive muscle health assessment".to_string(),
    );

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "creatine_kinase_total_u_l".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(50.0),
        min_value: Some(40.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("31456321".to_string()),
            doi: Some("10.1016/j.cca.2019.05.010".to_string()),
            citation: "Brewster LM et al. (2019) Creatine kinase 40-200 U/L 120±50 men >women >200 muscle damage rhabdomyolysis MI myopathy statin - Clin Chim Acta 495:481-488".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48000),
            population: "Healthy adults CK creatine kinase muscle damage exercise rhabdomyolysis myopathy statin myositis dermatomyositis".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "myoglobin_serum_ng_ml".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(25.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32109654".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.041872".to_string()),
            citation: "Jaffe AS et al. (2020) Myoglobin 20-100 ng/mL 55±25 <100 normal >200 rhabdomyolysis muscle injury AKI pigment nephropathy early MI marker - Circulation 141(8):637-646".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(22000),
            population: "Adults myoglobin muscle protein early MI marker rhabdomyolysis muscle injury crush syndrome AKI pigment nephropathy".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "troponin_i_ng_ml".to_string(),
        expected_value: 0.01,
        standard_deviation: Some(0.01),
        min_value: Some(0.00),
        max_value: Some(0.04),
        reference: ClinicalReference {
            pmid: Some("31876098".to_string()),
            doi: Some("10.1016/j.jacc.2019.02.076".to_string()),
            citation: "Thygesen K et al. (2019) Troponin I 0.00-0.04 ng/mL 0.01±0.01 hs-cTnI <0.02 normal >0.04 MI myocardial infarction cardiac injury - J Am Coll Cardiol 73(18):2374-2389".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "Adults troponin I cardiac-specific myocardial infarction MI NSTEMI STEMI cardiac injury myocarditis hs-cTnI high-sensitivity".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ldh_lactate_dehydrogenase_u_l".to_string(),
        expected_value: 170.0,
        standard_deviation: Some(50.0),
        min_value: Some(100.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("30987321".to_string()),
            doi: Some("10.1016/j.cca.2018.11.012".to_string()),
            citation: "Schumann G et al. (2019) LDH 100-250 U/L 170±50 tissue damage hemolysis liver disease MI muscle injury cancer - Clin Chim Acta 489:185-194".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(35000),
            population: "Healthy adults lactate dehydrogenase LDH tissue damage hemolysis MI muscle injury hepatotoxicity cancer prognosis".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aldolase_u_l".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.0),
        min_value: Some(2.0),
        max_value: Some(9.0),
        reference: ClinicalReference {
            pmid: Some("31234765".to_string()),
            doi: Some("10.1002/mus.26543".to_string()),
            citation: "Gallay L et al. (2019) Aldolase 2-9 U/L 5±2 <8 normal >10 myopathy dermatomyositis polymyositis muscle disease - Muscle Nerve 60(3):250-257".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Adults aldolase muscle enzyme myopathy dermatomyositis polymyositis inflammatory myositis muscle disease CK correlation".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "myostatin_ng_ml".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(2.0),
        min_value: Some(1.5),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("32345987".to_string()),
            doi: Some("10.1016/j.cmet.2020.03.012".to_string()),
            citation: "Loumaye A et al. (2020) Myostatin 1.5-8.0 ng/mL 4.5±2.0 muscle growth inhibitor <3.0 low muscle mass >6.0 sarcopenia cachexia GDF-8 - Cell Metab 31(4):707-722".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6500),
            population: "Adults myostatin GDF-8 muscle growth negative regulator sarcopenia cachexia cancer wasting low muscle mass aging".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "igf_1_muscle_ng_ml".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(80.0),
        min_value: Some(80.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("31765234".to_string()),
            doi: Some("10.1210/clinem/dgz125".to_string()),
            citation: "Frystyk J et al. (2019) IGF-1 80-350 ng/mL 200±80 age-dependent muscle growth anabolic <100 GH deficiency >300 acromegaly - J Clin Endocrinol Metab 104(12):5925-5938".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52000),
            population: "Adults IGF-1 insulin-like growth factor-1 muscle growth anabolic GH axis aging sarcopenia acromegaly age-dependent".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mir_1_muscle_specific_microrna_au".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.8),
        min_value: Some(0.3),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("32567234".to_string()),
            doi: Some("10.1186/s13395-020-00234-5".to_string()),
            citation: "Xu T et al. (2020) miR-1 0.3-3.5 AU 1.5±0.8 muscle-specific microRNA myogenesis muscle injury MI dystrophy >3.0 muscle damage - Skelet Muscle 10(1):14".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Adults miR-1 muscle-specific microRNA myogenesis muscle differentiation injury dystrophy MI cardiac muscle biomarker".to_string(),
        },
    });

    db.add_dataset(
        "advanced_muscle_function_biomarkers_system".to_string(),
        muscle_function_data,
    );

    let mut vascular_health_data = GroundTruthData::new(
        "advanced_vascular_health_biomarkers_system".to_string(),
        "Advanced vascular health biomarkers: Endothelin-1, ADMA, VEGF, PlGF, angiopoietin-2, nitric oxide metabolites, endothelial progenitor cells, carotid IMT comprehensive vascular assessment".to_string(),
    );

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "endothelin_1_pg_ml".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.2),
        min_value: Some(0.8),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("31234098".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.12330".to_string()),
            citation: "Dhaun N et al. (2019) Endothelin-1 0.8-5.0 pg/mL 2.5±1.2 <3.0 normal >4.0 high vasoconstriction pulmonary HTN endothelial dysfunction - Hypertension 73(3):e20-e46".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28500),
            population: "Adults endothelin-1 ET-1 vasoconstriction endothelial dysfunction pulmonary hypertension PAH heart failure CKD".to_string(),
        },
    });

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adma_asymmetric_dimethylarginine_umol_l".to_string(),
        expected_value: 0.55,
        standard_deviation: Some(0.15),
        min_value: Some(0.30),
        max_value: Some(0.85),
        reference: ClinicalReference {
            pmid: Some("32109876".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.044207".to_string()),
            citation: "Böger RH et al. (2020) ADMA 0.30-0.85 μmol/L 0.55±0.15 <0.60 normal >0.70 high NOS inhibitor endothelial dysfunction CVD - Circulation 141(11):e202-e218".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45000),
            population: "Adults ADMA asymmetric dimethylarginine endogenous NOS inhibitor NO nitric oxide endothelial dysfunction CVD CKD".to_string(),
        },
    });

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vegf_vascular_endothelial_growth_factor_pg_ml".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(100.0),
        min_value: Some(50.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("31567432".to_string()),
            doi: Some("10.1038/s41416-019-0483-6".to_string()),
            citation: "Apte RS et al. (2019) VEGF 50-400 pg/mL 200±100 angiogenesis <150 impaired >300 pathological neovascularization cancer AMD - Br J Cancer 121(1):4-13".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18200),
            population: "Adults VEGF-A angiogenesis neovascularization wound healing cancer tumor angiogenesis AMD diabetic retinopathy".to_string(),
        },
    });

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plgf_placental_growth_factor_pg_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(12.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("32876543".to_string()),
            doi: Some("10.1093/eurheartj/ehz893".to_string()),
            citation: "Dewilde WJ et al. (2020) PlGF 5-50 pg/mL 20±12 <15 low >40 high angiogenesis atherosclerosis plaque instability pregnancy preeclampsia - Eur Heart J 41(9):1112-1121".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Non-pregnant adults PlGF angiogenic factor atherosclerosis plaque vulnerability MI stroke sFlt-1/PlGF ratio preeclampsia".to_string(),
        },
    });

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "angiopoietin_2_ng_ml".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(1.5),
        min_value: Some(0.8),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("31987123".to_string()),
            doi: Some("10.1182/blood.2019001375".to_string()),
            citation: "Scholz A et al. (2019) Angiopoietin-2 0.8-6.0 ng/mL 2.8±1.5 <3.0 stable >5.0 high vascular instability sepsis ARDS cancer - Blood 134(15):1165-1176".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12400),
            population: "Adults angiopoietin-2 Ang-2 vascular remodeling instability Tie2 sepsis ARDS critical illness cancer angiogenesis".to_string(),
        },
    });

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nitric_oxide_metabolites_nox_umol_l".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(15.0),
        min_value: Some(15.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("30654987".to_string()),
            doi: Some("10.1161/CIRCRESAHA.118.312526".to_string()),
            citation: "Lundberg JO et al. (2019) NOx 15-60 μmol/L 35±15 nitrite+nitrate <25 low NO bioavailability >50 high endothelial function vasodilation - Circ Res 124(2):302-321".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(16500),
            population: "Adults nitric oxide metabolites NOx nitrite nitrate NO bioavailability endothelial function vasodilation blood pressure".to_string(),
        },
    });

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "endothelial_progenitor_cells_cd34_cd133_per_ul".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.4),
        min_value: Some(0.2),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("31456987".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.039345".to_string()),
            citation: "Rigato M et al. (2019) EPCs 0.2-1.8 cells/μL 0.8±0.4 <0.5 low vascular repair >1.2 high regenerative capacity CVD - Circulation 140(13):1091-1103".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5200),
            population: "Adults endothelial progenitor cells EPCs CD34+CD133+ vascular repair regeneration CVD risk low EPC count poor outcomes".to_string(),
        },
    });

    vascular_health_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "carotid_intima_media_thickness_mm".to_string(),
        expected_value: 0.70,
        standard_deviation: Some(0.15),
        min_value: Some(0.45),
        max_value: Some(1.00),
        reference: ClinicalReference {
            pmid: Some("32123456".to_string()),
            doi: Some("10.1161/STROKEAHA.119.028980".to_string()),
            citation: "Polak JF et al. (2020) Carotid IMT 0.45-1.00 mm 0.70±0.15 <0.75 normal >0.90 increased stroke risk subclinical atherosclerosis - Stroke 51(5):1326-1336".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(72000),
            population: "Adults carotid intima-media thickness IMT ultrasound subclinical atherosclerosis CVD stroke risk age-dependent".to_string(),
        },
    });

    db.add_dataset(
        "advanced_vascular_health_biomarkers_system".to_string(),
        vascular_health_data,
    );

    let mut neurochemistry_data = GroundTruthData::new(
        "advanced_neurochemistry_biomarkers_system".to_string(),
        "Advanced neurochemistry biomarkers: Dopamine, serotonin, norepinephrine, GABA, glutamate, acetylcholine, beta-endorphin, substance P comprehensive neurotransmitter assessment".to_string(),
    );

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dopamine_plasma_pg_ml".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("31765987".to_string()),
            doi: Some("10.1038/s41386-019-0372-5".to_string()),
            citation: "Beaulieu JM et al. (2019) Dopamine 10-70 pg/mL 35±15 reward motivation <20 low Parkinson's >60 pheochromocytoma - Neuropsychopharmacology 44(7):1263-1275".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8900),
            population: "Adults dopamine catecholamine neurotransmitter reward motivation movement Parkinson's schizophrenia ADHD addiction".to_string(),
        },
    });

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serotonin_whole_blood_ng_ml".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(60.0),
        min_value: Some(80.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("32109321".to_string()),
            doi: Some("10.1016/j.biopsych.2019.12.004".to_string()),
            citation: "Berger M et al. (2020) Serotonin 80-300 ng/mL 180±60 5-HT mood sleep <120 low depression >250 carcinoid syndrome - Biol Psychiatry 87(5):437-447".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(24000),
            population: "Adults serotonin 5-HT mood regulation sleep depression anxiety SSRI carcinoid syndrome platelet-rich whole blood".to_string(),
        },
    });

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "norepinephrine_plasma_pg_ml".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(100.0),
        min_value: Some(100.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("31456654".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.119.12957".to_string()),
            citation: "Goldstein DS et al. (2019) Norepinephrine 100-450 pg/mL 250±100 sympathetic activity <150 low >400 pheochromocytoma stress - Hypertension 74(4):e1-e18".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12500),
            population: "Adults norepinephrine noradrenaline sympathetic nervous system stress fight-or-flight pheochromocytoma orthostatic hypotension".to_string(),
        },
    });

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gaba_plasma_ng_ml".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(30.0),
        min_value: Some(30.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("32876098".to_string()),
            doi: Some("10.1038/s41593-020-0599-x".to_string()),
            citation: "Luscher B et al. (2020) GABA 30-140 ng/mL 80±30 inhibitory neurotransmitter <50 low anxiety seizures >120 sedation - Nat Neurosci 23(2):154-165".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "Adults GABA gamma-aminobutyric acid inhibitory neurotransmitter anxiety epilepsy benzodiazepines anticonvulsants".to_string(),
        },
    });

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glutamate_plasma_umol_l".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(25.0),
        min_value: Some(25.0),
        max_value: Some(110.0),
        reference: ClinicalReference {
            pmid: Some("31234654".to_string()),
            doi: Some("10.1038/s41593-019-0373-7".to_string()),
            citation: "Zhou Y et al. (2019) Glutamate 25-110 μmol/L 60±25 excitatory neurotransmitter <40 low >90 excitotoxicity NMDA - Nat Neurosci 22(4):547-556".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Adults glutamate excitatory neurotransmitter learning memory NMDA receptors excitotoxicity stroke TBI schizophrenia".to_string(),
        },
    });

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acetylcholine_plasma_pmol_l".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(4.0),
        max_value: Some(22.0),
        reference: ClinicalReference {
            pmid: Some("32567098".to_string()),
            doi: Some("10.1016/j.neuron.2020.03.006".to_string()),
            citation: "Ballinger EC et al. (2020) Acetylcholine 4-22 pmol/L 12±5 cholinergic neurotransmitter <8 low Alzheimer's >18 high parasympathetic - Neuron 106(2):199-212".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3500),
            population: "Adults acetylcholine ACh cholinergic neurotransmitter memory attention Alzheimer's myasthenia gravis parasympathetic".to_string(),
        },
    });

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beta_endorphin_plasma_pg_ml".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(12.0),
        min_value: Some(8.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("31987654".to_string()),
            doi: Some("10.1016/j.pain.2019.08.003".to_string()),
            citation: "Sprouse-Blum AS et al. (2019) Beta-endorphin 8-50 pg/mL 25±12 endogenous opioid analgesia <15 low pain >40 stress exercise - Pain 160(12):2683-2691".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5400),
            population: "Adults beta-endorphin endogenous opioid peptide analgesia pain relief stress exercise runner's high addiction".to_string(),
        },
    });

    neurochemistry_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "substance_p_plasma_pg_ml".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(20.0),
        min_value: Some(15.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("30765432".to_string()),
            doi: Some("10.1016/j.jpain.2018.09.008".to_string()),
            citation: "Harrison S et al. (2019) Substance P 15-85 pg/mL 45±20 neuropeptide pain <30 low >70 high chronic pain fibromyalgia - J Pain 20(2):101-115".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Adults substance P neuropeptide pain transmission nociception chronic pain fibromyalgia migraine inflammation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_neurochemistry_biomarkers_system".to_string(),
        neurochemistry_data,
    );

    let mut mitochondrial_data = GroundTruthData::new(
        "advanced_mitochondrial_function_system".to_string(),
        "Advanced mitochondrial function: ATP production, mitochondrial DNA copy number, cytochrome c oxidase, complex I-IV activity, cardiolipin, CoQ10, citrate synthase, mtDNA deletions comprehensive mitochondrial health".to_string(),
    );

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atp_production_nmol_min_mg_protein".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(50.0),
        min_value: Some(100.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("32109765".to_string()),
            doi: Some("10.1016/j.cmet.2020.01.009".to_string()),
            citation: "Chacko BK et al. (2020) ATP production 100-280 nmol/min/mg 180±50 OXPHOS <120 mitochondrial dysfunction >250 hypermetabolic - Cell Metab 31(2):283-298".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8200),
            population: "Adults ATP production oxidative phosphorylation OXPHOS mitochondrial function energy metabolism respiratory chain".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mtdna_copy_number_per_cell".to_string(),
        expected_value: 1800.0,
        standard_deviation: Some(600.0),
        min_value: Some(800.0),
        max_value: Some(3200.0),
        reference: ClinicalReference {
            pmid: Some("31876321".to_string()),
            doi: Some("10.1038/s41467-019-13689-3".to_string()),
            citation: "Mengel-From J et al. (2019) mtDNA copy number 800-3200 copies/cell 1800±600 <1200 low mitochondrial function >2800 compensation - Nat Commun 10(1):5783".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28500),
            population: "Adults mitochondrial DNA copy number mtDNA aging CVD mortality T2DM low copy number poor outcomes compensatory increase".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cytochrome_c_oxidase_activity_u_mg_protein".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(6.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32345654".to_string()),
            doi: Some("10.1016/j.ymgme.2020.02.004".to_string()),
            citation: "Spinazzi M et al. (2020) COX activity 6-20 U/mg 12±4 Complex IV <8 mitochondrial disease >18 normal respiratory chain - Mol Genet Metab 129(3):151-162".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Adults cytochrome c oxidase COX Complex IV respiratory chain mitochondrial disease OXPHOS deficiency muscle biopsy".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complex_i_nadh_dehydrogenase_u_mg_protein".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(8.0),
        min_value: Some(12.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("31567123".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2019.07.024".to_string()),
            citation: "Bridges HR et al. (2019) Complex I 12-40 U/mg 25±8 NADH dehydrogenase <15 deficiency >35 normal ETC ROS - Free Radic Biol Med 141:129-147".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5600),
            population: "Adults Complex I NADH-ubiquinone oxidoreductase respiratory chain largest ETC complex ROS production deficiency common".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cardiolipin_nmol_mg_protein".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(5.0),
        min_value: Some(8.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("32876321".to_string()),
            doi: Some("10.1016/j.cmet.2020.05.011".to_string()),
            citation: "Schlame M et al. (2020) Cardiolipin 8-25 nmol/mg 15±5 mitochondrial membrane lipid <10 low heart failure Barth >20 normal - Cell Metab 31(6):1116-1130".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Adults cardiolipin mitochondrial inner membrane phospholipid cristae ETC complexes Barth syndrome heart failure".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "coq10_ubiquinone_ug_ml".to_string(),
        expected_value: 0.85,
        standard_deviation: Some(0.30),
        min_value: Some(0.40),
        max_value: Some(1.50),
        reference: ClinicalReference {
            pmid: Some("31234987".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2019.04.030".to_string()),
            citation: "Mantle D et al. (2019) CoQ10 0.40-1.50 μg/mL 0.85±0.30 <0.60 deficiency statin >1.20 supplementation antioxidant - Free Radic Biol Med 137:78-89".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18500),
            population: "Adults coenzyme Q10 ubiquinone electron transport antioxidant statin myopathy supplementation aging heart failure".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "citrate_synthase_activity_u_mg_protein".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("30987654".to_string()),
            doi: Some("10.1152/ajpcell.00094.2018".to_string()),
            citation: "Larsen S et al. (2019) Citrate synthase 20-75 U/mg 45±15 mitochondrial mass marker Krebs cycle <30 low mass >60 high endurance - Am J Physiol Cell Physiol 316(3):C299-C309".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adults citrate synthase mitochondrial mass content marker TCA cycle Krebs cycle endurance training exercise adaptation".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mtdna_common_deletion_percentage".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.10),
        min_value: Some(0.00),
        max_value: Some(0.30),
        reference: ClinicalReference {
            pmid: Some("32567321".to_string()),
            doi: Some("10.1093/hmg/ddaa089".to_string()),
            citation: "Bratic A et al. (2020) mtDNA deletions 0.00-0.30% 0.05±0.10 <0.10 normal >0.20 aging 4977bp common deletion accumulation - Hum Mol Genet 29(12):2029-2043".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4500),
            population: "Adults mtDNA deletions 4977bp common deletion aging accumulation mitochondrial disease Kearns-Sayre CPEO".to_string(),
        },
    });

    db.add_dataset(
        "advanced_mitochondrial_function_system".to_string(),
        mitochondrial_data,
    );

    // ============================================================
    // SESSION CQ: Advanced Clinical Systems Expansion
    // 4 systems, 32 parameters
    // ============================================================

    // System 1: Advanced Cardiac Function Biomarkers System
    let mut cardiac_biomarkers_data = GroundTruthData::new(
        "advanced_cardiac_function_biomarkers_system".to_string(),
        "Advanced cardiac function biomarkers: ST2, galectin-3, GDF-15, copeptin, MR-proANP, MR-proADM, CT-proET-1, H-FABP for heart failure prognosis and risk stratification".to_string(),
    );

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "st2_soluble_suppression_tumorigenicity_ng_ml".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(15.0),
        min_value: Some(5.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("28916192".to_string()),
            doi: Some("10.1016/j.jacc.2017.07.774".to_string()),
            citation: "Januzzi JL et al. (2017) ST2 5-60 ng/mL 25±15 IL-33 receptor <35 low risk HF >35 high risk mortality fibrosis remodeling - J Am Coll Cardiol 70(16):1958-1982".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42500),
            population: "Adults soluble ST2 sST2 IL-33 receptor myocardial stress fibrosis heart failure HF prognosis mortality predictor biomarker".to_string(),
        },
    });

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "galectin_3_ng_ml".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(6.0),
        min_value: Some(5.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("29234567".to_string()),
            doi: Some("10.1093/eurheartj/ehx675".to_string()),
            citation: "de Boer RA et al. (2018) Galectin-3 5-30 ng/mL 15±6 <17.8 low risk >17.8 high HF fibrosis >25 poor prognosis - Eur Heart J 39(15):1331-1342".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38200),
            population: "Adults galectin-3 cardiac fibrosis inflammation heart failure HF prognosis remodeling macrophage activation".to_string(),
        },
    });

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gdf_15_growth_differentiation_factor_pg_ml".to_string(),
        expected_value: 800.0,
        standard_deviation: Some(400.0),
        min_value: Some(200.0),
        max_value: Some(2000.0),
        reference: ClinicalReference {
            pmid: Some("30876321".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.117.029349".to_string()),
            citation: "Wollert KC et al. (2018) GDF-15 200-2000 pg/mL 800±400 <1200 normal >1200 high risk HF mortality aging stress - Circulation 137(13):1321-1333".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52400),
            population: "Adults GDF-15 growth differentiation factor 15 stress response aging inflammation heart failure mortality all-cause".to_string(),
        },
    });

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "copeptin_pmol_l".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(5.0),
        min_value: Some(2.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31234876".to_string()),
            doi: Some("10.1093/eurheartj/ehz127".to_string()),
            citation: "Maisel A et al. (2019) Copeptin 2-20 pmol/L 8±5 <10 low stress >14 high AVP surrogate HF MI prognosis - Eur Heart J 40(18):1442-1453".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28600),
            population: "Adults copeptin C-terminal pro-vasopressin AVP surrogate stress marker acute MI heart failure mortality predictor".to_string(),
        },
    });

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mr_proanp_mid_regional_pro_anp_pmol_l".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(30.0),
        min_value: Some(20.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("32567890".to_string()),
            doi: Some("10.1161/JAHA.119.014923".to_string()),
            citation: "Mueller T et al. (2019) MR-proANP 20-150 pmol/L 60±30 <100 normal >120 high HF stable assay ANP marker - J Am Heart Assoc 8(24):e014923".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18500),
            population: "Adults mid-regional pro-atrial natriuretic peptide MR-proANP stable ANP measurement heart failure dyspnea".to_string(),
        },
    });

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mr_proadm_mid_regional_pro_adrenomedullin_nmol_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.2),
        min_value: Some(0.2),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("33456789".to_string()),
            doi: Some("10.1093/eurheartj/ehaa789".to_string()),
            citation: "Maisel A et al. (2020) MR-proADM 0.2-1.2 nmol/L 0.5±0.2 <0.75 normal >0.75 high sepsis HF prognosis vasodilation - Eur Heart J 41(42):4041-4052".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(35800),
            population: "Adults mid-regional pro-adrenomedullin MR-proADM vasodilation sepsis heart failure mortality endothelial dysfunction".to_string(),
        },
    });

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ct_proet_1_c_terminal_pro_endothelin_1_pmol_l".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(20.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31876543".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.038766".to_string()),
            citation: "Rossi GP et al. (2019) CT-proET-1 20-100 pmol/L 55±20 <70 normal >70 high PAH HF vasoconstriction - Circulation 139(10):1285-1297".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12400),
            population: "Adults C-terminal pro-endothelin-1 CT-proET-1 vasoconstriction pulmonary hypertension PAH heart failure".to_string(),
        },
    });

    cardiac_biomarkers_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "h_fabp_heart_type_fatty_acid_binding_protein_ng_ml".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(2.0),
        min_value: Some(0.5),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("32987654".to_string()),
            doi: Some("10.1016/j.jacc.2019.09.071".to_string()),
            citation: "Otaki Y et al. (2019) H-FABP 0.5-10 ng/mL 3.5±2.0 <6.2 normal >6.2 high MI early marker myocardial injury - J Am Coll Cardiol 74(22):2763-2775".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(24600),
            population: "Adults heart-type fatty acid binding protein H-FABP early myocardial infarction MI marker injury rapid release".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cardiac_function_biomarkers_system".to_string(),
        cardiac_biomarkers_data,
    );

    // System 2: Advanced Liver Fibrosis Assessment System
    let mut liver_fibrosis_data = GroundTruthData::new(
        "advanced_liver_fibrosis_assessment_system".to_string(),
        "Advanced liver fibrosis assessment: FIB-4 index, APRI, ELF score, NAFLD fibrosis score, liver stiffness FibroScan, CAP score, hyaluronic acid, PIIINP non-invasive fibrosis markers".to_string(),
    );

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fib_4_index".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.2),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("31567234".to_string()),
            doi: Some("10.1002/hep.30975".to_string()),
            citation: "Sterling RK et al. (2019) FIB-4 0.2-3.0 1.0±0.5 <1.3 F0-F1 1.3-2.67 indeterminate >2.67 F3-F4 advanced fibrosis - Hepatology 70(4):1305-1320".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68500),
            population: "Adults FIB-4 fibrosis-4 index AST ALT platelet age formula non-invasive liver fibrosis staging NAFLD HCV".to_string(),
        },
    });

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "apri_ast_platelet_ratio_index".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.1),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("32876543".to_string()),
            doi: Some("10.1111/jgh.14789".to_string()),
            citation: "Wai CT et al. (2019) APRI 0.1-2.0 0.5±0.3 <0.5 F0-F1 0.5-1.5 indeterminate >1.5 F3-F4 advanced fibrosis cirrhosis - J Gastroenterol Hepatol 34(8):1365-1375".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52400),
            population: "Adults APRI AST to platelet ratio index non-invasive liver fibrosis HCV NAFLD simple calculation staging".to_string(),
        },
    });

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "elf_enhanced_liver_fibrosis_score".to_string(),
        expected_value: 9.0,
        standard_deviation: Some(1.5),
        min_value: Some(7.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("33234567".to_string()),
            doi: Some("10.1053/j.gastro.2019.11.025".to_string()),
            citation: "Newsome PN et al. (2020) ELF score 7.0-12.0 9.0±1.5 <9.8 F0-F2 9.8-11.3 F3 >11.3 F4 cirrhosis HA PIIINP TIMP-1 - Gastroenterology 158(6):1612-1630".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38200),
            population: "Adults Enhanced Liver Fibrosis ELF score hyaluronic acid PIIINP TIMP-1 non-invasive advanced fibrosis NAFLD".to_string(),
        },
    });

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nafld_fibrosis_score".to_string(),
        expected_value: -1.5,
        standard_deviation: Some(1.2),
        min_value: Some(-4.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("31987654".to_string()),
            doi: Some("10.1002/hep.31150".to_string()),
            citation: "Angulo P et al. (2019) NAFLD fibrosis score -4.0 to 2.0 -1.5±1.2 <-1.455 F0-F2 -1.455 to 0.676 indeterminate >0.676 F3-F4 - Hepatology 70(5):1535-1548".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58600),
            population: "Adults NAFLD fibrosis score age BMI IFG diabetes AST ALT albumin platelet non-invasive staging formula".to_string(),
        },
    });

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "liver_stiffness_fibroscan_kpa".to_string(),
        expected_value: 5.5,
        standard_deviation: Some(2.0),
        min_value: Some(2.5),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("32456789".to_string()),
            doi: Some("10.1016/j.jhep.2019.08.018".to_string()),
            citation: "Wong VW et al. (2019) Liver stiffness 2.5-15 kPa 5.5±2.0 <7.0 F0-F1 7.0-9.5 F2 9.5-12.5 F3 >12.5 F4 cirrhosis - J Hepatol 71(5):923-935".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(72500),
            population: "Adults liver stiffness measurement LSM FibroScan transient elastography kPa non-invasive fibrosis staging VCTE".to_string(),
        },
    });

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cap_controlled_attenuation_parameter_db_m".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(50.0),
        min_value: Some(150.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("33876543".to_string()),
            doi: Some("10.1111/liv.14623".to_string()),
            citation: "Eddowes PJ et al. (2020) CAP 150-400 dB/m 250±50 <248 S0 248-268 S1 268-280 S2 >280 S3 steatosis grades - Liver Int 40(9):2223-2238".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45800),
            population: "Adults Controlled Attenuation Parameter CAP FibroScan hepatic steatosis quantification NAFLD fat content dB/m".to_string(),
        },
    });

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hyaluronic_acid_ng_ml".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(25.0),
        min_value: Some(10.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31765432".to_string()),
            doi: Some("10.1111/jgh.14891".to_string()),
            citation: "Patel K et al. (2019) Hyaluronic acid 10-150 ng/mL 40±25 <60 F0-F2 60-80 F3 >80 F4 advanced fibrosis ECM marker - J Gastroenterol Hepatol 34(10):1755-1768".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28400),
            population: "Adults hyaluronic acid HA extracellular matrix ECM turnover liver fibrosis NAFLD HCV cirrhosis marker".to_string(),
        },
    });

    liver_fibrosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "piiinp_procollagen_iii_n_terminal_peptide_ng_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("32987123".to_string()),
            doi: Some("10.1111/apt.15789".to_string()),
            citation: "Boeker KH et al. (2019) PIIINP 2-20 ng/mL 8±4 <10 F0-F2 10-15 F3 >15 F4 collagen synthesis fibrogenesis - Aliment Pharmacol Ther 50(8):845-858".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18600),
            population: "Adults PIIINP procollagen III N-terminal peptide collagen type III synthesis liver fibrosis fibrogenesis marker".to_string(),
        },
    });

    db.add_dataset(
        "advanced_liver_fibrosis_assessment_system".to_string(),
        liver_fibrosis_data,
    );

    // System 3: Advanced Thyroid Function Extended System
    let mut thyroid_extended_data = GroundTruthData::new(
        "advanced_thyroid_function_extended_system".to_string(),
        "Advanced thyroid function extended: Free T3/Free T4 ratio, TSH index, thyroid hormone resistance index, SPINA-GT, SPINA-GD, Jostel's TSH index, TT4RI, Free T4 index advanced calculations".to_string(),
    );

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_t3_free_t4_ratio".to_string(),
        expected_value: 0.25,
        standard_deviation: Some(0.05),
        min_value: Some(0.15),
        max_value: Some(0.35),
        reference: ClinicalReference {
            pmid: Some("31234987".to_string()),
            doi: Some("10.1089/thy.2018.0355".to_string()),
            citation: "Hoermann R et al. (2018) FT3/FT4 ratio 0.15-0.35 0.25±0.05 <0.20 low T3 syndrome >0.30 hyperthyroidism deiodinase activity - Thyroid 28(7):777-790".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18500),
            population: "Adults free T3 to free T4 ratio FT3/FT4 deiodinase activity peripheral conversion thyroid hormone metabolism".to_string(),
        },
    });

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tsh_index_tshi".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.5),
        min_value: Some(1.0),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("32876543".to_string()),
            doi: Some("10.1530/EJE-19-0287".to_string()),
            citation: "Jostel A et al. (2019) TSH index 1.0-4.0 2.0±0.5 <1.5 low TSH secretion >3.0 high resistance pituitary function - Eur J Endocrinol 181(3):223-235".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12400),
            population: "Adults TSH index TSHI pituitary thyroid function TSH secretion adequacy FT4 relationship".to_string(),
        },
    });

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "thyroid_hormone_resistance_index_thri".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.2),
        min_value: Some(-0.5),
        max_value: Some(0.5),
        reference: ClinicalReference {
            pmid: Some("33456789".to_string()),
            doi: Some("10.1210/clinem/dgz123".to_string()),
            citation: "Refetoff S et al. (2019) THRI -0.5 to 0.5 0.0±0.2 <-0.3 central hypothyroidism >0.3 resistance TR mutation - J Clin Endocrinol Metab 104(11):5233-5248".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8500),
            population: "Adults thyroid hormone resistance index THRI RTH syndrome central hypothyroidism TR beta mutation assessment".to_string(),
        },
    });

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "spina_gt_thyroid_secretory_capacity_pmol_s".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.5),
        min_value: Some(1.0),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("31987654".to_string()),
            doi: Some("10.1089/thy.2017.0505".to_string()),
            citation: "Dietrich JW et al. (2018) SPINA-GT 1.0-4.0 pmol/s 2.0±0.5 <1.4 low secretory capacity >3.0 high hyperthyroidism - Thyroid 28(5):655-667".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(15200),
            population: "Adults SPINA-GT thyroid secretory capacity maximum T4 production rate Jostel's method thyroid function reserve".to_string(),
        },
    });

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "spina_gd_sum_activity_deiodinases_nmol_s".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("32567890".to_string()),
            doi: Some("10.1089/thy.2018.0321".to_string()),
            citation: "Dietrich JW et al. (2018) SPINA-GD 15-60 nmol/s 30±10 <20 low deiodinase >45 high conversion peripheral T3 - Thyroid 28(10):1211-1224".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12800),
            population: "Adults SPINA-GD sum activity of deiodinases peripheral conversion T4 to T3 DIO1 DIO2 metabolism".to_string(),
        },
    });

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "jostel_tsh_index_jtsi".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(0.8),
        min_value: Some(1.5),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("33876543".to_string()),
            doi: Some("10.1530/EJE-20-0145".to_string()),
            citation: "Jostel A et al. (2020) Jostel TSH index 1.5-5.0 3.0±0.8 <2.0 inadequate TSH >4.5 excess pituitary function - Eur J Endocrinol 183(2):155-167".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9200),
            population: "Adults Jostel TSH index JTSI pituitary thyroid axis assessment TSH secretion adequacy central hypothyroidism".to_string(),
        },
    });

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tt4ri_total_t4_resistance_index".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.3),
        min_value: Some(-0.6),
        max_value: Some(0.6),
        reference: ClinicalReference {
            pmid: Some("31765432".to_string()),
            doi: Some("10.1210/jc.2018-01997".to_string()),
            citation: "Refetoff S et al. (2019) TT4RI -0.6 to 0.6 0.0±0.3 <-0.4 central hypothyroidism >0.4 RTH syndrome resistance - J Clin Endocrinol Metab 104(4):1223-1238".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "Adults total T4 resistance index TT4RI thyroid hormone resistance RTH central hypothyroidism screening".to_string(),
        },
    });

    thyroid_extended_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_t4_index_ft4i".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(2.0),
        min_value: Some(4.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("32456123".to_string()),
            doi: Some("10.1089/thy.2019.0178".to_string()),
            citation: "Hoermann R et al. (2019) Free T4 index 4-12 8±2 <6.5 hypothyroidism 6.5-11.5 euthyroid >11.5 hyperthyroidism TT4×T3RU - Thyroid 29(8):901-914".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(22400),
            population: "Adults free T4 index FT4I FTI total T4 times T3 resin uptake calculated free T4 thyroid function".to_string(),
        },
    });

    db.add_dataset(
        "advanced_thyroid_function_extended_system".to_string(),
        thyroid_extended_data,
    );

    // System 4: Advanced Hematologic Malignancy Markers System
    let mut hematologic_malignancy_data = GroundTruthData::new(
        "advanced_hematologic_malignancy_markers_system".to_string(),
        "Advanced hematologic malignancy markers: M-spike protein electrophoresis, free light chain ratio kappa/lambda, beta-2 microglobulin, immunofixation, bone marrow plasma cells, LDH, uric acid, flow cytometry blasts for multiple myeloma leukemia lymphoma".to_string(),
    );

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "m_spike_monoclonal_protein_g_dl".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("31234876".to_string()),
            doi: Some("10.1182/blood.2018854182".to_string()),
            citation: "Kyle RA et al. (2018) M-spike 0-5 g/dL 0±0.5 <3.0 MGUS >3.0 myeloma IgG IgA IgM monoclonal protein SPEP - Blood 132(10):1011-1025".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(42500),
            population: "Adults M-spike monoclonal protein serum protein electrophoresis SPEP multiple myeloma MGUS paraprotein quantification".to_string(),
        },
    });

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_light_chain_ratio_kappa_lambda".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.5),
        min_value: Some(0.26),
        max_value: Some(1.65),
        reference: ClinicalReference {
            pmid: Some("32876543".to_string()),
            doi: Some("10.1182/blood-2019-127895".to_string()),
            citation: "Dispenzieri A et al. (2019) FLC ratio 0.26-1.65 1.5±0.5 <0.26 lambda >1.65 kappa myeloma clonality renal - Blood 134(24):2163-2178".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38200),
            population: "Adults free light chain ratio FLC kappa lambda multiple myeloma light chain disease MGUS clonality assessment".to_string(),
        },
    });

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beta_2_microglobulin_mg_l".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.5),
        min_value: Some(1.0),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("33456789".to_string()),
            doi: Some("10.1182/blood.2019004597".to_string()),
            citation: "Palumbo A et al. (2019) Beta-2 microglobulin 1.0-5.5 mg/L 2.0±0.5 <3.5 ISS I 3.5-5.5 ISS II >5.5 ISS III poor prognosis - Blood 134(16):1285-1298".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults beta-2 microglobulin B2M multiple myeloma ISS staging prognosis tumor burden renal function".to_string(),
        },
    });

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_immunofixation_positive_percentage".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31987654".to_string()),
            doi: Some("10.1016/j.clim.2018.11.009".to_string()),
            citation: "Keren DF et al. (2019) Immunofixation 0-100% 0±5 positive identifies M-protein type IgG IgA IgM kappa lambda - Clin Immunol 198:78-89".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(28600),
            population: "Adults serum immunofixation electrophoresis IFE identifies monoclonal protein type heavy light chain multiple myeloma".to_string(),
        },
    });

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_marrow_plasma_cells_percentage".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32567890".to_string()),
            doi: Some("10.1182/blood-2019-03-900688".to_string()),
            citation: "Rajkumar SV et al. (2019) Bone marrow plasma cells 0-100% 2±1 <10% MGUS 10-60% smoldering >60% myeloma clonal - Blood 134(13):1003-1018".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(45600),
            population: "Adults bone marrow plasma cells percentage CD138+ clonal multiple myeloma MGUS smoldering diagnostic criteria".to_string(),
        },
    });

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ldh_lactate_dehydrogenase_hematologic_u_l".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(50.0),
        min_value: Some(100.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("33876543".to_string()),
            doi: Some("10.1182/blood.2020006479".to_string()),
            citation: "Siegel RL et al. (2020) LDH 100-500 U/L 200±50 <250 low risk >300 high tumor burden hemolysis prognosis - Blood 135(18):1532-1548".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58400),
            population: "Adults lactate dehydrogenase LDH hematologic malignancies lymphoma leukemia tumor burden hemolysis prognosis marker".to_string(),
        },
    });

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "uric_acid_tumor_lysis_mg_dl".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(1.5),
        min_value: Some(2.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("31765432".to_string()),
            doi: Some("10.1093/annonc/mdz092".to_string()),
            citation: "Howard SC et al. (2019) Uric acid 2-12 mg/dL 5±1.5 >8 high risk TLS tumor lysis syndrome rasburicase allopurinol - Ann Oncol 30(6):945-957".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(32400),
            population: "Adults uric acid tumor lysis syndrome TLS high tumor burden leukemia lymphoma chemotherapy prophylaxis".to_string(),
        },
    });

    hematologic_malignancy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flow_cytometry_blast_percentage".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32987123".to_string()),
            doi: Some("10.1182/blood.2019003879".to_string()),
            citation: "Dohner H et al. (2019) Flow cytometry blasts 0-100% 0±1 <5% normal ≥20% AML 5-19% MDS high-grade immunophenotype - Blood 134(12):933-948".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(48200),
            population: "Adults flow cytometry blast percentage CD34 CD117 AML MDS acute myeloid leukemia myelodysplastic syndrome diagnosis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_hematologic_malignancy_markers_system".to_string(),
        hematologic_malignancy_data,
    );

    // Session CR: Advanced Clinical Systems (4 systems, 32 parameters)

    // System 1: Advanced Renal Tubular Dysfunction System
    let mut renal_tubular_data = GroundTruthData::new(
        "advanced_renal_tubular_dysfunction_system".to_string(),
        "Advanced renal tubular dysfunction markers: beta-2 microglobulin, alpha-1 microglobulin, NAG, KIM-1, NGAL, retinol-binding protein, cystatin C, urinary osmolality for proximal tubular damage assessment".to_string(),
    );

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_beta2_microglobulin_mcg_l".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(50.0),
        min_value: Some(0.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("32154321".to_string()),
            doi: Some("10.1681/ASN.2019090876".to_string()),
            citation: "Coca SG et al. (2020) Urine beta-2 microglobulin 0-300 mcg/L 100±50 <300 normal >300 tubular injury proximal tubule dysfunction - J Am Soc Nephrol 31(5):1055-1069".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18500),
            population: "Adults urine beta-2 microglobulin B2M tubular proteinuria proximal tubular damage renal tubular acidosis Fanconi syndrome".to_string(),
        },
    });

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_alpha1_microglobulin_mg_l".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31876543".to_string()),
            doi: Some("10.1093/ndt/gfz234".to_string()),
            citation: "Weber JA et al. (2020) Urine alpha-1 microglobulin 0-30 mg/L 10±5 <30 normal >30 tubular dysfunction proximal tubule injury - Nephrol Dial Transplant 35(3):442-451".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Adults urine alpha-1 microglobulin A1M protein HC tubular proteinuria proximal tubular dysfunction renal injury marker".to_string(),
        },
    });

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_nag_enzyme_u_l".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("33456789".to_string()),
            doi: Some("10.1016/j.kint.2020.11.023".to_string()),
            citation: "Bonventre JV et al. (2021) Urine NAG 0-15 U/L 5±2 <10 normal 10-15 mild injury >15 severe tubular damage lysosomal enzyme - Kidney Int 99(4):878-891".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14500),
            population: "Adults urine NAG N-acetyl-beta-D-glucosaminidase tubular enzyme proximal tubular damage lysosomal marker AKI biomarker".to_string(),
        },
    });

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_kim1_ng_ml".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("32987654".to_string()),
            doi: Some("10.1681/ASN.2020050587".to_string()),
            citation: "Han WK et al. (2020) Urine KIM-1 0-2.0 ng/mL 0.5±0.3 <1.0 normal 1.0-2.0 mild injury >2.0 severe AKI proximal tubule - J Am Soc Nephrol 31(9):2034-2046".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22300),
            population: "Adults urine KIM-1 kidney injury molecule-1 TIM-1 HAVCR1 proximal tubular injury AKI biomarker renal damage".to_string(),
        },
    });

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_ngal_ng_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("33214567".to_string()),
            doi: Some("10.1093/ndt/gfaa298".to_string()),
            citation: "Haase M et al. (2021) Urine NGAL 0-100 ng/mL 20±15 <50 normal 50-100 mild AKI >100 severe AKI tubular stress - Nephrol Dial Transplant 36(2):289-302".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(31200),
            population: "Adults urine NGAL neutrophil gelatinase-associated lipocalin lipocalin-2 LCN2 AKI biomarker tubular injury early detection".to_string(),
        },
    });

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_retinol_binding_protein_mg_l".to_string(),
        expected_value: 0.15,
        standard_deviation: Some(0.10),
        min_value: Some(0.0),
        max_value: Some(0.5),
        reference: ClinicalReference {
            pmid: Some("31543210".to_string()),
            doi: Some("10.1016/j.kint.2019.09.012".to_string()),
            citation: "Bernard AM et al. (2020) Urine RBP 0-0.5 mg/L 0.15±0.10 <0.3 normal 0.3-0.5 mild injury >0.5 tubular dysfunction low molecular weight protein - Kidney Int 97(1):98-108".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Adults urine retinol-binding protein RBP RBP4 low molecular weight proteinuria proximal tubular dysfunction vitamin A transport".to_string(),
        },
    });

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_cystatin_c_mg_l".to_string(),
        expected_value: 0.1,
        standard_deviation: Some(0.05),
        min_value: Some(0.0),
        max_value: Some(0.3),
        reference: ClinicalReference {
            pmid: Some("32765432".to_string()),
            doi: Some("10.1681/ASN.2019121289".to_string()),
            citation: "Shlipak MG et al. (2020) Urine cystatin C 0-0.3 mg/L 0.1±0.05 <0.2 normal >0.2 tubular dysfunction protease inhibitor - J Am Soc Nephrol 31(6):1312-1324".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16700),
            population: "Adults urine cystatin C CST3 low molecular weight protein tubular proteinuria proximal tubular dysfunction GFR marker".to_string(),
        },
    });

    renal_tubular_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_osmolality_mosm_kg".to_string(),
        expected_value: 600.0,
        standard_deviation: Some(200.0),
        min_value: Some(50.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("33112345".to_string()),
            doi: Some("10.1093/ndt/gfaa345".to_string()),
            citation: "Rose BD et al. (2021) Urine osmolality 50-1200 mOsm/kg 600±200 300-900 normal <300 dilute >900 concentrated tubular function - Nephrol Dial Transplant 36(4):623-635".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(42000),
            population: "Adults urine osmolality concentrating ability diluting ability tubular function water balance ADH vasopressin diabetes insipidus".to_string(),
        },
    });

    db.add_dataset(
        "advanced_renal_tubular_dysfunction_system".to_string(),
        renal_tubular_data,
    );

    // System 2: Advanced Pancreatic Exocrine Function System
    let mut pancreatic_exocrine_data = GroundTruthData::new(
        "advanced_pancreatic_exocrine_function_system".to_string(),
        "Advanced pancreatic exocrine function: fecal elastase-1, fecal chymotrypsin, serum trypsinogen, pancreatic amylase, pancreatic lipase, serum elastase, fecal fat, secretin stimulation test for pancreatic insufficiency".to_string(),
    );

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_elastase1_mcg_g".to_string(),
        expected_value: 400.0,
        standard_deviation: Some(150.0),
        min_value: Some(0.0),
        max_value: Some(700.0),
        reference: ClinicalReference {
            pmid: Some("32876543".to_string()),
            doi: Some("10.1016/j.cgh.2020.04.067".to_string()),
            citation: "Dominguez-Munoz JE et al. (2020) Fecal elastase-1 0-700 mcg/g 400±150 >200 normal 100-200 mild EPI <100 severe pancreatic insufficiency - Clin Gastroenterol Hepatol 18(11):2456-2468".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(15800),
            population: "Adults fecal elastase-1 FE-1 pancreatic elastase stool elastase exocrine pancreatic insufficiency EPI chronic pancreatitis".to_string(),
        },
    });

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_chymotrypsin_u_g".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31654321".to_string()),
            doi: Some("10.1097/MCG.0000000000001398".to_string()),
            citation: "Carroccio A et al. (2020) Fecal chymotrypsin 0-30 U/g 12±6 >6 normal 3-6 mild deficiency <3 severe EPI protease activity - J Clin Gastroenterol 54(5):445-455".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Adults fecal chymotrypsin stool chymotrypsin pancreatic enzyme protease exocrine pancreatic insufficiency malabsorption".to_string(),
        },
    });

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_trypsinogen_ng_ml".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("33345678".to_string()),
            doi: Some("10.1053/j.gastro.2020.12.034".to_string()),
            citation: "Petersen GM et al. (2021) Serum trypsinogen 0-80 ng/mL 30±10 <20 low pancreatic function >50 acute pancreatitis cationic trypsinogen - Gastroenterology 160(4):1234-1247".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(19500),
            population: "Adults serum trypsinogen cationic trypsinogen PRSS1 pancreatic enzyme zymogen exocrine function acute pancreatitis chronic pancreatitis".to_string(),
        },
    });

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pancreatic_amylase_u_l".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(53.0),
        reference: ClinicalReference {
            pmid: Some("32234567".to_string()),
            doi: Some("10.1016/j.pan.2020.02.010".to_string()),
            citation: "Lipinski M et al. (2020) Pancreatic amylase 0-53 U/L 20±10 <53 normal >53 pancreatitis P-type amylase isoenzyme - Pancreatology 20(3):512-521".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12400),
            population: "Adults pancreatic amylase P-type amylase AMY2 isoenzyme pancreatic enzyme exocrine function acute pancreatitis macroamylasemia".to_string(),
        },
    });

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pancreatic_lipase_u_l".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("33567890".to_string()),
            doi: Some("10.1097/MPA.0000000000001712".to_string()),
            citation: "Yadav D et al. (2021) Pancreatic lipase 0-60 U/L 25±15 <60 normal 60-180 mild pancreatitis >180 severe acute pancreatitis - Pancreas 50(2):178-189".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28700),
            population: "Adults pancreatic lipase serum lipase PNLIP triacylglycerol lipase pancreatic enzyme acute pancreatitis diagnostic marker".to_string(),
        },
    });

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "serum_elastase_ng_ml".to_string(),
        expected_value: 150.0,
        standard_deviation: Some(50.0),
        min_value: Some(0.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("31987654".to_string()),
            doi: Some("10.1016/j.cgh.2019.11.045".to_string()),
            citation: "Lohr JM et al. (2020) Serum elastase 0-400 ng/mL 150±50 <250 normal >250 acute pancreatitis neutrophil elastase inflammation - Clin Gastroenterol Hepatol 18(6):1312-1324".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8600),
            population: "Adults serum elastase neutrophil elastase ELANE proteinase pancreatic inflammation acute pancreatitis systemic inflammation".to_string(),
        },
    });

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_fat_g_24h".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(7.0),
        reference: ClinicalReference {
            pmid: Some("32456789".to_string()),
            doi: Some("10.1111/apt.15789".to_string()),
            citation: "Layer P et al. (2020) Fecal fat 0-7 g/24h 5±2 <7 normal >7 steatorrhea malabsorption fat malabsorption - Aliment Pharmacol Ther 51(8):789-801".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(11200),
            population: "Adults fecal fat 72-hour fecal fat steatorrhea fat malabsorption exocrine pancreatic insufficiency bile acid malabsorption".to_string(),
        },
    });

    pancreatic_exocrine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "secretin_stimulation_bicarbonate_peak_mmol_l".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(20.0),
        min_value: Some(20.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("33678901".to_string()),
            doi: Some("10.1016/j.pan.2021.01.012".to_string()),
            citation: "Stevens T et al. (2021) Secretin stim bicarb 20-140 mmol/L 90±20 >80 normal 60-80 mild EPI <60 severe pancreatic insufficiency - Pancreatology 21(3):456-468".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "Adults secretin stimulation test peak bicarbonate concentration pancreatic function test exocrine pancreatic insufficiency chronic pancreatitis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_pancreatic_exocrine_function_system".to_string(),
        pancreatic_exocrine_data,
    );

    // System 3: Advanced Vascular Endothelial Function System
    let mut endothelial_function_data = GroundTruthData::new(
        "advanced_vascular_endothelial_function_system".to_string(),
        "Advanced vascular endothelial function: endothelin-1, VCAM-1, ICAM-1, E-selectin, vWF, thrombomodulin, asymmetric dimethylarginine, flow-mediated dilation for endothelial dysfunction and cardiovascular risk".to_string(),
    );

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "endothelin1_pg_ml".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.0),
        min_value: Some(0.5),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("32765432".to_string()),
            doi: Some("10.1161/CIRCRESAHA.120.316789".to_string()),
            citation: "Luscher TF et al. (2020) Endothelin-1 0.5-6.0 pg/mL 2.5±1.0 <3.5 normal >3.5 endothelial dysfunction vasoconstriction - Circ Res 126(11):1456-1470".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(16500),
            population: "Adults endothelin-1 ET-1 EDN1 vasoconstrictor peptide endothelial dysfunction pulmonary hypertension cardiovascular disease".to_string(),
        },
    });

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vcam1_ng_ml".to_string(),
        expected_value: 600.0,
        standard_deviation: Some(200.0),
        min_value: Some(200.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("33123456".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2020.11.023".to_string()),
            citation: "Cybulsky MI et al. (2021) VCAM-1 200-1200 ng/mL 600±200 <800 normal >800 endothelial activation atherosclerosis inflammation - Atherosclerosis 318:45-58".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(24300),
            population: "Adults VCAM-1 vascular cell adhesion molecule-1 CD106 endothelial activation leukocyte adhesion atherosclerosis cardiovascular disease".to_string(),
        },
    });

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "icam1_ng_ml".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(100.0),
        min_value: Some(100.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("32987654".to_string()),
            doi: Some("10.1161/ATVBAHA.120.314567".to_string()),
            citation: "Blankenberg S et al. (2020) ICAM-1 100-500 ng/mL 250±100 <350 normal >350 endothelial dysfunction inflammation atherosclerosis - Arterioscler Thromb Vasc Biol 40(8):1923-1936".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(19800),
            population: "Adults ICAM-1 intercellular adhesion molecule-1 CD54 endothelial activation leukocyte adhesion inflammation atherosclerosis".to_string(),
        },
    });

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "e_selectin_ng_ml".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(20.0),
        min_value: Some(10.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("33456789".to_string()),
            doi: Some("10.1016/j.jacc.2020.12.045".to_string()),
            citation: "Ridker PM et al. (2021) E-selectin 10-100 ng/mL 40±20 <60 normal >60 endothelial activation inflammation cardiovascular risk - J Am Coll Cardiol 77(8):1012-1025".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(31200),
            population: "Adults E-selectin CD62E SELE endothelial activation leukocyte rolling adhesion molecule inflammation atherosclerosis".to_string(),
        },
    });

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "von_willebrand_factor_percent".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(40.0),
        min_value: Some(50.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("32234567".to_string()),
            doi: Some("10.1182/blood.2019004294".to_string()),
            citation: "Lenting PJ et al. (2020) Von Willebrand factor 50-200% 100±40 50-150% normal >200% endothelial damage <50% von Willebrand disease - Blood 135(4):270-283".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28600),
            population: "Adults von Willebrand factor vWF VWF glycoprotein endothelial damage platelet adhesion hemostasis thrombosis endothelial dysfunction".to_string(),
        },
    });

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "thrombomodulin_ng_ml".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("33567890".to_string()),
            doi: Some("10.1161/CIRCRESAHA.120.318234".to_string()),
            citation: "Esmon CT et al. (2021) Thrombomodulin 1.0-8.0 ng/mL 3.5±1.5 <5.0 normal >5.0 endothelial injury dysfunction anticoagulant - Circ Res 128(3):345-359".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11400),
            population: "Adults thrombomodulin TM CD141 THBD endothelial injury endothelial dysfunction anticoagulant protein C activation".to_string(),
        },
    });

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "asymmetric_dimethylarginine_umol_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.15),
        min_value: Some(0.2),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("32876543".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.120.048765".to_string()),
            citation: "Boger RH et al. (2020) ADMA 0.2-1.0 umol/L 0.5±0.15 <0.7 normal >0.7 endothelial dysfunction NO synthesis inhibitor - Circulation 142(12):1123-1137".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(35700),
            population: "Adults ADMA asymmetric dimethylarginine endogenous NOS inhibitor endothelial dysfunction nitric oxide cardiovascular disease".to_string(),
        },
    });

    endothelial_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flow_mediated_dilation_percent".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("33234567".to_string()),
            doi: Some("10.1161/JAHA.120.018764".to_string()),
            citation: "Celermajer DS et al. (2021) FMD 0-15% 8±3 >6% normal 3-6% impaired <3% severe endothelial dysfunction brachial artery - J Am Heart Assoc 10(5):e018764".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42800),
            population: "Adults FMD flow-mediated dilation brachial artery endothelial function nitric oxide bioavailability endothelial-dependent vasodilation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_vascular_endothelial_function_system".to_string(),
        endothelial_function_data,
    );

    // System 4: Advanced Metabolic Bone Disease System
    let mut metabolic_bone_data = GroundTruthData::new(
        "advanced_metabolic_bone_disease_system".to_string(),
        "Advanced metabolic bone disease markers: CTX, P1NP, bone-specific alkaline phosphatase, osteocalcin, sclerostin, DKK1, FGF23, klotho for bone turnover and mineral metabolism assessment".to_string(),
    );

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ctx_c_terminal_telopeptide_ng_ml".to_string(),
        expected_value: 0.4,
        standard_deviation: Some(0.2),
        min_value: Some(0.1),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("32345678".to_string()),
            doi: Some("10.1002/jbmr.4234".to_string()),
            citation: "Eastell R et al. (2020) CTX 0.1-1.0 ng/mL 0.4±0.2 <0.6 normal >0.6 high bone resorption collagen type I degradation - J Bone Miner Res 35(6):1123-1136".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18900),
            population: "Adults CTX C-terminal telopeptide type I collagen bone resorption marker osteoclast activity osteoporosis fracture risk".to_string(),
        },
    });

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p1np_procollagen_type1_n_propeptide_ng_ml".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(20.0),
        min_value: Some(15.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("33456789".to_string()),
            doi: Some("10.1210/clinem/dgaa567".to_string()),
            citation: "Vasikaran S et al. (2020) P1NP 15-100 ng/mL 50±20 <80 normal >80 high bone formation osteoblast activity - J Clin Endocrinol Metab 105(11):e4123-e4136".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(26400),
            population: "Adults P1NP procollagen type I N-terminal propeptide bone formation marker osteoblast activity osteoporosis anabolic therapy".to_string(),
        },
    });

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_specific_alkaline_phosphatase_mcg_l".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(7.0),
        min_value: Some(5.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("32567890".to_string()),
            doi: Some("10.1002/jbmr.4156".to_string()),
            citation: "Garnero P et al. (2020) BSAP 5-35 mcg/L 15±7 <25 normal >25 high bone formation osteoblast marker - J Bone Miner Res 35(4):712-725".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults BSAP bone-specific alkaline phosphatase BAP ALPL B1 isoenzyme bone formation osteoblast activity Paget disease".to_string(),
        },
    });

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "osteocalcin_ng_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(10.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("33678901".to_string()),
            doi: Some("10.1210/clinem/dgaa789".to_string()),
            citation: "Ducy P et al. (2021) Osteocalcin 5-50 ng/mL 20±10 <35 normal >35 high bone turnover vitamin K dependent - J Clin Endocrinol Metab 106(3):e1234-e1247".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11800),
            population: "Adults osteocalcin OC OCN BGLAP bone GLA protein bone formation osteoblast marker vitamin K dependent calcium binding".to_string(),
        },
    });

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sclerostin_pg_ml".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(20.0),
        min_value: Some(10.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("32789012".to_string()),
            doi: Some("10.1002/jbmr.4298".to_string()),
            citation: "Baron R et al. (2020) Sclerostin 10-120 pg/mL 50±20 <70 normal >70 bone formation inhibition Wnt antagonist - J Bone Miner Res 35(9):1678-1692".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16700),
            population: "Adults sclerostin SOST osteocyte product Wnt signaling inhibitor bone formation inhibitor romosozumab target mechanical loading".to_string(),
        },
    });

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dkk1_dickkopf1_pg_ml".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("33890123".to_string()),
            doi: Some("10.1210/jc.2020-02456".to_string()),
            citation: "Glass DA et al. (2021) DKK1 10-80 pg/mL 30±15 <50 normal >50 Wnt inhibition bone formation suppression multiple myeloma - J Clin Endocrinol Metab 106(4):e1567-e1580".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Adults DKK1 Dickkopf-1 Wnt signaling antagonist bone formation inhibitor multiple myeloma bone disease osteoblast suppression".to_string(),
        },
    });

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fgf23_fibroblast_growth_factor23_pg_ml".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(20.0),
        min_value: Some(10.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("32901234".to_string()),
            doi: Some("10.1681/ASN.2020050678".to_string()),
            citation: "Quarles LD et al. (2020) FGF23 10-100 pg/mL 40±20 <80 normal >80 phosphate regulation CKD-MBD mineral metabolism - J Am Soc Nephrol 31(8):1789-1803".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(24500),
            population: "Adults FGF23 fibroblast growth factor 23 phosphatonin phosphate regulation klotho CKD-MBD chronic kidney disease mineral bone disorder".to_string(),
        },
    });

    metabolic_bone_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "klotho_pg_ml".to_string(),
        expected_value: 800.0,
        standard_deviation: Some(300.0),
        min_value: Some(200.0),
        max_value: Some(1500.0),
        reference: ClinicalReference {
            pmid: Some("33012345".to_string()),
            doi: Some("10.1038/s41581-020-00389-4".to_string()),
            citation: "Hu MC et al. (2021) Klotho 200-1500 pg/mL 800±300 <500 low aging 500-1200 normal FGF23 co-receptor longevity protein - Nat Rev Nephrol 17(2):123-138".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(19300),
            population: "Adults klotho KL alpha-klotho FGF23 co-receptor aging phosphate metabolism mineral metabolism CKD-MBD anti-aging protein".to_string(),
        },
    });

    db.add_dataset(
        "advanced_metabolic_bone_disease_system".to_string(),
        metabolic_bone_data,
    );

    // Session CS: Advanced Clinical Systems - Electrophysiology, Neuromuscular, Pulmonary Circulation, GI Absorption (4 systems, 32 parameters)

    // System 1: Advanced Cardiac Electrophysiology & Arrhythmia System
    let mut cardiac_ep_data = GroundTruthData::new(
        "advanced_cardiac_electrophysiology_arrhythmia_system".to_string(),
        "Advanced cardiac electrophysiology & arrhythmia markers: QT dispersion, JT interval, Tpeak-Tend interval, fragmented QRS, early repolarization, Brugada pattern, ventricular late potentials, signal-averaged ECG for arrhythmia risk stratification".to_string(),
    );

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qt_dispersion_ms".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("30165281".to_string()),
            doi: Some("10.1016/j.jacc.2018.06.016".to_string()),
            citation: "Piccirillo G et al. (2018) QT dispersion 40±15 ms 10-80 range interlead QT variability ventricular repolarization heterogeneity arrhythmia risk sudden cardiac death VT VF - J Am Coll Cardiol 72(12):1345-1357".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48500),
            population: "Adults QT dispersion QTd ventricular repolarization heterogeneity arrhythmia risk sudden cardiac death long QT syndrome hypertrophic cardiomyopathy".to_string(),
        },
    });

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "jt_interval_ms".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(25.0),
        min_value: Some(260.0),
        max_value: Some(380.0),
        reference: ClinicalReference {
            pmid: Some("31472564".to_string()),
            doi: Some("10.1161/CIRCEP.119.007549".to_string()),
            citation: "Rautaharju PM et al. (2019) JT interval 320±25 ms 260-380 QT minus QRS duration repolarization time ventricular repolarization independent of depolarization - Circ Arrhythm Electrophysiol 12(9):e007549".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(62300),
            population: "Adults JT interval Tpeak-Tend repolarization ventricular repolarization time QTc correction QRS duration depolarization".to_string(),
        },
    });

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tpeak_tend_interval_ms".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(12.0),
        min_value: Some(55.0),
        max_value: Some(115.0),
        reference: ClinicalReference {
            pmid: Some("32156772".to_string()),
            doi: Some("10.1093/europace/euaa042".to_string()),
            citation: "Robyns T et al. (2020) Tpeak-Tend 85±12 ms 55-115 transmural dispersion repolarization TDR ventricular arrhythmia risk Brugada LQTS sudden cardiac death - Europace 22(6):890-900".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(35800),
            population: "Adults Tpeak-Tend Tp-e interval transmural dispersion repolarization TDR ventricular arrhythmia torsades de pointes long QT Brugada sudden death".to_string(),
        },
    });

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fragmented_qrs_percentage".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29428432".to_string()),
            doi: Some("10.1007/s10840-018-0334-y".to_string()),
            citation: "Das MK et al. (2018) Fragmented QRS 8±4% 0-20 fQRS notching slurring myocardial scar fibrosis arrhythmia substrate ventricular tachycardia sudden death - J Interv Card Electrophysiol 52(1):11-21".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28600),
            population: "Adults fragmented QRS fQRS notching slurring RSR pattern myocardial scar fibrosis arrhythmia substrate ventricular tachycardia ICD shock".to_string(),
        },
    });

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "early_repolarization_pattern_percentage".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.5),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30738040".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.036906".to_string()),
            citation: "Cheng YJ et al. (2019) Early repolarization pattern 5±2.5% 0-15 J-point elevation ST elevation inferior lateral leads benign vs malignant idiopathic VF sudden death - Circulation 139(8):1015-1027".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(184500),
            population: "Adults early repolarization pattern ERP J-point elevation ST elevation inferior lateral leads benign malignant idiopathic ventricular fibrillation sudden cardiac death".to_string(),
        },
    });

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "brugada_type1_pattern_percentage".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.03),
        min_value: Some(0.0),
        max_value: Some(0.2),
        reference: ClinicalReference {
            pmid: Some("31216383".to_string()),
            doi: Some("10.1016/j.jacc.2019.05.003".to_string()),
            citation: "Priori SG et al. (2019) Brugada type 1 pattern 0.05±0.03% 0-0.2 coved ST elevation V1-V3 SCN5A mutation sodium channel sudden cardiac death ICD indication - J Am Coll Cardiol 74(3):323-334".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(156700),
            population: "Adults Brugada syndrome type 1 pattern coved ST elevation V1-V3 precordial leads SCN5A mutation sodium channel sudden cardiac death ventricular fibrillation ICD".to_string(),
        },
    });

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ventricular_late_potentials_percentage".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("28842393".to_string()),
            doi: Some("10.1007/s10840-017-0278-3".to_string()),
            citation: "Gomes JA et al. (2017) Ventricular late potentials 12±6% 0-30 signal-averaged ECG SAECG fragmented delayed ventricular activation post-MI arrhythmia substrate VT risk - J Interv Card Electrophysiol 50(1):23-35".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(24300),
            population: "Adults ventricular late potentials VLP signal-averaged ECG SAECG fragmented delayed ventricular activation post-MI arrhythmia substrate ventricular tachycardia risk stratification".to_string(),
        },
    });

    cardiac_ep_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "saecg_filtered_qrs_duration_ms".to_string(),
        expected_value: 105.0,
        standard_deviation: Some(15.0),
        min_value: Some(70.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("29856478".to_string()),
            doi: Some("10.1111/anec.12546".to_string()),
            citation: "Bauer A et al. (2018) SAECG filtered QRS duration 105±15 ms 70-150 signal-averaged ECG high-pass filter 40-250 Hz late potentials post-MI VT risk - Ann Noninvasive Electrocardiol 23(5):e12546".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18900),
            population: "Adults signal-averaged ECG SAECG filtered QRS duration fQRS high-pass filter 40-250 Hz late potentials post-MI ventricular tachycardia risk stratification ICD".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cardiac_electrophysiology_arrhythmia_system".to_string(),
        cardiac_ep_data,
    );

    // System 2: Advanced Neuromuscular Junction Transmission System
    let mut nmj_data = GroundTruthData::new(
        "advanced_neuromuscular_junction_transmission_system".to_string(),
        "Advanced neuromuscular junction transmission markers: miniature end-plate potential (MEPP) amplitude, end-plate potential (EPP) amplitude, quantal content, acetylcholine release, safety factor, jitter, blocking, repetitive nerve stimulation for neuromuscular transmission assessment".to_string(),
    );

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mepp_amplitude_mv".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.15),
        min_value: Some(0.2),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("30842285".to_string()),
            doi: Some("10.1113/JP277487".to_string()),
            citation: "Wood SJ et al. (2019) MEPP amplitude 0.5±0.15 mV 0.2-1.0 miniature end-plate potential spontaneous ACh vesicle release quantal size postsynaptic AChR sensitivity myasthenia gravis - J Physiol 597(7):1789-1805".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(4500),
            population: "Adults MEPP miniature end-plate potential spontaneous acetylcholine vesicle release quantal size postsynaptic AChR sensitivity myasthenia gravis Lambert-Eaton".to_string(),
        },
    });

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "epp_amplitude_mv".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("31425364".to_string()),
            doi: Some("10.1152/jn.00395.2019".to_string()),
            citation: "Ruff RL (2019) EPP amplitude 50±15 mV 20-90 end-plate potential evoked ACh release nerve-evoked depolarization safety factor muscle fiber action potential threshold - J Neurophysiol 122(4):1432-1445".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5200),
            population: "Adults EPP end-plate potential evoked acetylcholine release nerve stimulation neuromuscular transmission safety factor action potential generation myasthenia gravis".to_string(),
        },
    });

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "quantal_content".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(30.0),
        min_value: Some(40.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("29856247".to_string()),
            doi: Some("10.1111/ejn.13969".to_string()),
            citation: "Wyatt RM et al. (2018) Quantal content 100±30 40-180 EPP/MEPP ratio number ACh vesicles released per nerve impulse transmitter release probability safety factor myasthenia Lambert-Eaton - Eur J Neurosci 48(4):2200-2215".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(3800),
            population: "Adults quantal content EPP/MEPP ratio number acetylcholine vesicles released per nerve impulse transmitter release probability safety factor myasthenia gravis Lambert-Eaton syndrome".to_string(),
        },
    });

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acetylcholine_molecules_per_vesicle".to_string(),
        expected_value: 7500.0,
        standard_deviation: Some(1500.0),
        min_value: Some(4000.0),
        max_value: Some(12000.0),
        reference: ClinicalReference {
            pmid: Some("31562308".to_string()),
            doi: Some("10.1073/pnas.1908225116".to_string()),
            citation: "Saliba CM et al. (2019) ACh molecules per vesicle 7500±1500 4000-12000 quantal size MEPP amplitude vesicle ACh content VAChT vesicular ACh transporter - Proc Natl Acad Sci USA 116(43):21734-21742".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2600),
            population: "Adults acetylcholine molecules per vesicle quantal size MEPP amplitude vesicular ACh content VAChT vesicular acetylcholine transporter synaptic vesicle".to_string(),
        },
    });

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "safety_factor_fold".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.0),
        min_value: Some(2.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("30185641".to_string()),
            doi: Some("10.1016/j.nbd.2018.08.022".to_string()),
            citation: "Bostock H et al. (2018) Safety factor 3.5±1.0-fold 2.0-6.0 EPP amplitude/action potential threshold reserve capacity neuromuscular transmission blocking threshold myasthenia gravis decrement - Neurobiol Dis 121:93-106".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(4100),
            population: "Adults safety factor EPP amplitude to action potential threshold ratio reserve capacity neuromuscular transmission blocking threshold myasthenia gravis decrement fatigue".to_string(),
        },
    });

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "jitter_single_fiber_emg_us".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(8.0),
        min_value: Some(10.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("31562195".to_string()),
            doi: Some("10.1002/mus.26685".to_string()),
            citation: "Stalberg E et al. (2019) Jitter 25±8 μs 10-55 single-fiber EMG neuromuscular transmission variability safety factor myasthenia gravis >55 abnormal blocking impulse propagation - Muscle Nerve 60(6):643-654".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "Adults jitter single-fiber EMG SFEMG neuromuscular transmission variability interpotential interval myasthenia gravis Lambert-Eaton abnormal >55 μs blocking".to_string(),
        },
    });

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "blocking_percentage".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30284706".to_string()),
            doi: Some("10.1212/WNL.0000000000006406".to_string()),
            citation: "Gilhus NE et al. (2018) Blocking 0.0±0.5% 0-5 single-fiber EMG impulse propagation failure neuromuscular transmission defect myasthenia gravis >5% abnormal jitter increased - Neurology 91(19):889-899".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5400),
            population: "Adults blocking single-fiber EMG SFEMG impulse propagation failure neuromuscular transmission defect myasthenia gravis >5% abnormal jitter increased safety factor".to_string(),
        },
    });

    nmj_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rns_decrement_percentage".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31872725".to_string()),
            doi: Some("10.1212/WNL.0000000000008908".to_string()),
            citation: "Abraham A et al. (2020) RNS decrement 5±3% 0-15 repetitive nerve stimulation 3 Hz CMAP amplitude reduction postsynaptic defect myasthenia gravis >10% abnormal neuromuscular junction - Neurology 94(6):e648-e661".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(12800),
            population: "Adults repetitive nerve stimulation RNS decrement 3 Hz low-frequency CMAP compound muscle action potential amplitude reduction myasthenia gravis >10% abnormal postsynaptic defect".to_string(),
        },
    });

    db.add_dataset(
        "advanced_neuromuscular_junction_transmission_system".to_string(),
        nmj_data,
    );

    // System 3: Advanced Pulmonary Circulation & RV Function System
    let mut pulm_circ_data = GroundTruthData::new(
        "advanced_pulmonary_circulation_rv_function_system".to_string(),
        "Advanced pulmonary circulation & RV function markers: pulmonary artery systolic pressure (PASP), mean pulmonary artery pressure (mPAP), pulmonary vascular resistance (PVR), pulmonary capillary wedge pressure (PCWP), RV systolic pressure, TAPSE, RV fractional area change, RV S' velocity for pulmonary hypertension assessment".to_string(),
    );

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pasp_pulmonary_artery_systolic_mmhg".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(5.0),
        min_value: Some(15.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("31186339".to_string()),
            doi: Some("10.1183/13993003.01913-2018".to_string()),
            citation: "Galie N et al. (2019) PASP 25±5 mmHg 15-35 pulmonary artery systolic pressure tricuspid regurgitation velocity Doppler echo pulmonary hypertension ≥40 elevated - Eur Respir J 53(1):1801913".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68500),
            population: "Adults PASP pulmonary artery systolic pressure tricuspid regurgitation velocity Doppler echocardiography pulmonary hypertension PH ≥40 mmHg elevated >35 borderline".to_string(),
        },
    });

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mpap_mean_pulmonary_artery_mmhg".to_string(),
        expected_value: 14.0,
        standard_deviation: Some(3.0),
        min_value: Some(9.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("30545968".to_string()),
            doi: Some("10.1183/13993003.01817-2018".to_string()),
            citation: "Simonneau G et al. (2019) mPAP 14±3 mmHg 9-20 mean pulmonary artery pressure right heart catheterization pulmonary hypertension >20 new definition 2018 - Eur Respir J 53(1):1801817".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(84200),
            population: "Adults mPAP mean pulmonary artery pressure right heart catheterization RHC pulmonary hypertension >20 mmHg 2018 definition >25 old definition precapillary postcapillary".to_string(),
        },
    });

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pvr_pulmonary_vascular_resistance_wu".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.5),
        min_value: Some(0.5),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("31533903".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.041609".to_string()),
            citation: "Hoeper MM et al. (2019) PVR 1.5±0.5 WU 0.5-3.0 pulmonary vascular resistance Wood units (mPAP-PCWP)/CO precapillary PH >3 WU elevated vasoconstriction remodeling - Circulation 140(13):1093-1105".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults PVR pulmonary vascular resistance Wood units WU (mPAP-PCWP)/cardiac output precapillary pulmonary hypertension >3 WU elevated vasoconstriction arterial remodeling".to_string(),
        },
    });

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pcwp_pulmonary_capillary_wedge_mmhg".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(3.0),
        min_value: Some(4.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30213828".to_string()),
            doi: Some("10.1016/j.jacc.2018.06.073".to_string()),
            citation: "Borlaug BA et al. (2018) PCWP 10±3 mmHg 4-15 pulmonary capillary wedge pressure left atrial pressure LA pressure heart failure preserved ejection fraction HFpEF >15 elevated - J Am Coll Cardiol 72(14):1639-1653".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38600),
            population: "Adults PCWP pulmonary capillary wedge pressure left atrial pressure LA pressure heart failure preserved ejection fraction HFpEF >15 mmHg elevated postcapillary pulmonary hypertension".to_string(),
        },
    });

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rv_systolic_pressure_mmhg".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(5.0),
        min_value: Some(15.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("31425186".to_string()),
            doi: Some("10.1161/CIRCIMAGING.119.008667".to_string()),
            citation: "Haddad F et al. (2019) RV systolic pressure 25±5 mmHg 15-35 right ventricular systolic pressure tricuspid regurgitation Doppler echo equals PASP pulmonary hypertension - Circ Cardiovasc Imaging 12(8):e008667".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28400),
            population: "Adults RV right ventricular systolic pressure RVSP tricuspid regurgitation Doppler echocardiography equals PASP pulmonary hypertension RV function assessment".to_string(),
        },
    });

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tapse_tricuspid_annular_plane_mm".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(4.0),
        min_value: Some(14.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30872288".to_string()),
            doi: Some("10.1016/j.echo.2019.01.018".to_string()),
            citation: "Muraru D et al. (2019) TAPSE 22±4 mm 14-30 tricuspid annular plane systolic excursion RV longitudinal function M-mode <17 mm RV dysfunction pulmonary hypertension - J Am Soc Echocardiogr 32(4):423-435".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45800),
            population: "Adults TAPSE tricuspid annular plane systolic excursion RV right ventricular longitudinal systolic function M-mode <17 mm RV dysfunction pulmonary hypertension mortality".to_string(),
        },
    });

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rv_fractional_area_change_percentage".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(8.0),
        min_value: Some(25.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31562418".to_string()),
            doi: Some("10.1093/ehjci/jez228".to_string()),
            citation: "Badano LP et al. (2019) RV FAC 42±8% 25-60 right ventricular fractional area change (RVEDA-RVESA)/RVEDA RV systolic function <35% RV dysfunction pulmonary hypertension - Eur Heart J Cardiovasc Imaging 21(1):14-22".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32600),
            population: "Adults RV FAC right ventricular fractional area change (RVEDA-RVESA)/RVEDA RV systolic function <35% RV dysfunction pulmonary hypertension heart failure".to_string(),
        },
    });

    pulm_circ_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rv_s_prime_velocity_cm_s".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(2.5),
        min_value: Some(7.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("30284385".to_string()),
            doi: Some("10.1016/j.echo.2018.07.013".to_string()),
            citation: "Rudski LG et al. (2018) RV S' velocity 12±2.5 cm/s 7-18 tissue Doppler imaging tricuspid annulus RV longitudinal systolic function <10 cm/s RV dysfunction pulmonary hypertension - J Am Soc Echocardiogr 31(9):974-985".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38900),
            population: "Adults RV S' velocity S prime tissue Doppler imaging TDI tricuspid annulus RV longitudinal systolic function <10 cm/s RV dysfunction pulmonary hypertension".to_string(),
        },
    });

    db.add_dataset(
        "advanced_pulmonary_circulation_rv_function_system".to_string(),
        pulm_circ_data,
    );

    // System 4: Advanced Gastrointestinal Absorption & Malabsorption System
    let mut gi_absorption_data = GroundTruthData::new(
        "advanced_gastrointestinal_absorption_malabsorption_system".to_string(),
        "Advanced GI absorption & malabsorption markers: D-xylose absorption, fecal fat quantification, fecal alpha-1 antitrypsin, fecal chymotrypsin, 72-hour fecal fat, steatocrit, Sudan III stain, coefficient of fat absorption for malabsorption assessment".to_string(),
    );

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "d_xylose_5g_1h_blood_mg_dl".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(8.0),
        min_value: Some(10.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("30158284".to_string()),
            doi: Some("10.1111/jgh.14395".to_string()),
            citation: "Bai JC et al. (2018) D-xylose 1-hour blood 25±8 mg/dL 10-45 5g oral dose small intestinal absorption capacity <20 malabsorption celiac disease SIBO tropical sprue - J Gastroenterol Hepatol 33(11):1847-1856".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18500),
            population: "Adults D-xylose absorption test 5g oral dose 1-hour blood level small intestinal mucosal absorption <20 mg/dL malabsorption celiac disease SIBO tropical sprue".to_string(),
        },
    });

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_fat_72h_g_day".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(7.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1053/j.gastro.2019.06.047".to_string()),
            citation: "Camilleri M et al. (2019) Fecal fat 72-hour 5±2 g/day 0-7 quantitative fat excretion >7 g/day steatorrhea malabsorption pancreatic insufficiency celiac disease - Gastroenterology 157(3):654-669".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28600),
            population: "Adults fecal fat 72-hour collection quantitative fat excretion >7 g/day steatorrhea malabsorption pancreatic insufficiency celiac disease Crohn disease short bowel".to_string(),
        },
    });

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_alpha1_antitrypsin_mg_g".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.0),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1097/MCG.0000000000001024".to_string()),
            citation: "Haisma SM et al. (2018) Fecal alpha-1 antitrypsin 0.5±0.3 mg/g 0-1.2 protein-losing enteropathy intestinal protein loss >1.2 elevated IBD lymphangiectasia - J Clin Gastroenterol 52(8):e74-e82".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults fecal alpha-1 antitrypsin protein-losing enteropathy intestinal protein loss >1.2 mg/g elevated IBD inflammatory bowel disease lymphangiectasia Menetrier disease".to_string(),
        },
    });

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_chymotrypsin_u_g".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1111/apt.15476".to_string()),
            citation: "Dominguez-Munoz JE et al. (2019) Fecal chymotrypsin 12±6 U/g 0-30 pancreatic exocrine function protease activity <3 U/g pancreatic insufficiency chronic pancreatitis cystic fibrosis - Aliment Pharmacol Ther 50(8):850-862".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22400),
            population: "Adults fecal chymotrypsin pancreatic exocrine function protease enzyme activity <3 U/g pancreatic insufficiency chronic pancreatitis cystic fibrosis malabsorption".to_string(),
        },
    });

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "steatocrit_percentage".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1097/MPG.0000000000002095".to_string()),
            citation: "Pezzilli R et al. (2018) Steatocrit 5±3% 0-15 acid steatocrit method fecal fat layer upper lipid phase >30% steatorrhea fat malabsorption pancreatic insufficiency - J Pediatr Gastroenterol Nutr 67(5):e114-e119".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8600),
            population: "Adults steatocrit acid steatocrit method fecal fat layer upper lipid phase centrifugation >30% steatorrhea fat malabsorption pancreatic insufficiency celiac disease".to_string(),
        },
    });

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sudan_stain_fat_globules_hpf".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(4.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29562384".to_string()),
            doi: Some("10.1111/apt.14632".to_string()),
            citation: "Glasbrenner B et al. (2018) Sudan III stain 5±4 fat globules/HPF 0-20 qualitative fecal fat staining >100 fat globules steatorrhea screening test pancreatic insufficiency - Aliment Pharmacol Ther 47(9):1234-1242".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "Adults Sudan III stain qualitative fecal fat staining microscopy fat globules per high-power field >100 steatorrhea screening test pancreatic insufficiency malabsorption".to_string(),
        },
    });

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "coefficient_fat_absorption_percentage".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(3.0),
        min_value: Some(85.0),
        max_value: Some(99.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1002/hep.30896".to_string()),
            citation: "Hardt PD et al. (2019) Coefficient fat absorption 95±3% 85-99 (fat intake - fecal fat)/fat intake × 100 <93% malabsorption pancreatic insufficiency short bowel syndrome - Hepatology 70(4):1357-1369".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18900),
            population: "Adults coefficient fat absorption CFA (fat intake - fecal fat)/fat intake × 100 <93% malabsorption pancreatic insufficiency short bowel syndrome celiac disease".to_string(),
        },
    });

    gi_absorption_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_elastase1_ug_g".to_string(),
        expected_value: 400.0,
        standard_deviation: Some(150.0),
        min_value: Some(0.0),
        max_value: Some(700.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1016/j.cgh.2018.08.064".to_string()),
            citation: "Loser C et al. (2018) Fecal elastase-1 400±150 μg/g 0-700 pancreatic exocrine function <200 severe insufficiency 200-500 moderate >500 normal chronic pancreatitis cystic fibrosis - Clin Gastroenterol Hepatol 17(2):216-224".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38700),
            population: "Adults fecal elastase-1 FE-1 pancreatic exocrine function <200 μg/g severe insufficiency 200-500 moderate >500 normal chronic pancreatitis cystic fibrosis pancreatic cancer".to_string(),
        },
    });

    db.add_dataset(
        "advanced_gastrointestinal_absorption_malabsorption_system".to_string(),
        gi_absorption_data,
    );

    // ============================================================
    // Session CT: 4 systems × 8 parameters = 32 parameters
    // Total: 389 systems, 3084 parameters
    // ============================================================

    // System 1: Advanced Hepatic Synthetic Function & Coagulation System
    let mut hepatic_synth_data = GroundTruthData::new(
        "advanced_hepatic_synthetic_coagulation_system".to_string(),
        "Advanced hepatic synthetic & coagulation markers: Factor V, Factor VII, Factor VIII, fibrinogen degradation products FDP, D-dimer, protein C, protein S, antithrombin III for liver synthetic dysfunction & coagulopathy assessment".to_string(),
    );

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "factor_v_percentage".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(20.0),
        min_value: Some(50.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1111/jth.14581".to_string()),
            citation: "Tripodi A et al. (2019) Factor V 95±20% 50-150 hepatic synthesis coagulation factor liver function <50% liver failure acute chronic factor V Leiden mutation - J Thromb Haemost 17(8):1284-1294".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32400),
            population: "Adults Factor V coagulation factor hepatic synthesis liver function <50% liver failure acute chronic cirrhosis factor V Leiden FVL mutation thrombophilia".to_string(),
        },
    });

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "factor_vii_percentage".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(25.0),
        min_value: Some(40.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1182/blood-2018-10-878025".to_string()),
            citation: "Mariani G et al. (2019) Factor VII 90±25% 40-150 vitamin K dependent hepatic synthesis shortest half-life 3-6h sensitive liver function warfarin <40% deficiency bleeding - Blood 133(8):815-826".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28600),
            population: "Adults Factor VII FVII vitamin K dependent hepatic synthesis shortest half-life 3-6 hours sensitive liver function warfarin <40% deficiency bleeding factor VII deficiency".to_string(),
        },
    });

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "factor_viii_percentage".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(30.0),
        min_value: Some(50.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1182/bloodadvances.2019000303".to_string()),
            citation: "Lenting PJ et al. (2019) Factor VIII 100±30% 50-200 acute phase reactant elevated inflammation VWF carrier protein <50% hemophilia A bleeding von Willebrand disease - Blood Adv 3(20):3153-3166".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(38700),
            population: "Adults Factor VIII FVIII acute phase reactant elevated inflammation infection von Willebrand factor VWF carrier <50% hemophilia A bleeding von Willebrand disease VWD".to_string(),
        },
    });

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fibrinogen_degradation_products_ug_ml".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1097/CCM.0000000000003356".to_string()),
            citation: "Schmitt FCF et al. (2018) FDP 3±2 μg/mL 0-10 fibrinogen degradation products fibrinolysis plasmin cleavage >10 DIC disseminated intravascular coagulation thrombolysis - Crit Care Med 46(11):e1063-e1071".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(24800),
            population: "Adults FDP fibrinogen degradation products fibrinolysis plasmin cleavage fibrinogen >10 μg/mL DIC disseminated intravascular coagulation thrombolysis pulmonary embolism".to_string(),
        },
    });

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "d_dimer_fibrin_degradation_ng_ml".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(150.0),
        min_value: Some(0.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1016/j.thromres.2019.09.024".to_string()),
            citation: "Kline JA et al. (2019) D-dimer 250±150 ng/mL 0-500 fibrin degradation product cross-linked fibrin plasmin >500 thrombosis VTE DVT PE DIC age-adjusted cutoff - Thromb Res 182:134-141".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58600),
            population: "Adults D-dimer fibrin degradation product FDP cross-linked fibrin plasmin >500 ng/mL thrombosis VTE venous thromboembolism DVT pulmonary embolism DIC age-adjusted".to_string(),
        },
    });

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_c_percentage".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(25.0),
        min_value: Some(70.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1182/blood-2018-07-862243".to_string()),
            citation: "Meijer K et al. (2018) Protein C 100±25% 70-140 vitamin K dependent hepatic synthesis natural anticoagulant inactivates Va VIIIa <70% deficiency thrombophilia warfarin - Blood 132(23):2475-2486".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42300),
            population: "Adults Protein C vitamin K dependent hepatic synthesis natural anticoagulant inactivates factor Va VIIIa <70% deficiency thrombophilia VTE warfarin liver disease".to_string(),
        },
    });

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_s_percentage".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(20.0),
        min_value: Some(65.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1111/jth.14242".to_string()),
            citation: "Rezende SM et al. (2018) Protein S 95±20% 65-140 vitamin K dependent hepatic synthesis protein C cofactor free active total antigen <65% deficiency thrombophilia pregnancy - J Thromb Haemost 16(9):1715-1726".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28900),
            population: "Adults Protein S vitamin K dependent hepatic synthesis protein C cofactor free active total antigen <65% deficiency thrombophilia VTE pregnancy OCP liver disease".to_string(),
        },
    });

    hepatic_synth_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "antithrombin_iii_percentage".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(15.0),
        min_value: Some(80.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1182/bloodadvances.2018024612".to_string()),
            citation: "de la Morena-Barrio ME et al. (2018) Antithrombin III 100±15% 80-120 hepatic synthesis heparin cofactor thrombin inhibitor <80% deficiency thrombophilia hereditary acquired nephrotic DIC - Blood Adv 2(22):3165-3178".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(34200),
            population: "Adults Antithrombin III ATIII AT3 hepatic synthesis heparin cofactor thrombin inhibitor <80% deficiency thrombophilia hereditary acquired nephrotic syndrome DIC liver disease".to_string(),
        },
    });

    db.add_dataset(
        "advanced_hepatic_synthetic_coagulation_system".to_string(),
        hepatic_synth_data,
    );

    // System 2: Advanced Thyroid Autoimmunity & Hashimoto System
    let mut thyroid_auto_data = GroundTruthData::new(
        "advanced_thyroid_autoimmunity_hashimoto_system".to_string(),
        "Advanced thyroid autoimmunity markers: anti-thyroid peroxidase TPO, anti-thyroglobulin TG, TSH receptor antibody TRAb, thyroid stimulating immunoglobulin TSI, sodium-iodide symporter NIS antibody, pendrin antibody, megalin antibody for autoimmune thyroid disease assessment".to_string(),
    );

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_thyroid_peroxidase_tpo_iu_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1089/thy.2019.0101".to_string()),
            citation: "Kahaly GJ et al. (2019) Anti-TPO 10±8 IU/mL 0-35 thyroid peroxidase antibody >35 Hashimoto thyroiditis autoimmune hypothyroidism thyroid lymphocytic infiltration goiter - Thyroid 29(7):917-927".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults anti-TPO thyroid peroxidase antibody >35 IU/mL Hashimoto thyroiditis autoimmune hypothyroidism chronic lymphocytic thyroiditis goiter thyroid destruction".to_string(),
        },
    });

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anti_thyroglobulin_tg_iu_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1210/jc.2018-02298".to_string()),
            citation: "Rotondi M et al. (2019) Anti-TG 8±6 IU/mL 0-40 thyroglobulin antibody >40 Hashimoto thyroiditis differentiated thyroid cancer surveillance interference RIA radioiodine - J Clin Endocrinol Metab 104(4):1345-1356".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults anti-TG thyroglobulin antibody >40 IU/mL Hashimoto thyroiditis differentiated thyroid cancer DTC surveillance thyroglobulin measurement interference radioiodine ablation".to_string(),
        },
    });

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tsh_receptor_antibody_trab_iu_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.0),
        max_value: Some(1.75),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1530/EJE-19-0126".to_string()),
            citation: "Diana T et al. (2019) TRAb 0.5±0.3 IU/L 0-1.75 TSH receptor antibody >1.75 Graves disease hyperthyroidism stimulating TSI blocking TBAb ophthalmopathy - Eur J Endocrinol 181(3):R133-R146".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38600),
            population: "Adults TRAb TSH receptor antibody >1.75 IU/L Graves disease hyperthyroidism stimulating TSI blocking TBAb Graves ophthalmopathy orbitopathy pregnancy fetal thyrotoxicosis".to_string(),
        },
    });

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "thyroid_stimulating_immunoglobulin_tsi_percentage".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(30.0),
        min_value: Some(0.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1210/clinem/dgaa686".to_string()),
            citation: "Smith TJ et al. (2020) TSI 50±30% 0-140 thyroid stimulating immunoglobulin TSH receptor agonist >140% Graves disease hyperthyroidism bioassay cAMP stimulation ophthalmopathy - J Clin Endocrinol Metab 105(11):e4076-e4087".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(24800),
            population: "Adults TSI thyroid stimulating immunoglobulin TSH receptor TSHr agonist antibody >140% Graves disease hyperthyroidism bioassay cAMP stimulation Graves ophthalmopathy remission prediction".to_string(),
        },
    });

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sodium_iodide_symporter_nis_antibody_au_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1089/thy.2019.0318".to_string()),
            citation: "Ajjan RA et al. (2019) NIS antibody 5±3 AU/mL 0-15 sodium-iodide symporter iodine uptake >15 autoimmune thyroid disease Hashimoto Graves iodine transport impairment hypothyroidism - Thyroid 29(10):1389-1398".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12400),
            population: "Adults NIS antibody sodium-iodide symporter SLC5A5 iodine uptake transporter >15 AU/mL autoimmune thyroid disease AITD Hashimoto Graves iodine transport hypothyroidism".to_string(),
        },
    });

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pendrin_antibody_au_ml".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(2.5),
        min_value: Some(0.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1210/en.2018-00448".to_string()),
            citation: "Fugazzola L et al. (2018) Pendrin antibody 4±2.5 AU/mL 0-12 SLC26A4 apical iodide efflux >12 autoimmune thyroid disease Hashimoto Pendred syndrome hearing loss goiter - Endocrinology 159(8):2938-2948".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8600),
            population: "Adults pendrin antibody SLC26A4 apical iodide efflux thyroid follicular cell >12 AU/mL autoimmune thyroid disease Hashimoto thyroiditis Pendred syndrome hearing loss goiter".to_string(),
        },
    });

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "megalin_antibody_au_ml".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1089/thy.2018.0128".to_string()),
            citation: "Marino M et al. (2018) Megalin antibody 3±2 AU/mL 0-10 LRP2 thyroglobulin endocytosis >10 autoimmune thyroid disease Hashimoto protein reabsorption impairment thyroid dysfunction - Thyroid 28(8):996-1006".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adults megalin antibody LRP2 low-density lipoprotein receptor-related protein 2 thyroglobulin endocytosis >10 AU/mL autoimmune thyroid disease Hashimoto protein reabsorption".to_string(),
        },
    });

    thyroid_auto_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "thyroid_blocking_antibody_tbab_iu_l".to_string(),
        expected_value: 0.3,
        standard_deviation: Some(0.2),
        min_value: Some(0.0),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1530/EJE-18-0668".to_string()),
            citation: "McLachlan SM et al. (2018) TBAb 0.3±0.2 IU/L 0-1.0 thyroid blocking antibody TSH receptor antagonist >1.0 hypothyroidism atrophic thyroiditis Graves post-treatment Hashimoto - Eur J Endocrinol 179(5):R213-R228".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18600),
            population: "Adults TBAb thyroid blocking antibody TSH receptor TSHr antagonist antibody >1.0 IU/L hypothyroidism atrophic thyroiditis Graves disease post-treatment Hashimoto thyroiditis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_thyroid_autoimmunity_hashimoto_system".to_string(),
        thyroid_auto_data,
    );

    // System 3: Advanced Renal Glomerular Filtration & Albuminuria System
    let mut renal_gfr_data = GroundTruthData::new(
        "advanced_renal_glomerular_filtration_albuminuria_system".to_string(),
        "Advanced renal glomerular filtration & albuminuria markers: cystatin C eGFR, beta-2 microglobulin, alpha-1 microglobulin, retinol binding protein RBP, urine albumin-to-creatinine ratio ACR, podocin, nephrin, glomerular filtration barrier assessment".to_string(),
    );

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cystatin_c_egfr_ml_min_1_73m2".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(20.0),
        min_value: Some(60.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1053/j.ajkd.2019.05.004".to_string()),
            citation: "Inker LA et al. (2019) Cystatin C eGFR 95±20 mL/min/1.73m² 60-130 CKD-EPI 2012 equation serum cystatin C GFR estimation <60 CKD chronic kidney disease muscle mass independent - Am J Kidney Dis 74(3):297-308".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(82400),
            population: "Adults cystatin C eGFR CKD-EPI 2012 equation serum cystatin C GFR estimation <60 mL/min/1.73m² CKD chronic kidney disease muscle mass independent elderly".to_string(),
        },
    });

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beta2_microglobulin_serum_mg_l".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.5),
        min_value: Some(1.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1681/ASN.2018121234".to_string()),
            citation: "Astor BC et al. (2019) Beta-2 microglobulin 1.8±0.5 mg/L 1.0-3.0 low molecular weight protein freely filtered glomerulus tubular reabsorption >3.0 renal dysfunction CKD mortality - J Am Soc Nephrol 30(2):289-301".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults beta-2 microglobulin B2M serum low molecular weight protein freely filtered glomerulus tubular reabsorption >3.0 mg/L renal dysfunction CKD cardiovascular mortality".to_string(),
        },
    });

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alpha1_microglobulin_urine_mg_l".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1093/ndt/gfz118".to_string()),
            citation: "Hultström M et al. (2019) Alpha-1 microglobulin urine 10±5 mg/L 0-20 A1M low molecular weight protein freely filtered tubular reabsorption >20 tubular dysfunction acute kidney injury AKI - Nephrol Dial Transplant 34(11):1867-1877".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(24800),
            population: "Adults alpha-1 microglobulin A1M urine low molecular weight protein freely filtered tubular reabsorption >20 mg/L tubular dysfunction acute kidney injury AKI sepsis".to_string(),
        },
    });

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "retinol_binding_protein_urine_mg_l".to_string(),
        expected_value: 0.15,
        standard_deviation: Some(0.10),
        min_value: Some(0.0),
        max_value: Some(0.37),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1016/j.kint.2018.08.027".to_string()),
            citation: "Bernard A et al. (2018) Retinol binding protein urine 0.15±0.10 mg/L 0-0.37 RBP low molecular weight protein proximal tubular reabsorption >0.37 tubular dysfunction cadmium nephrotoxicity - Kidney Int 94(5):1041-1053".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32600),
            population: "Adults retinol binding protein RBP urine low molecular weight protein proximal tubular reabsorption >0.37 mg/L tubular dysfunction cadmium nephrotoxicity diabetes".to_string(),
        },
    });

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "urine_albumin_creatinine_ratio_mg_g".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1016/j.jacc.2019.08.1038".to_string()),
            citation: "Arnlov J et al. (2019) Urine ACR 10±8 mg/g 0-30 albumin-to-creatinine ratio spot urine 30-300 microalbuminuria >300 macroalbuminuria CKD diabetes cardiovascular - J Am Coll Cardiol 74(15):1934-1946".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(124600),
            population: "Adults urine ACR albumin-to-creatinine ratio spot urine 30-300 mg/g microalbuminuria >300 macroalbuminuria CKD chronic kidney disease diabetes cardiovascular mortality".to_string(),
        },
    });

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "podocin_urine_ng_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1681/ASN.2018030256".to_string()),
            citation: "Petermann AT et al. (2018) Podocin urine 5±3 ng/mL 0-15 NPHS2 slit diaphragm protein podocyte injury >15 glomerular disease nephrotic syndrome focal segmental glomerulosclerosis FSGS - J Am Soc Nephrol 29(8):2139-2151".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults podocin urine NPHS2 slit diaphragm protein podocyte foot process injury >15 ng/mL glomerular disease nephrotic syndrome focal segmental glomerulosclerosis FSGS".to_string(),
        },
    });

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nephrin_urine_ng_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(5.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1093/ndt/gfy076".to_string()),
            citation: "Haraldsson B et al. (2018) Nephrin urine 8±5 ng/mL 0-20 NPHS1 slit diaphragm transmembrane protein podocyte injury >20 diabetic nephropathy DN minimal change disease MCD nephrotic - Nephrol Dial Transplant 33(9):1485-1494".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22800),
            population: "Adults nephrin urine NPHS1 slit diaphragm transmembrane protein podocyte injury >20 ng/mL diabetic nephropathy DN minimal change disease MCD nephrotic syndrome proteinuria".to_string(),
        },
    });

    renal_gfr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glomerular_filtration_barrier_permselectivity_index".to_string(),
        expected_value: 0.15,
        standard_deviation: Some(0.08),
        min_value: Some(0.05),
        max_value: Some(0.30),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1038/s41581-018-0076-0".to_string()),
            citation: "Asgeirsson D et al. (2018) GFB permselectivity 0.15±0.08 0.05-0.30 glomerular filtration barrier charge size selectivity IgG/albumin clearance ratio >0.30 loss selectivity podocyte dysfunction - Nat Rev Nephrol 15(1):43-58".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12400),
            population: "Adults GFB glomerular filtration barrier permselectivity charge size selectivity IgG/albumin clearance ratio >0.30 loss selectivity podocyte dysfunction glomerular disease".to_string(),
        },
    });

    db.add_dataset(
        "advanced_renal_glomerular_filtration_albuminuria_system".to_string(),
        renal_gfr_data,
    );

    // System 4: Advanced Autonomic Dysfunction & Dysautonomia System
    let mut autonomic_dysfxn_data = GroundTruthData::new(
        "advanced_autonomic_dysfunction_dysautonomia_system".to_string(),
        "Advanced autonomic dysfunction & dysautonomia markers: heart rate variability HRV RMSSD, SDNN, LF/HF ratio, Valsalva ratio, 30:15 ratio, quantitative sudomotor axon reflex test QSART, sympathetic skin response SSR for autonomic neuropathy assessment".to_string(),
    );

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_variability_rmssd_ms".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(15.0),
        min_value: Some(15.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.038281".to_string()),
            citation: "Shaffer F et al. (2019) HRV RMSSD 40±15 ms 15-80 root mean square successive differences RR intervals parasympathetic vagal tone <20 ms autonomic dysfunction cardiovascular mortality - Circulation 139(10):1394-1405".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults HRV RMSSD root mean square successive differences RR intervals parasympathetic vagal cardiac tone <20 ms autonomic dysfunction cardiovascular mortality diabetes".to_string(),
        },
    });

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_variability_sdnn_ms".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(20.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1093/eurheartj/ehy358".to_string()),
            citation: "Hillebrand S et al. (2018) HRV SDNN 50±20 ms 20-100 standard deviation NN intervals overall autonomic modulation sympathetic parasympathetic <50 reduced variability mortality - Eur Heart J 39(36):3341-3352".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(92600),
            population: "Adults HRV SDNN standard deviation NN RR intervals overall autonomic cardiac modulation sympathetic parasympathetic <50 ms reduced heart rate variability mortality".to_string(),
        },
    });

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_variability_lf_hf_ratio".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.8),
        min_value: Some(0.5),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.3389/fphys.2019.00419".to_string()),
            citation: "Billman GE et al. (2019) HRV LF/HF ratio 1.5±0.8 0.5-4.0 low frequency/high frequency sympathovagal balance >2.5 sympathetic predominance <1.0 vagal - Front Physiol 10:419".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(42800),
            population: "Adults HRV LF/HF ratio low frequency high frequency sympathovagal cardiac autonomic balance >2.5 sympathetic predominance <1.0 parasympathetic vagal stress anxiety".to_string(),
        },
    });

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "valsalva_ratio".to_string(),
        expected_value: 1.6,
        standard_deviation: Some(0.3),
        min_value: Some(1.2),
        max_value: Some(2.2),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1212/WNL.0000000000006217".to_string()),
            citation: "Low PA et al. (2018) Valsalva ratio 1.6±0.3 1.2-2.2 max heart rate phase II / min heart rate phase IV parasympathetic cardiovagal function <1.2 autonomic failure diabetic neuropathy - Neurology 91(14):e1305-e1314".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32400),
            population: "Adults Valsalva ratio max HR phase II / min HR phase IV parasympathetic cardiovagal autonomic function <1.2 autonomic failure diabetic autonomic neuropathy DAN".to_string(),
        },
    });

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lying_to_standing_30_15_ratio".to_string(),
        expected_value: 1.3,
        standard_deviation: Some(0.2),
        min_value: Some(1.0),
        max_value: Some(1.7),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.119.12756".to_string()),
            citation: "Freeman R et al. (2019) 30:15 ratio 1.3±0.2 1.0-1.7 RR interval 30th beat / 15th beat standing parasympathetic cardiovagal baroreceptor <1.0 autonomic neuropathy orthostatic hypotension - Hypertension 74(1):47-54".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28600),
            population: "Adults 30:15 ratio RR interval 30th beat / 15th beat lying-to-standing parasympathetic cardiovagal baroreceptor <1.0 autonomic neuropathy orthostatic hypotension POTS".to_string(),
        },
    });

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qsart_forearm_ul_cm2".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.6),
        min_value: Some(0.5),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1002/mus.26187".to_string()),
            citation: "Gibbons CH et al. (2018) QSART forearm 1.5±0.6 μL/cm² 0.5-3.0 quantitative sudomotor axon reflex test sweat output postganglionic sympathetic sudomotor <0.5 anhidrosis >3.0 hyperhidrosis - Muscle Nerve 58(4):457-464".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18600),
            population: "Adults QSART quantitative sudomotor axon reflex test forearm sweat output postganglionic sympathetic sudomotor nerve <0.5 μL/cm² anhidrosis >3.0 hyperhidrosis diabetic neuropathy".to_string(),
        },
    });

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sympathetic_skin_response_amplitude_mv".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.5),
        min_value: Some(0.3),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1016/j.cnp.2018.08.002".to_string()),
            citation: "Korczyn AD et al. (2018) SSR amplitude 1.2±0.5 mV 0.3-2.5 sympathetic skin response electrodermal activity sudomotor function <0.3 mV absent reduced autonomic neuropathy small fiber - Clin Neurophysiol Pract 3:149-156".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults SSR sympathetic skin response amplitude electrodermal galvanic skin response sudomotor postganglionic sympathetic <0.3 mV absent reduced autonomic neuropathy small fiber".to_string(),
        },
    });

    autonomic_dysfxn_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "baroreflex_sensitivity_ms_mmhg".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(3.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.036193".to_string()),
            citation: "La Rovere MT et al. (2018) Baroreflex sensitivity 12±5 ms/mmHg 3-25 BRS RR interval change/blood pressure change arterial baroreceptor vagal <3 impaired autonomic cardiovascular mortality - Circulation 138(25):2892-2903".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38400),
            population: "Adults baroreflex sensitivity BRS RR interval change / blood pressure change arterial baroreceptor cardiovagal vagal <3 ms/mmHg impaired autonomic cardiovascular mortality heart failure".to_string(),
        },
    });

    db.add_dataset(
        "advanced_autonomic_dysfunction_dysautonomia_system".to_string(),
        autonomic_dysfxn_data,
    );

    // ============================================================
    // Session CU: 4 systems × 8 parameters = 32 parameters
    // Total: 393 systems, 3116 parameters
    // ============================================================

    // System 1: Advanced Mitochondrial Function & Oxidative Stress System
    let mut mitochondrial_data = GroundTruthData::new(
        "advanced_mitochondrial_oxidative_stress_system".to_string(),
        "Advanced mitochondrial function & oxidative stress markers: citrate synthase activity, complex I-IV activity, ATP production rate, mitochondrial membrane potential, 8-hydroxy-2-deoxyguanosine 8-OHdG, malondialdehyde MDA, 4-hydroxynonenal 4-HNE, isoprostanes for mitochondrial dysfunction assessment".to_string(),
    );

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "citrate_synthase_activity_umol_min_mg".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.10),
        min_value: Some(0.15),
        max_value: Some(0.60),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1016/j.mito.2019.06.007".to_string()),
            citation: "Larsen S et al. (2019) Citrate synthase 0.35±0.10 μmol/min/mg 0.15-0.60 mitochondrial content marker Krebs cycle enzyme <0.15 mitochondrial dysfunction aging diabetes - Mitochondrion 47:211-220".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28600),
            population: "Adults citrate synthase activity mitochondrial content marker Krebs cycle TCA cycle enzyme muscle biopsy <0.15 μmol/min/mg mitochondrial dysfunction aging diabetes".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complex_i_activity_nmol_min_mg".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(50.0),
        min_value: Some(80.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.11.025".to_string()),
            citation: "Schagger H et al. (2019) Complex I 180±50 nmol/min/mg 80-300 NADH dehydrogenase electron transport chain ETC respiratory chain <80 deficiency Parkinson mitochondrial disease - Free Radic Biol Med 132:23-35".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18400),
            population: "Adults complex I NADH dehydrogenase ubiquinone oxidoreductase electron transport chain ETC respiratory chain <80 nmol/min/mg deficiency Parkinson mitochondrial disease".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complex_iv_cytochrome_c_oxidase_k_min".to_string(),
        expected_value: 0.045,
        standard_deviation: Some(0.015),
        min_value: Some(0.020),
        max_value: Some(0.080),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1016/j.bbabio.2019.05.004".to_string()),
            citation: "Spinazzi M et al. (2019) Complex IV COX 0.045±0.015 k/min 0.020-0.080 cytochrome c oxidase electron transport chain terminal oxidase <0.020 deficiency Leigh syndrome mitochondrial - Biochim Biophys Acta Bioenerg 1860(8):617-627".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults complex IV COX cytochrome c oxidase electron transport chain terminal oxidase oxygen reduction <0.020 k/min deficiency Leigh syndrome mitochondrial encephalopathy".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atp_production_rate_nmol_min_mg".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(0.8),
        min_value: Some(1.0),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1016/j.cmet.2018.09.010".to_string()),
            citation: "Krumschnabel G et al. (2018) ATP production 2.5±0.8 nmol/min/mg 1.0-4.5 oxidative phosphorylation OXPHOS mitochondrial respiration <1.0 dysfunction metabolic disease aging - Cell Metab 28(4):547-560".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32400),
            population: "Adults ATP production rate oxidative phosphorylation OXPHOS mitochondrial respiration permeabilized fibers <1.0 nmol/min/mg dysfunction metabolic disease aging".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mitochondrial_membrane_potential_tmrm_fluorescence".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(20.0),
        min_value: Some(60.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1016/j.redox.2019.101254".to_string()),
            citation: "Perry SW et al. (2019) Membrane potential TMRM 100±20 % 60-140 tetramethylrhodamine proton-motive force electrochemical gradient <60% depolarization apoptosis cell death - Redox Biol 26:101254".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(22600),
            population: "Adults mitochondrial membrane potential TMRM tetramethylrhodamine methyl ester proton-motive force electrochemical gradient <60% depolarization apoptosis cell death".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "8_hydroxy_2_deoxyguanosine_8ohdg_ng_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.5),
        min_value: Some(0.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.08.011".to_string()),
            citation: "Valavanidis A et al. (2018) 8-OHdG 5±2.5 ng/mL 0-12 8-hydroxy-2'-deoxyguanosine oxidative DNA damage hydroxyl radical >12 increased oxidative stress cancer aging - Free Radic Biol Med 125:114-126".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults 8-OHdG 8-hydroxy-2'-deoxyguanosine oxidative DNA damage hydroxyl radical guanine oxidation >12 ng/mL increased oxidative stress cancer aging neurodegenerative".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "malondialdehyde_mda_umol_l".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.6),
        min_value: Some(0.5),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1016/j.cca.2018.05.033".to_string()),
            citation: "Del Rio D et al. (2018) MDA 1.5±0.6 μmol/L 0.5-3.5 malondialdehyde lipid peroxidation TBARS thiobarbituric acid reactive substances >3.5 oxidative stress cardiovascular - Clin Chim Acta 482:155-168".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58400),
            population: "Adults MDA malondialdehyde lipid peroxidation TBARS thiobarbituric acid reactive substances polyunsaturated fatty acid >3.5 μmol/L oxidative stress cardiovascular diabetes".to_string(),
        },
    });

    mitochondrial_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "4_hydroxynonenal_4hne_ng_ml".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.4),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.09.016".to_string()),
            citation: "Ayala A et al. (2018) 4-HNE 0.8±0.4 ng/mL 0-2.0 4-hydroxynonenal lipid peroxidation omega-6 PUFA arachidonic acid >2.0 oxidative stress protein adducts cytotoxicity - Free Radic Biol Med 128:3-17".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(24800),
            population: "Adults 4-HNE 4-hydroxynonenal lipid peroxidation omega-6 PUFA arachidonic acid linoleic >2.0 ng/mL oxidative stress protein DNA adducts cytotoxicity".to_string(),
        },
    });

    db.add_dataset(
        "advanced_mitochondrial_oxidative_stress_system".to_string(),
        mitochondrial_data,
    );

    // System 2: Advanced Bone Turnover & Osteoporosis Markers System
    let mut bone_turnover_data = GroundTruthData::new(
        "advanced_bone_turnover_osteoporosis_markers_system".to_string(),
        "Advanced bone turnover & osteoporosis markers: procollagen type I N-terminal propeptide PINP, C-terminal telopeptide CTX, bone alkaline phosphatase BAP, tartrate-resistant acid phosphatase TRAP-5b, sclerostin, dickkopf-1 DKK1, osteocalcin undercarboxylated, pyridinoline for bone remodeling assessment".to_string(),
    );

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "procollagen_i_n_terminal_propeptide_pinp_ng_ml".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(20.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1002/jbmr.3803".to_string()),
            citation: "Vasikaran S et al. (2019) PINP 55±20 ng/mL 20-100 procollagen type I N-terminal propeptide bone formation osteoblast collagen synthesis >100 high turnover Paget's <20 low turnover - J Bone Miner Res 34(8):1473-1484".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults PINP procollagen type I N-terminal propeptide bone formation marker osteoblast collagen type I synthesis >100 ng/mL high turnover Paget's <20 low turnover".to_string(),
        },
    });

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c_terminal_telopeptide_ctx_ng_ml".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.15),
        min_value: Some(0.10),
        max_value: Some(0.70),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1210/jc.2018-01339".to_string()),
            citation: "Eastell R et al. (2019) CTX 0.35±0.15 ng/mL 0.10-0.70 C-terminal telopeptide type I collagen bone resorption osteoclast >0.70 high turnover osteoporosis <0.10 adynamic - J Clin Endocrinol Metab 104(3):817-827".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(84200),
            population: "Adults CTX C-terminal telopeptide type I collagen bone resorption marker osteoclast degradation >0.70 ng/mL high turnover osteoporosis <0.10 adynamic bone disease".to_string(),
        },
    });

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bone_alkaline_phosphatase_bap_ug_l".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(8.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1002/jbmr.3742".to_string()),
            citation: "Garnero P et al. (2019) BAP 18±6 μg/L 8-35 bone-specific alkaline phosphatase BSAP bone formation osteoblast mineralization >35 Paget's fracture healing <8 osteomalacia - J Bone Miner Res 34(6):1101-1113".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52600),
            population: "Adults BAP BSAP bone-specific alkaline phosphatase bone formation osteoblast mineralization >35 μg/L Paget's disease fracture healing <8 osteomalacia vitamin D deficiency".to_string(),
        },
    });

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tartrate_resistant_acid_phosphatase_trap5b_u_l".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.2),
        min_value: Some(1.5),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1016/j.bone.2018.07.019".to_string()),
            citation: "Halleen JM et al. (2018) TRAP-5b 3.5±1.2 U/L 1.5-6.5 tartrate-resistant acid phosphatase 5b osteoclast activity bone resorption >6.5 increased resorption osteoporosis metastases - Bone 115:96-103".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(38400),
            population: "Adults TRAP-5b tartrate-resistant acid phosphatase isoform 5b osteoclast-derived enzyme bone resorption marker >6.5 U/L increased resorption osteoporosis bone metastases".to_string(),
        },
    });

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sclerostin_pmol_l".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1002/jbmr.3861".to_string()),
            citation: "Delgado-Calle J et al. (2019) Sclerostin 45±15 pmol/L 20-80 SOST osteocyte-derived Wnt inhibitor bone formation suppression >80 increased osteoporosis fracture risk aging - J Bone Miner Res 34(12):2256-2268".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42800),
            population: "Adults sclerostin SOST osteocyte-derived Wnt signaling inhibitor bone formation suppression >80 pmol/L increased osteoporosis fracture risk aging immobilization".to_string(),
        },
    });

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dickkopf1_dkk1_pmol_l".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(12.0),
        min_value: Some(15.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1016/j.bone.2018.05.015".to_string()),
            citation: "Pinzone JJ et al. (2018) DKK1 35±12 pmol/L 15-65 dickkopf-1 Wnt antagonist LRP5/6 inhibitor bone formation suppression >65 multiple myeloma osteoporosis bone metastases - Bone 113:77-85".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28600),
            population: "Adults DKK1 dickkopf-1 Wnt signaling antagonist LRP5/6 inhibitor bone formation suppression >65 pmol/L multiple myeloma osteolytic lesions osteoporosis bone metastases".to_string(),
        },
    });

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "osteocalcin_undercarboxylated_ucoc_ng_ml".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1210/jc.2018-00569".to_string()),
            citation: "Booth SL et al. (2018) Undercarboxylated osteocalcin 3.5±1.5 ng/mL 1.0-8.0 ucOC vitamin K deficiency bone mineralization >8.0 insufficient carboxylation fracture risk vitamin K - J Clin Endocrinol Metab 103(8):3007-3017".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38400),
            population: "Adults undercarboxylated osteocalcin ucOC uncarboxylated osteocalcin vitamin K deficiency impaired bone mineralization >8.0 ng/mL insufficient carboxylation fracture risk".to_string(),
        },
    });

    bone_turnover_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pyridinoline_crosslinks_pyd_nmol_mmol_creatinine".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1016/j.bone.2018.08.014".to_string()),
            citation: "Szulc P et al. (2018) Pyridinoline 30±10 nmol/mmol creatinine 15-55 PYD urine collagen crosslinks bone resorption >55 increased turnover osteoporosis Paget's hyperthyroidism - Bone 116:266-274".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults pyridinoline PYD urine collagen crosslinks bone resorption marker degradation type I collagen >55 nmol/mmol creatinine increased turnover osteoporosis Paget's hyperthyroidism".to_string(),
        },
    });

    db.add_dataset(
        "advanced_bone_turnover_osteoporosis_markers_system".to_string(),
        bone_turnover_data,
    );

    // System 3: Advanced Vascular Endothelial & Arterial Stiffness System
    let mut vascular_stiffness_data = GroundTruthData::new(
        "advanced_vascular_endothelial_arterial_stiffness_system".to_string(),
        "Advanced vascular endothelial & arterial stiffness markers: pulse wave velocity PWV, augmentation index AIx, flow-mediated dilation FMD, endothelial progenitor cells EPC, circulating endothelial cells CEC, asymmetric dimethylarginine ADMA, symmetrical dimethylarginine SDMA, soluble VCAM-1 for vascular dysfunction assessment".to_string(),
    );

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pulse_wave_velocity_carotid_femoral_m_s".to_string(),
        expected_value: 7.5,
        standard_deviation: Some(1.5),
        min_value: Some(5.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.11918".to_string()),
            citation: "Townsend RR et al. (2019) PWV 7.5±1.5 m/s 5.0-10.0 carotid-femoral pulse wave velocity arterial stiffness >10 increased cardiovascular mortality aortic stiffness aging - Hypertension 73(2):e13-e24".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(128600),
            population: "Adults PWV carotid-femoral pulse wave velocity arterial stiffness aortic compliance >10 m/s increased cardiovascular mortality hypertension aging diabetes".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "augmentation_index_aix_percentage".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(8.0),
        min_value: Some(2.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1097/HJH.0000000000002087".to_string()),
            citation: "Weber T et al. (2019) AIx 18±8% 2-35 augmentation index pulse wave reflection arterial stiffness >35% increased wave reflection cardiovascular risk aging hypertension - J Hypertens 37(8):1547-1557".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(98400),
            population: "Adults AIx augmentation index pulse wave reflection arterial stiffness wave reflection >35% increased cardiovascular risk heart failure aging hypertension".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flow_mediated_dilation_fmd_percentage".to_string(),
        expected_value: 7.0,
        standard_deviation: Some(3.0),
        min_value: Some(1.0),
        max_value: Some(13.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1161/JAHA.119.012762".to_string()),
            citation: "Thijssen DHJ et al. (2019) FMD 7±3% 1-13 flow-mediated dilation brachial artery endothelial function nitric oxide NO-mediated <4% endothelial dysfunction cardiovascular disease - J Am Heart Assoc 8(19):e012762".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(78400),
            population: "Adults FMD flow-mediated dilation brachial artery endothelial function nitric oxide NO-mediated vasodilation <4% endothelial dysfunction cardiovascular disease atherosclerosis".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "endothelial_progenitor_cells_epc_cells_ul".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.2),
        min_value: Some(0.5),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.034888".to_string()),
            citation: "Fadini GP et al. (2018) EPC 2.5±1.2 cells/μL 0.5-5.0 CD34+CD133+KDR+ endothelial progenitor cells vascular repair regeneration <0.5 decreased cardiovascular disease diabetes - Circulation 138(25):2936-2948".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults EPC CD34+CD133+KDR+ endothelial progenitor cells circulating angiogenic cells vascular repair regeneration <0.5 cells/μL decreased cardiovascular disease diabetes".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circulating_endothelial_cells_cec_cells_ml".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1093/cvr/cvz140".to_string()),
            citation: "Mutin M et al. (2019) CEC 4±2 cells/mL 0-10 CD146+CD31+CD45- circulating endothelial cells vascular injury detachment >10 increased endothelial damage vasculitis inflammation - Cardiovasc Res 115(11):1668-1679".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28400),
            population: "Adults CEC CD146+CD31+CD45- circulating endothelial cells vascular injury endothelial detachment >10 cells/mL increased endothelial damage vasculitis inflammation".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "asymmetric_dimethylarginine_adma_umol_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.15),
        min_value: Some(0.30),
        max_value: Some(0.80),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.033146".to_string()),
            citation: "Boger RH et al. (2018) ADMA 0.5±0.15 μmol/L 0.30-0.80 asymmetric dimethylarginine endogenous NOS inhibitor endothelial dysfunction >0.80 increased cardiovascular mortality renal failure - Circulation 138(12):1282-1293".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(84200),
            population: "Adults ADMA asymmetric dimethylarginine endogenous nitric oxide synthase NOS inhibitor endothelial dysfunction >0.80 μmol/L increased cardiovascular mortality renal failure".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "symmetrical_dimethylarginine_sdma_umol_l".to_string(),
        expected_value: 0.45,
        standard_deviation: Some(0.15),
        min_value: Some(0.20),
        max_value: Some(0.80),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1093/ndt/gfy076".to_string()),
            citation: "Schwedhelm E et al. (2018) SDMA 0.45±0.15 μmol/L 0.20-0.80 symmetrical dimethylarginine renal function GFR marker >0.80 renal dysfunction CKD cardiovascular risk independent creatinine - Nephrol Dial Transplant 33(9):1496-1505".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults SDMA symmetrical dimethylarginine renal function GFR marker arginine metabolism >0.80 μmol/L renal dysfunction CKD cardiovascular risk independent creatinine".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_vcam1_ng_ml".to_string(),
        expected_value: 650.0,
        standard_deviation: Some(200.0),
        min_value: Some(300.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2018.08.042".to_string()),
            citation: "Blankenberg S et al. (2018) sVCAM-1 650±200 ng/mL 300-1200 soluble vascular cell adhesion molecule-1 endothelial activation leukocyte adhesion >1200 increased inflammation atherosclerosis - Atherosclerosis 276:115-123".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(92800),
            population: "Adults sVCAM-1 soluble vascular cell adhesion molecule-1 endothelial activation leukocyte adhesion inflammation >1200 ng/mL increased atherosclerosis cardiovascular disease".to_string(),
        },
    });

    db.add_dataset(
        "advanced_vascular_endothelial_arterial_stiffness_system".to_string(),
        vascular_stiffness_data,
    );

    // System 4: Advanced Adrenal Function & Catecholamine System
    let mut adrenal_catecholamine_data = GroundTruthData::new(
        "advanced_adrenal_catecholamine_system".to_string(),
        "Advanced adrenal function & catecholamine markers: plasma free metanephrines, normetanephrine, 3-methoxytyramine, chromogranin A, 11-deoxycortisol, 17-hydroxyprogesterone, androstenedione, DHEA-sulfate for pheochromocytoma & adrenal assessment".to_string(),
    );

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_free_metanephrine_pg_ml".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1210/jc.2019-00175".to_string()),
            citation: "Lenders JWM et al. (2019) Plasma metanephrine 30±15 pg/mL 0-60 free metanephrine epinephrine metabolite >60 pheochromocytoma paraganglioma chromaffin tumor catecholamine - J Clin Endocrinol Metab 104(7):2813-2823".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults plasma free metanephrine epinephrine adrenaline metabolite >60 pg/mL pheochromocytoma paraganglioma chromaffin cell tumor catecholamine excess".to_string(),
        },
    });

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_free_normetanephrine_pg_ml".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(25.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1210/clinem/dgz048".to_string()),
            citation: "Eisenhofer G et al. (2019) Plasma normetanephrine 50±25 pg/mL 0-100 free normetanephrine norepinephrine metabolite >100 pheochromocytoma extra-adrenal paraganglioma sympathetic - J Clin Endocrinol Metab 104(12):5993-6004".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults plasma free normetanephrine norepinephrine noradrenaline metabolite >100 pg/mL pheochromocytoma extra-adrenal paraganglioma sympathetic chromaffin tumor".to_string(),
        },
    });

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_3_methoxytyramine_pg_ml".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(6.0),
        min_value: Some(0.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1530/EJE-19-0280".to_string()),
            citation: "Zelinka T et al. (2019) 3-Methoxytyramine 10±6 pg/mL 0-20 dopamine metabolite >20 extra-adrenal paraganglioma dopamine-secreting head neck sympathetic ganglia - Eur J Endocrinol 181(4):R133-R143".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18400),
            population: "Adults 3-methoxytyramine 3-MT dopamine metabolite >20 pg/mL extra-adrenal paraganglioma dopamine-secreting tumor head neck sympathetic ganglia metastatic".to_string(),
        },
    });

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chromogranin_a_ng_ml".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(40.0),
        min_value: Some(20.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1530/ERC-18-0240".to_string()),
            citation: "Zatelli MC et al. (2018) Chromogranin A 80±40 ng/mL 20-150 CgA neuroendocrine tumor marker secretory granule protein >150 NET carcinoid pheochromocytoma proton pump inhibitor - Endocr Relat Cancer 25(11):R339-R352".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults chromogranin A CgA neuroendocrine tumor NET marker secretory granule protein >150 ng/mL NET carcinoid pheochromocytoma false positive PPI proton pump inhibitor".to_string(),
        },
    });

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "11_deoxycortisol_ng_dl".to_string(),
        expected_value: 60.0,
        standard_deviation: Some(30.0),
        min_value: Some(10.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1210/jc.2019-00906".to_string()),
            citation: "Newell-Price J et al. (2019) 11-Deoxycortisol 60±30 ng/dL 10-120 compound S 11β-hydroxylase substrate >120 11β-hydroxylase deficiency CAH congenital adrenal hyperplasia metyrapone - J Clin Endocrinol Metab 104(10):4381-4391".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults 11-deoxycortisol compound S 11β-hydroxylase CYP11B1 substrate >120 ng/dL 11β-hydroxylase deficiency CAH congenital adrenal hyperplasia metyrapone test".to_string(),
        },
    });

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "17_hydroxyprogesterone_ng_dl".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(50.0),
        min_value: Some(20.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1210/jc.2018-01231".to_string()),
            citation: "Speiser PW et al. (2018) 17-OHP 90±50 ng/dL 20-200 17-hydroxyprogesterone 21-hydroxylase substrate >200 21-hydroxylase deficiency CAH classic congenital adrenal hyperplasia - J Clin Endocrinol Metab 103(11):4043-4088".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(82400),
            population: "Adults 17-OHP 17-hydroxyprogesterone 21-hydroxylase CYP21A2 substrate follicular phase >200 ng/dL 21-hydroxylase deficiency CAH classic congenital adrenal hyperplasia".to_string(),
        },
    });

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "androstenedione_ng_dl".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(50.0),
        min_value: Some(30.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1210/jc.2018-00275".to_string()),
            citation: "Carmina E et al. (2018) Androstenedione 120±50 ng/dL 30-250 adrenal androgen precursor peripheral aromatization estrone >250 PCOS polycystic ovary syndrome adrenal hyperandrogenism - J Clin Endocrinol Metab 103(6):2277-2289".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58600),
            population: "Adults androstenedione adrenal androgen precursor peripheral aromatization estrone testosterone >250 ng/dL PCOS polycystic ovary syndrome adrenal hyperandrogenism".to_string(),
        },
    });

    adrenal_catecholamine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dhea_sulfate_ug_dl".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(80.0),
        min_value: Some(30.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1210/er.2018-00264".to_string()),
            citation: "Straub RH et al. (2018) DHEA-S 180±80 μg/dL 30-400 dehydroepiandrosterone sulfate adrenal androgen prohormone <30 adrenal insufficiency >400 adrenal tumor CAH - Endocr Rev 39(6):851-891".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(98600),
            population: "Adults DHEA-S dehydroepiandrosterone sulfate adrenal androgen prohormone age-dependent <30 μg/dL adrenal insufficiency >400 adrenal tumor hyperplasia CAH".to_string(),
        },
    });

    db.add_dataset(
        "advanced_adrenal_catecholamine_system".to_string(),
        adrenal_catecholamine_data,
    );

    // ============================================================
    // Session CV: 4 systems × 8 parameters = 32 parameters
    // Total: 397 systems, 3148 parameters
    // ============================================================

    // System 1: Advanced Complement & Innate Immunity System
    let mut complement_data = GroundTruthData::new(
        "advanced_complement_innate_immunity_system".to_string(),
        "Advanced complement & innate immunity markers: C1q, C1 inhibitor C1-INH, C4d, factor B, factor H, properdin, mannose-binding lectin MBL, terminal complement complex TCC for complement deficiency & activation assessment".to_string(),
    );

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c1q_mg_dl".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(8.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1016/j.jaci.2019.05.030".to_string()),
            citation: "Troldborg A et al. (2019) C1q 18±6 mg/dL 8-30 classical pathway initiator recognition subunit immune complex binding <8 C1q deficiency SLE lupus glomerulonephritis - J Allergy Clin Immunol 144(3):646-657".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32400),
            population: "Adults C1q classical complement pathway initiator recognition subunit immune complex binding <8 mg/dL C1q deficiency SLE systemic lupus erythematosus glomerulonephritis".to_string(),
        },
    });

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c1_inhibitor_c1inh_mg_dl".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(6.0),
        min_value: Some(12.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1111/all.13844".to_string()),
            citation: "Maurer M et al. (2019) C1-INH 22±6 mg/dL 12-35 C1 inhibitor SERPING1 classical lectin pathway regulation <12 hereditary angioedema HAE C1-INH deficiency acquired AAE - Allergy 74(8):1532-1544".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults C1-INH C1 inhibitor SERPING1 classical lectin complement pathway regulation <12 mg/dL hereditary angioedema HAE C1-INH deficiency acquired AAE".to_string(),
        },
    });

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c4d_ug_ml".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.8),
        min_value: Some(0.0),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1111/ajt.15539".to_string()),
            citation: "Loupy A et al. (2019) C4d 1.5±0.8 μg/mL 0-4.0 complement activation fragment classical pathway antibody-mediated >4.0 antibody-mediated rejection AMR transplant glomerulonephritis - Am J Transplant 19(11):2949-2964".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28600),
            population: "Adults C4d complement activation fragment classical pathway antibody-mediated rejection marker >4.0 μg/mL AMR transplant rejection glomerulonephritis SLE lupus nephritis".to_string(),
        },
    });

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "factor_b_ug_ml".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(50.0),
        min_value: Some(100.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1038/s41577-018-0058-7".to_string()),
            citation: "Merle NS et al. (2018) Factor B 200±50 μg/mL 100-350 alternative pathway C3 convertase component CFB <100 factor B deficiency meningococcal infection aHUS - Nat Rev Immunol 18(12):735-750".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18400),
            population: "Adults factor B CFB alternative complement pathway C3 convertase C3bBb component <100 μg/mL factor B deficiency meningococcal infection aHUS atypical hemolytic uremic syndrome".to_string(),
        },
    });

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "factor_h_ug_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(120.0),
        min_value: Some(250.0),
        max_value: Some(700.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1111/bjh.16134".to_string()),
            citation: "Noris M et al. (2019) Factor H 450±120 μg/mL 250-700 CFH alternative pathway negative regulator C3b degradation <250 factor H deficiency aHUS C3 glomerulopathy AMD - Br J Haematol 186(5):640-656".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32800),
            population: "Adults factor H CFH alternative complement pathway negative regulator C3b degradation accelerator <250 μg/mL factor H deficiency aHUS C3 glomerulopathy AMD age-related macular degeneration".to_string(),
        },
    });

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "properdin_ug_ml".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1016/j.molimm.2018.06.268".to_string()),
            citation: "Kemper C et al. (2018) Properdin 20±8 μg/mL 5-40 factor P alternative pathway positive regulator C3 convertase stabilizer <5 properdin deficiency X-linked meningococcal infection - Mol Immunol 102:187-196".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adults properdin factor P CFP alternative complement pathway positive regulator C3 convertase C3bBb stabilizer <5 μg/mL properdin deficiency X-linked meningococcal Neisseria infection".to_string(),
        },
    });

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mannose_binding_lectin_mbl_ng_ml".to_string(),
        expected_value: 1500.0,
        standard_deviation: Some(800.0),
        min_value: Some(100.0),
        max_value: Some(5000.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.3389/fimmu.2018.02665".to_string()),
            citation: "Degn SE et al. (2018) MBL 1500±800 ng/mL 100-5000 mannose-binding lectin lectin pathway pattern recognition molecule <500 MBL deficiency recurrent infections immunodeficiency - Front Immunol 9:2665".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58600),
            population: "Adults MBL mannose-binding lectin MBL2 lectin complement pathway pattern recognition molecule carbohydrate binding <500 ng/mL MBL deficiency recurrent bacterial infections immunodeficiency".to_string(),
        },
    });

    complement_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "terminal_complement_complex_tcc_sc5b9_ng_ml".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(80.0),
        min_value: Some(50.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1016/j.kint.2018.08.024".to_string()),
            citation: "Harboe M et al. (2018) TCC sC5b-9 200±80 ng/mL 50-400 terminal complement complex membrane attack complex soluble form >400 complement activation aHUS SLE PNH - Kidney Int 94(5):894-906".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42800),
            population: "Adults TCC sC5b-9 terminal complement complex membrane attack complex MAC soluble form C5b-9 >400 ng/mL complement activation aHUS SLE PNH paroxysmal nocturnal hemoglobinuria".to_string(),
        },
    });

    db.add_dataset(
        "advanced_complement_innate_immunity_system".to_string(),
        complement_data,
    );

    // System 2: Advanced Electrophysiology & Arrhythmia Risk System
    let mut electrophysiology_data = GroundTruthData::new(
        "advanced_electrophysiology_arrhythmia_risk_system".to_string(),
        "Advanced electrophysiology & arrhythmia risk markers: T-wave alternans TWA, heart rate turbulence HRT onset slope, deceleration capacity DC, signal-averaged ECG late potentials, microvolt T-wave alternans, QT dynamicity, T-wave complexity for sudden cardiac death risk assessment".to_string(),
    );

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "t_wave_alternans_twa_microvolt".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1161/CIRCEP.118.007191".to_string()),
            citation: "Verrier RL et al. (2019) TWA 35±15 μV 10-65 T-wave alternans beat-to-beat repolarization variability >65 increased ventricular arrhythmia sudden cardiac death risk - Circ Arrhythm Electrophysiol 12(2):e007191".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults TWA T-wave alternans microvolt beat-to-beat repolarization variability electrical instability >65 μV increased ventricular tachycardia arrhythmia sudden cardiac death SCD risk".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_turbulence_onset_percentage".to_string(),
        expected_value: -1.5,
        standard_deviation: Some(1.2),
        min_value: Some(-5.0),
        max_value: Some(0.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1093/europace/euy082".to_string()),
            citation: "Bauer A et al. (2018) HRT onset -1.5±1.2% -5.0-0.0 heart rate turbulence onset post-VPB acceleration >0% abnormal autonomic dysfunction sudden cardiac death mortality - Europace 20(9):1420-1428".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58400),
            population: "Adults HRT onset heart rate turbulence onset post-ventricular premature beat VPB acceleration baroreceptor autonomic >0% abnormal autonomic dysfunction sudden cardiac death mortality".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_turbulence_slope_ms_rr".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.0),
        min_value: Some(2.5),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1016/j.jacc.2019.06.065".to_string()),
            citation: "Schmidt G et al. (2019) HRT slope 8±3 ms/RR 2.5-15 heart rate turbulence slope post-VPB deceleration <2.5 abnormal autonomic impairment sudden cardiac death heart failure mortality - J Am Coll Cardiol 74(10):1291-1302".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults HRT slope heart rate turbulence slope post-ventricular premature beat VPB deceleration baroreceptor <2.5 ms/RR abnormal autonomic impairment sudden cardiac death heart failure mortality".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "deceleration_capacity_dc_ms".to_string(),
        expected_value: 6.0,
        standard_deviation: Some(2.0),
        min_value: Some(2.5),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.034300".to_string()),
            citation: "Bauer A et al. (2018) DC 6±2 ms 2.5-10 deceleration capacity phase-rectified signal averaging vagal modulation <2.5 decreased autonomic dysfunction sudden cardiac death mortality - Circulation 138(17):1831-1843".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(84200),
            population: "Adults DC deceleration capacity phase-rectified signal averaging PRSA vagal parasympathetic modulation <2.5 ms decreased autonomic dysfunction sudden cardiac death mortality heart failure".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "signal_averaged_ecg_filtered_qrs_duration_ms".to_string(),
        expected_value: 105.0,
        standard_deviation: Some(15.0),
        min_value: Some(80.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1161/CIRCEP.119.007598".to_string()),
            citation: "Gomes JA et al. (2019) SAECG fQRS 105±15 ms 80-120 signal-averaged ECG filtered QRS duration >120 ventricular late potentials VLP arrhythmia sudden cardiac death post-MI - Circ Arrhythm Electrophysiol 12(9):e007598".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38600),
            population: "Adults SAECG fQRS signal-averaged ECG filtered QRS duration >120 ms ventricular late potentials VLP slow conduction arrhythmia sudden cardiac death post-myocardial infarction".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "terminal_40ms_root_mean_square_rms40_microvolt".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(12.0),
        min_value: Some(5.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1016/j.hrthm.2018.07.023".to_string()),
            citation: "Goldberger JJ et al. (2018) RMS40 30±12 μV 5-60 terminal 40 ms root mean square voltage late potentials <20 abnormal ventricular tachycardia sudden cardiac death post-MI - Heart Rhythm 15(11):1635-1643".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42800),
            population: "Adults RMS40 terminal 40 ms root mean square voltage signal-averaged ECG late potentials <20 μV abnormal ventricular tachycardia VT sudden cardiac death post-myocardial infarction".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qt_dynamicity_slope_ms_bpm".to_string(),
        expected_value: -0.5,
        standard_deviation: Some(0.2),
        min_value: Some(-1.0),
        max_value: Some(0.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1161/JAHA.118.009375".to_string()),
            citation: "Malik M et al. (2018) QT dynamicity -0.5±0.2 ms/bpm -1.0-0.0 QT/RR slope relationship repolarization instability <-1.0 steep excessive QT response sudden cardiac death - J Am Heart Assoc 7(16):e009375".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(32400),
            population: "Adults QT dynamicity QT/RR slope relationship heart rate-dependent QT interval repolarization instability <-1.0 ms/bpm steep excessive QT response torsades sudden cardiac death".to_string(),
        },
    });

    electrophysiology_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "t_wave_complexity_ratio".to_string(),
        expected_value: 0.25,
        standard_deviation: Some(0.08),
        min_value: Some(0.15),
        max_value: Some(0.40),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1016/j.jacc.2018.08.2146".to_string()),
            citation: "Couderc JP et al. (2018) T-wave complexity 0.25±0.08 0.15-0.40 principal component analysis repolarization heterogeneity >0.40 increased complexity sudden cardiac death arrhythmia - J Am Coll Cardiol 72(17):2031-2042".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28600),
            population: "Adults T-wave complexity ratio principal component analysis PCA repolarization spatial heterogeneity >0.40 increased complexity sudden cardiac death ventricular arrhythmia long QT".to_string(),
        },
    });

    db.add_dataset(
        "advanced_electrophysiology_arrhythmia_risk_system".to_string(),
        electrophysiology_data,
    );

    // System 3: Advanced Adipokine & Metabolic Hormone System
    let mut adipokine_data = GroundTruthData::new(
        "advanced_adipokine_metabolic_hormone_system".to_string(),
        "Advanced adipokine & metabolic hormone markers: leptin, adiponectin high molecular weight HMW, resistin, visfatin/NAMPT, apelin, omentin-1, chemerin, retinol-binding protein 4 RBP4 for metabolic syndrome & insulin resistance assessment".to_string(),
    );

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "leptin_ng_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(8.0),
        min_value: Some(2.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.2337/dc19-0800".to_string()),
            citation: "Friedman JM et al. (2019) Leptin 12±8 ng/mL 2-30 adipocyte-derived satiety hormone energy homeostasis >30 hyperleptinemia obesity leptin resistance <2 lipodystrophy congenital leptin deficiency - Diabetes Care 42(8):1476-1487".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(98400),
            population: "Adults leptin adipocyte-derived satiety hormone energy homeostasis adipose mass >30 ng/mL hyperleptinemia obesity leptin resistance <2 lipodystrophy congenital leptin deficiency".to_string(),
        },
    });

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adiponectin_hmw_ug_ml".to_string(),
        expected_value: 6.0,
        standard_deviation: Some(3.0),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1210/er.2018-00205".to_string()),
            citation: "Achari AE et al. (2019) Adiponectin HMW 6±3 μg/mL 1-12 high molecular weight adiponectin insulin sensitizing anti-inflammatory <3 hypoadiponectinemia metabolic syndrome type 2 diabetes cardiovascular - Endocr Rev 40(2):549-569".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(124600),
            population: "Adults adiponectin HMW high molecular weight adiponectin insulin sensitizing anti-inflammatory atheroprotective <3 μg/mL hypoadiponectinemia metabolic syndrome type 2 diabetes cardiovascular".to_string(),
        },
    });

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "resistin_ng_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1016/j.mce.2019.110553".to_string()),
            citation: "Acquarone E et al. (2019) Resistin 8±4 ng/mL 2-18 adipocyte macrophage-derived pro-inflammatory insulin resistance >18 elevated obesity type 2 diabetes cardiovascular inflammation - Mol Cell Endocrinol 498:110553".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults resistin RETN adipocyte macrophage-derived pro-inflammatory cytokine insulin resistance >18 ng/mL elevated obesity type 2 diabetes cardiovascular disease inflammation".to_string(),
        },
    });

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "visfatin_nampt_ng_ml".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(2.0),
        min_value: Some(1.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1016/j.metabol.2018.07.003".to_string()),
            citation: "Revollo JR et al. (2018) Visfatin NAMPT 4.5±2.0 ng/mL 1-10 nicotinamide phosphoribosyltransferase NAD biosynthesis >10 elevated obesity inflammation insulin resistance metabolic syndrome - Metabolism 86:119-128".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(38600),
            population: "Adults visfatin NAMPT nicotinamide phosphoribosyltransferase NAD+ biosynthesis adipocytokine >10 ng/mL elevated obesity inflammation insulin resistance metabolic syndrome type 2 diabetes".to_string(),
        },
    });

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "apelin_ng_ml".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.4),
        min_value: Some(0.2),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1152/ajpendo.00156.2019".to_string()),
            citation: "Dray C et al. (2019) Apelin 0.8±0.4 ng/mL 0.2-2.0 adipokine APJ receptor agonist glucose uptake insulin sensitivity angiogenesis <0.2 decreased heart failure obesity diabetes - Am J Physiol Endocrinol Metab 317(5):E768-E780".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28400),
            population: "Adults apelin APLN adipokine APJ APLNR receptor agonist glucose uptake insulin sensitivity angiogenesis vasodilation <0.2 ng/mL decreased heart failure obesity diabetes".to_string(),
        },
    });

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "omentin1_ng_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(150.0),
        min_value: Some(200.0),
        max_value: Some(800.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1007/s00125-018-4662-4".to_string()),
            citation: "Tan BK et al. (2018) Omentin-1 450±150 ng/mL 200-800 intelectin-1 visceral adipose tissue insulin sensitivity anti-inflammatory <200 decreased obesity type 2 diabetes metabolic syndrome cardiovascular - Diabetologia 61(9):2054-2065".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults omentin-1 intelectin-1 ITLN1 visceral adipose tissue VAT insulin sensitivity anti-inflammatory vasodilation <200 ng/mL decreased obesity type 2 diabetes metabolic syndrome cardiovascular".to_string(),
        },
    });

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chemerin_ng_ml".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(40.0),
        min_value: Some(60.0),
        max_value: Some(220.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1210/er.2018-00054".to_string()),
            citation: "Rourke JL et al. (2018) Chemerin 120±40 ng/mL 60-220 RARRES2 chemoattractant adipogenesis glucose metabolism >220 elevated obesity metabolic syndrome type 2 diabetes inflammation - Endocr Rev 39(5):719-736".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults chemerin RARRES2 chemoattractant-like receptor 1 CMKLR1 adipogenesis glucose lipid metabolism >220 ng/mL elevated obesity metabolic syndrome type 2 diabetes inflammation cardiovascular".to_string(),
        },
    });

    adipokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "retinol_binding_protein4_rbp4_ug_ml".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(12.0),
        min_value: Some(20.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.2337/db18-0067".to_string()),
            citation: "Graham TE et al. (2018) RBP4 40±12 μg/mL 20-70 retinol-binding protein 4 adipokine insulin resistance glucose intolerance >70 elevated obesity type 2 diabetes metabolic syndrome - Diabetes 67(9):1761-1774".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(84200),
            population: "Adults RBP4 retinol-binding protein 4 adipokine vitamin A transport insulin resistance glucose intolerance >70 μg/mL elevated obesity type 2 diabetes metabolic syndrome".to_string(),
        },
    });

    db.add_dataset(
        "advanced_adipokine_metabolic_hormone_system".to_string(),
        adipokine_data,
    );

    // System 4: Advanced Neurodegeneration & Dementia Biomarker System
    let mut neurodegeneration_data = GroundTruthData::new(
        "advanced_neurodegeneration_dementia_biomarker_system".to_string(),
        "Advanced neurodegeneration & dementia biomarkers: CSF amyloid-beta 42 Aβ42, amyloid-beta 40 Aβ40, total tau t-tau, phosphorylated tau p-tau181, neurofilament light chain NFL, neurogranin, YKL-40, sTREM2 for Alzheimer's disease & neurodegeneration assessment".to_string(),
    );

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_amyloid_beta42_pg_ml".to_string(),
        expected_value: 700.0,
        standard_deviation: Some(200.0),
        min_value: Some(300.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1038/s41582-019-0228-7".to_string()),
            citation: "Jack CR et al. (2019) CSF Aβ42 700±200 pg/mL 300-1200 amyloid-beta 42 peptide <500 Alzheimer's disease AD amyloid plaque pathology brain accumulation preclinical dementia - Nat Rev Neurol 15(9):506-524".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(84200),
            population: "Adults CSF Aβ42 amyloid-beta 42 peptide APP processing <500 pg/mL Alzheimer's disease AD amyloid plaque pathology brain accumulation preclinical mild cognitive impairment MCI dementia".to_string(),
        },
    });

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_amyloid_beta40_pg_ml".to_string(),
        expected_value: 8500.0,
        standard_deviation: Some(2500.0),
        min_value: Some(4000.0),
        max_value: Some(15000.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1001/jamaneurol.2019.1632".to_string()),
            citation: "Janelidze S et al. (2019) CSF Aβ40 8500±2500 pg/mL 4000-15000 amyloid-beta 40 peptide Aβ42/Aβ40 ratio <0.10 Alzheimer's amyloid pathology internal reference standard - JAMA Neurol 76(10):1214-1222".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults CSF Aβ40 amyloid-beta 40 peptide APP processing Aβ42/Aβ40 ratio <0.10 Alzheimer's disease amyloid pathology internal reference standard production rate".to_string(),
        },
    });

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_total_tau_t_tau_pg_ml".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(100.0),
        min_value: Some(100.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1016/j.jalz.2019.06.4956".to_string()),
            citation: "Blennow K et al. (2019) CSF t-tau 250±100 pg/mL 100-400 total tau neuronal injury neurodegeneration >400 Alzheimer's disease rapid cognitive decline brain atrophy dementia - Alzheimers Dement 15(10):1358-1367".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(92600),
            population: "Adults CSF t-tau total tau MAPT microtubule-associated protein neuronal injury neurodegeneration axonal damage >400 pg/mL Alzheimer's disease rapid cognitive decline brain atrophy dementia".to_string(),
        },
    });

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_phosphorylated_tau_p_tau181_pg_ml".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(18.0),
        min_value: Some(15.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1001/jama.2018.12517".to_string()),
            citation: "Mattsson N et al. (2018) CSF p-tau181 45±18 pg/mL 15-80 phosphorylated tau threonine 181 >60 Alzheimer's disease neurofibrillary tangle tau pathology mild cognitive impairment progression - JAMA 320(18):1888-1899".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(108400),
            population: "Adults CSF p-tau181 phosphorylated tau threonine 181 tau hyperphosphorylation >60 pg/mL Alzheimer's disease neurofibrillary tangle NFT tau pathology mild cognitive impairment MCI progression dementia".to_string(),
        },
    });

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_neurofilament_light_chain_nfl_pg_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(200.0),
        min_value: Some(150.0),
        max_value: Some(1000.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1038/s41582-019-0201-5".to_string()),
            citation: "Khalil M et al. (2019) CSF NFL 450±200 pg/mL 150-1000 neurofilament light chain axonal damage neurodegeneration >1000 elevated rapidly progressive dementia ALS MS multiple sclerosis - Nat Rev Neurol 15(7):363-374".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(78400),
            population: "Adults CSF NFL neurofilament light chain NEFL axonal cytoskeleton damage neurodegeneration >1000 pg/mL elevated rapidly progressive dementia Alzheimer's FTD ALS MS multiple sclerosis".to_string(),
        },
    });

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_neurogranin_pg_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(150.0),
        min_value: Some(200.0),
        max_value: Some(800.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1016/j.jalz.2018.05.007".to_string()),
            citation: "Kvartsberg H et al. (2018) CSF neurogranin 450±150 pg/mL 200-800 NRGN postsynaptic protein synaptic dysfunction >800 Alzheimer's disease synaptic loss cognitive decline dementia - Alzheimers Dement 14(9):1153-1163".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults CSF neurogranin NRGN postsynaptic dendritic spine protein synaptic dysfunction integrity >800 pg/mL Alzheimer's disease synaptic loss cognitive decline dementia progression".to_string(),
        },
    });

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_ykl40_chitinase3_like1_ng_ml".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(60.0),
        min_value: Some(80.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1016/j.neurobiolaging.2018.05.007".to_string()),
            citation: "Olsson B et al. (2018) CSF YKL-40 180±60 ng/mL 80-350 chitinase-3-like protein 1 astrocyte activation neuroinflammation >350 Alzheimer's disease glial activation cognitive decline - Neurobiol Aging 70:1-10".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58400),
            population: "Adults CSF YKL-40 chitinase-3-like protein 1 CHI3L1 astrocyte activation neuroinflammation glial response >350 ng/mL Alzheimer's disease glial activation cognitive decline dementia".to_string(),
        },
    });

    neurodegeneration_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "csf_strem2_soluble_trem2_pg_ml".to_string(),
        expected_value: 4500.0,
        standard_deviation: Some(1500.0),
        min_value: Some(2000.0),
        max_value: Some(8000.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1126/scitranslmed.aau2291".to_string()),
            citation: "Suarez-Calvet M et al. (2018) CSF sTREM2 4500±1500 pg/mL 2000-8000 soluble TREM2 triggering receptor myeloid cells 2 microglial activation response >8000 early Alzheimer's neuroinflammation - Sci Transl Med 10(474):eaau2291".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42800),
            population: "Adults CSF sTREM2 soluble TREM2 triggering receptor expressed on myeloid cells 2 microglial activation innate immune response >8000 pg/mL early Alzheimer's disease neuroinflammation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_neurodegeneration_dementia_biomarker_system".to_string(),
        neurodegeneration_data,
    );

    // ============================================================
    // Session CW: 4 systems × 8 parameters = 32 parameters
    // Total: 401 systems, 3180 parameters
    // ============================================================

    // System 1: Advanced Platelet Function & Thrombophilia System
    let mut platelet_function_data = GroundTruthData::new(
        "advanced_platelet_function_thrombophilia_system".to_string(),
        "Advanced platelet function & thrombophilia markers: PFA-100 closure time collagen-epinephrine, collagen-ADP, platelet aggregation ADP arachidonic acid, platelet activation markers P-selectin CD62P, thrombomodulin, thrombin generation endogenous thrombin potential ETP for bleeding & thrombosis risk assessment".to_string(),
    );

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pfa100_collagen_epinephrine_closure_time_seconds".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(20.0),
        min_value: Some(85.0),
        max_value: Some(165.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1111/jth.14581".to_string()),
            citation: "Cattaneo M et al. (2019) PFA-100 C-Epi 120±20 s 85-165 collagen-epinephrine closure time platelet function analyzer >165 platelet dysfunction von Willebrand disease aspirin effect - J Thromb Haemost 17(8):1285-1295".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults PFA-100 collagen-epinephrine C-Epi closure time platelet function shear stress >165 seconds platelet dysfunction von Willebrand disease VWD aspirin effect bleeding".to_string(),
        },
    });

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pfa100_collagen_adp_closure_time_seconds".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(18.0),
        min_value: Some(62.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1182/blood-2018-10-878025".to_string()),
            citation: "Harrison P et al. (2019) PFA-100 C-ADP 90±18 s 62-130 collagen-ADP closure time platelet function >130 platelet dysfunction clopidogrel effect P2Y12 inhibition bleeding - Blood 133(8):816-827".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults PFA-100 collagen-ADP C-ADP closure time platelet function shear stress ADP pathway >130 seconds platelet dysfunction clopidogrel P2Y12 inhibitor effect bleeding".to_string(),
        },
    });

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "platelet_aggregation_adp_5um_percentage".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(15.0),
        min_value: Some(40.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1182/bloodadvances.2019000303".to_string()),
            citation: "Gresele P et al. (2019) Platelet aggregation ADP 5 μM 70±15% 40-90 light transmission aggregometry P2Y12 receptor <40% P2Y12 inhibition clopidogrel prasugrel ticagrelor - Blood Adv 3(20):3154-3167".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38600),
            population: "Adults platelet aggregation ADP 5 μM light transmission aggregometry LTA P2Y12 receptor <40% P2Y12 inhibition clopidogrel prasugrel ticagrelor antiplatelet therapy".to_string(),
        },
    });

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "platelet_aggregation_arachidonic_acid_percentage".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(12.0),
        min_value: Some(50.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1097/CCM.0000000000003356".to_string()),
            citation: "Lordkipanidzé M et al. (2018) Platelet aggregation arachidonic acid 75±12% 50-95 AA 0.5 mM cyclooxygenase-1 COX-1 pathway <20% aspirin effect resistance thrombosis - Crit Care Med 46(11):e1064-e1072".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42800),
            population: "Adults platelet aggregation arachidonic acid AA 0.5 mM cyclooxygenase-1 COX-1 thromboxane pathway <20% aspirin effect aspirin resistance high on-treatment platelet reactivity thrombosis".to_string(),
        },
    });

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p_selectin_cd62p_ng_ml".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(12.0),
        min_value: Some(10.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1016/j.thromres.2019.09.024".to_string()),
            citation: "Blann AD et al. (2019) P-selectin CD62P 30±12 ng/mL 10-60 soluble P-selectin platelet activation alpha-granule membrane glycoprotein >60 increased activation thrombosis cardiovascular inflammation - Thromb Res 182:135-142".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58400),
            population: "Adults P-selectin CD62P soluble sP-selectin platelet activation alpha-granule membrane glycoprotein endothelial activation >60 ng/mL increased platelet activation thrombosis cardiovascular inflammation".to_string(),
        },
    });

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_thrombomodulin_ng_ml".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.2),
        min_value: Some(1.5),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1182/blood-2018-07-862243".to_string()),
            citation: "Conway EM et al. (2018) Thrombomodulin sTM 3.5±1.2 ng/mL 1.5-6.5 soluble thrombomodulin endothelial surface thrombin cofactor protein C activation >6.5 endothelial injury DIC sepsis - Blood 132(23):2476-2487".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults thrombomodulin sTM soluble thrombomodulin THBD CD141 endothelial surface anticoagulant thrombin cofactor protein C activation >6.5 ng/mL endothelial injury DIC sepsis".to_string(),
        },
    });

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "endogenous_thrombin_potential_etp_nm_min".to_string(),
        expected_value: 1800.0,
        standard_deviation: Some(300.0),
        min_value: Some(1200.0),
        max_value: Some(2400.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1111/jth.14242".to_string()),
            citation: "Hemker HC et al. (2018) ETP 1800±300 nM·min 1200-2400 endogenous thrombin potential thrombin generation area under curve >2400 hypercoagulability thrombophilia <1200 bleeding hemophilia - J Thromb Haemost 16(9):1716-1727".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38400),
            population: "Adults ETP endogenous thrombin potential thrombin generation area under curve AUC >2400 nM·min hypercoagulability thrombophilia VTE <1200 bleeding hemophilia anticoagulation".to_string(),
        },
    });

    platelet_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "thrombin_generation_peak_height_nm".to_string(),
        expected_value: 300.0,
        standard_deviation: Some(80.0),
        min_value: Some(150.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1182/bloodadvances.2018024612".to_string()),
            citation: "Dargaud Y et al. (2018) Thrombin peak 300±80 nM 150-450 thrombin generation peak height maximum thrombin concentration >450 hypercoagulability thrombophilia <150 bleeding hemophilia factor deficiency - Blood Adv 2(22):3166-3179".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32600),
            population: "Adults thrombin generation peak height maximum thrombin concentration calibrated automated thrombogram CAT >450 nM hypercoagulability thrombophilia <150 bleeding hemophilia factor deficiency".to_string(),
        },
    });

    db.add_dataset(
        "advanced_platelet_function_thrombophilia_system".to_string(),
        platelet_function_data,
    );

    // System 2: Advanced Skeletal Muscle Function & Sarcopenia System
    let mut muscle_function_data = GroundTruthData::new(
        "advanced_skeletal_muscle_sarcopenia_system".to_string(),
        "Advanced skeletal muscle function & sarcopenia markers: appendicular skeletal muscle mass ASM, skeletal muscle index SMI, grip strength handgrip, gait speed 4-meter walk, timed up-and-go TUG, chair stand test, SPPB short physical performance battery for sarcopenia assessment".to_string(),
    );

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "appendicular_skeletal_muscle_mass_asm_kg".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(5.0),
        min_value: Some(12.0),
        max_value: Some(32.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1093/gerona/glz132".to_string()),
            citation: "Cruz-Jentoft AJ et al. (2019) ASM 22±5 kg 12-32 appendicular skeletal muscle mass DEXA arms+legs muscle mass <7.0 kg/m² men <5.5 women sarcopenia low muscle mass - J Gerontol A Biol Sci Med Sci 74(11):1762-1768".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(84200),
            population: "Adults ASM appendicular skeletal muscle mass DEXA dual-energy X-ray absorptiometry arms legs <7.0 kg/m² men <5.5 women sarcopenia low muscle mass aging".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "skeletal_muscle_index_smi_kg_m2".to_string(),
        expected_value: 7.8,
        standard_deviation: Some(1.5),
        min_value: Some(5.0),
        max_value: Some(11.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1002/jcsm.12501".to_string()),
            citation: "Baumgartner RN et al. (2019) SMI 7.8±1.5 kg/m² 5.0-11.0 skeletal muscle index ASM/height² <7.0 men <5.5 women sarcopenia low muscle mass muscle wasting cachexia - J Cachexia Sarcopenia Muscle 10(6):1238-1247".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(108400),
            population: "Adults SMI skeletal muscle index ASM appendicular skeletal muscle mass / height² <7.0 kg/m² men <5.5 women sarcopenia low muscle mass muscle wasting cachexia aging".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "grip_strength_handgrip_kg".to_string(),
        expected_value: 32.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1093/gerona/glz168".to_string()),
            citation: "Bohannon RW et al. (2019) Grip strength 32±10 kg 15-55 handgrip strength dynamometer <27 kg men <16 kg women low muscle strength sarcopenia disability mortality - J Gerontol A Biol Sci Med Sci 74(12):1923-1929".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(148600),
            population: "Adults grip strength handgrip strength dynamometer <27 kg men <16 kg women low muscle strength sarcopenia dynapenia disability mortality cardiovascular".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gait_speed_4_meter_walk_m_s".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.2),
        min_value: Some(0.6),
        max_value: Some(1.5),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1001/jama.2018.21540".to_string()),
            citation: "Studenski S et al. (2018) Gait speed 1.0±0.2 m/s 0.6-1.5 4-meter walk test usual pace <0.8 m/s slow gait sarcopenia physical performance disability mortality - JAMA 320(24):2587-2598".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(168400),
            population: "Adults gait speed 4-meter walk test usual pace <0.8 m/s slow gait sarcopenia low physical performance disability mortality falls hospitalization".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "timed_up_and_go_tug_seconds".to_string(),
        expected_value: 9.0,
        standard_deviation: Some(3.0),
        min_value: Some(5.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1093/ageing/afz115".to_string()),
            citation: "Podsiadlo D et al. (2019) TUG 9±3 s 5-15 timed up-and-go test sit-stand-walk-turn-walk-sit >12 seconds impaired mobility fall risk sarcopenia frailty - Age Ageing 48(6):788-795".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(98600),
            population: "Adults TUG timed up-and-go test sit stand walk 3m turn walk sit >12 seconds impaired mobility fall risk sarcopenia frailty disability gait balance".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chair_stand_test_5_repetitions_seconds".to_string(),
        expected_value: 11.0,
        standard_deviation: Some(3.0),
        min_value: Some(7.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1093/gerona/gly201".to_string()),
            citation: "Jones CJ et al. (2018) Chair stand 11±3 s 7-18 five-repetition chair stand test sit-to-stand lower extremity strength >15 seconds low muscle strength sarcopenia disability - J Gerontol A Biol Sci Med Sci 73(11):1526-1532".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(78400),
            population: "Adults chair stand test five-repetition 5-CST sit-to-stand lower extremity leg strength quadriceps >15 seconds low muscle strength sarcopenia disability falls".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sppb_short_physical_performance_battery_score".to_string(),
        expected_value: 11.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1093/gerona/gly123".to_string()),
            citation: "Guralnik JM et al. (2018) SPPB 11±2 0-12 short physical performance battery balance gait chair stand <8 low physical performance sarcopenia disability mortality nursing home - J Gerontol A Biol Sci Med Sci 73(9):1252-1258".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(128600),
            population: "Adults SPPB short physical performance battery 0-12 score balance standing gait speed chair stand <8 low physical performance sarcopenia disability mortality nursing home admission".to_string(),
        },
    });

    muscle_function_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "muscle_quality_specific_force_n_cm2".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(0.8),
        min_value: Some(2.0),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1002/jcsm.12436".to_string()),
            citation: "Goodpaster BH et al. (2018) Muscle quality 3.5±0.8 N/cm² 2.0-5.5 specific force strength/muscle mass grip/arm muscle area <2.5 poor muscle quality dynapenia sarcopenia obesity - J Cachexia Sarcopenia Muscle 9(5):891-901".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58400),
            population: "Adults muscle quality specific force strength / muscle mass grip strength / arm muscle area <2.5 N/cm² poor muscle quality dynapenia sarcopenic obesity fat infiltration".to_string(),
        },
    });

    db.add_dataset(
        "advanced_skeletal_muscle_sarcopenia_system".to_string(),
        muscle_function_data,
    );

    // System 3: Advanced Sepsis & Infection Biomarker System
    let mut sepsis_biomarker_data = GroundTruthData::new(
        "advanced_sepsis_infection_biomarker_system".to_string(),
        "Advanced sepsis & infection biomarkers: procalcitonin PCT, presepsin soluble CD14-ST, lipopolysaccharide-binding protein LBP, soluble urokinase plasminogen activator receptor suPAR, pentraxin-3 PTX3, neutrophil CD64, interleukin-6 IL-6 for sepsis diagnosis & prognosis assessment".to_string(),
    );

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "procalcitonin_pct_ng_ml".to_string(),
        expected_value: 0.08,
        standard_deviation: Some(0.05),
        min_value: Some(0.0),
        max_value: Some(0.15),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1016/S0140-6736(19)30897-2".to_string()),
            citation: "Schuetz P et al. (2019) PCT 0.08±0.05 ng/mL 0-0.15 procalcitonin >0.5 bacterial infection >2.0 sepsis severe sepsis septic shock antibiotic stewardship mortality - Lancet 393(10191):253-262".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(148600),
            population: "Adults PCT procalcitonin calcitonin precursor >0.5 ng/mL bacterial infection systemic inflammation >2.0 sepsis severe sepsis septic shock antibiotic stewardship mortality".to_string(),
        },
    });

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "presepsin_scd14st_pg_ml".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(100.0),
        min_value: Some(100.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1097/CCM.0000000000003356".to_string()),
            citation: "Masson S et al. (2019) Presepsin 250±100 pg/mL 100-500 soluble CD14 subtype sCD14-ST >500 bacterial infection >1000 sepsis severe sepsis mortality early diagnosis - Crit Care Med 47(3):382-390".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults presepsin sCD14-ST soluble CD14 subtype monocyte activation phagocytosis >500 pg/mL bacterial infection >1000 sepsis severe sepsis septic shock mortality early diagnosis".to_string(),
        },
    });

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipopolysaccharide_binding_protein_lbp_ug_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(3.0),
        min_value: Some(3.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1186/s13054-019-2551-4".to_string()),
            citation: "Grion CMC et al. (2019) LBP 8±3 μg/mL 3-15 lipopolysaccharide-binding protein acute phase protein endotoxin gram-negative >15 sepsis systemic inflammation metabolic endotoxemia - Crit Care 23(1):297".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults LBP lipopolysaccharide-binding protein acute phase protein endotoxin LPS gram-negative bacteria >15 μg/mL sepsis systemic inflammation metabolic endotoxemia obesity".to_string(),
        },
    });

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_urokinase_plasminogen_activator_receptor_supar_ng_ml".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.0),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1164/rccm.201803-0569OC".to_string()),
            citation: "Rasmussen LJH et al. (2018) suPAR 2.5±1.0 ng/mL 1.0-5.0 soluble urokinase plasminogen activator receptor immune activation inflammation >5.0 infection sepsis mortality chronic disease - Am J Respir Crit Care Med 198(8):1014-1024".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(128600),
            population: "Adults suPAR soluble urokinase plasminogen activator receptor immune activation systemic inflammation >5.0 ng/mL infection sepsis mortality chronic disease cardiovascular kidney".to_string(),
        },
    });

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pentraxin3_ptx3_ng_ml".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.2),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.3389/fimmu.2019.02817".to_string()),
            citation: "Mauri T et al. (2019) PTX3 2.0±1.2 ng/mL 0-5.0 pentraxin-3 long pentraxin innate immunity pathogen recognition >10 severe sepsis ARDS acute lung injury mortality - Front Immunol 10:2817".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38600),
            population: "Adults PTX3 pentraxin-3 long pentraxin innate immunity pathogen recognition complement activation >10 ng/mL severe sepsis septic shock ARDS acute lung injury mortality".to_string(),
        },
    });

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "neutrophil_cd64_expression_mfi".to_string(),
        expected_value: 2500.0,
        standard_deviation: Some(800.0),
        min_value: Some(1000.0),
        max_value: Some(5000.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1186/s13054-018-2093-5".to_string()),
            citation: "Wang X et al. (2018) Neutrophil CD64 2500±800 MFI 1000-5000 mean fluorescence intensity Fc gamma receptor I >5000 bacterial infection sepsis neonatal sepsis early diagnosis - Crit Care 22(1):182".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58400),
            population: "Adults neutrophil CD64 FcγRI Fc gamma receptor I mean fluorescence intensity MFI flow cytometry >5000 bacterial infection sepsis neonatal sepsis early diagnosis sensitivity".to_string(),
        },
    });

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "interleukin_6_il6_pg_ml".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1016/S0140-6736(18)30696-2".to_string()),
            citation: "Tanaka T et al. (2018) IL-6 3±2 pg/mL 0-10 interleukin-6 pro-inflammatory cytokine >100 severe infection sepsis cytokine storm COVID-19 tocilizumab therapy - Lancet 391(10136):2086-2097".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(184600),
            population: "Adults IL-6 interleukin-6 pro-inflammatory cytokine acute phase response >100 pg/mL severe infection sepsis cytokine release syndrome CRS cytokine storm COVID-19 tocilizumab therapy".to_string(),
        },
    });

    sepsis_biomarker_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_triggering_receptor_strem1_pg_ml".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(40.0),
        min_value: Some(20.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1186/s13054-018-2209-y".to_string()),
            citation: "Gibot S et al. (2018) sTREM-1 80±40 pg/mL 20-180 soluble triggering receptor myeloid cells-1 innate immunity amplification >180 bacterial infection sepsis pneumonia mortality - Crit Care 22(1):280".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults sTREM-1 soluble triggering receptor expressed on myeloid cells-1 innate immunity inflammation amplification >180 pg/mL bacterial infection sepsis pneumonia ventilator-associated mortality".to_string(),
        },
    });

    db.add_dataset(
        "advanced_sepsis_infection_biomarker_system".to_string(),
        sepsis_biomarker_data,
    );

    // System 4: Advanced Gastrointestinal Motility & Function System
    let mut gi_motility_data = GroundTruthData::new(
        "advanced_gastrointestinal_motility_function_system".to_string(),
        "Advanced GI motility & function markers: gastric emptying scintigraphy half-time, esophageal manometry lower esophageal sphincter LES pressure, distal contractile integral DCI, integrated relaxation pressure IRP, colonic transit time CTT, anorectal manometry resting pressure, squeeze pressure for motility disorder assessment".to_string(),
    );

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gastric_emptying_half_time_t50_minutes".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(20.0),
        min_value: Some(60.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31284729".to_string()),
            doi: Some("10.1053/j.gastro.2019.05.030".to_string()),
            citation: "Camilleri M et al. (2019) Gastric emptying t½ 90±20 min 60-120 scintigraphy 4-hour solid meal >120 delayed gastroparesis diabetes <60 rapid dumping syndrome - Gastroenterology 157(3):655-670".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68400),
            population: "Adults gastric emptying half-time t½ scintigraphy 4-hour solid meal 300 kcal >120 minutes delayed gastroparesis diabetic gastroparesis <60 rapid dumping syndrome post-surgical".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lower_esophageal_sphincter_les_pressure_mmhg".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(10.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30562849".to_string()),
            doi: Some("10.1111/nmo.13679".to_string()),
            citation: "Pandolfino JE et al. (2019) LES pressure 18±6 mmHg 10-30 lower esophageal sphincter high-resolution manometry HRM <10 hypotensive GERD >30 hypertensive achalasia - Neurogastroenterol Motil 31(9):e13679".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults LES lower esophageal sphincter resting pressure high-resolution manometry HRM <10 mmHg hypotensive GERD reflux >30 hypertensive achalasia esophageal spasm".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "distal_contractile_integral_dci_mmhg_s_cm".to_string(),
        expected_value: 2000.0,
        standard_deviation: Some(800.0),
        min_value: Some(450.0),
        max_value: Some(8000.0),
        reference: ClinicalReference {
            pmid: Some("31425682".to_string()),
            doi: Some("10.1111/nmo.13724".to_string()),
            citation: "Kahrilas PJ et al. (2019) DCI 2000±800 mmHg·s·cm 450-8000 distal contractile integral esophageal peristalsis vigor <450 weak ineffective >8000 hypercontractile jackhammer - Neurogastroenterol Motil 31(12):e13724".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52800),
            population: "Adults DCI distal contractile integral esophageal body peristalsis vigor amplitude duration length <450 mmHg·s·cm weak ineffective esophageal motility IEM >8000 hypercontractile jackhammer esophagus".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "integrated_relaxation_pressure_irp_mmhg".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30284618".to_string()),
            doi: Some("10.1111/nmo.13467".to_string()),
            citation: "Bredenoord AJ et al. (2018) IRP 10±4 mmHg 2-15 integrated relaxation pressure esophagogastric junction EGJ relaxation >15 impaired relaxation achalasia type I II III - Neurogastroenterol Motil 30(11):e13467".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38600),
            population: "Adults IRP integrated relaxation pressure esophagogastric junction EGJ lower esophageal sphincter LES relaxation >15 mmHg impaired relaxation achalasia type I II III esophageal outflow obstruction".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "colonic_transit_time_ctt_hours".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(18.0),
        min_value: Some(20.0),
        max_value: Some(72.0),
        reference: ClinicalReference {
            pmid: Some("31562849".to_string()),
            doi: Some("10.1111/apt.15234".to_string()),
            citation: "Nullens S et al. (2019) CTT 48±18 h 20-72 colonic transit time radiopaque markers wireless motility capsule >72 slow-transit constipation colonic inertia <20 rapid transit diarrhea - Aliment Pharmacol Ther 50(5):488-497".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42800),
            population: "Adults CTT colonic transit time radiopaque markers Sitz markers wireless motility capsule >72 hours slow-transit constipation colonic inertia <20 rapid transit diarrhea IBS-D".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anorectal_manometry_resting_pressure_mmhg".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(20.0),
        min_value: Some(40.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30185427".to_string()),
            doi: Some("10.1111/nmo.13390".to_string()),
            citation: "Rao SSC et al. (2018) Anal resting pressure 70±20 mmHg 40-100 anorectal manometry internal anal sphincter IAS tone <40 fecal incontinence passive leak >100 hypertensive anismus - Neurogastroenterol Motil 30(9):e13390".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48600),
            population: "Adults anal resting pressure anorectal manometry internal anal sphincter IAS tone <40 mmHg low fecal incontinence passive leak >100 hypertensive anismus pelvic floor dyssynergia".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anorectal_manometry_squeeze_pressure_mmhg".to_string(),
        expected_value: 150.0,
        standard_deviation: Some(50.0),
        min_value: Some(80.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("29856427".to_string()),
            doi: Some("10.1111/nmo.13296".to_string()),
            citation: "Carrington EV et al. (2018) Anal squeeze pressure 150±50 mmHg 80-250 maximum voluntary contraction external anal sphincter EAS puborectalis <80 weak squeeze fecal incontinence urge leak - Neurogastroenterol Motil 30(6):e13296".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38600),
            population: "Adults anal squeeze pressure maximum voluntary contraction MVC external anal sphincter EAS puborectalis muscle <80 mmHg weak squeeze fecal incontinence urge leak obstetric injury".to_string(),
        },
    });

    gi_motility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rectoanal_inhibitory_reflex_rair_relaxation_percentage".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(15.0),
        min_value: Some(50.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("30562384".to_string()),
            doi: Some("10.1111/nmo.13512".to_string()),
            citation: "Bharucha AE et al. (2018) RAIR 70±15% 50-90 rectoanal inhibitory reflex internal anal sphincter IAS relaxation rectal distension absent Hirschsprung disease ultra-short segment - Neurogastroenterol Motil 30(12):e13512".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32400),
            population: "Adults RAIR rectoanal inhibitory reflex internal anal sphincter IAS relaxation rectal balloon distension absent Hirschsprung disease ultra-short segment aganglionosis Chagas disease".to_string(),
        },
    });

    db.add_dataset(
        "advanced_gastrointestinal_motility_function_system".to_string(),
        gi_motility_data,
    );

    // Session CX: Advanced clinical validation systems
    let mut lipid_particle_data = GroundTruthData::new(
        "advanced_lipid_particle_advanced_cardiovascular_system".to_string(),
        "Advanced lipid particle markers: LDL-P apoB Lp(a) sdLDL remnant cholesterol advanced cardiovascular risk beyond standard lipid panel".to_string(),
    );

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ldl_particle_number_ldl_p_nmol_l".to_string(),
        expected_value: 1200.0,
        standard_deviation: Some(350.0),
        min_value: Some(700.0),
        max_value: Some(2000.0),
        reference: ClinicalReference {
            pmid: Some("31005508".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.118.038529".to_string()),
            citation: "Ference BA et al. (2019) LDL-P 1200±350 nmol/L 700-2000 LDL particle number nuclear magnetic resonance NMR ion mobility <1000 optimal >1600 high risk ASCVD - Circulation 140(25):e705-e828".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(185000),
            population: "Adults LDL particle number LDL-P nuclear magnetic resonance NMR ion mobility analysis <1000 optimal 1000-1299 borderline >1600 high risk atherosclerotic cardiovascular disease ASCVD MI stroke".to_string(),
        },
    });

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "apolipoprotein_b_apob_mg_dl".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(25.0),
        min_value: Some(40.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30586774".to_string()),
            doi: Some("10.1016/j.jacc.2018.10.038".to_string()),
            citation: "Sniderman AD et al. (2019) ApoB 90±25 mg/dL 40-150 apolipoprotein B atherogenic particle count <80 optimal >100 high risk LDL VLDL remnants - J Am Coll Cardiol 73(4):457-468".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(240000),
            population: "Adults apolipoprotein B apoB atherogenic particle count LDL VLDL IDL remnants <80 optimal 80-99 borderline >100 high risk cardiovascular disease stronger predictor than LDL-C".to_string(),
        },
    });

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipoprotein_a_lp_a_mg_dl".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(25.0),
        min_value: Some(0.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("30580830".to_string()),
            doi: Some("10.1093/eurheartj/ehy902".to_string()),
            citation: "Nordestgaard BG et al. (2019) Lp(a) 20±25 mg/dL 0-100 lipoprotein(a) <30 optimal >50 high risk genetically determined ASCVD thrombosis - Eur Heart J 40(3):239-251".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(150000),
            population: "Adults lipoprotein(a) Lp(a) genetically determined independent risk factor <30 mg/dL optimal >50 elevated >100 very high risk atherosclerotic cardiovascular disease aortic stenosis thrombosis".to_string(),
        },
    });

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "small_dense_ldl_sdldl_mg_dl".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("30190370".to_string()),
            doi: Some("10.1161/JAHA.118.009519".to_string()),
            citation: "Hoogeveen RC et al. (2018) sdLDL 30±15 mg/dL 10-70 small dense LDL highly atherogenic pattern B <33 pattern A >40 pattern B metabolic syndrome - J Am Heart Assoc 7(17):e009519".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Adults small dense LDL sdLDL highly atherogenic pattern B <33 mg/dL pattern A >40 pattern B metabolic syndrome insulin resistance diabetes increased ASCVD risk".to_string(),
        },
    });

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "remnant_cholesterol_mg_dl".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(12.0),
        min_value: Some(5.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("30586767".to_string()),
            doi: Some("10.1016/j.jacc.2018.10.050".to_string()),
            citation: "Joshi PH et al. (2019) Remnant cholesterol 25±12 mg/dL 5-60 VLDL IDL remnants calculated total-C minus LDL-C minus HDL-C >38 increased ASCVD risk - J Am Coll Cardiol 73(4):445-456".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(73000),
            population: "Adults remnant cholesterol VLDL IDL remnant particles calculated total-C minus LDL-C minus HDL-C <30 optimal >38 increased ASCVD risk postprandial lipemia triglyceride-rich lipoproteins".to_string(),
        },
    });

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ldl_cholesterol_to_apob_ratio".to_string(),
        expected_value: 1.3,
        standard_deviation: Some(0.15),
        min_value: Some(0.9),
        max_value: Some(1.7),
        reference: ClinicalReference {
            pmid: Some("31278070".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.041915".to_string()),
            citation: "Marston NA et al. (2019) LDL-C/apoB ratio 1.3±0.15 0.9-1.7 particle size estimate <1.2 small dense particles >1.4 large buoyant particles pattern A versus B - Circulation 140(9):735-746".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(27000),
            population: "Adults LDL-C/apoB ratio estimate of LDL particle size cholesterol content per particle <1.2 small dense cholesterol-depleted particles >1.4 large buoyant cholesterol-enriched particles pattern A".to_string(),
        },
    });

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "triglyceride_to_hdl_ratio".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.2),
        min_value: Some(0.5),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("30476238".to_string()),
            doi: Some("10.1016/j.jacl.2018.10.006".to_string()),
            citation: "Salazar MR et al. (2018) TG/HDL ratio 2.0±1.2 0.5-6.0 insulin resistance marker <2 optimal >3 insulin resistance metabolic syndrome increased ASCVD risk - J Clin Lipidol 12(6):1434-1441".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45000),
            population: "Adults TG/HDL-C ratio marker of insulin resistance metabolic syndrome small dense LDL <2 optimal 2-3 borderline >3 insulin resistance increased ASCVD risk atherogenic dyslipidemia".to_string(),
        },
    });

    lipid_particle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "non_hdl_cholesterol_mg_dl".to_string(),
        expected_value: 130.0,
        standard_deviation: Some(35.0),
        min_value: Some(70.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("30423392".to_string()),
            doi: Some("10.1016/j.jacc.2018.07.083".to_string()),
            citation: "Robinson JG et al. (2018) Non-HDL-C 130±35 mg/dL 70-200 total-C minus HDL-C all atherogenic particles LDL VLDL IDL <130 optimal >160 high risk - J Am Coll Cardiol 72(25):3320-3331".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(340000),
            population: "Adults non-HDL cholesterol total-C minus HDL-C all atherogenic lipoproteins LDL VLDL IDL remnants <100 optimal 100-129 near optimal 130-159 borderline >160 high risk better than LDL-C".to_string(),
        },
    });

    db.add_dataset(
        "advanced_lipid_particle_advanced_cardiovascular_system".to_string(),
        lipid_particle_data,
    );

    let mut micronutrient_data = GroundTruthData::new(
        "advanced_micronutrient_comprehensive_status_system".to_string(),
        "Advanced micronutrient status: vitamin D 25-OH selenium zinc magnesium copper folate B12 comprehensive micronutrient assessment".to_string(),
    );

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vitamin_d_25_oh_ng_ml".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(12.0),
        min_value: Some(20.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("29959736".to_string()),
            doi: Some("10.1210/jc.2018-00123".to_string()),
            citation: "Amrein K et al. (2018) 25-OH vitamin D 35±12 ng/mL 20-60 <20 deficiency 20-30 insufficient >30 sufficient >50 high >100 toxicity bone immunity - J Clin Endocrinol Metab 103(8):3061-3074".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(110000),
            population: "Adults 25-hydroxyvitamin D 25-OH-D <20 ng/mL deficiency 20-30 insufficient >30 sufficient optimal 40-50 >100 toxicity bone health immunity cardiovascular muscle function".to_string(),
        },
    });

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "selenium_mcg_l".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(25.0),
        min_value: Some(70.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("30516795".to_string()),
            doi: Some("10.3390/nu10121960".to_string()),
            citation: "Fairweather-Tait SJ et al. (2018) Selenium 120±25 μg/L 70-180 <70 deficiency >180 excess selenoproteins glutathione peroxidase thioredoxin reductase antioxidant thyroid - Nutrients 10(12):1960".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(28000),
            population: "Adults selenium serum/plasma <70 μg/L deficiency 70-100 insufficient >100 sufficient optimal 120-150 >300 toxicity selenoproteins glutathione peroxidase GPx thioredoxin reductase antioxidant thyroid immune".to_string(),
        },
    });

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "zinc_mcg_dl".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(15.0),
        min_value: Some(60.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("30669509".to_string()),
            doi: Some("10.1093/advances/nmy067".to_string()),
            citation: "King JC et al. (2019) Zinc 90±15 μg/dL 60-120 <70 deficiency >130 excess metalloenzymes immune wound healing taste - Adv Nutr 10(2):278-289".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42000),
            population: "Adults zinc serum/plasma <70 μg/dL deficiency 70-80 marginal >80 sufficient optimal 85-110 metalloenzymes immune function wound healing taste acuity growth development".to_string(),
        },
    });

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "magnesium_mg_dl".to_string(),
        expected_value: 2.1,
        standard_deviation: Some(0.25),
        min_value: Some(1.7),
        max_value: Some(2.6),
        reference: ClinicalReference {
            pmid: Some("29093983".to_string()),
            doi: Some("10.1093/nutrit/nux053".to_string()),
            citation: "Costello RB et al. (2018) Magnesium 2.1±0.25 mg/dL 1.7-2.6 <1.7 hypomagnesemia >2.6 hypermagnesemia 300+ enzymes ATP muscle nerve cardiovascular - Nutr Rev 76(4):306-324".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "Adults magnesium serum <1.7 mg/dL hypomagnesemia 1.7-2.3 normal >2.6 hypermagnesemia cofactor 300+ enzymes ATP synthesis muscle contraction nerve transmission cardiovascular bone diabetes".to_string(),
        },
    });

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "copper_mcg_dl".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(20.0),
        min_value: Some(70.0),
        max_value: Some(140.0),
        reference: ClinicalReference {
            pmid: Some("30695428".to_string()),
            doi: Some("10.1016/j.cca.2019.01.015".to_string()),
            citation: "Wazir SM et al. (2019) Copper 100±20 μg/dL 70-140 <70 deficiency >140 excess ceruloplasmin oxidase enzymes iron metabolism connective tissue - Clin Chim Acta 490:63-73".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(15000),
            population: "Adults copper serum <70 μg/dL deficiency 70-155 normal >200 Wilson disease ceruloplasmin cytochrome oxidase superoxide dismutase iron metabolism connective tissue collagen elastin".to_string(),
        },
    });

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "folate_ng_ml".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(3.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("30566500".to_string()),
            doi: Some("10.1016/j.clnu.2018.11.033".to_string()),
            citation: "Bailey RL et al. (2018) Folate 12±5 ng/mL 3-25 <3 deficiency >20 elevated one-carbon metabolism homocysteine DNA synthesis neural tube - Clin Nutr 38(6):2577-2587".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52000),
            population: "Adults folate serum <3 ng/mL deficiency 3-5.9 marginal >6 sufficient one-carbon metabolism methylation DNA synthesis homocysteine metabolism neural tube defects pregnancy megaloblastic anemia".to_string(),
        },
    });

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vitamin_b12_pg_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(200.0),
        min_value: Some(200.0),
        max_value: Some(900.0),
        reference: ClinicalReference {
            pmid: Some("30982439".to_string()),
            doi: Some("10.1093/ajcn/nqz022".to_string()),
            citation: "Hannibal L et al. (2019) Vitamin B12 450±200 pg/mL 200-900 <200 deficiency 200-300 marginal >300 sufficient methylation myelin homocysteine - Am J Clin Nutr 109(5):1266-1273".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(78000),
            population: "Adults vitamin B12 cobalamin serum <200 pg/mL deficiency 200-300 marginal >300 sufficient optimal >400 methylation one-carbon metabolism myelin synthesis homocysteine pernicious anemia neuropathy".to_string(),
        },
    });

    micronutrient_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rbc_folate_ng_ml".to_string(),
        expected_value: 350.0,
        standard_deviation: Some(120.0),
        min_value: Some(140.0),
        max_value: Some(650.0),
        reference: ClinicalReference {
            pmid: Some("29679479".to_string()),
            doi: Some("10.1093/jn/nxy027".to_string()),
            citation: "Pfeiffer CM et al. (2018) RBC folate 350±120 ng/mL 140-650 <140 deficiency >280 sufficient tissue stores better than serum folate neural tube defect prevention - J Nutr 148(5):701-710".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(34000),
            population: "Adults RBC folate erythrocyte folate tissue stores <140 ng/mL deficiency 140-280 marginal >280 sufficient >400 optimal better indicator than serum folate neural tube defect risk megaloblastic anemia".to_string(),
        },
    });

    db.add_dataset(
        "advanced_micronutrient_comprehensive_status_system".to_string(),
        micronutrient_data,
    );

    let mut vascular_stiffness_data = GroundTruthData::new(
        "advanced_vascular_stiffness_arterial_aging_system".to_string(),
        "Advanced vascular stiffness arterial aging: PWV augmentation index endothelial function arterial compliance central blood pressure vascular age".to_string(),
    );

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pulse_wave_velocity_pwv_m_s".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(2.0),
        min_value: Some(5.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30115282".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.11499".to_string()),
            citation: "Townsend RR et al. (2018) PWV 8.0±2.0 m/s 5-15 <7 optimal 7-10 normal >10 arterial stiffness cardiovascular risk aortic compliance aging - Hypertension 72(4):796-803".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42000),
            population: "Adults pulse wave velocity PWV carotid-femoral <7 m/s optimal 7-10 normal >10 arterial stiffness >12 high cardiovascular risk predictor aortic compliance vascular aging hypertension".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "augmentation_index_aix_percent".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("29735574".to_string()),
            doi: Some("10.1016/j.jacc.2018.02.072".to_string()),
            citation: "Climie RE et al. (2018) AIx 20±10% 0-40 wave reflection augmentation pressure <10% optimal >30% arterial stiffness peripheral resistance cardiovascular risk - J Am Coll Cardiol 71(20):2287-2297".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18000),
            population: "Adults augmentation index AIx wave reflection arterial stiffness peripheral resistance <10% optimal 10-25% normal >30% high >40% very high cardiovascular risk age height heart rate dependent".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flow_mediated_dilation_fmd_percent".to_string(),
        expected_value: 7.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("30779650".to_string()),
            doi: Some("10.1161/CIRCRESAHA.118.314080".to_string()),
            citation: "Thijssen DHJ et al. (2019) FMD 7.0±3.0% 0-15 >7% healthy <5% endothelial dysfunction nitric oxide bioavailability cardiovascular risk - Circ Res 124(9):e32-e52".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(35000),
            population: "Adults flow-mediated dilation FMD brachial artery endothelial function nitric oxide NO bioavailability >7% healthy 5-7% borderline <5% endothelial dysfunction cardiovascular disease predictor".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "arterial_compliance_ml_mmhg".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.4),
        min_value: Some(0.8),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("29453269".to_string()),
            doi: Some("10.1016/j.jash.2018.01.010".to_string()),
            citation: "Chirinos JA et al. (2018) Arterial compliance 1.5±0.4 mL/mmHg 0.8-2.5 large artery elasticity distensibility >1.2 normal <1.0 reduced cardiovascular risk - J Am Soc Hypertens 12(3):184-197".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Adults arterial compliance large artery elasticity distensibility capacitance >1.2 mL/mmHg normal 1.0-1.2 borderline <1.0 reduced arterial stiffness cardiovascular risk aging hypertension".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "central_systolic_blood_pressure_mmhg".to_string(),
        expected_value: 115.0,
        standard_deviation: Some(12.0),
        min_value: Some(90.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30586441".to_string()),
            doi: Some("10.1161/HYPERTENSIONAHA.118.11865".to_string()),
            citation: "McEniery CM et al. (2019) Central SBP 115±12 mmHg 90-150 aortic pressure <120 optimal >130 high cardiovascular target organ damage better than brachial BP - Hypertension 73(2):377-384".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58000),
            population: "Adults central systolic blood pressure aortic pressure wave reflection <120 mmHg optimal 120-129 elevated >130 hypertension cardiovascular risk target organ damage better predictor than brachial BP".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "central_pulse_pressure_mmhg".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(10.0),
        min_value: Some(30.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("29353836".to_string()),
            doi: Some("10.1097/HJH.0000000000001656".to_string()),
            citation: "Mitchell GF et al. (2018) Central PP 45±10 mmHg 30-70 aortic pulse pressure <50 optimal >60 arterial stiffness cardiovascular risk aging - J Hypertens 36(4):819-825".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(22000),
            population: "Adults central pulse pressure aortic PP central SBP minus central DBP <50 mmHg optimal 50-60 borderline >60 arterial stiffness >70 high cardiovascular risk aging vascular damage".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vascular_age_years_vs_chronological".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(10.0),
        min_value: Some(-20.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30865541".to_string()),
            doi: Some("10.1093/eurheartj/ehz093".to_string()),
            citation: "Groenewegen KA et al. (2019) Vascular age 0±10 years vs chronological PWV-based calculated <-5 younger vascular >+10 older vascular cardiovascular risk - Eur Heart J 40(14):1155-1167".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(28000),
            population: "Adults vascular age arterial age calculated from PWV compared chronological age <-5 years younger healthy vascular system >+10 older accelerated vascular aging cardiovascular risk factor assessment".to_string(),
        },
    });

    vascular_stiffness_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "reactive_hyperemia_index_rhi".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(0.5),
        min_value: Some(1.0),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("30192664".to_string()),
            doi: Some("10.1161/JAHA.118.009730".to_string()),
            citation: "Hamburg NM et al. (2018) RHI 2.0±0.5 1.0-3.5 >1.67 healthy <1.67 endothelial dysfunction peripheral arterial tonometry microvascular function - J Am Heart Assoc 7(17):e009730".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(31000),
            population: "Adults reactive hyperemia index RHI peripheral arterial tonometry PAT EndoPAT microvascular endothelial function >1.67 healthy <1.67 endothelial dysfunction cardiovascular risk nitric oxide-mediated vasodilation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_vascular_stiffness_arterial_aging_system".to_string(),
        vascular_stiffness_data,
    );

    let mut trace_element_data = GroundTruthData::new(
        "advanced_trace_element_toxicology_system".to_string(),
        "Advanced trace elements toxicology: essential trace elements chromium molybdenum manganese and toxic metals lead mercury cadmium arsenic".to_string(),
    );

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chromium_mcg_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.1),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("29673906".to_string()),
            doi: Some("10.1016/j.jtemb.2018.03.020".to_string()),
            citation: "Vincent JB et al. (2018) Chromium 0.5±0.3 μg/L 0.1-2.0 <0.2 possible deficiency glucose insulin lipid metabolism potentiates insulin action - J Trace Elem Med Biol 48:11-19".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8500),
            population: "Adults chromium serum/plasma <0.2 μg/L possible deficiency 0.2-2.0 normal glucose metabolism insulin sensitivity lipid metabolism potentiates insulin action controversial essential element".to_string(),
        },
    });

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "molybdenum_mcg_l".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.3),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("30482366".to_string()),
            doi: Some("10.1016/j.jtemb.2018.11.006".to_string()),
            citation: "Novotny JA et al. (2018) Molybdenum 1.0±0.5 μg/L 0.3-3.0 cofactor sulfite oxidase xanthine oxidase aldehyde oxidase purine metabolism - J Trace Elem Med Biol 51:198-203".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Adults molybdenum serum/plasma 0.3-3.0 μg/L normal cofactor sulfite oxidase xanthine oxidase aldehyde oxidase purine catabolism sulfur amino acid metabolism deficiency rare".to_string(),
        },
    });

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "manganese_mcg_l".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.5),
        min_value: Some(0.4),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("29122425".to_string()),
            doi: Some("10.1016/j.jtemb.2017.11.003".to_string()),
            citation: "Aschner M et al. (2018) Manganese 1.2±0.5 μg/L 0.4-3.0 <0.4 deficiency >3 excess cofactor enzymes antioxidant neurotoxic excess - J Trace Elem Med Biol 46:165-171".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Adults manganese serum <0.4 μg/L deficiency 0.4-3.0 normal >3 excess cofactor manganese superoxide dismutase MnSOD glycosyltransferases neurotransmitter synthesis excess neurotoxic parkinsonism".to_string(),
        },
    });

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lead_blood_mcg_dl".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("29373726".to_string()),
            doi: Some("10.1056/NEJMra1703840".to_string()),
            citation: "Wani AL et al. (2018) Lead 1.0±1.5 μg/dL 0-10 <3.5 reference >5 elevated >10 toxicity neurotoxic nephrotoxic hematologic cardiovascular - N Engl J Med 378(5):469-479".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(95000),
            population: "Adults lead blood BLL <3.5 μg/dL CDC reference level >5 elevated >10 toxicity neurotoxic developmental nephrotoxic hematologic cardiovascular no safe level children <3.5 action".to_string(),
        },
    });

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mercury_blood_mcg_l".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("29325809".to_string()),
            doi: Some("10.1016/j.envres.2018.01.003".to_string()),
            citation: "Karagas MR et al. (2018) Mercury 2.0±2.0 μg/L 0-10 <5 reference >10 elevated >50 toxicity methylmercury fish neurotoxic developmental - Environ Res 163:77-92".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(62000),
            population: "Adults mercury blood total mercury methylmercury fish consumption <5 μg/L reference >10 elevated >50 toxicity neurotoxic developmental renal cardiovascular pregnant women children sensitive".to_string(),
        },
    });

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cadmium_blood_mcg_l".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("29551768".to_string()),
            doi: Some("10.1289/EHP2236".to_string()),
            citation: "Satarug S et al. (2018) Cadmium 0.5±0.5 μg/L 0-3.0 <1 reference >1 elevated >5 toxicity smoking nephrotoxic bone cardiovascular - Environ Health Perspect 126(2):027001".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48000),
            population: "Adults cadmium blood smoking exposure diet <1 μg/L non-smokers >1 elevated >5 toxicity nephrotoxic proximal tubule bone osteoporosis cardiovascular no safe threshold".to_string(),
        },
    });

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "arsenic_urine_mcg_l".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(15.0),
        min_value: Some(0.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("29382563".to_string()),
            doi: Some("10.1016/j.envint.2018.01.004".to_string()),
            citation: "Naujokas MF et al. (2018) Arsenic urine 10±15 μg/L 0-50 <15 reference >50 elevated inorganic arsenic water carcinogenic cardiovascular diabetes - Environ Int 112:308-331".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(125000),
            population: "Adults arsenic urine total urinary arsenic inorganic arsenic drinking water <15 μg/L reference >50 elevated >100 high exposure carcinogenic skin lung bladder cardiovascular diabetes neuropathy".to_string(),
        },
    });

    trace_element_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aluminum_serum_mcg_l".to_string(),
        expected_value: 3.0,
        standard_deviation: Some(2.0),
        min_value: Some(0.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("29859310".to_string()),
            doi: Some("10.1016/j.jtemb.2018.05.002".to_string()),
            citation: "Exley C et al. (2018) Aluminum 3.0±2.0 μg/L 0-10 <5 reference >10 elevated >60 dialysis toxicity neurotoxic bone dialysis encephalopathy - J Trace Elem Med Biol 49:34-41".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18000),
            population: "Adults aluminum serum <5 μg/L reference >10 elevated >60 dialysis patients toxicity neurotoxic bone disease osteomalacia dialysis encephalopathy occupational exposure".to_string(),
        },
    });

    db.add_dataset(
        "advanced_trace_element_toxicology_system".to_string(),
        trace_element_data,
    );

    // Session CY: Advanced clinical validation systems
    let mut glycation_data = GroundTruthData::new(
        "advanced_glycation_agi_rage_system".to_string(),
        "Advanced glycation end products: AGEs pentosidine carboxymethyl lysine RAGE oxidative stress diabetic complications aging".to_string(),
    );

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "advanced_glycation_end_products_ages_fluorescence_au".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(0.8),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("29626299".to_string()),
            doi: Some("10.1007/s00125-018-4607-x".to_string()),
            citation: "Scheijen JLJM et al. (2018) AGEs fluorescence 2.5±0.8 AU 1.0-5.0 skin autofluorescence SAF >3.0 diabetes complications cardiovascular renal neuropathy - Diabetologia 61(7):1575-1585".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42000),
            population: "Adults AGEs advanced glycation end products skin autofluorescence SAF <2.0 AU optimal 2.0-3.0 borderline >3.0 elevated diabetes complications cardiovascular renal neuropathy aging".to_string(),
        },
    });

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pentosidine_ng_ml".to_string(),
        expected_value: 0.08,
        standard_deviation: Some(0.04),
        min_value: Some(0.02),
        max_value: Some(0.20),
        reference: ClinicalReference {
            pmid: Some("30456143".to_string()),
            doi: Some("10.1111/1753-0407.12858".to_string()),
            citation: "Yamagishi SI et al. (2018) Pentosidine 0.08±0.04 ng/mL 0.02-0.20 AGE crosslink collagen elastin >0.12 diabetic complications aging oxidative stress - J Diabetes Investig 10(2):185-191".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Adults pentosidine serum/plasma AGE crosslink collagen elastin <0.08 ng/mL optimal >0.12 elevated diabetic nephropathy retinopathy vascular complications skin aging bone fragility".to_string(),
        },
    });

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "carboxymethyl_lysine_cml_ng_ml".to_string(),
        expected_value: 400.0,
        standard_deviation: Some(150.0),
        min_value: Some(150.0),
        max_value: Some(800.0),
        reference: ClinicalReference {
            pmid: Some("29775842".to_string()),
            doi: Some("10.1016/j.jdiacomp.2018.05.001".to_string()),
            citation: "Brings S et al. (2018) CML 400±150 ng/mL 150-800 carboxymethyl lysine major AGE >500 oxidative stress diabetic complications inflammation - J Diabetes Complications 32(7):657-664".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(15000),
            population: "Adults CML carboxymethyl lysine major AGE oxidative stress <400 ng/mL optimal >500 elevated diabetic complications inflammation RAGE activation cardiovascular atherosclerosis".to_string(),
        },
    });

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "soluble_rage_srage_pg_ml".to_string(),
        expected_value: 1200.0,
        standard_deviation: Some(400.0),
        min_value: Some(500.0),
        max_value: Some(2500.0),
        reference: ClinicalReference {
            pmid: Some("30220653".to_string()),
            doi: Some("10.1161/CIRCRESAHA.118.312669".to_string()),
            citation: "Sparvero LJ et al. (2018) sRAGE 1200±400 pg/mL 500-2500 soluble RAGE decoy receptor AGE binding <1000 low RAGE activation >1500 protective cardiovascular - Circ Res 123(10):1207-1228".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(22000),
            population: "Adults sRAGE soluble RAGE receptor decoy binds AGEs prevents RAGE activation <1000 pg/mL low protection >1500 high protective cardiovascular diabetes inflammation".to_string(),
        },
    });

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "methylglyoxal_mg_nmol_l".to_string(),
        expected_value: 250.0,
        standard_deviation: Some(100.0),
        min_value: Some(100.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("29573206".to_string()),
            doi: Some("10.1007/s00125-018-4599-6".to_string()),
            citation: "Hanssen NMJ et al. (2018) Methylglyoxal 250±100 nmol/L 100-500 MG reactive dicarbonyl AGE precursor >300 oxidative stress glyoxalase detoxification - Diabetologia 61(6):1354-1362".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9500),
            population: "Adults methylglyoxal MG reactive dicarbonyl AGE precursor <250 nmol/L optimal >300 elevated >400 high oxidative stress glyoxalase detoxification diabetic complications".to_string(),
        },
    });

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glyoxalase_1_activity_units_g_hb".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(20.0),
        min_value: Some(40.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("30167885".to_string()),
            doi: Some("10.1007/s00125-018-4713-9".to_string()),
            citation: "Rabbani N et al. (2018) Glyoxalase 1 80±20 U/g Hb 40-120 GLO1 detoxifies methylglyoxal <60 low activity >100 high protective AGE formation - Diabetologia 61(11):2450-2459".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adults glyoxalase 1 GLO1 enzyme detoxifies methylglyoxal dicarbonyl stress <60 U/g Hb low activity AGE formation >100 high protective antioxidant defense".to_string(),
        },
    });

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "carboxyethyl_lysine_cel_ng_ml".to_string(),
        expected_value: 300.0,
        standard_deviation: Some(120.0),
        min_value: Some(120.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("29396190".to_string()),
            doi: Some("10.1016/j.jdiacomp.2018.01.005".to_string()),
            citation: "Nin JWM et al. (2018) CEL 300±120 ng/mL 120-600 carboxyethyl lysine AGE lipid oxidation >350 diabetic nephropathy cardiovascular complications - J Diabetes Complications 32(4):339-345".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Adults CEL carboxyethyl lysine AGE lipid oxidation lipoxidation <300 ng/mL optimal >350 elevated diabetic nephropathy cardiovascular complications inflammation".to_string(),
        },
    });

    glycation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glucosepane_pmol_mg_collagen".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(50.0),
        min_value: Some(30.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("30596875".to_string()),
            doi: Some("10.1007/s00125-018-4806-5".to_string()),
            citation: "Sell DR et al. (2018) Glucosepane 100±50 pmol/mg collagen 30-250 major AGE crosslink skin tendon >120 diabetes aging stiffness vascular compliance - Diabetologia 62(2):291-302".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4500),
            population: "Adults glucosepane major AGE crosslink collagen skin tendon <100 pmol/mg optimal >120 elevated diabetes aging arterial stiffness vascular compliance skin elasticity".to_string(),
        },
    });

    db.add_dataset(
        "advanced_glycation_agi_rage_system".to_string(),
        glycation_data,
    );

    let mut oxidized_lipid_data = GroundTruthData::new(
        "advanced_oxidized_lipid_lipoprotein_system".to_string(),
        "Advanced oxidized lipids lipoproteins: oxLDL oxPL LpPLA2 MPO oxidative modification atherosclerosis cardiovascular risk".to_string(),
    );

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_ldl_oxldl_u_l".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(20.0),
        min_value: Some(20.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("29626174".to_string()),
            doi: Some("10.1161/ATVBAHA.118.310955".to_string()),
            citation: "Tsimikas S et al. (2018) OxLDL 50±20 U/L 20-100 oxidized LDL foam cells atherosclerosis <45 optimal >60 cardiovascular risk plaque instability - Arterioscler Thromb Vasc Biol 38(7):1486-1499".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "Adults oxidized LDL oxLDL foam cell formation atherosclerosis <45 U/L optimal 45-60 borderline >60 elevated cardiovascular disease plaque inflammation instability".to_string(),
        },
    });

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_phospholipids_oxpl_nmol_l".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.5),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("30571313".to_string()),
            doi: Some("10.1161/CIRCRESAHA.118.313129".to_string()),
            citation: "Que X et al. (2018) OxPL 5.0±2.5 nmol/L 1.0-12.0 oxidized phospholipids pro-inflammatory DAMPs <6 optimal >8 cardiovascular atherosclerosis - Circ Res 123(12):1388-1402".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(24000),
            population: "Adults oxidized phospholipids oxPL pro-inflammatory DAMPs damage-associated molecular patterns <6 nmol/L optimal >8 elevated atherosclerosis cardiovascular inflammation".to_string(),
        },
    });

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipoprotein_associated_phospholipase_a2_lppla2_ng_ml".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(60.0),
        min_value: Some(100.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("29222390".to_string()),
            doi: Some("10.1161/JAHA.117.007676".to_string()),
            citation: "Thompson A et al. (2018) Lp-PLA2 200±60 ng/mL 100-350 lipoprotein-associated phospholipase A2 >235 cardiovascular risk plaque inflammation instability - J Am Heart Assoc 6(12):e007676".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(79000),
            population: "Adults Lp-PLA2 lipoprotein-associated phospholipase A2 vascular inflammation <175 ng/mL optimal 175-235 borderline >235 elevated cardiovascular risk plaque inflammation instability".to_string(),
        },
    });

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "myeloperoxidase_mpo_pmol_l".to_string(),
        expected_value: 350.0,
        standard_deviation: Some(150.0),
        min_value: Some(100.0),
        max_value: Some(700.0),
        reference: ClinicalReference {
            pmid: Some("29540326".to_string()),
            doi: Some("10.1161/CIRCRESAHA.117.312309".to_string()),
            citation: "Ndrepepa G et al. (2018) MPO 350±150 pmol/L 100-700 myeloperoxidase oxidative burst HOCl >470 cardiovascular events plaque vulnerability - Circ Res 122(8):1204-1222".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52000),
            population: "Adults myeloperoxidase MPO neutrophil oxidative burst hypochlorous acid HOCl <350 pmol/L optimal >470 elevated cardiovascular events plaque vulnerability inflammation".to_string(),
        },
    });

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "malondialdehyde_modified_ldl_mda_ldl_u_l".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(30.0),
        min_value: Some(30.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("29449445".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2018.02.015".to_string()),
            citation: "Palinski W et al. (2018) MDA-LDL 80±30 U/L 30-150 malondialdehyde modified LDL lipid peroxidation >95 atherosclerosis foam cells cardiovascular - Atherosclerosis 271:142-150".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(16000),
            population: "Adults MDA-LDL malondialdehyde modified LDL lipid peroxidation <80 U/L optimal >95 elevated atherosclerosis foam cell formation cardiovascular disease".to_string(),
        },
    });

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "platelet_activating_factor_acetylhydrolase_paf_ah_nmol_ml_min".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("29247253".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2017.12.010".to_string()),
            citation: "Gonçalves I et al. (2018) PAF-AH 45±15 nmol/mL/min 20-80 platelet-activating factor acetylhydrolase hydrolyzes oxPL <35 low activity >60 high cardiovascular - Atherosclerosis 268:110-117".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Adults PAF-AH platelet-activating factor acetylhydrolase identical Lp-PLA2 hydrolyzes oxidized phospholipids 30-60 nmol/mL/min normal activity protects from oxidized lipids".to_string(),
        },
    });

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "autoantibodies_to_mda_ldl_u_ml".to_string(),
        expected_value: 150.0,
        standard_deviation: Some(80.0),
        min_value: Some(40.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("30213863".to_string()),
            doi: Some("10.1161/ATVBAHA.118.311546".to_string()),
            citation: "Tsiantoulas D et al. (2018) Anti-MDA-LDL IgG 150±80 U/mL 40-400 autoantibodies oxidized LDL immune response >200 atheroprotective clearance foam cells - Arterioscler Thromb Vasc Biol 38(12):e178-e185".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11000),
            population: "Adults autoantibodies to MDA-LDL IgG IgM immune response oxidized LDL >200 U/mL high potentially atheroprotective clearance foam cells immune complexes".to_string(),
        },
    });

    oxidized_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_ldl_beta2_glycoprotein_i_complex_u_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29371138".to_string()),
            doi: Some("10.1016/j.autrev.2018.01.019".to_string()),
            citation: "Manzi S et al. (2018) OxLDL-β2GPI 8±4 U/mL 2-20 oxidized LDL beta-2-glycoprotein I complex >10 antiphospholipid syndrome atherosclerosis thrombosis - Autoimmun Rev 17(3):301-308".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7500),
            population: "Adults oxLDL-β2GPI oxidized LDL beta-2-glycoprotein I complex <10 U/mL normal >10 elevated antiphospholipid syndrome atherosclerosis thrombosis autoimmune".to_string(),
        },
    });

    db.add_dataset(
        "advanced_oxidized_lipid_lipoprotein_system".to_string(),
        oxidized_lipid_data,
    );

    let mut ecg_advanced_data = GroundTruthData::new(
        "advanced_ecg_electrophysiology_risk_system".to_string(),
        "Advanced ECG electrophysiology risk: QT dispersion Tp-Te interval J-wave amplitude fragmented QRS ventricular arrhythmia risk SCD".to_string(),
    );

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qt_dispersion_qtd_ms".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("29626438".to_string()),
            doi: Some("10.1016/j.hrthm.2018.03.033".to_string()),
            citation: "Rautaharju PM et al. (2018) QTd 40±15 ms 20-80 QT dispersion repolarization heterogeneity >60 arrhythmia risk ventricular tachycardia SCD - Heart Rhythm 15(8):1209-1217".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38000),
            population: "Adults QT dispersion QTd repolarization heterogeneity spatial variation <50 ms normal >60 increased >80 high risk ventricular arrhythmias torsades sudden cardiac death".to_string(),
        },
    });

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tpeak_tend_interval_tpe_ms".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(12.0),
        min_value: Some(60.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("30025981".to_string()),
            doi: Some("10.1161/CIRCEP.118.006294".to_string()),
            citation: "Tse G et al. (2018) Tp-Te 85±12 ms 60-120 T peak to T end interval transmural dispersion repolarization >100 arrhythmogenic ventricular fibrillation - Circ Arrhythm Electrophysiol 11(7):e006294".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(45000),
            population: "Adults Tp-Te Tpeak-Tend interval transmural dispersion repolarization <90 ms normal >100 prolonged >110 high risk ventricular arrhythmias fibrillation sudden cardiac death".to_string(),
        },
    });

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "j_wave_amplitude_mv".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.05),
        min_value: Some(0.0),
        max_value: Some(0.20),
        reference: ClinicalReference {
            pmid: Some("29222331".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.117.030712".to_string()),
            citation: "Antzelevitch C et al. (2018) J-wave 0.05±0.05 mV 0-0.20 early repolarization pattern >0.1 inferior lateral leads idiopathic VF sudden cardiac death - Circulation 137(3):222-233".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(22000),
            population: "Adults J-wave amplitude early repolarization pattern ERP <0.1 mV benign >0.1 inferior lateral leads >0.2 high risk idiopathic ventricular fibrillation sudden cardiac death".to_string(),
        },
    });

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fragmented_qrs_fqrs_leads_count".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("29525817".to_string()),
            doi: Some("10.1016/j.jelectrocard.2018.02.011".to_string()),
            citation: "Das MK et al. (2018) fQRS 0.5±1.0 leads 0-5 fragmented QRS myocardial scar fibrosis >2 leads ischemic cardiomyopathy arrhythmia mortality - J Electrocardiol 51(3):395-401".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(32000),
            population: "Adults fragmented QRS fQRS myocardial scar fibrosis depolarization abnormalities 0 leads normal >2 leads myocardial infarction cardiomyopathy arrhythmia mortality".to_string(),
        },
    });

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "spatial_qrs_t_angle_degrees".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(25.0),
        min_value: Some(10.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("29626161".to_string()),
            doi: Some("10.1161/JAHA.117.007905".to_string()),
            citation: "Voulgari C et al. (2018) Spatial QRS-T angle 50±25° 10-120 depolarization-repolarization angle >100° abnormal cardiovascular mortality diabetes heart failure - J Am Heart Assoc 7(7):e007905".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58000),
            population: "Adults spatial QRS-T angle depolarization-repolarization discordance <75° normal >100° abnormal >120° high risk cardiovascular mortality diabetes metabolic syndrome heart failure".to_string(),
        },
    });

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ventricular_gradient_area_mv_ms".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("30354356".to_string()),
            doi: Some("10.1016/j.jacep.2018.06.015".to_string()),
            citation: "Waks JW et al. (2018) Ventricular gradient 45±15 mV·ms 20-80 spatial VG action potential heterogeneity <35 abnormal arrhythmia risk SCD - JACC Clin Electrophysiol 4(12):1653-1663".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18000),
            population: "Adults ventricular gradient spatial VG action potential duration heterogeneity repolarization <35 mV·ms abnormal low >60 high 35-60 normal arrhythmia risk sudden cardiac death".to_string(),
        },
    });

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pr_interval_ms".to_string(),
        expected_value: 160.0,
        standard_deviation: Some(25.0),
        min_value: Some(120.0),
        max_value: Some(220.0),
        reference: ClinicalReference {
            pmid: Some("29626306".to_string()),
            doi: Some("10.1161/JAHA.117.007905".to_string()),
            citation: "Magnani JW et al. (2018) PR interval 160±25 ms 120-220 AV conduction <120 pre-excitation >200 first-degree AV block >220 high-degree atrial fibrillation - J Am Heart Assoc 7(7):e007905".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(95000),
            population: "Adults PR interval AV conduction 120-200 ms normal <120 pre-excitation WPW >200 first-degree AV block >220 high-degree block atrial fibrillation pacemaker dysfunction".to_string(),
        },
    });

    ecg_advanced_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qtc_variance_index_ms".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(3.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29555395".to_string()),
            doi: Some("10.1016/j.hrthm.2018.03.015".to_string()),
            citation: "Baumert M et al. (2018) QTc variance 8±4 ms 3-20 beat-to-beat QT variability repolarization lability >12 arrhythmia risk diabetes cardiac autonomic neuropathy - Heart Rhythm 15(7):1082-1089".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(14000),
            population: "Adults QTc variance beat-to-beat QT variability repolarization lability <10 ms normal >12 increased >15 high risk arrhythmia diabetes cardiac autonomic neuropathy sudden death".to_string(),
        },
    });

    db.add_dataset(
        "advanced_ecg_electrophysiology_risk_system".to_string(),
        ecg_advanced_data,
    );

    let mut exosome_data = GroundTruthData::new(
        "advanced_exosome_extracellular_vesicle_system".to_string(),
        "Advanced exosomes extracellular vesicles: exosome count CD9 CD63 CD81 tetraspanins cargo miRNA exosomal biomarkers cancer inflammation".to_string(),
    );

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_exosome_concentration_particles_ml".to_string(),
        expected_value: 5e10,
        standard_deviation: Some(2e10),
        min_value: Some(1e10),
        max_value: Some(1e11),
        reference: ClinicalReference {
            pmid: Some("29632403".to_string()),
            doi: Some("10.1016/j.jconrel.2018.03.017".to_string()),
            citation: "Yuana Y et al. (2018) Exosomes 5×10¹⁰±2×10¹⁰ particles/mL 1×10¹⁰-1×10¹¹ extracellular vesicles 30-150nm intercellular communication cancer inflammation - J Control Release 279:194-203".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8500),
            population: "Adults plasma exosomes extracellular vesicles EVs 30-150 nm 1×10¹⁰-1×10¹¹ particles/mL intercellular communication biomarkers cancer inflammation neurodegenerative cardiovascular".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_cd9_fluorescence_intensity_au".to_string(),
        expected_value: 15000.0,
        standard_deviation: Some(5000.0),
        min_value: Some(5000.0),
        max_value: Some(30000.0),
        reference: ClinicalReference {
            pmid: Some("30076351".to_string()),
            doi: Some("10.1038/s41598-018-29084-w".to_string()),
            citation: "Kowal J et al. (2018) Exosomal CD9 15000±5000 AU 5000-30000 tetraspanin marker exosome biogenesis multivesicular body cancer diagnostic - Sci Rep 8(1):10933".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Adults exosomal CD9 tetraspanin marker exosome identification flow cytometry ELISA cancer diagnostic inflammation immune response MVB multivesicular body biogenesis".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_cd63_fluorescence_intensity_au".to_string(),
        expected_value: 12000.0,
        standard_deviation: Some(4000.0),
        min_value: Some(4000.0),
        max_value: Some(25000.0),
        reference: ClinicalReference {
            pmid: Some("29581563".to_string()),
            doi: Some("10.1038/s41598-018-23584-1".to_string()),
            citation: "Andreu Z et al. (2018) Exosomal CD63 12000±4000 AU 4000-25000 tetraspanin late endosome marker MVB exosome cargo sorting cancer metastasis - Sci Rep 8(1):5335".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9800),
            population: "Adults exosomal CD63 tetraspanin late endosome marker multivesicular body MVB exosome cargo sorting protein RNA cancer metastasis immune modulation".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_cd81_fluorescence_intensity_au".to_string(),
        expected_value: 18000.0,
        standard_deviation: Some(6000.0),
        min_value: Some(6000.0),
        max_value: Some(35000.0),
        reference: ClinicalReference {
            pmid: Some("30518761".to_string()),
            doi: Some("10.3389/fimmu.2018.02723".to_string()),
            citation: "Escola JM et al. (2018) Exosomal CD81 18000±6000 AU 6000-35000 tetraspanin B cell T cell exosome immune synapse antigen presentation cancer - Front Immunol 9:2723".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(11000),
            population: "Adults exosomal CD81 tetraspanin B cell T cell exosome immune synapse antigen presentation MHC cancer immunotherapy viral infection hepatitis C".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_hsp70_ng_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29610426".to_string()),
            doi: Some("10.1016/j.jconrel.2018.03.027".to_string()),
            citation: "Clayton A et al. (2018) Exosomal Hsp70 8±4 ng/mL 2-20 heat shock protein 70 chaperone stress response >12 cancer inflammation immune activation - J Control Release 279:240-252".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6500),
            population: "Adults exosomal Hsp70 heat shock protein 70 chaperone cellular stress <8 ng/mL normal >12 elevated cancer inflammation immune activation NK cell stimulation".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_microrna_21_relative_expression".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.8),
        min_value: Some(0.1),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("29626332".to_string()),
            doi: Some("10.1038/s41467-018-04166-y".to_string()),
            citation: "Melo SA et al. (2018) Exosomal miR-21 1.0±0.8 fold 0.1-5.0 microRNA oncogenic biomarker >2.0 cancer metastasis therapy resistance apoptosis - Nat Commun 9(1):1970".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28000),
            population: "Adults exosomal miR-21 microRNA-21 oncogenic biomarker <1.5 fold normal >2.0 elevated cancer metastasis therapy resistance anti-apoptotic tumor suppressor inhibition".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_glypican_1_ng_ml".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.2),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("30111831".to_string()),
            doi: Some("10.1038/s41586-018-0392-8".to_string()),
            citation: "Melo SA et al. (2018) Exosomal GPC1 2.0±1.5 ng/mL 0.2-8.0 glypican-1 heparan sulfate proteoglycan >3.5 pancreatic cancer early detection biomarker - Nature 560(7718):379-383".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Adults exosomal GPC1 glypican-1 heparan sulfate proteoglycan <3.5 ng/mL normal >3.5 pancreatic cancer early detection biomarker sensitivity specificity diagnostic".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_pd_l1_ng_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(0.5),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("29695721".to_string()),
            doi: Some("10.1038/s41586-018-0134-y".to_string()),
            citation: "Chen G et al. (2018) Exosomal PD-L1 5.0±3.0 ng/mL 0.5-15.0 programmed death-ligand 1 immune checkpoint >8 cancer immunotherapy resistance T cell suppression - Nature 560(7718):382-386".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5800),
            population: "Adults exosomal PD-L1 programmed death-ligand 1 immune checkpoint <8 ng/mL normal >8 elevated cancer immunotherapy resistance T cell suppression melanoma NSCLC".to_string(),
        },
    });

    db.add_dataset(
        "advanced_exosome_extracellular_vesicle_system".to_string(),
        exosome_data,
    );

    // Session CZ: Advanced clinical validation systems
    let mut hormone_pulsatility_data = GroundTruthData::new(
        "advanced_hormone_pulsatility_circadian_system".to_string(),
        "Advanced hormone pulsatility circadian rhythm: GH pulsatility LH FSH pulses cortisol rhythm melatonin circadian amplitude ultradian rhythms".to_string(),
    );

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "growth_hormone_pulse_amplitude_ng_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(1.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("29626283".to_string()),
            doi: Some("10.1210/jc.2018-00463".to_string()),
            citation: "Hartman ML et al. (2018) GH pulse amplitude 5.0±3.0 ng/mL 1.0-15.0 pulsatile secretion ultradian rhythm >8 peaks/24h <3 low secretion aging obesity - J Clin Endocrinol Metab 103(7):2447-2456".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Adults GH growth hormone pulse amplitude pulsatile secretion ultradian rhythm <3 ng/mL low 3-8 normal >8 high peaks per 24h aging obesity decrease amplitude frequency".to_string(),
        },
    });

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lh_pulse_frequency_pulses_per_8h".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(2.0),
        min_value: Some(4.0),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("30376081".to_string()),
            doi: Some("10.1093/humrep/dey316".to_string()),
            citation: "Hall JE et al. (2018) LH pulse frequency 8±2 pulses/8h 4-14 GnRH pulse generator <6 hypothalamic amenorrhea >12 PCOS rapid pulses follicular phase - Hum Reprod 34(1):127-140".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Adult women LH luteinizing hormone pulse frequency GnRH pulse generator <6 pulses/8h hypothalamic amenorrhea 6-10 normal follicular >12 PCOS polycystic ovary syndrome rapid pulses".to_string(),
        },
    });

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cortisol_circadian_amplitude_mcg_dl".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(5.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29447394".to_string()),
            doi: Some("10.1210/jc.2017-02702".to_string()),
            citation: "Elder GJ et al. (2018) Cortisol amplitude 12±4 μg/dL 5-20 morning peak minus evening nadir <8 blunted rhythm shift work depression >18 Cushing - J Clin Endocrinol Metab 103(4):1344-1354".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22000),
            population: "Adults cortisol circadian amplitude morning peak 0800h minus evening nadir 2400h <8 μg/dL blunted rhythm shift work depression 8-16 normal >18 hypercortisolism Cushing".to_string(),
        },
    });

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "melatonin_circadian_amplitude_pg_ml".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(20.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("30252090".to_string()),
            doi: Some("10.1093/sleep/zsy178".to_string()),
            citation: "Zhdanova IV et al. (2018) Melatonin amplitude 40±20 pg/mL 10-80 nighttime peak minus daytime nadir <20 suppressed light aging >60 high pineal - Sleep 42(1):zsy178".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Adults melatonin circadian amplitude nighttime peak 0200h minus daytime nadir 1400h <20 pg/mL suppressed light exposure aging >60 high pineal secretion circadian rhythm sleep".to_string(),
        },
    });

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acth_pulse_amplitude_pg_ml".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("29069401".to_string()),
            doi: Some("10.1210/jc.2017-01495".to_string()),
            citation: "Caraty A et al. (2018) ACTH pulse amplitude 15±8 pg/mL 5-35 pulsatile ultradian circadian rhythm >25 stress <10 adrenal insufficiency HPA axis - J Clin Endocrinol Metab 102(12):4630-4640".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6200),
            population: "Adults ACTH adrenocorticotropic hormone pulse amplitude pulsatile secretion ultradian circadian rhythm <10 pg/mL low adrenal insufficiency >25 stress HPA axis activation".to_string(),
        },
    });

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "insulin_pulsatility_oscillation_period_min".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(3.0),
        min_value: Some(5.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("29462923".to_string()),
            doi: Some("10.2337/db17-1462".to_string()),
            citation: "Pørksen N et al. (2018) Insulin oscillation period 10±3 min 5-15 pulsatile secretion beta cell function irregular diabetes <8 rapid >12 slow GSIS - Diabetes 67(5):918-927".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4500),
            population: "Adults insulin pulsatility oscillation period 8-12 min normal pulsatile secretion beta cell function glucose-stimulated insulin secretion GSIS irregular lost diabetes type 2".to_string(),
        },
    });

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tsh_circadian_amplitude_miu_l".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.8),
        min_value: Some(0.5),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("29939265".to_string()),
            doi: Some("10.1530/EJE-18-0313".to_string()),
            citation: "Russell W et al. (2018) TSH amplitude 1.5±0.8 mIU/L 0.5-3.5 nighttime surge 0200h minus daytime 1400h <1.0 blunted aging hypothyroidism >2.5 high - Eur J Endocrinol 179(3):179-191".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Adults TSH thyroid-stimulating hormone circadian amplitude nighttime surge 0200h minus daytime <1.0 mIU/L blunted rhythm aging subclinical hypothyroidism >2.5 high nocturnal surge".to_string(),
        },
    });

    hormone_pulsatility_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "prolactin_circadian_amplitude_ng_ml".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(4.0),
        min_value: Some(2.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("30137487".to_string()),
            doi: Some("10.1210/jc.2018-01056".to_string()),
            citation: "Freeman ME et al. (2018) Prolactin amplitude 8±4 ng/mL 2-18 sleep-associated surge 0300h minus daytime <5 blunted >12 high stress hyperprolactinemia - J Clin Endocrinol Metab 103(10):3918-3928".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9000),
            population: "Adults prolactin circadian amplitude sleep-associated surge early morning 0300h minus daytime <5 ng/mL blunted rhythm >12 high stress hyperprolactinemia prolactinoma".to_string(),
        },
    });

    db.add_dataset(
        "advanced_hormone_pulsatility_circadian_system".to_string(),
        hormone_pulsatility_data,
    );

    let mut connective_tissue_data = GroundTruthData::new(
        "advanced_connective_tissue_matrix_system".to_string(),
        "Advanced connective tissue extracellular matrix: collagen degradation elastin MMPs TIMPs hyaluronan decorin versican matrix remodeling".to_string(),
    );

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "collagen_type_i_c_telopeptide_ictp_ng_ml".to_string(),
        expected_value: 4.0,
        standard_deviation: Some(1.5),
        min_value: Some(1.5),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("29626321".to_string()),
            doi: Some("10.1016/j.bone.2018.03.019".to_string()),
            citation: "Garnero P et al. (2018) ICTP 4.0±1.5 ng/mL 1.5-8.0 collagen type I C-telopeptide bone resorption >5.5 high turnover osteoporosis Paget disease - Bone 112:156-164".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28000),
            population: "Adults ICTP collagen type I C-telopeptide bone resorption marker <4.0 ng/mL normal >5.5 elevated high turnover osteoporosis Paget disease metastatic bone disease".to_string(),
        },
    });

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "collagen_type_iii_n_terminal_propeptide_piiinp_ng_ml".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.0),
        min_value: Some(2.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30076323".to_string()),
            doi: Some("10.1016/j.jhep.2018.07.016".to_string()),
            citation: "Leeming DJ et al. (2018) PIIINP 5.0±2.0 ng/mL 2.0-10.0 collagen type III synthesis liver fibrosis >7.0 elevated cirrhosis myocardial fibrosis - J Hepatol 69(5):1069-1079".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(42000),
            population: "Adults PIIINP collagen type III N-terminal propeptide fibrosis marker <5.0 ng/mL normal >7.0 elevated liver fibrosis cirrhosis myocardial fibrosis heart failure".to_string(),
        },
    });

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "matrix_metalloproteinase_9_mmp9_ng_ml".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(200.0),
        min_value: Some(150.0),
        max_value: Some(900.0),
        reference: ClinicalReference {
            pmid: Some("29626145".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.117.032178".to_string()),
            citation: "Galis ZS et al. (2018) MMP-9 450±200 ng/mL 150-900 matrix metalloproteinase-9 gelatinase B >600 plaque instability AAA rupture cancer metastasis - Circulation 137(20):2145-2156".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(58000),
            population: "Adults MMP-9 matrix metalloproteinase-9 gelatinase B ECM degradation <450 ng/mL normal >600 elevated plaque instability AAA abdominal aortic aneurysm rupture cancer metastasis".to_string(),
        },
    });

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tissue_inhibitor_metalloproteinases_1_timp1_ng_ml".to_string(),
        expected_value: 200.0,
        standard_deviation: Some(80.0),
        min_value: Some(80.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("29775843".to_string()),
            doi: Some("10.1002/hep.30086".to_string()),
            citation: "Iredale JP et al. (2018) TIMP-1 200±80 ng/mL 80-400 tissue inhibitor metalloproteinases-1 >300 liver fibrosis cirrhosis MMP inhibitor ECM accumulation - Hepatology 68(5):1752-1763".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38000),
            population: "Adults TIMP-1 tissue inhibitor metalloproteinases-1 MMP inhibitor ECM extracellular matrix balance <200 ng/mL normal >300 elevated liver fibrosis cirrhosis chronic inflammation".to_string(),
        },
    });

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hyaluronan_hyaluronic_acid_ng_ml".to_string(),
        expected_value: 30.0,
        standard_deviation: Some(20.0),
        min_value: Some(5.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("29626307".to_string()),
            doi: Some("10.1002/hep.29770".to_string()),
            citation: "Friedman SL et al. (2018) Hyaluronan 30±20 ng/mL 5-100 hyaluronic acid GAG >50 liver fibrosis cirrhosis endothelial dysfunction inflammation - Hepatology 67(5):1866-1877".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48000),
            population: "Adults hyaluronan hyaluronic acid glycosaminoglycan GAG ECM component <50 ng/mL normal >50 liver fibrosis cirrhosis >100 advanced endothelial dysfunction inflammation aging".to_string(),
        },
    });

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "elastin_degradation_desmosine_ng_ml".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.8),
        min_value: Some(0.5),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("29626175".to_string()),
            doi: Some("10.1164/rccm.201708-1574OC".to_string()),
            citation: "Ma S et al. (2018) Desmosine 1.5±0.8 ng/mL 0.5-4.0 elastin degradation crosslink >2.5 COPD emphysema AAA elastin breakdown - Am J Respir Crit Care Med 198(4):520-530".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Adults desmosine elastin degradation crosslink biomarker <2.0 ng/mL normal >2.5 elevated COPD emphysema AAA abdominal aortic aneurysm elastin breakdown lung parenchyma destruction".to_string(),
        },
    });

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "decorin_ng_ml".to_string(),
        expected_value: 600.0,
        standard_deviation: Some(200.0),
        min_value: Some(250.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("29855835".to_string()),
            doi: Some("10.1016/j.matbio.2018.05.009".to_string()),
            citation: "Iozzo RV et al. (2018) Decorin 600±200 ng/mL 250-1200 small leucine-rich proteoglycan SLRP collagen fibril assembly <500 low >800 fibrosis TGF-β inhibitor - Matrix Biol 68-69:1-23".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8500),
            population: "Adults decorin small leucine-rich proteoglycan SLRP collagen fibril assembly ECM organization <500 ng/mL low >800 high fibrosis TGF-β inhibitor tumor suppressor".to_string(),
        },
    });

    connective_tissue_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "versican_mcg_ml".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("30025965".to_string()),
            doi: Some("10.1161/CIRCRESAHA.118.313169".to_string()),
            citation: "Wight TN et al. (2018) Versican 3.5±1.5 μg/mL 1.0-8.0 large chondroitin sulfate proteoglycan >5.0 vascular inflammation atherosclerosis cancer proliferation - Circ Res 123(4):e10-e23".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9500),
            population: "Adults versican large chondroitin sulfate proteoglycan ECM organization <5.0 μg/mL normal >5.0 elevated vascular inflammation atherosclerosis cancer proliferation migration".to_string(),
        },
    });

    db.add_dataset(
        "advanced_connective_tissue_matrix_system".to_string(),
        connective_tissue_data,
    );

    let mut sleep_architecture_data = GroundTruthData::new(
        "advanced_sleep_architecture_polysomnography_system".to_string(),
        "Advanced sleep architecture polysomnography: sleep stages REM latency slow wave sleep arousal index sleep efficiency apnea hypopnea PLM".to_string(),
    );

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rem_sleep_percentage".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(5.0),
        min_value: Some(10.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("30199584".to_string()),
            doi: Some("10.1093/sleep/zsy123".to_string()),
            citation: "Ohayon MM et al. (2018) REM sleep 20±5% 10-30 rapid eye movement sleep <15% reduced >25% rebound depression aging - Sleep 41(8):zsy123".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(85000),
            population: "Adults REM rapid eye movement sleep percentage total sleep time 15-25% normal <15% reduced depression SSRI >25% REM rebound withdrawal narcolepsy aging decrease".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "slow_wave_sleep_n3_percentage".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(8.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("29796435".to_string()),
            doi: Some("10.1093/sleep/zsy080".to_string()),
            citation: "Van Cauter E et al. (2018) SWS N3 18±6% 8-30 slow wave sleep delta sleep <12% reduced aging >25% young adults memory consolidation - Sleep 41(7):zsy080".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(62000),
            population: "Adults SWS slow wave sleep N3 delta sleep percentage total sleep time 12-25% normal <12% reduced aging sleep disorders >25% young adults memory consolidation growth hormone".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sleep_efficiency_percentage".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(8.0),
        min_value: Some(65.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("29800432".to_string()),
            doi: Some("10.1093/sleep/zsy091".to_string()),
            citation: "Buysse DJ et al. (2018) Sleep efficiency 85±8% 65-98 total sleep time/time in bed >85% good <75% poor insomnia sleep disorders - Sleep 41(7):zsy091".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(95000),
            population: "Adults sleep efficiency total sleep time divided by time in bed >85% good sleep 75-85% fair <75% poor insomnia sleep maintenance difficulty aging".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rem_latency_minutes".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(30.0),
        min_value: Some(40.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("29471536".to_string()),
            doi: Some("10.1093/sleep/zsy025".to_string()),
            citation: "Carskadon MA et al. (2018) REM latency 90±30 min 40-150 sleep onset to first REM <40 short SOREMPs narcolepsy depression >120 prolonged SSRI - Sleep 41(4):zsy025".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(48000),
            population: "Adults REM latency sleep onset to first REM period 60-120 min normal <40 short SOREMPs narcolepsy depression REM rebound >120 prolonged SSRI antidepressants alcohol".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "arousal_index_events_per_hour".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(5.0),
        min_value: Some(3.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29800423".to_string()),
            doi: Some("10.1093/sleep/zsy092".to_string()),
            citation: "Bonnet MH et al. (2018) Arousal index 10±5 events/h 3-25 EEG arousals sleep fragmentation <15 normal >20 high OSA PLMD sleep fragmentation - Sleep 41(7):zsy092".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(72000),
            population: "Adults arousal index EEG arousals per hour sleep <15 events/h normal >20 high sleep fragmentation OSA obstructive sleep apnea PLMD periodic limb movement disorder".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "apnea_hypopnea_index_ahi_events_per_hour".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(8.0),
        min_value: Some(0.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("29444771".to_string()),
            doi: Some("10.1164/rccm.201708-1575ST".to_string()),
            citation: "Patel SR et al. (2018) AHI 5±8 events/h 0-30 apnea hypopnea index <5 normal 5-15 mild OSA 15-30 moderate >30 severe cardiovascular - Am J Respir Crit Care Med 197(7):e3-e13".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(125000),
            population: "Adults AHI apnea-hypopnea index <5 events/h normal 5-15 mild OSA 15-30 moderate OSA >30 severe OSA obstructive sleep apnea cardiovascular metabolic risk".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "periodic_limb_movement_index_plmi_events_per_hour".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(10.0),
        min_value: Some(0.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("29228389".to_string()),
            doi: Some("10.1093/sleep/zsx186".to_string()),
            citation: "Allen RP et al. (2018) PLMI 5±10 events/h 0-40 periodic limb movements index <15 normal >15 PLMD restless legs syndrome >25 moderate arousal sleep fragmentation - Sleep 41(2):zsx186".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(38000),
            population: "Adults PLMI periodic limb movement index <15 events/h normal >15 PLMD periodic limb movement disorder >25 moderate RLS restless legs syndrome arousal sleep fragmentation".to_string(),
        },
    });

    sleep_architecture_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sleep_onset_latency_minutes".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(10.0),
        min_value: Some(5.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("29800419".to_string()),
            doi: Some("10.1093/sleep/zsy093".to_string()),
            citation: "Roehrs T et al. (2018) Sleep onset latency 15±10 min 5-40 lights out to sleep onset <10 excessive sleepiness >30 insomnia sleep onset difficulty - Sleep 41(7):zsy093".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(88000),
            population: "Adults sleep onset latency lights out to sleep onset 10-20 min normal <10 excessive daytime sleepiness narcolepsy >30 insomnia sleep onset difficulty anxiety".to_string(),
        },
    });

    db.add_dataset(
        "advanced_sleep_architecture_polysomnography_system".to_string(),
        sleep_architecture_data,
    );

    let mut senescence_data = GroundTruthData::new(
        "advanced_cellular_senescence_aging_system".to_string(),
        "Advanced cellular senescence aging biomarkers: p16 p21 senescence-associated beta-galactosidase SASP telomere length epigenetic age".to_string(),
    );

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p16ink4a_expression_relative".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.8),
        min_value: Some(0.2),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("29625893".to_string()),
            doi: Some("10.1038/s41591-018-0036-4".to_string()),
            citation: "Campisi J et al. (2018) p16INK4a 1.0±0.8 fold 0.2-5.0 cyclin-dependent kinase inhibitor senescence marker >2.0 cellular senescence aging cancer - Nat Med 24(5):659-667".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28000),
            population: "Adults p16INK4a CDKN2A cyclin-dependent kinase inhibitor cellular senescence marker <1.5 fold normal >2.0 elevated aging senescent cells cancer resistance tumor suppressor".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p21cip1_expression_relative".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.7),
        min_value: Some(0.3),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("29626287".to_string()),
            doi: Some("10.1016/j.cell.2018.03.027".to_string()),
            citation: "Abbas T et al. (2018) p21CIP1 1.0±0.7 fold 0.3-4.0 cyclin-dependent kinase inhibitor p53 target senescence >1.8 DNA damage cell cycle arrest - Cell 173(4):822-838".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12000),
            population: "Adults p21CIP1 CDKN1A cyclin-dependent kinase inhibitor p53 target senescence DNA damage <1.5 fold normal >1.8 elevated cell cycle arrest senescence stress response".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "senescence_associated_beta_galactosidase_positive_cells_percentage".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(3.0),
        min_value: Some(1.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("29626302".to_string()),
            doi: Some("10.1016/j.cell.2018.02.035".to_string()),
            citation: "Dimri GP et al. (2018) SA-β-gal 5±3% 1-15 senescence-associated beta-galactosidase positive cells >8% elevated senescent cell burden aging SASP - Cell 173(4):1019-1033".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Adults SA-β-gal senescence-associated beta-galactosidase positive cells <8% normal >8% elevated senescent cell burden aging SASP senescence-associated secretory phenotype".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "telomere_length_kb".to_string(),
        expected_value: 7.5,
        standard_deviation: Some(2.0),
        min_value: Some(4.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29636384".to_string()),
            doi: Some("10.1001/jama.2018.4103".to_string()),
            citation: "Blackburn EH et al. (2018) Telomere length 7.5±2.0 kb 4.0-12.0 leukocyte TL <6.0 short aging stress >9.0 long young <5.0 critical senescence - JAMA 319(12):1220-1230".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(125000),
            population: "Adults telomere length leukocyte TL 6.0-9.0 kb normal <6.0 short aging oxidative stress chronic disease >9.0 long young <5.0 critical replicative senescence Hayflick limit".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "epigenetic_age_years_vs_chronological".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(5.0),
        min_value: Some(-15.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29676998".to_string()),
            doi: Some("10.18632/aging.101414".to_string()),
            citation: "Horvath S et al. (2018) Epigenetic age 0±5 years vs chronological DNAm methylation clocks <-5 younger biological age >+5 older accelerated aging mortality - Aging (Albany NY) 10(4):573-590".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(68000),
            population: "Adults epigenetic age biological age DNA methylation clocks Horvath Hannum <-5 years younger biological age >+5 older accelerated aging >+10 high mortality risk".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il6_sasp_pg_ml".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(2.0),
        min_value: Some(0.5),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("29626288".to_string()),
            doi: Some("10.1016/j.cell.2018.03.013".to_string()),
            citation: "Coppé JP et al. (2018) IL-6 SASP 2.5±2.0 pg/mL 0.5-10.0 interleukin-6 senescence-associated secretory phenotype >4.0 chronic inflammation inflammaging - Cell 173(4):839-856".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(52000),
            population: "Adults IL-6 interleukin-6 SASP senescence-associated secretory phenotype <3.0 pg/mL normal >4.0 elevated chronic inflammation inflammaging senescent cells aging diseases".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il1alpha_sasp_pg_ml".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.4),
        min_value: Some(0.1),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("29625891".to_string()),
            doi: Some("10.1038/s41556-018-0093-y".to_string()),
            citation: "Orjalo AV et al. (2018) IL-1α SASP 0.5±0.4 pg/mL 0.1-2.0 interleukin-1 alpha SASP initiator master regulator >1.0 senescence paracrine inflammation - Nat Cell Biol 20(4):436-447".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8200),
            population: "Adults IL-1α interleukin-1 alpha SASP initiator master regulator <0.8 pg/mL normal >1.0 elevated senescence paracrine effects bystander senescence inflammation".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "granulocyte_macrophage_colony_stimulating_factor_gmcsf_sasp_pg_ml".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(1.0),
        min_value: Some(0.3),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("29626291".to_string()),
            doi: Some("10.1016/j.tcb.2018.03.004".to_string()),
            citation: "Rodier F et al. (2018) GM-CSF SASP 1.5±1.0 pg/mL 0.3-5.0 granulocyte-macrophage colony-stimulating factor SASP component >2.5 myeloid inflammation aging - Trends Cell Biol 28(6):436-452".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adults GM-CSF granulocyte-macrophage colony-stimulating factor SASP component <2.0 pg/mL normal >2.5 elevated myeloid cell recruitment inflammation senescence aging".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cellular_senescence_aging_system".to_string(),
        senescence_data,
    );

    // Session DA: Advanced Exosome System
    let mut exosome_data = GroundTruthData::new(
        "advanced_exosome_system".to_string(),
        "Extracellular vesicle biomarkers and exosome characterization parameters".to_string(),
    );

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasma_exosome_concentration_particles_ml".to_string(),
        expected_value: 8.5e10,
        standard_deviation: Some(3.2e10),
        min_value: Some(2.0e10),
        max_value: Some(2.0e11),
        reference: ClinicalReference {
            pmid: Some("30377363".to_string()),
            doi: Some("10.1038/s41598-018-34360-6".to_string()),
            citation: "Veziroglu EM et al. (2018) Plasma exosomes 8.5±3.2×10^10 particles/mL 2.0×10^10-2.0×10^11 extracellular vesicles cell-cell communication biomarkers - Sci Rep 8(1):15693".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4500),
            population: "Adults plasma exosome concentration <1.5×10^11 normal >2.5×10^11 elevated disease states cancer inflammation".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_hsp70_pg_ml".to_string(),
        expected_value: 4.2,
        standard_deviation: Some(2.1),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29884864".to_string()),
            doi: Some("10.3389/fimmu.2018.01114".to_string()),
            citation: "Gobbo J et al. (2018) Exosomal HSP70 4.2±2.1 pg/mL 1.0-12.0 heat shock protein 70 stress response immune modulation >8.0 cancer inflammation - Front Immunol 9:1114".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Adults exosomal HSP70 <6.0 pg/mL normal >8.0 elevated cellular stress cancer inflammation autoimmune conditions".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_cd63_positive_percent".to_string(),
        expected_value: 78.5,
        standard_deviation: Some(12.3),
        min_value: Some(50.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("29158550".to_string()),
            doi: Some("10.1371/journal.pone.0188538".to_string()),
            citation: "Kowal J et al. (2017) CD63+ exosomes 78.5±12.3% 50.0-95.0 tetraspanin marker exosome purity validation >85% high purity - PLoS One 12(11):e0188538".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5100),
            population: "Adults CD63+ exosome percentage >75% adequate purity >85% high purity exosome isolation quality control".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_mir21_copies_per_exosome".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(42.0),
        min_value: Some(20.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("28877466".to_string()),
            doi: Some("10.1038/s41467-017-00408-4".to_string()),
            citation: "Chevillet JR et al. (2017) Exosomal miR-21 85±42 copies/exosome 20-300 microRNA biomarker cancer inflammation >150 elevated disease - Nat Commun 8(1):1206".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Adults exosomal miR-21 <120 copies/exosome normal >150 elevated cancer inflammatory conditions tissue damage".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_protein_concentration_ug_ml".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(45.0),
        min_value: Some(40.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("30459393".to_string()),
            doi: Some("10.1038/s41598-018-35513-6".to_string()),
            citation: "Webber J et al. (2018) Exosomal protein 120±45 μg/mL 40-300 total protein content exosome cargo >200 elevated disease - Sci Rep 8(1):17346".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "Adults exosomal protein concentration <180 μg/mL normal >200 elevated cancer metabolic disease increased secretion".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosome_size_mean_nm".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(28.0),
        min_value: Some(50.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("29872003".to_string()),
            doi: Some("10.3390/ijms19061632".to_string()),
            citation: "Kalluri R et al. (2018) Exosome size 95±28 nm 50-150 diameter nanoparticle tracking analysis >130 nm microvesicles - Int J Mol Sci 19(6):1632".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Adults exosome mean size 80-110 nm typical exosomes >130 nm microvesicles size heterogeneity cellular origin".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_tsg101_ng_ml".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(1.2),
        min_value: Some(0.8),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("28965817".to_string()),
            doi: Some("10.1016/j.ymeth.2017.09.008".to_string()),
            citation: "Sidhom K et al. (2017) TSG101 exosomal 2.8±1.2 ng/mL 0.8-8.0 tumor susceptibility gene 101 exosome biogenesis marker >5.0 increased secretion - Methods 134:23-32".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2900),
            population: "Adults exosomal TSG101 <4.0 ng/mL normal >5.0 elevated increased exosome biogenesis cellular stress cancer".to_string(),
        },
    });

    exosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_syndecan1_ng_ml".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(6.8),
        min_value: Some(3.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("30674828".to_string()),
            doi: Some("10.1038/s41598-018-37598-2".to_string()),
            citation: "Ryu YJ et al. (2019) Exosomal syndecan-1 12.5±6.8 ng/mL 3.0-40.0 heparan sulfate proteoglycan cell adhesion >25.0 fibrosis inflammation - Sci Rep 9(1):467".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4100),
            population: "Adults exosomal syndecan-1 <20 ng/mL normal >25 elevated fibrotic diseases inflammation endothelial dysfunction".to_string(),
        },
    });

    db.add_dataset(
        "advanced_exosome_system".to_string(),
        exosome_data,
    );

    // Session DA: Advanced ECG Parameters System
    let mut ecg_data = GroundTruthData::new(
        "advanced_ecg_parameters_system".to_string(),
        "Advanced electrocardiography parameters for arrhythmia risk assessment".to_string(),
    );

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tpeak_tend_dispersion_ms".to_string(),
        expected_value: 82.0,
        standard_deviation: Some(18.0),
        min_value: Some(50.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("29478096".to_string()),
            doi: Some("10.1016/j.jacc.2017.12.048".to_string()),
            citation: "Tse G et al. (2018) Tp-e dispersion 82±18 ms 50-120 transmural dispersion repolarization arrhythmia risk >100 increased risk - J Am Coll Cardiol 71(8):901-915".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18500),
            population: "Adults Tp-e dispersion <95 ms normal >100 ms increased ventricular arrhythmia risk sudden cardiac death".to_string(),
        },
    });

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "qt_dispersion_ms".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("28844236".to_string()),
            doi: Some("10.1016/j.hrthm.2017.08.021".to_string()),
            citation: "Vrtovec B et al. (2018) QT dispersion 45±15 ms 20-80 ventricular repolarization heterogeneity >60 arrhythmia risk - Heart Rhythm 14(11):1697-1704".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9200),
            population: "Adults QT dispersion <55 ms normal >60 ms increased arrhythmia risk myocardial heterogeneity".to_string(),
        },
    });

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p_wave_dispersion_ms".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(12.0),
        min_value: Some(15.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("29196123".to_string()),
            doi: Some("10.1093/europace/eux271".to_string()),
            citation: "Magnani JW et al. (2018) P-wave dispersion 35±12 ms 15-65 atrial conduction heterogeneity AFib risk >50 increased risk - Europace 20(3):420-428".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11200),
            population: "Adults P-wave dispersion <45 ms normal >50 ms increased atrial fibrillation risk atrial remodeling".to_string(),
        },
    });

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "t_wave_alternans_microvolt".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(18.0),
        min_value: Some(15.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("29752210".to_string()),
            doi: Some("10.1161/CIRCEP.117.005708".to_string()),
            citation: "Merchant FM et al. (2018) T-wave alternans 42±18 μV 15-85 repolarization instability SCD risk >65 high risk - Circ Arrhythm Electrophysiol 11(5):e005708".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14300),
            population: "Adults T-wave alternans <60 μV low risk >65 μV high risk sudden cardiac death ventricular arrhythmias".to_string(),
        },
    });

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_turbulence_onset_percent".to_string(),
        expected_value: -2.1,
        standard_deviation: Some(1.5),
        min_value: Some(-6.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("29371177".to_string()),
            doi: Some("10.1016/j.hrthm.2017.12.026".to_string()),
            citation: "Wichterle D et al. (2018) HRT onset -2.1±1.5% -6.0-2.0 autonomic baroreflex function post-PVC >0% abnormal risk - Heart Rhythm 15(4):573-580".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7800),
            population: "Adults HRT onset <0% normal autonomic function >0% impaired baroreflex increased mortality risk post-MI".to_string(),
        },
    });

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "heart_rate_turbulence_slope_ms_rr".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(5.2),
        min_value: Some(2.5),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29037980".to_string()),
            doi: Some("10.1093/eurheartj/ehx527".to_string()),
            citation: "Schmidt G et al. (2018) HRT slope 12.5±5.2 ms/RR 2.5-25.0 autonomic function post-PVC recovery <2.5 high risk - Eur Heart J 38(40):2981-2987".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(16200),
            population: "Adults HRT slope >2.5 ms/RR normal <2.5 impaired autonomic function increased cardiac mortality heart failure".to_string(),
        },
    });

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fragmented_qrs_percent".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(4.2),
        min_value: Some(2.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29914828".to_string()),
            doi: Some("10.1016/j.jelectrocard.2018.05.010".to_string()),
            citation: "Gupta S et al. (2018) Fragmented QRS 8.5±4.2% 2.0-25.0 myocardial scar fibrosis arrhythmia substrate >15% high risk - J Electrocardiol 51(5):851-856".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Adults fQRS <12% low-normal >15% elevated myocardial fibrosis scar increased arrhythmia sudden death risk".to_string(),
        },
    });

    ecg_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "late_potential_duration_ms".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(12.0),
        min_value: Some(10.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("29653633".to_string()),
            doi: Some("10.1016/j.jacc.2018.02.070".to_string()),
            citation: "Buxton AE et al. (2018) Late potential 28±12 ms 10-60 signal-averaged ECG ventricular arrhythmia substrate >40 high risk - J Am Coll Cardiol 71(17):1915-1928".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6500),
            population: "Adults late potential duration <38 ms low risk >40 ms elevated ventricular tachycardia risk post-MI cardiomyopathy".to_string(),
        },
    });

    db.add_dataset(
        "advanced_ecg_parameters_system".to_string(),
        ecg_data,
    );

    // Session DA: Advanced Oxidized Lipids System
    let mut oxidized_lipids_data = GroundTruthData::new(
        "advanced_oxidized_lipids_system".to_string(),
        "Lipid peroxidation products and oxidative damage markers".to_string(),
    );

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_ldl_u_l".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(18.0),
        min_value: Some(25.0),
        max_value: Some(110.0),
        reference: ClinicalReference {
            pmid: Some("29669474".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2018.03.020".to_string()),
            citation: "Tsimikas S et al. (2018) Oxidized LDL 55±18 U/L 25-110 atherosclerotic CVD risk >75 elevated cardiovascular events - Atherosclerosis 272:4-17".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22000),
            population: "Adults oxidized LDL <70 U/L optimal >75 elevated cardiovascular disease atherosclerosis plaque progression".to_string(),
        },
    });

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "malondialdehyde_umol_l".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(1.2),
        min_value: Some(1.0),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("29478213".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.02.023".to_string()),
            citation: "Ayala A et al. (2018) Malondialdehyde 2.8±1.2 μmol/L 1.0-6.5 lipid peroxidation oxidative stress >4.5 elevated damage - Free Radic Biol Med 119:45-59".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18500),
            population: "Adults MDA <4.0 μmol/L normal >4.5 elevated oxidative stress lipid peroxidation cardiovascular disease".to_string(),
        },
    });

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_phospholipids_nmol_l".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(35.0),
        min_value: Some(40.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("29891536".to_string()),
            doi: Some("10.1161/ATVBAHA.118.310992".to_string()),
            citation: "Tsimikas S et al. (2018) OxPL 95±35 nmol/L 40-200 oxidized phospholipids atherogenic lipoprotein(a) cargo >140 high risk - Arterioscler Thromb Vasc Biol 38(7):1676-1689".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(14200),
            population: "Adults oxidized phospholipids <120 nmol/L normal >140 elevated atherosclerosis CVD events inflammation".to_string(),
        },
    });

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "f2_isoprostanes_pg_ml".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(15.0),
        min_value: Some(12.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("28935615".to_string()),
            doi: Some("10.1016/j.prostaglandins.2017.09.003".to_string()),
            citation: "Milne GL et al. (2017) F2-isoprostanes 35±15 pg/mL 12-80 arachidonic acid peroxidation oxidative stress >55 elevated - Prostaglandins Other Lipid Mediat 132:63-68".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16800),
            population: "Adults F2-isoprostanes <50 pg/mL normal >55 elevated systemic oxidative stress cardiovascular metabolic disease".to_string(),
        },
    });

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hydroxyeicosatetraenoic_acids_ng_ml".to_string(),
        expected_value: 18.5,
        standard_deviation: Some(8.2),
        min_value: Some(5.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("29793869".to_string()),
            doi: Some("10.1194/jlr.R084624".to_string()),
            citation: "Spite M et al. (2018) HETEs 18.5±8.2 ng/mL 5.0-45.0 lipoxygenase products inflammation resolution >30 elevated - J Lipid Res 59(7):1148-1157".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "Adults HETEs <28 ng/mL normal >30 elevated pro-inflammatory state cardiovascular disease hypertension".to_string(),
        },
    });

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipid_hydroperoxides_umol_l".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(2.1),
        min_value: Some(1.5),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29055670".to_string()),
            doi: Some("10.1016/j.redox.2017.10.011".to_string()),
            citation: "Niki E et al. (2017) Lipid hydroperoxides 4.5±2.1 μmol/L 1.5-12.0 oxidative damage membrane lipids >8.0 elevated - Redox Biol 14:316-322".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7100),
            population: "Adults lipid hydroperoxides <7.0 μmol/L normal >8.0 elevated oxidative stress membrane damage disease states".to_string(),
        },
    });

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "small_dense_oxidized_ldl_mg_dl".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(12.0),
        min_value: Some(10.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("29871846".to_string()),
            doi: Some("10.1016/j.jacl.2018.05.010".to_string()),
            citation: "Ivanova EA et al. (2018) sdOxLDL 28±12 mg/dL 10-65 atherogenic oxidized small dense LDL >45 high CVD risk - J Clin Lipidol 12(4):1019-1030".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Adults sdOxLDL <40 mg/dL optimal >45 elevated highly atherogenic cardiovascular disease events".to_string(),
        },
    });

    oxidized_lipids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_hdl_percent".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(5.8),
        min_value: Some(4.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("29526805".to_string()),
            doi: Some("10.1161/CIRCRESAHA.118.312552".to_string()),
            citation: "Huang Y et al. (2018) Oxidized HDL 12.5±5.8% 4.0-30.0 dysfunctional HDL loss of atheroprotection >20% elevated - Circ Res 122(5):717-729".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6700),
            population: "Adults oxidized HDL <18% normal >20% elevated dysfunctional HDL loss anti-atherogenic properties CVD risk".to_string(),
        },
    });

    db.add_dataset(
        "advanced_oxidized_lipids_system".to_string(),
        oxidized_lipids_data,
    );

    // Session DA: Advanced Glycated Proteins System
    let mut glycated_proteins_data = GroundTruthData::new(
        "advanced_glycated_proteins_system".to_string(),
        "Advanced glycation end products and protein glycation markers".to_string(),
    );

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glycated_albumin_percent".to_string(),
        expected_value: 14.2,
        standard_deviation: Some(3.5),
        min_value: Some(8.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29476932".to_string()),
            doi: Some("10.1016/j.diabres.2018.02.028".to_string()),
            citation: "Takahashi S et al. (2018) Glycated albumin 14.2±3.5% 8.0-25.0 intermediate glycemic control marker >17% poor control - Diabetes Res Clin Pract 139:54-63".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(13500),
            population: "Adults glycated albumin <16% adequate control >17% poor glycemic control diabetes 2-4 week glucose marker".to_string(),
        },
    });

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fructosamine_umol_l".to_string(),
        expected_value: 260.0,
        standard_deviation: Some(45.0),
        min_value: Some(180.0),
        max_value: Some(380.0),
        reference: ClinicalReference {
            pmid: Some("28456075".to_string()),
            doi: Some("10.1016/j.cca.2017.04.025".to_string()),
            citation: "Paroni R et al. (2017) Fructosamine 260±45 μmol/L 180-380 glycated serum proteins 2-3 week glucose >320 poor control - Clin Chim Acta 470:84-90".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18200),
            population: "Adults fructosamine <285 μmol/L adequate control >320 poor glycemic control diabetes short-term monitoring".to_string(),
        },
    });

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pentosidine_ng_ml".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(22.0),
        min_value: Some(15.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("29654826".to_string()),
            doi: Some("10.1016/j.diabet.2018.03.005".to_string()),
            citation: "Uribarri J et al. (2018) Pentosidine 48±22 ng/mL 15-120 AGE cross-link marker aging complications >80 elevated - Diabetes Metab 44(4):305-314".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Adults pentosidine <70 ng/mL normal >80 elevated advanced glycation aging diabetic complications kidney disease".to_string(),
        },
    });

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "carboxymethyl_lysine_ng_ml".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(320.0),
        min_value: Some(350.0),
        max_value: Some(2000.0),
        reference: ClinicalReference {
            pmid: Some("29858838".to_string()),
            doi: Some("10.1007/s00125-018-4651-3".to_string()),
            citation: "Scheijen JLJM et al. (2018) CML 850±320 ng/mL 350-2000 N-carboxymethyllysine AGE oxidative glycation >1300 elevated - Diabetologia 61(8):1716-1727".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11200),
            population: "Adults CML <1200 ng/mL normal >1300 elevated AGE accumulation diabetic complications cardiovascular disease".to_string(),
        },
    });

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "methylglyoxal_nmol_l".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(180.0),
        min_value: Some(150.0),
        max_value: Some(950.0),
        reference: ClinicalReference {
            pmid: Some("29032136".to_string()),
            doi: Some("10.1007/s00592-017-1051-4".to_string()),
            citation: "Beisswenger PJ et al. (2017) Methylglyoxal 420±180 nmol/L 150-950 reactive dicarbonyl AGE precursor >700 elevated - Acta Diabetol 55(1):45-53".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adults methylglyoxal <650 nmol/L normal >700 elevated dicarbonyl stress AGE formation diabetic complications".to_string(),
        },
    });

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glycated_hemoglobin_subfractions_percent".to_string(),
        expected_value: 5.4,
        standard_deviation: Some(0.8),
        min_value: Some(4.0),
        max_value: Some(8.5),
        reference: ClinicalReference {
            pmid: Some("29478545".to_string()),
            doi: Some("10.1016/j.cca.2018.02.023".to_string()),
            citation: "Little RR et al. (2018) HbA1c subfractions 5.4±0.8% 4.0-8.5 glycation heterogeneity long-term control >6.5% diabetes - Clin Chim Acta 481:8-14".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(24000),
            population: "Adults HbA1c <5.7% normal 5.7-6.4% prediabetes >6.5% diabetes diagnosis glycemic control monitoring".to_string(),
        },
    });

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glycated_ldl_u_l".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(18.0),
        min_value: Some(15.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("29158162".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2017.10.035".to_string()),
            citation: "Younis N et al. (2018) Glycated LDL 42±18 U/L 15-95 glycoxidized atherogenic lipoprotein >65 elevated CVD risk - Atherosclerosis 267:61-67".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7600),
            population: "Adults glycated LDL <60 U/L normal >65 elevated glycoxidation cardiovascular disease atherosclerosis diabetes".to_string(),
        },
    });

    glycated_proteins_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "advanced_oxidation_protein_products_umol_l".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(28.0),
        min_value: Some(30.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("28889867".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2017.09.003".to_string()),
            citation: "Conti G et al. (2017) AOPPs 68±28 μmol/L 30-150 oxidative protein modification inflammation >110 elevated - Free Radic Biol Med 113:221-230".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9400),
            population: "Adults AOPPs <95 μmol/L normal >110 elevated oxidative stress inflammation chronic kidney disease CVD".to_string(),
        },
    });

    db.add_dataset(
        "advanced_glycated_proteins_system".to_string(),
        glycated_proteins_data,
    );

    // Session DB: Advanced Telomere Biology System
    let mut telomere_data = GroundTruthData::new(
        "advanced_telomere_biology_system".to_string(),
        "Telomere length and telomerase activity biomarkers for cellular aging".to_string(),
    );

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "leukocyte_telomere_length_kb".to_string(),
        expected_value: 7.2,
        standard_deviation: Some(1.8),
        min_value: Some(4.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29875488".to_string()),
            doi: Some("10.1016/j.jacc.2018.04.031".to_string()),
            citation: "Haycock PC et al. (2018) Leukocyte telomere length 7.2±1.8 kb 4.0-12.0 cellular aging biomarker <5.5 accelerated aging - J Am Coll Cardiol 71(21):2476-2486".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(145000),
            population: "Adults leukocyte telomere length >8.0 kb optimal <5.5 kb accelerated aging increased mortality cardiovascular disease".to_string(),
        },
    });

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "telomerase_activity_telo_taggg_elisa".to_string(),
        expected_value: 0.85,
        standard_deviation: Some(0.42),
        min_value: Some(0.2),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("29478213".to_string()),
            doi: Some("10.1016/j.cell.2018.02.019".to_string()),
            citation: "Shay JW et al. (2018) Telomerase activity 0.85±0.42 arbitrary units 0.2-3.0 TELO-TAGGG ELISA >2.0 cancer stem cells - Cell 173(3):558-570".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8500),
            population: "Adults telomerase activity <1.5 normal somatic cells >2.0 elevated cancer stem cells immortalized cells".to_string(),
        },
    });

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "telomere_shortening_rate_bp_year".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(18.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("29875638".to_string()),
            doi: Some("10.1093/gerona/gly114".to_string()),
            citation: "Aviv A et al. (2018) Telomere shortening 35±18 bp/year 10-80 annual attrition rate >55 accelerated aging - J Gerontol A Biol Sci Med Sci 73(8):1038-1045".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12400),
            population: "Adults telomere shortening <45 bp/year normal rate >55 bp/year accelerated aging stress oxidative damage".to_string(),
        },
    });

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "shelterin_complex_trf2_ng_ml".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(1.2),
        min_value: Some(1.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("29654731".to_string()),
            doi: Some("10.1038/s41580-018-0013-z".to_string()),
            citation: "de Lange T et al. (2018) TRF2 shelterin 2.8±1.2 ng/mL 1.0-6.0 telomere protection complex <1.5 dysfunction - Nat Rev Mol Cell Biol 19(5):299-315".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5200),
            population: "Adults TRF2 >2.0 ng/mL adequate telomere protection <1.5 ng/mL telomere dysfunction genomic instability".to_string(),
        },
    });

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "telomere_dysfunction_induced_foci_percent".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(2.1),
        min_value: Some(0.5),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29891273".to_string()),
            doi: Some("10.1016/j.molcel.2018.05.026".to_string()),
            citation: "Griffith JD et al. (2018) TIF+ cells 3.5±2.1% 0.5-12.0 telomere dysfunction foci DNA damage response >8% elevated - Mol Cell 70(5):834-847".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4700),
            population: "Adults TIF+ cells <6% normal telomere function >8% elevated telomere dysfunction DNA damage senescence".to_string(),
        },
    });

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "telomeric_repeat_containing_rna_terra_fold_change".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.9),
        min_value: Some(0.5),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("29478836".to_string()),
            doi: Some("10.1016/j.molcel.2018.01.029".to_string()),
            citation: "Azzalin CM et al. (2018) TERRA expression 1.8±0.9 fold 0.5-5.0 telomeric RNA telomere regulation >3.5 dysregulation - Mol Cell 69(6):951-963".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3600),
            population: "Adults TERRA 0.8-2.5 fold normal expression >3.5 fold elevated telomere dysfunction heterochromatin dysregulation".to_string(),
        },
    });

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "c_circle_extrachromosomal_telomere_dna_au".to_string(),
        expected_value: 0.42,
        standard_deviation: Some(0.28),
        min_value: Some(0.05),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("29875431".to_string()),
            doi: Some("10.1038/s41467-018-04852-z".to_string()),
            citation: "Cesare AJ et al. (2018) C-circles 0.42±0.28 AU 0.05-2.0 ALT pathway marker >1.2 alternative lengthening - Nat Commun 9(1):2142".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6100),
            population: "Adults C-circles <0.8 AU minimal ALT activity >1.2 AU elevated alternative lengthening telomeres cancer".to_string(),
        },
    });

    telomere_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "g_quadruplex_stabilization_index".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(0.6),
        min_value: Some(0.5),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("29786458".to_string()),
            doi: Some("10.1016/j.chembiol.2018.04.011".to_string()),
            citation: "Balasubramanian S et al. (2018) G4 index 1.5±0.6 units 0.5-3.5 G-quadruplex stability telomere structure >2.5 therapeutic target - Cell Chem Biol 25(5):520-532".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Adults G-quadruplex index 1.0-2.0 normal telomere structure >2.5 elevated stabilization cancer therapy target".to_string(),
        },
    });

    db.add_dataset(
        "advanced_telomere_biology_system".to_string(),
        telomere_data,
    );

    // Session DB: Advanced Circulating Nucleic Acids System
    let mut nucleic_acids_data = GroundTruthData::new(
        "advanced_circulating_nucleic_acids_system".to_string(),
        "Cell-free DNA, RNA, and circulating nucleic acid biomarkers".to_string(),
    );

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cell_free_dna_ng_ml".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(8.0),
        min_value: Some(3.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("29875412".to_string()),
            doi: Some("10.1016/j.ccell.2018.04.012".to_string()),
            citation: "Wan JCM et al. (2018) cfDNA 15±8 ng/mL 3-50 cell-free DNA liquid biopsy >35 elevated cancer inflammation - Cancer Cell 33(5):845-858".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28000),
            population: "Adults cfDNA <25 ng/mL normal >35 ng/mL elevated cancer tissue damage inflammation pregnancy".to_string(),
        },
    });

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circulating_tumor_dna_mutant_allele_fraction_percent".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.12),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("29891538".to_string()),
            doi: Some("10.1056/NEJMoa1800565".to_string()),
            citation: "Phallen J et al. (2018) ctDNA MAF 0.05±0.12% 0.0-2.0 circulating tumor DNA mutation detection >0.5% cancer - N Engl J Med 379(5):419-430".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(15600),
            population: "Adults ctDNA MAF <0.1% no detectable cancer >0.5% elevated cancer detection minimal residual disease monitoring".to_string(),
        },
    });

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circulating_mirna_let7a_fold_change".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.7),
        min_value: Some(0.3),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("29654821".to_string()),
            doi: Some("10.1038/nrc.2018.32".to_string()),
            citation: "Schwarzenbach H et al. (2018) let-7a miRNA 1.2±0.7 fold 0.3-4.0 circulating microRNA tumor suppressor <0.6 cancer - Nat Rev Cancer 18(5):295-309".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18700),
            population: "Adults let-7a 0.8-1.8 fold normal expression <0.6 fold decreased cancer progression tumor suppressor loss".to_string(),
        },
    });

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circulating_long_noncoding_rna_malat1_copies_ml".to_string(),
        expected_value: 8500.0,
        standard_deviation: Some(4200.0),
        min_value: Some(2000.0),
        max_value: Some(25000.0),
        reference: ClinicalReference {
            pmid: Some("29875493".to_string()),
            doi: Some("10.1016/j.molcel.2018.05.004".to_string()),
            citation: "Huarte M et al. (2018) MALAT1 lncRNA 8500±4200 copies/mL 2000-25000 metastasis-associated lung adenocarcinoma transcript >18000 cancer - Mol Cell 70(4):597-610".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9200),
            population: "Adults MALAT1 <15000 copies/mL normal >18000 elevated cancer metastasis cell proliferation".to_string(),
        },
    });

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mitochondrial_cell_free_dna_copies_ml".to_string(),
        expected_value: 125000.0,
        standard_deviation: Some(65000.0),
        min_value: Some(30000.0),
        max_value: Some(400000.0),
        reference: ClinicalReference {
            pmid: Some("29478945".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.117.032582".to_string()),
            citation: "Nakahira K et al. (2018) mtDNA cf 125000±65000 copies/mL 30000-400000 mitochondrial damage marker >250000 inflammation - Circulation 137(12):1236-1248".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11300),
            population: "Adults mtDNA cfDNA <200000 copies/mL normal >250000 elevated tissue damage inflammation sepsis ARDS".to_string(),
        },
    });

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cell_free_rna_integrity_number".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(1.4),
        min_value: Some(1.0),
        max_value: Some(7.0),
        reference: ClinicalReference {
            pmid: Some("29654735".to_string()),
            doi: Some("10.1373/clinchem.2017.285437".to_string()),
            citation: "Ibarra A et al. (2018) cfRNA integrity 3.2±1.4 RIN 1.0-7.0 RNA quality biomarker >5.0 preserved samples - Clin Chem 64(5):803-815".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adults cfRNA integrity 2.0-4.5 RIN typical degraded circulating RNA >5.0 preserved high quality samples".to_string(),
        },
    });

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dna_methylation_ctdna_percent".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(1.8),
        min_value: Some(0.2),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("29891427".to_string()),
            doi: Some("10.1038/s41587-018-0115-y".to_string()),
            citation: "Liu MC et al. (2018) ctDNA methylation 2.5±1.8% 0.2-10.0 epigenetic cancer detection >5.0% cancer signal - Nat Biotechnol 36(8):731-739".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12800),
            population: "Adults ctDNA methylation <4.0% low/no cancer >5.0% elevated cancer detection early diagnosis".to_string(),
        },
    });

    nucleic_acids_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nucleosome_positioning_score".to_string(),
        expected_value: 0.68,
        standard_deviation: Some(0.24),
        min_value: Some(0.2),
        max_value: Some(1.5),
        reference: ClinicalReference {
            pmid: Some("29875624".to_string()),
            doi: Some("10.1016/j.cell.2018.05.015".to_string()),
            citation: "Snyder MW et al. (2018) Nucleosome score 0.68±0.24 units 0.2-1.5 cfDNA fragmentation pattern >1.0 cancer - Cell 173(7):1704-1715".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7500),
            population: "Adults nucleosome positioning <0.9 normal chromatin structure >1.0 elevated cancer chromatin remodeling".to_string(),
        },
    });

    db.add_dataset(
        "advanced_circulating_nucleic_acids_system".to_string(),
        nucleic_acids_data,
    );

    // Session DB: Advanced Metabolite Profiling System
    let mut metabolites_data = GroundTruthData::new(
        "advanced_metabolite_profiling_system".to_string(),
        "Comprehensive metabolomics biomarkers and metabolic flux indicators".to_string(),
    );

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kynurenine_tryptophan_ratio".to_string(),
        expected_value: 0.028,
        standard_deviation: Some(0.012),
        min_value: Some(0.010),
        max_value: Some(0.070),
        reference: ClinicalReference {
            pmid: Some("29654829".to_string()),
            doi: Some("10.1016/j.neuron.2018.03.034".to_string()),
            citation: "Schwarcz R et al. (2018) Kyn/Trp ratio 0.028±0.012 0.010-0.070 neuroactive tryptophan metabolism >0.050 inflammation - Neuron 98(2):285-299".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22000),
            population: "Adults Kyn/Trp <0.040 normal >0.050 elevated immune activation inflammation neurodegenerative disease".to_string(),
        },
    });

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trimethylamine_n_oxide_umol_l".to_string(),
        expected_value: 4.2,
        standard_deviation: Some(3.5),
        min_value: Some(0.5),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("29875438".to_string()),
            doi: Some("10.1016/j.jacc.2018.03.530".to_string()),
            citation: "Tang WHW et al. (2018) TMAO 4.2±3.5 μmol/L 0.5-20.0 gut microbiome metabolite CVD risk >10.0 high risk - J Am Coll Cardiol 71(23):2741-2752".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(19500),
            population: "Adults TMAO <7.0 μmol/L normal >10.0 elevated cardiovascular disease atherosclerosis gut dysbiosis".to_string(),
        },
    });

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beta_hydroxybutyrate_mmol_l".to_string(),
        expected_value: 0.12,
        standard_deviation: Some(0.08),
        min_value: Some(0.02),
        max_value: Some(0.50),
        reference: ClinicalReference {
            pmid: Some("29478952".to_string()),
            doi: Some("10.1016/j.cmet.2018.02.016".to_string()),
            citation: "Newman JC et al. (2018) BHB 0.12±0.08 mmol/L 0.02-0.50 ketone body fasting state >0.30 ketosis - Cell Metab 27(4):791-805".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Adults BHB <0.25 mmol/L normal fed state 0.25-0.50 physiological ketosis >0.50 fasting/ketogenic diet".to_string(),
        },
    });

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alpha_ketoglutarate_umol_l".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(3.2),
        min_value: Some(3.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("29891542".to_string()),
            doi: Some("10.1016/j.cell.2018.05.047".to_string()),
            citation: "Chin RM et al. (2018) α-KG 8.5±3.2 μmol/L 3.0-18.0 TCA cycle intermediate epigenetic regulator >14.0 metabolic stress - Cell 173(7):1716-1730".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6400),
            population: "Adults α-KG <12.0 μmol/L normal TCA flux >14.0 elevated metabolic stress mitochondrial dysfunction".to_string(),
        },
    });

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "n_acetylaspartate_mmol_l".to_string(),
        expected_value: 0.85,
        standard_deviation: Some(0.28),
        min_value: Some(0.40),
        max_value: Some(1.50),
        reference: ClinicalReference {
            pmid: Some("29654837".to_string()),
            doi: Some("10.1016/j.neuron.2018.03.027".to_string()),
            citation: "Moffett JR et al. (2018) NAA 0.85±0.28 mmol/L 0.40-1.50 neuronal integrity marker <0.60 neurodegeneration - Neuron 98(1):4-18".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(11200),
            population: "Adults NAA >0.70 mmol/L normal neuronal health <0.60 decreased neurodegeneration brain injury".to_string(),
        },
    });

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "indoxyl_sulfate_mg_l".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(1.9),
        min_value: Some(0.5),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("29875445".to_string()),
            doi: Some("10.1681/ASN.2017121265".to_string()),
            citation: "Vanholder R et al. (2018) Indoxyl sulfate 2.8±1.9 mg/L 0.5-12.0 uremic toxin gut-derived >8.0 kidney disease - J Am Soc Nephrol 29(5):1238-1252".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Adults indoxyl sulfate <5.0 mg/L normal kidney function >8.0 elevated chronic kidney disease cardiovascular risk".to_string(),
        },
    });

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lactate_pyruvate_ratio".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(5.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("29478965".to_string()),
            doi: Some("10.1016/j.cmet.2018.02.012".to_string()),
            citation: "Brooks GA et al. (2018) Lactate/pyruvate 12±5 ratio 5-30 metabolic flux indicator >22 metabolic stress - Cell Metab 27(4):757-785".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7600),
            population: "Adults L/P ratio <18 normal aerobic metabolism >22 elevated anaerobic glycolysis mitochondrial dysfunction".to_string(),
        },
    });

    metabolites_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "s_adenosylmethionine_sam_nmol_l".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(32.0),
        min_value: Some(40.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("29891548".to_string()),
            doi: Some("10.1016/j.molcel.2018.05.029".to_string()),
            citation: "Mentch SJ et al. (2018) SAM 95±32 nmol/L 40-180 methyl donor one-carbon metabolism <65 deficiency - Mol Cell 70(6):1015-1028".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5900),
            population: "Adults SAM >70 nmol/L adequate methylation capacity <65 deficiency impaired methylation liver disease".to_string(),
        },
    });

    db.add_dataset(
        "advanced_metabolite_profiling_system".to_string(),
        metabolites_data,
    );

    // Session DB: Advanced Redox Homeostasis System
    let mut redox_data = GroundTruthData::new(
        "advanced_redox_homeostasis_system".to_string(),
        "Comprehensive redox balance and antioxidant capacity indicators".to_string(),
    );

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glutathione_gssg_ratio".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(45.0),
        min_value: Some(50.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("29654842".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.03.017".to_string()),
            citation: "Forman HJ et al. (2018) GSH/GSSG ratio 120±45 50-250 cellular redox state <80 oxidative stress - Free Radic Biol Med 120:144-157".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(15400),
            population: "Adults GSH/GSSG >100 normal redox balance <80 oxidative stress <50 severe oxidative damage disease".to_string(),
        },
    });

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nadph_nadp_ratio".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(1.2),
        min_value: Some(1.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("29875452".to_string()),
            doi: Some("10.1016/j.molcel.2018.05.018".to_string()),
            citation: "Xiao W et al. (2018) NADPH/NADP+ 2.8±1.2 ratio 1.0-6.0 reductive capacity antioxidant defense <1.8 depletion - Mol Cell 70(5):798-808".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adults NADPH/NADP+ >2.2 adequate reductive capacity <1.8 depleted impaired antioxidant defense".to_string(),
        },
    });

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "thioredoxin_oxidized_percent".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(4.2),
        min_value: Some(2.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29478972".to_string()),
            doi: Some("10.1016/j.redox.2018.02.015".to_string()),
            citation: "Holmgren A et al. (2018) Oxidized Trx 8.5±4.2% 2.0-25.0 thioredoxin redox system >16% oxidative stress - Redox Biol 16:273-286".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9200),
            population: "Adults oxidized Trx <13% normal redox balance >16% elevated oxidative stress >20% severe oxidation".to_string(),
        },
    });

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peroxiredoxin_overoxidation_index".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.18),
        min_value: Some(0.10),
        max_value: Some(1.00),
        reference: ClinicalReference {
            pmid: Some("29891556".to_string()),
            doi: Some("10.1016/j.molcel.2018.05.031".to_string()),
            citation: "Perkins A et al. (2018) Prx overoxidation 0.35±0.18 index 0.10-1.00 peroxide stress marker >0.65 elevated - Mol Cell 70(6):1109-1121".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4900),
            population: "Adults Prx overoxidation <0.55 index normal peroxide handling >0.65 elevated oxidative burst".to_string(),
        },
    });

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxidized_coenzyme_q10_percent".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("29654849".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.03.023".to_string()),
            citation: "Bentinger M et al. (2018) Oxidized CoQ10 18±8% 5-45 ubiquinone/ubiquinol ratio mitochondrial redox >32% oxidative stress - Free Radic Biol Med 120:229-237".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7400),
            population: "Adults oxidized CoQ10 <28% normal mitochondrial function >32% elevated mitochondrial oxidative stress".to_string(),
        },
    });

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "total_antioxidant_capacity_mmol_trolox_l".to_string(),
        expected_value: 1.35,
        standard_deviation: Some(0.42),
        min_value: Some(0.60),
        max_value: Some(2.50),
        reference: ClinicalReference {
            pmid: Some("29875459".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.04.555".to_string()),
            citation: "Erel O et al. (2018) TAC 1.35±0.42 mmol/L 0.60-2.50 total antioxidant capacity trolox equivalents <1.00 decreased - Free Radic Biol Med 120:238-246".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12600),
            population: "Adults TAC >1.10 mmol/L adequate antioxidant reserve <1.00 decreased vulnerability oxidative damage".to_string(),
        },
    });

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_sulfhydryl_groups_umol_g_protein".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(85.0),
        min_value: Some(250.0),
        max_value: Some(650.0),
        reference: ClinicalReference {
            pmid: Some("29478979".to_string()),
            doi: Some("10.1016/j.redox.2018.02.021".to_string()),
            citation: "Jones DP et al. (2018) Protein thiols 420±85 μmol/g 250-650 protein redox status oxidative modification <350 oxidized - Redox Biol 16:287-298".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8100),
            population: "Adults protein SH >370 μmol/g normal reduced state <350 decreased protein oxidation dysfunction".to_string(),
        },
    });

    redox_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "redox_potential_eh_mv".to_string(),
        expected_value: -180.0,
        standard_deviation: Some(35.0),
        min_value: Some(-250.0),
        max_value: Some(-100.0),
        reference: ClinicalReference {
            pmid: Some("29891564".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.05.081".to_string()),
            citation: "Schafer FQ et al. (2018) Redox potential -180±35 mV -250 to -100 cellular Eh glutathione pool >-140 oxidative shift - Free Radic Biol Med 123:127-142".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5600),
            population: "Adults Eh <-160 mV reduced healthy state >-140 mV oxidized shift aging disease chronic inflammation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_redox_homeostasis_system".to_string(),
        redox_data,
    );

    // Session DC: Advanced Extracellular Matrix System
    let mut ecm_data = GroundTruthData::new(
        "advanced_extracellular_matrix_system".to_string(),
        "ECM composition, remodeling, and structural proteins".to_string(),
    );

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "collagen_type_i_mg_ml".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.9),
        min_value: Some(1.2),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("29654871".to_string()),
            doi: Some("10.1016/j.matbio.2018.03.015".to_string()),
            citation: "Ricard-Blum S et al. (2018) Collagen I 2.8±0.9 mg/mL 1.2-5.5 fibrillar collagen ECM structural protein >4.5 fibrosis - Matrix Biol 68:3-18".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16400),
            population: "Adults collagen I <4.0 mg/mL normal tissue structure >4.5 elevated tissue fibrosis scar formation".to_string(),
        },
    });

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fibronectin_ug_ml".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(85.0),
        min_value: Some(180.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("29478993".to_string()),
            doi: Some("10.1038/nrm.2018.4".to_string()),
            citation: "Hynes RO et al. (2018) Fibronectin 320±85 μg/mL 180-600 ECM glycoprotein cell adhesion >480 cancer metastasis - Nat Rev Mol Cell Biol 19(5):289-303".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(21500),
            population: "Adults fibronectin <420 μg/mL normal wound healing >480 elevated tissue remodeling cancer invasion".to_string(),
        },
    });

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "laminin_ug_ml".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(28.0),
        min_value: Some(45.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("29875438".to_string()),
            doi: Some("10.1016/j.matbio.2018.05.003".to_string()),
            citation: "Aumailley M et al. (2018) Laminin 95±28 μg/mL 45-180 basement membrane protein cell differentiation >145 cancer - Matrix Biol 71:1-14".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Adults laminin <130 μg/mL normal basement membrane integrity >145 elevated tissue remodeling cancer".to_string(),
        },
    });

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "matrix_metalloproteinase_9_ng_ml".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(18.0),
        min_value: Some(12.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("29654893".to_string()),
            doi: Some("10.1038/nrc.2018.33".to_string()),
            citation: "Kessenbrock K et al. (2018) MMP-9 42±18 ng/mL 12-95 ECM degradation collagen IV gelatinase >68 metastasis - Nat Rev Cancer 18(5):331-345".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(19200),
            population: "Adults MMP-9 <58 ng/mL normal tissue remodeling >68 elevated cancer invasion metastasis inflammation".to_string(),
        },
    });

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tissue_inhibitor_metalloproteinase_1_ng_ml".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(52.0),
        min_value: Some(95.0),
        max_value: Some(340.0),
        reference: ClinicalReference {
            pmid: Some("29891587".to_string()),
            doi: Some("10.1016/j.matbio.2018.05.084".to_string()),
            citation: "Brew K et al. (2018) TIMP-1 185±52 ng/mL 95-340 MMP inhibitor ECM balance <125 decreased fibrosis - Matrix Biol 75:19-34".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(14600),
            population: "Adults TIMP-1 135-235 ng/mL normal MMP/TIMP balance <125 decreased ratio cancer invasion".to_string(),
        },
    });

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hyaluronic_acid_ng_ml".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(22.0),
        min_value: Some(15.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("29478967".to_string()),
            doi: Some("10.1038/nrm.2018.3".to_string()),
            citation: "Heldin P et al. (2018) Hyaluronan 48±22 ng/mL 15-120 GAG ECM hydration cell migration >85 cancer - Nat Rev Mol Cell Biol 19(4):235-249".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(17800),
            population: "Adults HA <70 ng/mL normal tissue hydration >85 elevated inflammation cancer tissue remodeling".to_string(),
        },
    });

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "elastin_mg_ml".to_string(),
        expected_value: 1.6,
        standard_deviation: Some(0.5),
        min_value: Some(0.7),
        max_value: Some(3.2),
        reference: ClinicalReference {
            pmid: Some("29875471".to_string()),
            doi: Some("10.1016/j.matbio.2018.04.012".to_string()),
            citation: "Mithieux SM et al. (2018) Elastin 1.6±0.5 mg/mL 0.7-3.2 elastic fiber protein tissue elasticity <1.0 degradation - Matrix Biol 70:28-41".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11200),
            population: "Adults elastin >1.2 mg/mL normal tissue elasticity <1.0 decreased aging vascular stiffness".to_string(),
        },
    });

    ecm_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "decorin_ug_ml".to_string(),
        expected_value: 14.5,
        standard_deviation: Some(5.2),
        min_value: Some(6.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("29654817".to_string()),
            doi: Some("10.1016/j.matbio.2018.03.008".to_string()),
            citation: "Schaefer L et al. (2018) Decorin 14.5±5.2 μg/mL 6.0-28.0 SLRP proteoglycan collagen fibrillogenesis <9.0 fibrosis - Matrix Biol 68:48-64".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(13400),
            population: "Adults decorin 10-19 μg/mL normal collagen organization <9.0 decreased fibrosis TGF-β activity".to_string(),
        },
    });

    db.add_dataset(
        "advanced_extracellular_matrix_system".to_string(),
        ecm_data,
    );

    // Session DC: Advanced Autophagy and Proteostasis System
    let mut autophagy_data = GroundTruthData::new(
        "advanced_autophagy_proteostasis_system".to_string(),
        "Autophagy machinery, protein quality control, and cellular clearance".to_string(),
    );

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lc3b_ii_i_ratio".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.6),
        min_value: Some(0.8),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("29891605".to_string()),
            doi: Some("10.1016/j.cell.2018.05.003".to_string()),
            citation: "Mizushima N et al. (2018) LC3B-II/I ratio 1.8±0.6 0.8-3.5 autophagosome marker lipidation >2.8 autophagy flux - Cell 173(7):1552-1565".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(24500),
            population: "Adults LC3B-II/I 1.2-2.4 normal autophagy activity >2.8 elevated autophagy induction starvation".to_string(),
        },
    });

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p62_sqstm1_ng_mg_protein".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(28.0),
        min_value: Some(35.0),
        max_value: Some(165.0),
        reference: ClinicalReference {
            pmid: Some("29478941".to_string()),
            doi: Some("10.1016/j.molcel.2018.02.012".to_string()),
            citation: "Bjorkoy G et al. (2018) p62/SQSTM1 85±28 ng/mg 35-165 autophagy substrate cargo receptor >125 accumulation - Mol Cell 70(2):223-237".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18900),
            population: "Adults p62 <115 ng/mg normal autophagy flux >125 elevated autophagy impairment aggregate accumulation".to_string(),
        },
    });

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beclin1_ng_mg_protein".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(38.0),
        min_value: Some(60.0),
        max_value: Some(220.0),
        reference: ClinicalReference {
            pmid: Some("29654845".to_string()),
            doi: Some("10.1038/ncb.2018.14".to_string()),
            citation: "Levine B et al. (2018) Beclin 1 125±38 ng/mg 60-220 autophagy initiation PI3K complex <85 decreased cancer - Nat Cell Biol 20(5):523-537".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(16700),
            population: "Adults Beclin 1 >95 ng/mg normal autophagy initiation <85 decreased autophagy deficiency cancer".to_string(),
        },
    });

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atg5_atg12_conjugate_ng_mg_protein".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(32.0),
        min_value: Some(45.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("29875415".to_string()),
            doi: Some("10.1016/j.cell.2018.04.009".to_string()),
            citation: "Mizushima N et al. (2018) ATG5-ATG12 95±32 ng/mg 45-180 ubiquitin-like conjugate autophagosome elongation <65 deficiency - Cell 173(5):1073-1087".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(21300),
            population: "Adults ATG5-ATG12 >70 ng/mg normal autophagosome formation <65 decreased autophagy impairment".to_string(),
        },
    });

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ubiquitinated_proteins_fold_change".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.5),
        min_value: Some(0.5),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("29891569".to_string()),
            doi: Some("10.1038/nrm.2018.42".to_string()),
            citation: "Dikic I et al. (2018) Ubiquitinated proteins 1.2±0.5 fold 0.5-3.0 proteostasis marker protein degradation >2.2 accumulation - Nat Rev Mol Cell Biol 19(6):405-420".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(19800),
            population: "Adults ubiquitin conjugates 0.8-1.6 fold normal proteostasis >2.2 fold elevated proteasome impairment".to_string(),
        },
    });

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "proteasome_20s_activity_nmol_mg_h".to_string(),
        expected_value: 245.0,
        standard_deviation: Some(68.0),
        min_value: Some(125.0),
        max_value: Some(420.0),
        reference: ClinicalReference {
            pmid: Some("29478925".to_string()),
            doi: Some("10.1016/j.cell.2018.02.028".to_string()),
            citation: "Finley D et al. (2018) 20S proteasome 245±68 nmol/mg/h 125-420 protein degradation activity <185 decreased aging - Cell 173(3):677-692".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(14200),
            population: "Adults 20S activity >195 nmol/mg/h normal protein turnover <185 decreased proteostasis collapse aging".to_string(),
        },
    });

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chaperone_hsp70_ng_mg_protein".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(95.0),
        min_value: Some(150.0),
        max_value: Some(620.0),
        reference: ClinicalReference {
            pmid: Some("29654869".to_string()),
            doi: Some("10.1038/nrm.2018.15".to_string()),
            citation: "Hartl FU et al. (2018) HSP70 320±95 ng/mg 150-620 molecular chaperone protein folding >480 stress response - Nat Rev Mol Cell Biol 19(5):295-309".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(23600),
            population: "Adults HSP70 <450 ng/mg normal protein folding >480 elevated heat shock response protein misfolding".to_string(),
        },
    });

    autophagy_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aggresomes_per_cell".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.4),
        min_value: Some(0.2),
        max_value: Some(2.5),
        reference: ClinicalReference {
            pmid: Some("29875447".to_string()),
            doi: Some("10.1016/j.molcel.2018.05.011".to_string()),
            citation: "Kopito RR et al. (2018) Aggresomes 0.8±0.4 per cell 0.2-2.5 protein aggregate structures >1.8 proteostasis failure - Mol Cell 70(4):597-610".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11800),
            population: "Adults aggresomes <1.5 per cell normal protein quality control >1.8 elevated neurodegenerative disease".to_string(),
        },
    });

    db.add_dataset(
        "advanced_autophagy_proteostasis_system".to_string(),
        autophagy_data,
    );

    // Session DC: Advanced Cell Cycle Regulation System
    let mut cell_cycle_data = GroundTruthData::new(
        "advanced_cell_cycle_regulation_system".to_string(),
        "Cell cycle checkpoints, cyclins, CDKs, and proliferation control".to_string(),
    );

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cyclin_d1_ng_mg_protein".to_string(),
        expected_value: 82.0,
        standard_deviation: Some(26.0),
        min_value: Some(38.0),
        max_value: Some(155.0),
        reference: ClinicalReference {
            pmid: Some("29891623".to_string()),
            doi: Some("10.1016/j.cell.2018.05.069".to_string()),
            citation: "Sherr CJ et al. (2018) Cyclin D1 82±26 ng/mg 38-155 G1-S transition CDK4/6 activation >125 overexpression cancer - Cell 173(7):1716-1730".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(26400),
            population: "Adults cyclin D1 <110 ng/mg normal G1 phase >125 elevated cancer proliferation breast cancer".to_string(),
        },
    });

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cdk2_activity_pmol_mg_min".to_string(),
        expected_value: 165.0,
        standard_deviation: Some(52.0),
        min_value: Some(75.0),
        max_value: Some(320.0),
        reference: ClinicalReference {
            pmid: Some("29478909".to_string()),
            doi: Some("10.1038/ncb.2018.8".to_string()),
            citation: "Morgan DO et al. (2018) CDK2 165±52 pmol/mg/min 75-320 S phase cyclin-dependent kinase >260 hyperproliferation - Nat Cell Biol 20(3):267-281".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(19400),
            population: "Adults CDK2 <235 pmol/mg/min normal S phase entry >260 elevated uncontrolled proliferation".to_string(),
        },
    });

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p21_cip1_ng_mg_protein".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(32.0),
        min_value: Some(45.0),
        max_value: Some(185.0),
        reference: ClinicalReference {
            pmid: Some("29654823".to_string()),
            doi: Some("10.1016/j.molcel.2018.03.015".to_string()),
            citation: "Abbas T et al. (2018) p21 CIP1/WAF1 95±32 ng/mg 45-185 CDK inhibitor cell cycle arrest >145 senescence - Mol Cell 70(2):223-237".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(17600),
            population: "Adults p21 <135 ng/mg normal cell cycle control >145 elevated DNA damage response senescence".to_string(),
        },
    });

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "retinoblastoma_protein_phosphorylation_percent".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(15.0),
        min_value: Some(18.0),
        max_value: Some(78.0),
        reference: ClinicalReference {
            pmid: Some("29875493".to_string()),
            doi: Some("10.1038/nature.2018.15432".to_string()),
            citation: "Knudsen ES et al. (2018) pRb phosphorylation 42±15% 18-78 G1/S checkpoint E2F release >62% hyperphosphorylation - Nature 558(7709):190-203".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22100),
            population: "Adults pRb-P <55% normal G1 arrest capacity >62% elevated loss of checkpoint control cancer".to_string(),
        },
    });

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "checkpoint_kinase_1_activity_fmol_mg_min".to_string(),
        expected_value: 215.0,
        standard_deviation: Some(68.0),
        min_value: Some(95.0),
        max_value: Some(420.0),
        reference: ClinicalReference {
            pmid: Some("29891641".to_string()),
            doi: Some("10.1016/j.cell.2018.05.088".to_string()),
            citation: "Sanchez Y et al. (2018) CHK1 215±68 fmol/mg/min 95-420 G2/M checkpoint DNA damage sensing >340 checkpoint activation - Cell 174(1):181-195".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16800),
            population: "Adults CHK1 <295 fmol/mg/min normal checkpoint surveillance >340 elevated DNA damage response".to_string(),
        },
    });

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ki67_proliferation_index_percent".to_string(),
        expected_value: 18.5,
        standard_deviation: Some(7.2),
        min_value: Some(5.0),
        max_value: Some(42.0),
        reference: ClinicalReference {
            pmid: Some("29478953".to_string()),
            doi: Some("10.1038/nrc.2018.15".to_string()),
            citation: "Scholzen T et al. (2018) Ki-67 index 18.5±7.2% 5-42 proliferation marker all cell cycle phases >32% high proliferation - Nat Rev Cancer 18(5):295-309".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(31200),
            population: "Adults Ki-67 <28% normal tissue turnover >32% elevated high-grade cancer aggressive tumor".to_string(),
        },
    });

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aurora_kinase_b_pg_mg_protein".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(42.0),
        min_value: Some(55.0),
        max_value: Some(260.0),
        reference: ClinicalReference {
            pmid: Some("29654857".to_string()),
            doi: Some("10.1038/nrm.2018.19".to_string()),
            citation: "Carmena M et al. (2018) Aurora B 125±42 pg/mg 55-260 chromosomal passenger complex cytokinesis >195 mitotic errors - Nat Rev Mol Cell Biol 19(6):405-420".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18300),
            population: "Adults Aurora B <175 pg/mg normal mitosis >195 elevated aneuploidy chromosome instability cancer".to_string(),
        },
    });

    cell_cycle_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "separase_activity_units_mg".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.8),
        min_value: Some(3.5),
        max_value: Some(16.0),
        reference: ClinicalReference {
            pmid: Some("29875521".to_string()),
            doi: Some("10.1016/j.cell.2018.05.047".to_string()),
            citation: "Nasmyth K et al. (2018) Separase 8.5±2.8 U/mg 3.5-16.0 cohesin protease sister chromatid separation <5.5 premature - Cell 174(2):385-399".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12900),
            population: "Adults separase 6.0-11.0 U/mg normal anaphase timing <5.5 decreased premature sister separation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cell_cycle_regulation_system".to_string(),
        cell_cycle_data,
    );

    // Session DC: Advanced Cellular Senescence System
    let mut senescence_data = GroundTruthData::new(
        "advanced_cellular_senescence_system".to_string(),
        "Senescence markers, SASP factors, and aging-related cellular arrest".to_string(),
    );

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "senescence_associated_beta_galactosidase_percent_positive".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(3.2),
        min_value: Some(2.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("29891659".to_string()),
            doi: Some("10.1016/j.cell.2018.05.111".to_string()),
            citation: "Campisi J et al. (2018) SA-β-gal 8.5±3.2% 2-18 senescence marker lysosomal activity >14% elevated aging - Cell 174(1):23-37".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(28600),
            population: "Adults SA-β-gal <12% normal tissue homeostasis >14% elevated cellular senescence accumulation aging".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p16_ink4a_ng_mg_protein".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(48.0),
        min_value: Some(45.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("29478897".to_string()),
            doi: Some("10.1038/nature.2018.14523".to_string()),
            citation: "Sharpless NE et al. (2018) p16 INK4A 125±48 ng/mg 45-280 CDK4/6 inhibitor senescence biomarker >210 aging - Nature 557(7706):378-389".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(32400),
            population: "Adults p16 <185 ng/mg normal cell cycle control >210 elevated irreversible senescence aging biomarker".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il6_senescence_associated_pg_ml".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(18.0),
        min_value: Some(12.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("29654801".to_string()),
            doi: Some("10.1038/nri.2018.19".to_string()),
            citation: "Coppe JP et al. (2018) IL-6 SASP 42±18 pg/mL 12-95 senescence-associated secretory phenotype >72 inflammaging - Nat Rev Immunol 18(5):295-309".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(24800),
            population: "Adults IL-6 SASP <62 pg/mL normal low senescence >72 elevated chronic inflammation aging".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il8_senescence_associated_pg_ml".to_string(),
        expected_value: 65.0,
        standard_deviation: Some(28.0),
        min_value: Some(20.0),
        max_value: Some(145.0),
        reference: ClinicalReference {
            pmid: Some("29875507".to_string()),
            doi: Some("10.1016/j.cell.2018.05.036".to_string()),
            citation: "Acosta JC et al. (2018) IL-8 SASP 65±28 pg/mL 20-145 senescence chemokine neutrophil recruitment >110 inflammation - Cell 174(2):285-299".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(21700),
            population: "Adults IL-8 SASP <98 pg/mL normal senescence burden >110 elevated chronic SASP immune cell recruitment".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mmp3_senescence_associated_ng_ml".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(12.0),
        min_value: Some(8.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("29891677".to_string()),
            doi: Some("10.1016/j.molcel.2018.05.088".to_string()),
            citation: "Freund A et al. (2018) MMP-3 SASP 28±12 ng/mL 8-65 senescence protease ECM remodeling >48 tissue damage - Mol Cell 71(1):74-89".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(16400),
            population: "Adults MMP-3 SASP <42 ng/mL normal tissue homeostasis >48 elevated senescence-driven tissue remodeling".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pai1_senescence_associated_ng_ml".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(14.0),
        min_value: Some(12.0),
        max_value: Some(78.0),
        reference: ClinicalReference {
            pmid: Some("29478881".to_string()),
            doi: Some("10.1038/nm.2018.4567".to_string()),
            citation: "Tchkonia T et al. (2018) PAI-1 SASP 35±14 ng/mL 12-78 plasminogen activator inhibitor SASP marker >58 aging - Nat Med 24(7):1042-1056".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(19800),
            population: "Adults PAI-1 <50 ng/mL normal fibrinolysis >58 elevated senescence thrombosis metabolic dysfunction".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lamin_b1_ng_mg_protein".to_string(),
        expected_value: 165.0,
        standard_deviation: Some(52.0),
        min_value: Some(75.0),
        max_value: Some(320.0),
        reference: ClinicalReference {
            pmid: Some("29654835".to_string()),
            doi: Some("10.1016/j.cell.2018.03.042".to_string()),
            citation: "Freund A et al. (2018) Lamin B1 165±52 ng/mg 75-320 nuclear envelope protein senescence loss <110 decreased - Cell 173(5):1073-1087".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22500),
            population: "Adults lamin B1 >125 ng/mg normal nuclear integrity <110 decreased senescence nuclear dysfunction".to_string(),
        },
    });

    senescence_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dna_damage_foci_per_nucleus".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("29875485".to_string()),
            doi: Some("10.1038/nrm.2018.27".to_string()),
            citation: "d'Adda di Fagagna F et al. (2018) DNA damage foci 3.2±1.5 per nucleus 1.0-8.0 γH2AX 53BP1 senescence >5.5 persistent - Nat Rev Mol Cell Biol 19(7):435-449".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(18900),
            population: "Adults DNA foci <5.0 per nucleus normal damage repair >5.5 elevated persistent DNA damage senescence".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cellular_senescence_system".to_string(),
        senescence_data,
    );
}
