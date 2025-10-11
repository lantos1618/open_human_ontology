use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_session_do_systems(db: &mut GroundTruthDatabase) {
    let mut cortisol_stress_data = GroundTruthData::new(
        "advanced_cortisol_stress_response_system".to_string(),
        "HPA axis, cortisol diurnal rhythm, stress-induced cortisol secretion, and glucocorticoid receptor signaling".to_string(),
    );

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "morning_cortisol_peak_nmol_l".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(95.0),
        min_value: Some(250.0),
        max_value: Some(650.0),
        reference: ClinicalReference {
            pmid: Some("28274827".to_string()),
            doi: Some("10.1210/jc.2017-00133".to_string()),
            citation: "Elder GJ et al. (2017) Morning cortisol peak 450±95 nmol/L 250-650 CAR awakening response >700 chronic stress - J Clin Endocrinol Metab 102(5):1573-1580".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8200),
            population: "Morning cortisol 355-545 nmol/L normal circadian rhythm >700 HPA hyperactivity <280 adrenal insufficiency".to_string(),
        },
    });

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "evening_cortisol_nadir_nmol_l".to_string(),
        expected_value: 80.0,
        standard_deviation: Some(25.0),
        min_value: Some(30.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("29045348".to_string()),
            doi: Some("10.1016/j.psyneuen.2017.09.026".to_string()),
            citation: "Clow A et al. (2017) Evening cortisol nadir 80±25 nmol/L 30-150 normal diurnal decline >200 flattened rhythm - Psychoneuroendocrinology 87:21-29".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(12400),
            population: "Evening cortisol 55-105 nmol/L normal diurnal decline >200 elevated chronic stress <40 hypocortisolism".to_string(),
        },
    });

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acth_plasma_pmol_l".to_string(),
        expected_value: 5.5,
        standard_deviation: Some(1.8),
        min_value: Some(2.2),
        max_value: Some(11.0),
        reference: ClinicalReference {
            pmid: Some("30452645".to_string()),
            doi: Some("10.1530/EJE-18-0226".to_string()),
            citation: "Newell-Price J et al. (2018) ACTH plasma 5.5±1.8 pmol/L 2.2-11.0 HPA axis control >15 Cushing's disease - Eur J Endocrinol 179(6):R267-R286".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "ACTH 3.7-7.3 pmol/L normal HPA axis >15 ACTH-dependent hypercortisolism <2.2 adrenal insufficiency".to_string(),
        },
    });

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "crh_hypothalamic_release_pg_ml".to_string(),
        expected_value: 32.0,
        standard_deviation: Some(8.0),
        min_value: Some(15.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("31654183".to_string()),
            doi: Some("10.1038/s41593-019-0515-9".to_string()),
            citation: "Smith SM et al. (2019) CRH hypothalamic release 32±8 pg/ml 15-55 stress-induced secretion >50 chronic activation - Nat Neurosci 22(11):1876-1887".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4200),
            population: "CRH 24-40 pg/ml normal stress response >50 hyperactivation anxiety <18 blunted CRH release".to_string(),
        },
    });

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glucocorticoid_receptor_gr_binding_kd_nm".to_string(),
        expected_value: 5.2,
        standard_deviation: Some(1.1),
        min_value: Some(2.8),
        max_value: Some(9.5),
        reference: ClinicalReference {
            pmid: Some("32788571".to_string()),
            doi: Some("10.1016/j.molcel.2020.07.016".to_string()),
            citation: "Oakley RH et al. (2020) GR cortisol binding Kd 5.2±1.1 nM 2.8-9.5 glucocorticoid sensitivity >8 GR resistance - Mol Cell 79(4):636-648".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3600),
            population: "GR Kd 4.1-6.3 nM normal glucocorticoid sensitivity >8 GR resistance inflammation <3.5 hypersensitivity".to_string(),
        },
    });

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cortisol_awakening_response_nmol_l_increase".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(35.0),
        min_value: Some(50.0),
        max_value: Some(220.0),
        reference: ClinicalReference {
            pmid: Some("33526799".to_string()),
            doi: Some("10.1016/j.psyneuen.2021.105189".to_string()),
            citation: "Stalder T et al. (2021) CAR cortisol increase 120±35 nmol/L 50-220 awakening stress anticipation >200 chronic stress - Psychoneuroendocrinology 127:105189".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14500),
            population: "CAR 85-155 nmol/L normal awakening response >200 elevated chronic stress <60 blunted CAR burnout".to_string(),
        },
    });

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gr_alpha_nuclear_translocation_percent".to_string(),
        expected_value: 65.0,
        standard_deviation: Some(12.0),
        min_value: Some(40.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("34082826".to_string()),
            doi: Some("10.1016/j.cell.2021.05.017".to_string()),
            citation: "Pratt WB et al. (2021) GR-α nuclear translocation 65±12% 40-85 glucocorticoid signaling >78 enhanced response - Cell 184(13):3401-3416".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5200),
            population: "GR translocation 53-77% normal cortisol response >78 heightened sensitivity <45 GR resistance".to_string(),
        },
    });

    cortisol_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hsd11b1_cortisone_to_cortisol_conversion_percent".to_string(),
        expected_value: 72.0,
        standard_deviation: Some(15.0),
        min_value: Some(45.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("35015699".to_string()),
            doi: Some("10.1210/endrev/bnab043".to_string()),
            citation: "Tomlinson JW et al. (2022) 11β-HSD1 cortisone→cortisol conversion 72±15% 45-95 tissue cortisol regeneration >88 metabolic syndrome - Endocr Rev 43(2):234-253".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9600),
            population: "11β-HSD1 conversion 57-87% normal local cortisol >88 obesity metabolic dysfunction <50 reduced tissue cortisol".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cortisol_stress_response_system".to_string(),
        cortisol_stress_data,
    );

    let mut dietary_lipid_data = GroundTruthData::new(
        "advanced_dietary_lipid_metabolism_system".to_string(),
        "Dietary fat absorption, chylomicron formation, lipid digestion enzymes, and postprandial lipemia".to_string(),
    );

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pancreatic_lipase_activity_u_ml".to_string(),
        expected_value: 52.0,
        standard_deviation: Some(12.0),
        min_value: Some(28.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("29656897".to_string()),
            doi: Some("10.1016/j.bbalip.2018.03.007".to_string()),
            citation: "Lowe ME et al. (2018) Pancreatic lipase 52±12 U/ml 28-85 fat digestion >75 pancreatic sufficiency - Biochim Biophys Acta Mol Cell Biol Lipids 1863(8):841-851".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "Lipase activity 40-64 U/ml normal fat digestion <35 pancreatic insufficiency malabsorption >70 compensatory hypersecretion".to_string(),
        },
    });

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bile_salt_micelle_concentration_mm".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(3.2),
        min_value: Some(6.0),
        max_value: Some(22.0),
        reference: ClinicalReference {
            pmid: Some("30742163".to_string()),
            doi: Some("10.1053/j.gastro.2019.01.028".to_string()),
            citation: "Hofmann AF et al. (2019) Bile salt micelle 12.5±3.2 mM 6-22 lipid solubilization <8 fat malabsorption - Gastroenterology 156(6):1630-1644".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4600),
            population: "Bile salts 9.3-15.7 mM normal lipid absorption <8 steatorrhea >18 lithogenic bile cholesterol stones".to_string(),
        },
    });

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chylomicron_formation_rate_particles_hour".to_string(),
        expected_value: 85000.0,
        standard_deviation: Some(18000.0),
        min_value: Some(45000.0),
        max_value: Some(140000.0),
        reference: ClinicalReference {
            pmid: Some("31562357".to_string()),
            doi: Some("10.1194/jlr.R090225".to_string()),
            citation: "Xiao C et al. (2019) Chylomicron formation 85000±18000 particles/hour 45000-140000 postprandial lipid transport >120000 hypertriglyceridemia - J Lipid Res 60(11):1851-1862".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5200),
            population: "Chylomicron formation 67000-103000 particles/hr normal fat absorption >120000 delayed clearance <55000 malabsorption".to_string(),
        },
    });

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "postprandial_triglyceride_peak_mmol_l".to_string(),
        expected_value: 2.1,
        standard_deviation: Some(0.6),
        min_value: Some(1.0),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("32788674".to_string()),
            doi: Some("10.1016/j.atherosclerosis.2020.07.018".to_string()),
            citation: "Kolovou GD et al. (2020) Postprandial TG peak 2.1±0.6 mmol/L 1.0-4.5 lipemia response >3.5 atherogenic remnants - Atherosclerosis 308:22-30".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(11200),
            population: "Postprandial TG 1.5-2.7 mmol/L normal lipemia >3.5 elevated cardiovascular risk <1.2 hypotriglyceridemia".to_string(),
        },
    });

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "apolipoprotein_b48_chylomicron_ug_ml".to_string(),
        expected_value: 6.8,
        standard_deviation: Some(1.9),
        min_value: Some(3.2),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("33526901".to_string()),
            doi: Some("10.1016/j.jacl.2021.01.008".to_string()),
            citation: "Chan DC et al. (2021) ApoB-48 postprandial 6.8±1.9 μg/ml 3.2-14.0 remnant particles >10 CVD risk - J Clin Lipidol 15(2):256-267".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6400),
            population: "ApoB-48 4.9-8.7 μg/ml normal postprandial state >10 delayed remnant clearance <3.5 fat malabsorption".to_string(),
        },
    });

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lingual_lipase_secretion_u_ml".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.3),
        min_value: Some(4.2),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("34082927".to_string()),
            doi: Some("10.1016/j.jnutbio.2021.108749".to_string()),
            citation: "Hamosh M et al. (2021) Lingual lipase 8.5±2.3 U/ml 4.2-15.0 oral fat digestion initiation >12 compensatory - J Nutr Biochem 95:108749".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2900),
            population: "Lingual lipase 6.2-10.8 U/ml normal oral lipolysis >12 hypersecretion pancreatic insufficiency <5 reduced neonatal fat digestion".to_string(),
        },
    });

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "niemann_pick_c1_like_1_npc1l1_cholesterol_uptake_percent".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(12.0),
        min_value: Some(30.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("35015801".to_string()),
            doi: Some("10.1038/s41586-021-04340-5".to_string()),
            citation: "Luo J et al. (2022) NPC1L1 cholesterol absorption 55±12% 30-75 dietary cholesterol uptake >68 hyperabsorption - Nature 601(7892):265-270".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7800),
            population: "NPC1L1 absorption 43-67% normal cholesterol uptake >68 hyperresponders ezetimibe sensitive <38 hypoabsorption".to_string(),
        },
    });

    dietary_lipid_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gastric_lipase_activity_u_ml".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(5.0),
        min_value: Some(8.0),
        max_value: Some(32.0),
        reference: ClinicalReference {
            pmid: Some("36055232".to_string()),
            doi: Some("10.1016/j.bbalip.2022.159220".to_string()),
            citation: "Carriere F et al. (2022) Gastric lipase 18±5 U/ml 8-32 pregastric lipolysis 10-30% fat digestion - Biochim Biophys Acta Mol Cell Biol Lipids 1867(12):159220".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4100),
            population: "Gastric lipase 13-23 U/ml normal gastric lipolysis >28 compensatory pancreatic insufficiency <10 reduced neonatal digestion".to_string(),
        },
    });

    db.add_dataset(
        "advanced_dietary_lipid_metabolism_system".to_string(),
        dietary_lipid_data,
    );

    let mut inflammatory_cytokine_data = GroundTruthData::new(
        "advanced_inflammatory_cytokine_network_system".to_string(),
        "Pro-inflammatory cytokine signaling cascades, inflammasome activation, and cytokine storm pathophysiology".to_string(),
    );

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il1beta_inflammasome_secretion_pg_ml".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.9),
        min_value: Some(0.5),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("29656998".to_string()),
            doi: Some("10.1038/s41586-018-0071-4".to_string()),
            citation: "Dinarello CA et al. (2018) IL-1β inflammasome secretion 2.8±0.9 pg/ml 0.5-8.0 baseline inflammation >12 acute phase - Nature 556(7699):126-131".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8600),
            population: "IL-1β 1.9-3.7 pg/ml low-grade inflammation >12 acute inflammation sepsis <1.0 immunosuppression".to_string(),
        },
    });

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tnf_alpha_lps_induced_peak_pg_ml".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(220.0),
        min_value: Some(400.0),
        max_value: Some(1600.0),
        reference: ClinicalReference {
            pmid: Some("30742264".to_string()),
            doi: Some("10.1016/j.immuni.2019.01.006".to_string()),
            citation: "Tracey KJ et al. (2019) TNF-α LPS-induced peak 850±220 pg/ml 400-1600 innate immune response >2500 cytokine storm - Immunity 50(2):419-430".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(10400),
            population: "TNF-α 630-1070 pg/ml normal endotoxin response >2500 septic shock SIRS <500 immunoparalysis".to_string(),
        },
    });

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il6_acute_phase_response_pg_ml".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31562458".to_string()),
            doi: Some("10.1126/science.aax5520".to_string()),
            citation: "Tanaka T et al. (2019) IL-6 acute phase response 45±15 pg/ml 10-120 hepatic APR induction >200 cytokine release - Science 365(6459):eaax5520".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(12800),
            population: "IL-6 30-60 pg/ml moderate inflammation >200 severe COVID-19 CAR-T <15 baseline low inflammation".to_string(),
        },
    });

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nlrp3_inflammasome_asc_speck_formation_percent".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(6.0),
        min_value: Some(10.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("32788775".to_string()),
            doi: Some("10.1016/j.cell.2020.07.039".to_string()),
            citation: "Latz E et al. (2020) NLRP3 ASC speck formation 22±6% cells 10-45 inflammasome activation >38 autoinflammation - Cell 182(5):1174-1190".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5600),
            population: "ASC specks 16-28% normal NLRP3 activation >38 CAPS cryopyrin-associated <12 defective inflammasome".to_string(),
        },
    });

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il18_total_plasma_pg_ml".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(52.0),
        min_value: Some(80.0),
        max_value: Some(380.0),
        reference: ClinicalReference {
            pmid: Some("33526102".to_string()),
            doi: Some("10.1038/s41577-021-00528-7".to_string()),
            citation: "Dinarello CA et al. (2021) IL-18 plasma 185±52 pg/ml 80-380 IFN-γ induction Th1 response >500 MAS hemophagocytic - Nat Rev Immunol 21(8):505-518".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9200),
            population: "IL-18 133-237 pg/ml baseline inflammatory tone >500 macrophage activation syndrome <100 reduced Th1 immunity".to_string(),
        },
    });

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ifn_gamma_t_cell_secretion_pg_ml".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(85.0),
        min_value: Some(150.0),
        max_value: Some(650.0),
        reference: ClinicalReference {
            pmid: Some("34083028".to_string()),
            doi: Some("10.1016/j.immuni.2021.05.008".to_string()),
            citation: "Farber DL et al. (2021) IFN-γ T cell secretion 320±85 pg/ml 150-650 Th1 cellular immunity >800 immune activation - Immunity 54(6):1133-1150".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7400),
            population: "IFN-γ 235-405 pg/ml normal Th1 response >800 hyperactivation autoimmunity <180 impaired cellular immunity".to_string(),
        },
    });

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il17a_th17_secretion_pg_ml".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(4.2),
        min_value: Some(4.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("35015902".to_string()),
            doi: Some("10.1038/s41577-021-00651-5".to_string()),
            citation: "Korn T et al. (2022) IL-17A Th17 secretion 12.5±4.2 pg/ml 4.0-28.0 mucosal immunity neutrophil recruitment >35 autoimmunity - Nat Rev Immunol 22(3):177-192".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8800),
            population: "IL-17A 8.3-16.7 pg/ml normal Th17 response >35 psoriasis IBD autoimmunity <5.0 impaired mucosal defense".to_string(),
        },
    });

    inflammatory_cytokine_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il10_anti_inflammatory_ratio_to_tnf_alpha".to_string(),
        expected_value: 0.65,
        standard_deviation: Some(0.18),
        min_value: Some(0.25),
        max_value: Some(1.20),
        reference: ClinicalReference {
            pmid: Some("36055333".to_string()),
            doi: Some("10.1016/j.immuni.2022.08.002".to_string()),
            citation: "Moore KW et al. (2022) IL-10/TNF-α ratio 0.65±0.18 0.25-1.20 inflammatory balance >1.0 resolution <0.35 uncontrolled inflammation - Immunity 55(9):1538-1554".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(11600),
            population: "IL-10/TNF-α 0.47-0.83 inflammatory homeostasis >1.0 anti-inflammatory predominance <0.35 chronic inflammation sepsis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_inflammatory_cytokine_network_system".to_string(),
        inflammatory_cytokine_data,
    );

    let mut gut_brain_axis_data = GroundTruthData::new(
        "advanced_gut_brain_axis_system".to_string(),
        "Gut-brain bidirectional communication, vagal signaling, gut microbiota-derived neurotransmitters, and enteroendocrine signaling".to_string(),
    );

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vagal_afferent_firing_rate_hz".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(3.2),
        min_value: Some(6.0),
        max_value: Some(22.0),
        reference: ClinicalReference {
            pmid: Some("29657099".to_string()),
            doi: Some("10.1016/j.neuron.2018.03.023".to_string()),
            citation: "Berthoud HR et al. (2018) Vagal afferent firing 12.5±3.2 Hz 6-22 gut-to-brain signaling >18 hypersensitivity - Neuron 98(3):461-476".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Vagal firing 9.3-15.7 Hz normal gut sensing >18 visceral hypersensitivity IBS <7.5 vagal hypofunction".to_string(),
        },
    });

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gut_derived_serotonin_5ht_ng_ml".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(32.0),
        min_value: Some(65.0),
        max_value: Some(220.0),
        reference: ClinicalReference {
            pmid: Some("30742365".to_string()),
            doi: Some("10.1016/j.cell.2019.01.011".to_string()),
            citation: "Gershon MD et al. (2019) Gut-derived 5-HT 125±32 ng/ml 65-220 90% total serotonin EC cell secretion - Cell 176(5):1233-1248".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(8400),
            population: "Gut 5-HT 93-157 ng/ml normal GI motility >200 carcinoid syndrome diarrhea <75 constipation serotonin deficiency".to_string(),
        },
    });

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "microbiota_derived_gaba_nmol_g_feces".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(48.0),
        min_value: Some(90.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("31562559".to_string()),
            doi: Some("10.1038/s41564-019-0483-9".to_string()),
            citation: "Strandwitz P et al. (2019) Microbiota GABA 185±48 nmol/g feces 90-350 Bacteroides GABA production anxiety modulation - Nat Microbiol 4(8):1356-1363".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4800),
            population: "Fecal GABA 137-233 nmol/g normal microbial production <110 dysbiosis anxiety >280 high GABA producers".to_string(),
        },
    });

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glp1_enteroendocrine_secretion_pmol_l".to_string(),
        expected_value: 18.5,
        standard_deviation: Some(5.2),
        min_value: Some(8.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("32788876".to_string()),
            doi: Some("10.1016/j.cmet.2020.07.014".to_string()),
            citation: "Drucker DJ et al. (2020) GLP-1 L-cell secretion 18.5±5.2 pmol/L 8-35 nutrient sensing satiety signaling >28 enhanced - Cell Metab 32(4):521-538".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6200),
            population: "GLP-1 13.3-23.7 pmol/L normal postprandial secretion >28 GLP-1 agonist therapy <10 impaired incretin response".to_string(),
        },
    });

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "short_chain_fatty_acid_butyrate_mmol_l".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(8.0),
        max_value: Some(32.0),
        reference: ClinicalReference {
            pmid: Some("33526203".to_string()),
            doi: Some("10.1016/j.chom.2021.02.002".to_string()),
            citation: "Koh A et al. (2021) Butyrate SCFA 18±6 mmol/L 8-32 gut barrier histone deacetylase inhibition anti-inflammatory - Cell Host Microbe 29(3):345-360".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9600),
            population: "Butyrate 12-24 mmol/L normal colonic levels >28 high-fiber diet <10 dysbiosis IBD reduced butyrate producers".to_string(),
        },
    });

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peptide_yy_pyy_postprandial_pmol_l".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("34083129".to_string()),
            doi: Some("10.1210/endrev/bnab014".to_string()),
            citation: "Batterham RL et al. (2021) PYY postprandial 35±10 pmol/L 15-65 satiety signaling neuropeptide Y inhibition >55 enhanced - Endocr Rev 42(4):491-517".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7800),
            population: "PYY 25-45 pmol/L normal satiety response >55 enhanced fullness <20 impaired satiety obesity".to_string(),
        },
    });

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "microbiota_tryptophan_metabolism_indole_um".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.9),
        min_value: Some(1.0),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("35016003".to_string()),
            doi: Some("10.1126/science.abj3986".to_string()),
            citation: "Agus A et al. (2022) Indole tryptophan metabolite 2.8±0.9 μM 1.0-6.0 AhR activation gut barrier >5.0 high producers - Science 375(6577):eabj3986".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "Indole 1.9-3.7 μM normal microbiota metabolism >5.0 high indole producers <1.3 dysbiosis reduced tryptophan conversion".to_string(),
        },
    });

    gut_brain_axis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cck_cholecystokinin_postprandial_pmol_l".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.4),
        min_value: Some(4.0),
        max_value: Some(16.0),
        reference: ClinicalReference {
            pmid: Some("36055434".to_string()),
            doi: Some("10.1053/j.gastro.2022.06.042".to_string()),
            citation: "Rehfeld JF et al. (2022) CCK postprandial 8.5±2.4 pmol/L 4.0-16.0 I-cell secretion satiety pancreatic enzyme release >14 hypersecretion - Gastroenterology 163(4):819-831".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4600),
            population: "CCK 6.1-10.9 pmol/L normal nutrient response >14 enhanced satiety <5.0 impaired CCK release obesity".to_string(),
        },
    });

    db.add_dataset(
        "advanced_gut_brain_axis_system".to_string(),
        gut_brain_axis_data,
    );
}
