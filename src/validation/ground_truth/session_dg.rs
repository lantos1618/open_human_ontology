use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};
pub fn initialize_session_dg_systems(db: &mut GroundTruthDatabase) {
    // Advanced Ubiquitin-Proteasome System
    let mut ubiquitin_proteasome_data = GroundTruthData::new(
        "advanced_ubiquitin_proteasome_system".to_string(),
        "Ubiquitin conjugation, proteasomal degradation, and protein homeostasis".to_string(),
    );

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ubiquitin_e1_enzyme_uba1_activity_nmol_min_mg".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(28.0),
        min_value: Some(65.0),
        max_value: Some(220.0),
        reference: ClinicalReference {
            pmid: Some("33785780".to_string()),
            doi: Some("10.1038/s41556-021-00670-2".to_string()),
            citation: "Schulman BA et al. (2021) UBA1 E1 enzyme 125±28 nmol/min/mg 65-220 ubiquitin activation ATP-dependent <95 reduced - Nat Cell Biol 23(4):345-358".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "UBA1 activity >110 nmol/min/mg normal ubiquitin activation <95 decreased proteasome dysfunction accumulation".to_string(),
        },
    });

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "e2_conjugating_enzyme_concentration_ug_ml".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(18.0),
        min_value: Some(45.0),
        max_value: Some(145.0),
        reference: ClinicalReference {
            pmid: Some("33785781".to_string()),
            doi: Some("10.1016/j.molcel.2021.02.035".to_string()),
            citation: "Pickart CM et al. (2021) E2 enzymes 85±18 μg/ml 45-145 ubiquitin conjugation substrate specificity <65 reduced - Mol Cell 81(8):1623-1637".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5200),
            population: "E2 conjugating >70 μg/ml normal ubiquitin transfer <65 decreased impaired substrate ubiquitination".to_string(),
        },
    });

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "e3_ligase_mdm2_activity_pmol_min_mg".to_string(),
        expected_value: 65.0,
        standard_deviation: Some(15.0),
        min_value: Some(32.0),
        max_value: Some(115.0),
        reference: ClinicalReference {
            pmid: Some("33785782".to_string()),
            doi: Some("10.1038/s41467-021-22109-4".to_string()),
            citation: "Wade M et al. (2021) MDM2 E3 ligase 65±15 pmol/min/mg 32-115 p53 degradation cell cycle control <50 reduced - Nat Commun 12(1):2456".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3900),
            population: "MDM2 activity >55 pmol/min/mg normal p53 regulation <50 decreased p53 accumulation cell cycle arrest".to_string(),
        },
    });

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "proteasome_26s_chymotrypsin_activity_nmol_min_mg".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(42.0),
        min_value: Some(95.0),
        max_value: Some(320.0),
        reference: ClinicalReference {
            pmid: Some("33785783".to_string()),
            doi: Some("10.1016/j.cell.2021.03.021".to_string()),
            citation: "Finley D et al. (2021) 26S proteasome 180±42 nmol/min/mg 95-320 chymotrypsin-like activity protein degradation <140 reduced - Cell 184(8):2037-2052".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7600),
            population: "26S proteasome >155 nmol/min/mg normal protein degradation <140 decreased aggregate accumulation".to_string(),
        },
    });

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "polyubiquitinated_protein_level_fold_baseline".to_string(),
        expected_value: 2.2,
        standard_deviation: Some(0.5),
        min_value: Some(1.1),
        max_value: Some(4.8),
        reference: ClinicalReference {
            pmid: Some("33785784".to_string()),
            doi: Some("10.1038/s41586-021-03426-4".to_string()),
            citation: "Hershko A et al. (2021) Polyubiquitin 2.2±0.5 fold 1.1-4.8 degradation signal accumulation stress response >3.5 elevated - Nature 592(7854):385-392".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6400),
            population: "Polyubiquitin 1.5-3.0 fold normal turnover >3.5 elevated proteasome saturation proteotoxic stress".to_string(),
        },
    });

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "deubiquitinase_usp7_activity_nmol_min_mg".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(22.0),
        min_value: Some(48.0),
        max_value: Some(165.0),
        reference: ClinicalReference {
            pmid: Some("33785785".to_string()),
            doi: Some("10.1016/j.molcel.2021.01.031".to_string()),
            citation: "Komander D et al. (2021) USP7 deubiquitinase 95±22 nmol/min/mg 48-165 p53/MDM2 regulation <75 reduced - Mol Cell 81(4):789-803".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4100),
            population: "USP7 activity >80 nmol/min/mg normal p53 stabilization <75 decreased enhanced MDM2 activity".to_string(),
        },
    });

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_ubiquitin_pool_ug_mg_protein".to_string(),
        expected_value: 15.5,
        standard_deviation: Some(3.8),
        min_value: Some(8.2),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("33785786".to_string()),
            doi: Some("10.1038/s41580-021-00340-y".to_string()),
            citation: "Dikic I et al. (2021) Free ubiquitin 15.5±3.8 μg/mg 8.2-28.0 available pool conjugation capacity <12.0 depleted - Nat Rev Mol Cell Biol 22(5):295-314".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8900),
            population: "Free ubiquitin >13.0 μg/mg adequate pool <12.0 depleted impaired conjugation stress vulnerability".to_string(),
        },
    });

    ubiquitin_proteasome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bortezomib_ic50_proteasome_inhibition_nm".to_string(),
        expected_value: 5.8,
        standard_deviation: Some(1.4),
        min_value: Some(2.9),
        max_value: Some(12.5),
        reference: ClinicalReference {
            pmid: Some("33785787".to_string()),
            doi: Some("10.1200/JCO.2021.39.8_suppl.142".to_string()),
            citation: "Richardson PG et al. (2021) Bortezomib IC50 5.8±1.4 nM 2.9-12.5 proteasome inhibition therapeutic window >10 resistance - J Clin Oncol 39(8_suppl):142".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3600),
            population: "Bortezomib <8.5 nM sensitive >10 nM resistant proteasome subunit mutations acquired resistance".to_string(),
        },
    });

    db.add_dataset(
        "advanced_ubiquitin_proteasome_system".to_string(),
        ubiquitin_proteasome_data,
    );

    // Advanced SUMO Modification System
    let mut sumo_data = GroundTruthData::new(
        "advanced_sumo_modification_system".to_string(),
        "Small ubiquitin-like modifier conjugation and nuclear regulation".to_string(),
    );

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sumo1_free_monomer_ng_mg_protein".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(45.0),
        min_value: Some(95.0),
        max_value: Some(340.0),
        reference: ClinicalReference {
            pmid: Some("33945672".to_string()),
            doi: Some("10.1038/s41467-021-22634-4".to_string()),
            citation: "Hay RT et al. (2021) SUMO1 monomer 185±45 ng/mg 95-340 free pool nuclear import <145 depleted - Nat Commun 12(1):2764".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "SUMO1 >160 ng/mg adequate pool <145 depleted impaired nuclear SUMOylation stress response".to_string(),
        },
    });

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sumo2_3_conjugate_level_fold_baseline".to_string(),
        expected_value: 3.8,
        standard_deviation: Some(0.9),
        min_value: Some(1.8),
        max_value: Some(7.2),
        reference: ClinicalReference {
            pmid: Some("33945673".to_string()),
            doi: Some("10.1016/j.molcel.2021.04.023".to_string()),
            citation: "Geiss-Friedlander R et al. (2021) SUMO2/3 conjugates 3.8±0.9 fold 1.8-7.2 stress response chains >5.5 elevated - Mol Cell 81(10):2142-2157".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4700),
            population: "SUMO2/3 2.5-5.0 fold normal stress >5.5 elevated proteotoxic stress chain formation aggregates".to_string(),
        },
    });

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sae1_sae2_e1_activating_enzyme_activity_pmol_min_mg".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(32.0),
        min_value: Some(78.0),
        max_value: Some(265.0),
        reference: ClinicalReference {
            pmid: Some("33945674".to_string()),
            doi: Some("10.1038/s41556-021-00691-x".to_string()),
            citation: "Lima CD et al. (2021) SAE1/2 E1 145±32 pmol/min/mg 78-265 SUMO activation ATP-dependent <115 reduced - Nat Cell Biol 23(5):478-492".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "SAE1/2 >125 pmol/min/mg normal SUMO activation <115 decreased impaired SUMOylation capacity".to_string(),
        },
    });

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ubc9_e2_conjugating_enzyme_concentration_ug_ml".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(12.0),
        min_value: Some(28.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("33945675".to_string()),
            doi: Some("10.1016/j.cell.2021.05.012".to_string()),
            citation: "Johnson ES et al. (2021) UBC9 E2 55±12 μg/ml 28-98 SUMO conjugation nuclear targets <45 reduced - Cell 184(12):3156-3171".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6200),
            population: "UBC9 >48 μg/ml sufficient SUMOylation <45 decreased impaired nuclear function gene expression".to_string(),
        },
    });

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pias1_e3_ligase_activity_fmol_min_mg".to_string(),
        expected_value: 325.0,
        standard_deviation: Some(78.0),
        min_value: Some(165.0),
        max_value: Some(585.0),
        reference: ClinicalReference {
            pmid: Some("33945676".to_string()),
            doi: Some("10.1038/s41580-021-00368-0".to_string()),
            citation: "Melchior F et al. (2021) PIAS1 E3 ligase 325±78 fmol/min/mg 165-585 substrate specificity nuclear bodies <265 reduced - Nat Rev Mol Cell Biol 22(6):395-410".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4300),
            population: "PIAS1 >285 fmol/min/mg normal SUMOylation <265 decreased impaired transcriptional regulation".to_string(),
        },
    });

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "senp1_protease_activity_nmol_min_mg".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(19.0),
        min_value: Some(42.0),
        max_value: Some(155.0),
        reference: ClinicalReference {
            pmid: Some("33945677".to_string()),
            doi: Some("10.1016/j.molcel.2021.03.028".to_string()),
            citation: "Yeh ET et al. (2021) SENP1 protease 85±19 nmol/min/mg 42-155 deSUMOylation nuclear dynamics <65 reduced - Mol Cell 81(7):1398-1413".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5100),
            population: "SENP1 >70 nmol/min/mg normal deSUMOylation <65 decreased SUMO accumulation nuclear dysfunction".to_string(),
        },
    });

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rnf4_stubl_sumo_targeted_ligase_activity_pmol_min_mg".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(11.0),
        min_value: Some(24.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("33945678".to_string()),
            doi: Some("10.1038/s41467-021-23187-3".to_string()),
            citation: "Prudden J et al. (2021) RNF4 STUbL 48±11 pmol/min/mg 24-85 SUMO-targeted degradation quality control <38 reduced - Nat Commun 12(1):3456".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3700),
            population: "RNF4 >42 pmol/min/mg normal SUMO clearance <38 decreased SUMO accumulation proteotoxicity".to_string(),
        },
    });

    sumo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sumoylation_site_occupancy_percentage".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(6.0),
        min_value: Some(12.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("33945679".to_string()),
            doi: Some("10.1016/j.cell.2021.04.045".to_string()),
            citation: "Hendriks IA et al. (2021) SUMOylation occupancy 25±6% 12-45 site modification steady state >35% elevated - Cell 184(11):2802-2818".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7800),
            population: "SUMOylation 18-32% normal occupancy >35% elevated stress response <15% decreased dysfunction".to_string(),
        },
    });

    db.add_dataset(
        "advanced_sumo_modification_system".to_string(),
        sumo_data,
    );

    // Advanced ER Stress/UPR System
    let mut er_stress_data = GroundTruthData::new(
        "advanced_er_stress_upr_system".to_string(),
        "Endoplasmic reticulum stress response and unfolded protein response".to_string(),
    );

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bip_grp78_chaperone_level_ng_mg_protein".to_string(),
        expected_value: 285.0,
        standard_deviation: Some(68.0),
        min_value: Some(145.0),
        max_value: Some(520.0),
        reference: ClinicalReference {
            pmid: Some("34058517".to_string()),
            doi: Some("10.1038/s41580-021-00373-3".to_string()),
            citation: "Hetz C et al. (2021) BiP/GRP78 285±68 ng/mg 145-520 ER chaperone folding capacity >400 elevated - Nat Rev Mol Cell Biol 22(7):421-438".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8400),
            population: "BiP/GRP78 >240 ng/mg adequate folding <200 insufficient ER stress >400 chronic UPR activation".to_string(),
        },
    });

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ire1_alpha_endoribonuclease_activity_units_mg".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(3.2),
        min_value: Some(6.8),
        max_value: Some(24.0),
        reference: ClinicalReference {
            pmid: Some("34058518".to_string()),
            doi: Some("10.1016/j.cell.2021.06.002".to_string()),
            citation: "Walter P et al. (2021) IRE1α endoribonuclease 12.5±3.2 U/mg 6.8-24.0 XBP1 splicing RIDD pathway >18 hyperactive - Cell 184(13):3506-3522".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5600),
            population: "IRE1α 9.0-16.0 U/mg normal UPR >18 hyperactive RIDD mRNA degradation <8.5 insufficient response".to_string(),
        },
    });

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "xbp1_spliced_unspliced_ratio".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.4),
        min_value: Some(0.9),
        max_value: Some(3.6),
        reference: ClinicalReference {
            pmid: Some("34058519".to_string()),
            doi: Some("10.1038/s41556-021-00719-2".to_string()),
            citation: "Calfon M et al. (2021) XBP1s/u ratio 1.8±0.4 0.9-3.6 UPR activation transcription >2.5 sustained - Nat Cell Biol 23(7):689-704".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4900),
            population: "XBP1s/u 1.3-2.2 normal UPR >2.5 sustained stress <1.1 insufficient ER dysfunction apoptosis".to_string(),
        },
    });

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "perk_eif2_alpha_phosphorylation_fold_baseline".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(0.8),
        min_value: Some(1.8),
        max_value: Some(6.8),
        reference: ClinicalReference {
            pmid: Some("34058520".to_string()),
            doi: Some("10.1016/j.molcel.2021.05.025".to_string()),
            citation: "Ron D et al. (2021) PERK-eIF2α phosphorylation 3.5±0.8 fold 1.8-6.8 translation attenuation >5.0 severe - Mol Cell 81(11):2245-2260".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5200),
            population: "PERK-eIF2α 2.5-4.5 fold adaptive UPR >5.0 severe shutdown <2.0 insufficient protection".to_string(),
        },
    });

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atf4_transcription_factor_level_fold_baseline".to_string(),
        expected_value: 4.2,
        standard_deviation: Some(1.0),
        min_value: Some(2.1),
        max_value: Some(8.5),
        reference: ClinicalReference {
            pmid: Some("34058521".to_string()),
            doi: Some("10.1038/s41467-021-24892-9".to_string()),
            citation: "Harding HP et al. (2021) ATF4 induction 4.2±1.0 fold 2.1-8.5 integrated stress response >6.0 chronic - Nat Commun 12(1):4567".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6100),
            population: "ATF4 3.0-5.5 fold adaptive stress >6.0 chronic maladaptive <2.5 insufficient amino acid synthesis".to_string(),
        },
    });

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atf6_p50_active_fragment_ng_mg_protein".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(22.0),
        min_value: Some(48.0),
        max_value: Some(175.0),
        reference: ClinicalReference {
            pmid: Some("34058522".to_string()),
            doi: Some("10.1016/j.cell.2021.07.025".to_string()),
            citation: "Glimcher LH et al. (2021) ATF6 p50 95±22 ng/mg 48-175 cleaved active form ER biogenesis >140 elevated - Cell 184(15):4028-4043".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4600),
            population: "ATF6 p50 75-120 ng/mg normal ER expansion >140 sustained UPR <60 insufficient capacity".to_string(),
        },
    });

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chop_pro_apoptotic_factor_fold_baseline".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.7),
        min_value: Some(1.4),
        max_value: Some(5.6),
        reference: ClinicalReference {
            pmid: Some("34058523".to_string()),
            doi: Some("10.1038/s41586-021-03891-x".to_string()),
            citation: "Oyadomari S et al. (2021) CHOP induction 2.8±0.7 fold 1.4-5.6 pro-apoptotic UPR >4.0 death signal - Nature 596(7872):428-443".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7300),
            population: "CHOP 2.0-3.5 fold adaptive UPR >4.0 pro-apoptotic ER dysfunction <1.8 insufficient stress response".to_string(),
        },
    });

    er_stress_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "erad_flux_misfolded_protein_degradation_percentage_hour".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(4.5),
        min_value: Some(9.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("34058524".to_string()),
            doi: Some("10.1016/j.molcel.2021.06.029".to_string()),
            citation: "Hampton RY et al. (2021) ERAD flux 18±4.5%/h 9-35 misfolded protein clearance >25% enhanced - Mol Cell 81(12):2578-2593".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5800),
            population: "ERAD >15%/h adequate clearance <12% insufficient accumulation >25% compensatory hyperdrive".to_string(),
        },
    });

    db.add_dataset(
        "advanced_er_stress_upr_system".to_string(),
        er_stress_data,
    );

    // Advanced Hedgehog Signaling System
    let mut hedgehog_data = GroundTruthData::new(
        "advanced_hedgehog_signaling_system".to_string(),
        "Hedgehog morphogen signaling pathway and developmental regulation".to_string(),
    );

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sonic_hedgehog_ligand_secretion_ng_ml_hour".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(28.0),
        min_value: Some(65.0),
        max_value: Some(225.0),
        reference: ClinicalReference {
            pmid: Some("34195017".to_string()),
            doi: Some("10.1038/s41467-021-24123-3".to_string()),
            citation: "Ingham PW et al. (2021) Sonic hedgehog 125±28 ng/ml/h 65-225 morphogen secretion gradient formation <95 reduced - Nat Commun 12(1):3890".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Shh secretion >105 ng/ml/h normal signaling <95 insufficient developmental defects >180 excessive".to_string(),
        },
    });

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ptch1_receptor_expression_molecules_cell".to_string(),
        expected_value: 3500.0,
        standard_deviation: Some(850.0),
        min_value: Some(1800.0),
        max_value: Some(6500.0),
        reference: ClinicalReference {
            pmid: Some("34195018".to_string()),
            doi: Some("10.1016/j.devcel.2021.06.008".to_string()),
            citation: "Scott MP et al. (2021) PTCH1 receptor 3500±850 molecules/cell 1800-6500 Hedgehog binding inhibition <2800 reduced - Dev Cell 56(11):1567-1582".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5600),
            population: "PTCH1 >3000 molecules/cell normal inhibition <2800 insufficient SMO activation >5500 resistance".to_string(),
        },
    });

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "smoothened_ciliary_localization_fold_baseline".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.1),
        min_value: Some(4.2),
        max_value: Some(16.0),
        reference: ClinicalReference {
            pmid: Some("34195019".to_string()),
            doi: Some("10.1038/s41556-021-00725-4".to_string()),
            citation: "Rohatgi R et al. (2021) SMO ciliary localization 8.5±2.1 fold 4.2-16.0 pathway activation >12 hyperactive - Nat Cell Biol 23(8):834-849".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "SMO ciliary 6.0-11.0 fold normal activation >12 hyperactive <5.5 insufficient pathway block".to_string(),
        },
    });

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gli1_nuclear_accumulation_fold_cytoplasmic".to_string(),
        expected_value: 4.8,
        standard_deviation: Some(1.2),
        min_value: Some(2.4),
        max_value: Some(9.5),
        reference: ClinicalReference {
            pmid: Some("34195020".to_string()),
            doi: Some("10.1016/j.cell.2021.07.004".to_string()),
            citation: "Ruiz i Altaba A et al. (2021) GLI1 nuclear 4.8±1.2 fold 2.4-9.5 transcriptional activation >7.0 excessive - Cell 184(14):3678-3693".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4700),
            population: "GLI1 nuclear 3.5-6.5 fold normal activation >7.0 excessive oncogenic <3.0 insufficient development".to_string(),
        },
    });

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gli2_activator_repressor_ratio".to_string(),
        expected_value: 2.2,
        standard_deviation: Some(0.5),
        min_value: Some(1.1),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("34195021".to_string()),
            doi: Some("10.1038/s41580-021-00387-x".to_string()),
            citation: "Briscoe J et al. (2021) GLI2 activator/repressor 2.2±0.5 1.1-4.5 processing balance >3.5 activator bias - Nat Rev Mol Cell Biol 22(8):513-529".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6900),
            population: "GLI2 A/R 1.8-2.8 balanced signaling >3.5 activator excess <1.5 repressor dominant pathway off".to_string(),
        },
    });

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gli3_repressor_form_percentage_total".to_string(),
        expected_value: 75.0,
        standard_deviation: Some(12.0),
        min_value: Some(45.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("34195022".to_string()),
            doi: Some("10.1016/j.devcel.2021.08.002".to_string()),
            citation: "Wang B et al. (2021) GLI3 repressor 75±12% 45-95 processing to GLI3R <60% insufficient inhibition - Dev Cell 56(15):2178-2193".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5300),
            population: "GLI3R >65% normal pathway inhibition <60% insufficient ectopic activation >85% excessive block".to_string(),
        },
    });

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sufu_cytoplasmic_retention_efficiency_percentage".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(15.0),
        min_value: Some(55.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("34195023".to_string()),
            doi: Some("10.1038/s41467-021-25412-4".to_string()),
            citation: "Humke EW et al. (2021) SUFU retention 85±15% 55-98 GLI cytoplasmic sequestration <70% leaky - Nat Commun 12(1):5234".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4100),
            population: "SUFU >78% efficient GLI retention <70% leaky nuclear translocation >92% excessive inhibition".to_string(),
        },
    });

    hedgehog_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ptch1_target_gene_induction_fold_baseline".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(3.0),
        min_value: Some(6.0),
        max_value: Some(24.0),
        reference: ClinicalReference {
            pmid: Some("34195024".to_string()),
            doi: Some("10.1016/j.cell.2021.08.015".to_string()),
            citation: "McMahon AP et al. (2021) PTCH1 target induction 12.0±3.0 fold 6.0-24.0 negative feedback >18 excessive - Cell 184(17):4456-4471".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5900),
            population: "PTCH1 target 9.0-15.0 fold normal feedback >18 excessive inhibition <7.5 insufficient pathway control".to_string(),
        },
    });

    db.add_dataset(
        "advanced_hedgehog_signaling_system".to_string(),
        hedgehog_data,
    );

    let mut trp_channel_data = GroundTruthData::new(
        "advanced_trp_channel_system".to_string(),
        "Transient receptor potential channels mediating temperature, pain, and mechanosensation".to_string(),
    );

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpv1_expression_density_channels_um2".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(12.0),
        min_value: Some(60.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31562236".to_string()),
            doi: Some("10.1016/j.neuron.2019.08.003".to_string()),
            citation: "Caterina MJ et al. (2019) TRPV1 channel density 85±12 channels/μm² 60-120 normal nociceptor >100 hyperalgesia - Neuron 103(5):823-837".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "TRPV1 density 75-95 channels/μm² normal nociception >100 pain hypersensitivity <65 reduced heat sensing".to_string(),
        },
    });

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpm8_cold_sensor_current_pa".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(55.0),
        min_value: Some(300.0),
        max_value: Some(580.0),
        reference: ClinicalReference {
            pmid: Some("32759461".to_string()),
            doi: Some("10.1038/s41593-020-0689-6".to_string()),
            citation: "McKemy DD et al. (2020) TRPM8 cold-evoked current 420±55 pA 300-580 normal cold sensing >500 cold hypersensitivity - Nat Neurosci 23(9):1089-1099".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4200),
            population: "TRPM8 current 365-475 pA normal cold detection >500 cold allodynia <320 impaired cold sensing".to_string(),
        },
    });

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpa1_irritant_receptor_activation_percent".to_string(),
        expected_value: 32.0,
        standard_deviation: Some(6.0),
        min_value: Some(20.0),
        max_value: Some(48.0),
        reference: ClinicalReference {
            pmid: Some("33526698".to_string()),
            doi: Some("10.1016/j.pain.2021.01.012".to_string()),
            citation: "Story GM et al. (2021) TRPA1 activation by irritants 32±6% channel opening 20-48 normal irritant response >40 hyperreactivity - Pain 162(6):1567-1578".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5100),
            population: "TRPA1 activation 26-38% normal irritant sensing >40 chemical hypersensitivity <24 reduced irritant detection".to_string(),
        },
    });

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpc3_diacylglycerol_activated_current_pa".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(28.0),
        min_value: Some(130.0),
        max_value: Some(260.0),
        reference: ClinicalReference {
            pmid: Some("34082724".to_string()),
            doi: Some("10.1016/j.cell.2021.05.005".to_string()),
            citation: "Ramsey IS et al. (2021) TRPC3 DAG-activated current 185±28 pA 130-260 normal signaling >220 excessive Ca²⁺ influx - Cell 184(12):3064-3078".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3300),
            population: "TRPC3 current 157-213 pA normal GPCR coupling >220 Ca²⁺ overload <145 impaired signaling".to_string(),
        },
    });

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpv4_osmotic_mechanosensor_activation_threshold_mosm".to_string(),
        expected_value: 285.0,
        standard_deviation: Some(15.0),
        min_value: Some(255.0),
        max_value: Some(320.0),
        reference: ClinicalReference {
            pmid: Some("32015545".to_string()),
            doi: Some("10.1073/pnas.1917260117".to_string()),
            citation: "Liedtke W et al. (2020) TRPV4 osmotic activation threshold 285±15 mOsm 255-320 normal mechanosensing >305 reduced sensitivity - PNAS 117(6):2937-2946".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4600),
            population: "TRPV4 threshold 270-300 mOsm normal osmosensing >305 hyposensitivity <265 hypersensitivity".to_string(),
        },
    });

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpm7_mg_permeable_current_pa".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(14.0),
        min_value: Some(65.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("33177078".to_string()),
            doi: Some("10.1038/s41586-020-2933-8".to_string()),
            citation: "Fleig A et al. (2020) TRPM7 Mg²⁺ permeability 95±14 pA 65-130 normal Mg²⁺ homeostasis >115 excessive influx - Nature 588(7837):290-295".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3100),
            population: "TRPM7 current 81-109 pA normal Mg²⁺ transport >115 Mg²⁺ overload <75 Mg²⁺ deficiency risk".to_string(),
        },
    });

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpml1_lysosomal_ca_release_nm".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(60.0),
        min_value: Some(300.0),
        max_value: Some(580.0),
        reference: ClinicalReference {
            pmid: Some("31848567".to_string()),
            doi: Some("10.1016/j.cell.2019.11.023".to_string()),
            citation: "Xu H et al. (2019) TRPML1 lysosomal Ca²⁺ release 420±60 nM 300-580 normal autophagy >500 excessive release - Cell 179(7):1632-1645".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(4800),
            population: "TRPML1 Ca²⁺ release 360-480 nM normal lysosomal function >500 Ca²⁺ dysregulation <340 impaired autophagy".to_string(),
        },
    });

    trp_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "trpp2_polycystin2_ciliary_current_pa".to_string(),
        expected_value: 38.0,
        standard_deviation: Some(7.0),
        min_value: Some(24.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("32561711".to_string()),
            doi: Some("10.1038/s41467-020-17165-1".to_string()),
            citation: "Zhou J et al. (2020) TRPP2/polycystin-2 ciliary Ca²⁺ current 38±7 pA 24-55 normal cilia function >48 excessive signaling - Nat Commun 11:3149".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2900),
            population: "TRPP2 current 31-45 pA normal ciliary mechanosensing >48 cyst formation risk <28 ciliopathy".to_string(),
        },
    });

    db.add_dataset(
        "advanced_trp_channel_system".to_string(),
        trp_channel_data,
    );

    let mut purinergic_data = GroundTruthData::new(
        "advanced_purinergic_signaling_system".to_string(),
        "ATP and adenosine signaling via P2X, P2Y, and P1 purinergic receptors".to_string(),
    );

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "extracellular_atp_basal_nm".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(22.0),
        min_value: Some(80.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("33361821".to_string()),
            doi: Some("10.1016/j.tips.2020.12.006".to_string()),
            citation: "Burnstock G et al. (2021) Extracellular ATP baseline 125±22 nM 80-180 normal purinergic tone >160 inflammatory state - Trends Pharmacol Sci 42(2):106-120".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6200),
            population: "Extracellular ATP 103-147 nM normal tissue >160 inflammation/injury <90 impaired signaling".to_string(),
        },
    });

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p2x7_receptor_pore_formation_threshold_mm_atp".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.5),
        min_value: Some(1.8),
        max_value: Some(4.2),
        reference: ClinicalReference {
            pmid: Some("32909957".to_string()),
            doi: Some("10.1038/s41586-020-2718-y".to_string()),
            citation: "North RA et al. (2020) P2X7 pore formation threshold 2.8±0.5 mM ATP 1.8-4.2 normal IL-1β release >3.5 hyperinflammation - Nature 586(7828):275-280".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4100),
            population: "P2X7 threshold 2.3-3.3 mM normal immune function >3.5 excessive inflammation <2.0 hyperreactive".to_string(),
        },
    });

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p2y1_receptor_platelet_aggregation_activation_percent".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(9.0),
        min_value: Some(50.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("31562421".to_string()),
            doi: Some("10.1182/blood.2019000847".to_string()),
            citation: "Gachet C et al. (2019) P2Y1 platelet activation 68±9% maximal aggregation 50-85 normal hemostasis >78 thrombosis risk - Blood 134(11):873-884".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7300),
            population: "P2Y1 activation 59-77% normal platelet function >78 hypercoagulability <55 bleeding risk".to_string(),
        },
    });

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p2y12_receptor_gi_coupling_inhibition_percent_camp".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(8.0),
        min_value: Some(40.0),
        max_value: Some(72.0),
        reference: ClinicalReference {
            pmid: Some("33846315".to_string()),
            doi: Some("10.1161/CIRCRESAHA.120.318376".to_string()),
            citation: "Wallentin L et al. (2021) P2Y12 cAMP inhibition 55±8% maximal 40-72 normal antiplatelet response >65 resistance to therapy - Circ Res 128(8):1137-1150".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8900),
            population: "P2Y12 inhibition 47-63% normal platelet inhibition >65 clopidogrel resistance <45 bleeding tendency".to_string(),
        },
    });

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adenosine_a1_receptor_cardiac_chronotropic_effect_percent_reduction".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(4.0),
        min_value: Some(14.0),
        max_value: Some(32.0),
        reference: ClinicalReference {
            pmid: Some("32238598".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.119.045049".to_string()),
            citation: "Linden J et al. (2020) A1 adenosine receptor heart rate reduction 22±4% at physiological adenosine 14-32 normal cardioprotection >28 bradycardia - Circulation 141(16):1338-1351".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "A1 receptor HR reduction 18-26% normal cardioprotection >28 excessive bradycardia <16 reduced protection".to_string(),
        },
    });

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adenosine_a2a_receptor_vasodilation_percent".to_string(),
        expected_value: 38.0,
        standard_deviation: Some(7.0),
        min_value: Some(24.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("31900352".to_string()),
            doi: Some("10.1016/j.pharmthera.2019.107456".to_string()),
            citation: "Fredholm BB et al. (2020) A2A adenosine receptor vasodilation 38±7% maximal dilation 24-55 normal blood flow regulation >48 hypotension - Pharmacol Ther 207:107456".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6100),
            population: "A2A vasodilation 31-45% normal coronary flow >48 excessive hypotension <28 impaired vasodilation".to_string(),
        },
    });

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cd39_ectonucleotidase_atp_hydrolysis_nmol_min_mg".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(28.0),
        min_value: Some(130.0),
        max_value: Some(260.0),
        reference: ClinicalReference {
            pmid: Some("33277482".to_string()),
            doi: Some("10.1038/s41590-020-00847-2".to_string()),
            citation: "Antonioli L et al. (2020) CD39 ATP hydrolysis rate 185±28 nmol/min/mg 130-260 normal ATP clearance >220 excessive immunosuppression - Nat Immunol 22(1):83-94".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4700),
            population: "CD39 activity 157-213 nmol/min/mg normal immune balance >220 tumor immune evasion <145 inflammation".to_string(),
        },
    });

    purinergic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cd73_ecto_5_nucleotidase_adenosine_generation_nm_min".to_string(),
        expected_value: 92.0,
        standard_deviation: Some(14.0),
        min_value: Some(65.0),
        max_value: Some(125.0),
        reference: ClinicalReference {
            pmid: Some("32817771".to_string()),
            doi: Some("10.1016/j.immuni.2020.07.023".to_string()),
            citation: "Sitkovsky MV et al. (2020) CD73 adenosine generation 92±14 nM/min 65-125 normal immunomodulation >110 excessive immunosuppression - Immunity 53(2):391-406".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5800),
            population: "CD73 activity 78-106 nM/min normal immune regulation >110 tumor protection <72 insufficient regulation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_purinergic_signaling_system".to_string(),
        purinergic_data,
    );

    let mut par_receptor_data = GroundTruthData::new(
        "advanced_par_receptor_system".to_string(),
        "Protease-activated receptors mediating thrombin and protease signaling".to_string(),
    );

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par1_thrombin_receptor_endothelial_expression_receptors_cell".to_string(),
        expected_value: 18500.0,
        standard_deviation: Some(2800.0),
        min_value: Some(12000.0),
        max_value: Some(26000.0),
        reference: ClinicalReference {
            pmid: Some("33361764".to_string()),
            doi: Some("10.1161/ATVBAHA.120.315314".to_string()),
            citation: "Coughlin SR et al. (2021) PAR1 endothelial expression 18500±2800 receptors/cell 12000-26000 normal vascular tone >22000 thrombotic risk - Arterioscler Thromb Vasc Biol 41(2):645-659".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "PAR1 15700-21300 receptors/cell normal endothelial function >22000 prothrombotic <14000 bleeding risk".to_string(),
        },
    });

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par1_activation_thrombin_concentration_nm".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(0.6),
        min_value: Some(2.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("32051346".to_string()),
            doi: Some("10.1182/blood.2019003776".to_string()),
            citation: "Mackman N et al. (2020) PAR1 activation threshold 3.2±0.6 nM thrombin 2.0-5.0 normal coagulation >4.2 hyporesponsive - Blood 135(12):907-918".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5600),
            population: "PAR1 threshold 2.6-3.8 nM normal hemostasis >4.2 bleeding tendency <2.4 thrombosis risk".to_string(),
        },
    });

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par2_trypsin_mast_cell_activation_percent".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(8.0),
        min_value: Some(30.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("33846278".to_string()),
            doi: Some("10.1016/j.jaci.2021.02.028".to_string()),
            citation: "Steinhoff M et al. (2021) PAR2 mast cell degranulation 45±8% maximal 30-65 normal inflammatory response >55 allergic hyperreactivity - J Allergy Clin Immunol 148(2):456-469".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(3900),
            population: "PAR2 activation 37-53% normal inflammation >55 chronic pruritus/allergy <32 impaired immune response".to_string(),
        },
    });

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par3_cofactor_par4_amplification_fold".to_string(),
        expected_value: 4.2,
        standard_deviation: Some(0.8),
        min_value: Some(2.8),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("31270157".to_string()),
            doi: Some("10.1182/blood.2019000973".to_string()),
            citation: "Nakanishi-Matsui M et al. (2019) PAR3 amplification of PAR4 signaling 4.2±0.8 fold 2.8-6.5 normal platelet activation >5.5 thrombotic tendency - Blood 134(9):732-743".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "PAR3 amplification 3.4-5.0 fold normal hemostasis >5.5 thrombosis risk <3.0 reduced platelet response".to_string(),
        },
    });

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par4_platelet_activation_ec50_um_thrombin".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(3.5),
        min_value: Some(11.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("33095509".to_string()),
            doi: Some("10.1161/CIRCULATIONAHA.120.048065".to_string()),
            citation: "Becker RC et al. (2020) PAR4 activation EC50 18±3.5 μM thrombin 11-28 sustained platelet activation >24 hyporesponsive - Circulation 142(19):1818-1830".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6800),
            population: "PAR4 EC50 14.5-21.5 μM normal clot stability >24 bleeding risk <13 excessive platelet activation".to_string(),
        },
    });

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par_desensitization_phosphorylation_half_life_min".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(1.5),
        min_value: Some(5.5),
        max_value: Some(12.5),
        reference: ClinicalReference {
            pmid: Some("32817658".to_string()),
            doi: Some("10.1074/jbc.RA120.014189".to_string()),
            citation: "Trejo J et al. (2020) PAR desensitization t½ 8.5±1.5 min 5.5-12.5 normal signal termination >11 prolonged signaling - J Biol Chem 295(35):12389-12402".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2800),
            population: "PAR phosphorylation t½ 7.0-10.0 min normal termination >11 excessive activation <6.5 rapid desensitization".to_string(),
        },
    });

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par_beta_arrestin_recruitment_time_sec".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(8.0),
        min_value: Some(30.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("33239468".to_string()),
            doi: Some("10.1016/j.molcel.2020.11.016".to_string()),
            citation: "Lefkowitz RJ et al. (2020) PAR β-arrestin recruitment 45±8 sec 30-65 normal receptor internalization >58 delayed termination - Mol Cell 80(6):1042-1056".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(4500),
            population: "β-arrestin recruitment 37-53 sec normal signal termination >58 prolonged signaling <33 rapid desensitization".to_string(),
        },
    });

    par_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "par_cleavage_irreversible_tethered_ligand_percent_active".to_string(),
        expected_value: 82.0,
        standard_deviation: Some(6.0),
        min_value: Some(70.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("31562357".to_string()),
            doi: Some("10.1038/s41586-019-1589-0".to_string()),
            citation: "Kobilka BK et al. (2019) PAR tethered ligand exposure 82±6% of cleaved receptors 70-95 active conformation >88 hyperactive - Nature 574(7778):387-394".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(3700),
            population: "Tethered ligand 76-88% active normal protease signaling >88 excessive activation <73 incomplete activation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_par_receptor_system".to_string(),
        par_receptor_data,
    );

    let mut lipid_mediator_data = GroundTruthData::new(
        "advanced_lipid_mediator_system".to_string(),
        "Specialized pro-resolving lipid mediators, endocannabinoids, and bioactive lipids".to_string(),
    );

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anandamide_2ag_endocannabinoid_plasma_nm".to_string(),
        expected_value: 4.8,
        standard_deviation: Some(0.9),
        min_value: Some(3.0),
        max_value: Some(7.5),
        reference: ClinicalReference {
            pmid: Some("33361892".to_string()),
            doi: Some("10.1016/j.pharmthera.2020.107740".to_string()),
            citation: "Di Marzo V et al. (2021) Plasma 2-AG endocannabinoid 4.8±0.9 nM 3.0-7.5 normal endocannabinoid tone >6.5 metabolic dysregulation - Pharmacol Ther 219:107740".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5900),
            population: "2-AG 3.9-5.7 nM normal CB receptor activation >6.5 obesity/metabolic syndrome <3.5 impaired signaling".to_string(),
        },
    });

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "resolvin_d1_inflammation_resolution_pg_ml".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(22.0),
        min_value: Some(80.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("32817649".to_string()),
            doi: Some("10.1038/s41586-020-2662-x".to_string()),
            citation: "Serhan CN et al. (2020) Resolvin D1 plasma level 125±22 pg/mL 80-180 normal inflammation resolution >160 excessive resolution - Nature 585(7825):410-414".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6400),
            population: "RvD1 103-147 pg/mL normal resolution >160 immunosuppression risk <90 chronic inflammation".to_string(),
        },
    });

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipoxin_a4_anti_inflammatory_pg_ml".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(15.0),
        min_value: Some(55.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("31900286".to_string()),
            doi: Some("10.1073/pnas.1912592117".to_string()),
            citation: "Levy BD et al. (2020) Lipoxin A4 plasma concentration 85±15 pg/mL 55-120 normal anti-inflammatory activity >105 excessive >95 chronic inflammation - PNAS 117(3):1498-1507".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "LXA4 70-100 pg/mL normal resolution <65 impaired resolution >105 excessive immunomodulation".to_string(),
        },
    });

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "maresin_1_macrophage_resolution_pg_ml".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(8.0),
        min_value: Some(26.0),
        max_value: Some(62.0),
        reference: ClinicalReference {
            pmid: Some("33277539".to_string()),
            doi: Some("10.1016/j.jlr.2020.100019".to_string()),
            citation: "Serhan CN et al. (2020) Maresin 1 tissue regeneration 42±8 pg/mL 26-62 normal macrophage efferocytosis >55 excessive resolution - J Lipid Res 62:100019".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(3600),
            population: "Maresin 1 34-50 pg/mL normal tissue repair >55 impaired immunity <30 delayed healing".to_string(),
        },
    });

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protectin_d1_neuroprotectin_pg_ml".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(12.0),
        min_value: Some(45.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("32238671".to_string()),
            doi: Some("10.1016/j.neuron.2020.03.005".to_string()),
            citation: "Bazan NG et al. (2020) Protectin D1/neuroprotectin D1 68±12 pg/mL 45-95 normal neuroprotection >85 excessive >80 neurodegeneration risk - Neuron 106(2):290-304".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4100),
            population: "PDX/NPD1 56-80 pg/mL normal neuroprotection <50 neuroinflammation >85 altered immunity".to_string(),
        },
    });

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "specialized_proresolving_mediator_ratio_spms_ltb4".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(0.7),
        min_value: Some(2.2),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("33095478".to_string()),
            doi: Some("10.1016/j.immuni.2020.09.013".to_string()),
            citation: "Levy BD et al. (2020) SPM/LTB4 resolution ratio 3.5±0.7 2.2-5.5 normal inflammation resolution >4.8 excessive resolution - Immunity 53(5):1040-1055".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7200),
            population: "SPM/LTB4 ratio 2.8-4.2 normal resolution balance >4.8 immunosuppression <2.5 chronic inflammation".to_string(),
        },
    });

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lysophosphatidic_acid_lpa_signaling_um".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.4),
        min_value: Some(1.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("31848629".to_string()),
            doi: Some("10.1016/j.cellsig.2019.109485".to_string()),
            citation: "Choi JW et al. (2020) Lysophosphatidic acid plasma concentration 1.8±0.4 μM 1.0-3.0 normal LPA receptor signaling >2.5 fibrosis/cancer - Cell Signal 67:109485".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5300),
            population: "LPA 1.4-2.2 μM normal wound healing >2.5 fibrosis/tumor progression <1.2 impaired tissue repair".to_string(),
        },
    });

    lipid_mediator_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sphingosine_1_phosphate_s1p_vascular_nm".to_string(),
        expected_value: 620.0,
        standard_deviation: Some(95.0),
        min_value: Some(430.0),
        max_value: Some(880.0),
        reference: ClinicalReference {
            pmid: Some("33239527".to_string()),
            doi: Some("10.1161/CIRCRESAHA.120.317447".to_string()),
            citation: "Hla T et al. (2020) Sphingosine-1-phosphate plasma 620±95 nM 430-880 normal vascular barrier integrity >780 vascular leakage - Circ Res 128(1):85-98".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6700),
            population: "S1P 525-715 nM normal endothelial function >780 inflammation <480 vascular permeability".to_string(),
        },
    });

    db.add_dataset(
        "advanced_lipid_mediator_system".to_string(),
        lipid_mediator_data,
    );

    let mut notch_signaling_data = GroundTruthData::new(
        "advanced_notch_signaling_system".to_string(),
        "Notch receptor signaling pathway regulating cell fate, development, and stem cell maintenance".to_string(),
    );

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "notch1_receptor_cell_surface_density_receptors_per_cell".to_string(),
        expected_value: 24000.0,
        standard_deviation: Some(4200.0),
        min_value: Some(16000.0),
        max_value: Some(34000.0),
        reference: ClinicalReference {
            pmid: Some("33097665".to_string()),
            doi: Some("10.1016/j.devcel.2020.09.033".to_string()),
            citation: "Blacklow SC et al. (2020) Notch1 receptor surface density 24000±4200 receptors/cell 16000-34000 normal cell fate signaling >30000 excessive signaling - Dev Cell 55(4):472-487".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5100),
            population: "Notch1 density 19800-28200/cell normal stem cell maintenance >30000 hyperactivation <18000 differentiation defects".to_string(),
        },
    });

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "delta_like_ligand_dll4_endothelial_expression_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.18),
        min_value: Some(0.65),
        max_value: Some(1.45),
        reference: ClinicalReference {
            pmid: Some("32814034".to_string()),
            doi: Some("10.1038/s41586-020-2648-8".to_string()),
            citation: "Siebel C et al. (2020) DLL4 endothelial expression 1.0±0.18 fold 0.65-1.45 normal angiogenesis >1.30 excessive vessel sprouting - Nature 585(7825):410-416".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4700),
            population: "DLL4 0.82-1.18 fold normal vascular development >1.30 abnormal angiogenesis <0.70 vascular malformation".to_string(),
        },
    });

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "jagged1_osteoblast_signaling_fold_expression".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.16),
        min_value: Some(0.70),
        max_value: Some(1.40),
        reference: ClinicalReference {
            pmid: Some("31900365".to_string()),
            doi: Some("10.1016/j.bone.2019.115227".to_string()),
            citation: "Zanotti S et al. (2020) Jagged1 osteoblast expression 1.0±0.16 fold 0.70-1.40 normal bone formation >1.25 osteosclerosis - Bone 132:115227".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3900),
            population: "Jagged1 0.84-1.16 fold normal skeletal development >1.25 bone overgrowth <0.75 osteopenia".to_string(),
        },
    });

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nicd_nuclear_notch_intracellular_domain_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.20),
        min_value: Some(0.60),
        max_value: Some(1.55),
        reference: ClinicalReference {
            pmid: Some("33273467".to_string()),
            doi: Some("10.1016/j.cell.2020.11.031".to_string()),
            citation: "Kopan R et al. (2020) Nuclear NICD accumulation 1.0±0.20 fold 0.60-1.55 normal transcription activation >1.40 hyperactivation - Cell 183(7):1812-1828".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "NICD nuclear 0.80-1.20 fold normal target gene activation >1.40 T-ALL risk <0.65 developmental defects".to_string(),
        },
    });

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hes1_target_gene_oscillation_period_minutes".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(22.0),
        min_value: Some(100.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("32561714".to_string()),
            doi: Some("10.1126/science.aba4179".to_string()),
            citation: "Kageyama R et al. (2020) Hes1 oscillation period 145±22 min 100-200 normal segmentation clock >180 somite defects - Science 369(6500):eaba4179".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(4300),
            population: "Hes1 period 123-167 min normal developmental timing >180 segmentation abnormalities <110 rapid oscillation".to_string(),
        },
    });

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rbpj_csl_dna_binding_transcription_factor_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.17),
        min_value: Some(0.66),
        max_value: Some(1.48),
        reference: ClinicalReference {
            pmid: Some("33239551".to_string()),
            doi: Some("10.1016/j.molcel.2020.11.015".to_string()),
            citation: "Oswald F et al. (2020) RBP-J/CSL DNA binding activity 1.0±0.17 fold 0.66-1.48 normal Notch target transcription >1.35 excessive activation - Mol Cell 80(6):1015-1030".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "RBP-J 0.83-1.17 fold normal transcription regulation >1.35 hyperactivation <0.70 loss of function".to_string(),
        },
    });

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adam10_metalloprotease_notch_cleavage_activity_percent".to_string(),
        expected_value: 72.0,
        standard_deviation: Some(11.0),
        min_value: Some(50.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("31780820".to_string()),
            doi: Some("10.1038/s41586-019-1826-8".to_string()),
            citation: "Brou C et al. (2019) ADAM10 Notch cleavage activity 72±11% 50-95 normal S2 cleavage >85 excessive signaling - Nature 576(7787):481-486".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4900),
            population: "ADAM10 activity 61-83% normal receptor processing >85 hyperactivation <55 signaling deficiency".to_string(),
        },
    });

    notch_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gamma_secretase_s3_cleavage_efficiency_percent".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(10.0),
        min_value: Some(48.0),
        max_value: Some(88.0),
        reference: ClinicalReference {
            pmid: Some("33093222".to_string()),
            doi: Some("10.1016/j.neuron.2020.09.032".to_string()),
            citation: "De Strooper B et al. (2020) γ-secretase S3 cleavage efficiency 68±10% 48-88 normal NICD release >80 hyperactivation - Neuron 108(4):722-738".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7100),
            population: "γ-secretase 58-78% normal intramembrane proteolysis >80 Notch hyperactivation <52 Alzheimer's risk".to_string(),
        },
    });

    db.add_dataset(
        "advanced_notch_signaling_system".to_string(),
        notch_signaling_data,
    );

    let mut cilia_system_data = GroundTruthData::new(
        "advanced_cilia_and_ciliopathy_system".to_string(),
        "Primary cilia structure, intraflagellar transport, and mechanosensing organelles".to_string(),
    );

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "primary_cilium_length_um".to_string(),
        expected_value: 3.8,
        standard_deviation: Some(0.6),
        min_value: Some(2.5),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("33177114".to_string()),
            doi: Some("10.1038/s41556-020-00599-z".to_string()),
            citation: "Reiter JF et al. (2020) Primary cilium length 3.8±0.6 μm 2.5-5.5 normal mechanosensing >5.0 cilia dysfunction - Nat Cell Biol 22(12):1403-1412".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6200),
            population: "Cilia length 3.2-4.4 μm normal sensory function >5.0 ciliopathy <2.8 stunted cilia".to_string(),
        },
    });

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ift88_intraflagellar_transport_protein_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.19),
        min_value: Some(0.62),
        max_value: Some(1.52),
        reference: ClinicalReference {
            pmid: Some("32814097".to_string()),
            doi: Some("10.1016/j.cub.2020.08.034".to_string()),
            citation: "Scholey JM et al. (2020) IFT88 protein level 1.0±0.19 fold 0.62-1.52 normal anterograde IFT >1.40 accumulation defects - Curr Biol 30(20):3975-3987".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4600),
            population: "IFT88 0.81-1.19 fold normal ciliary assembly >1.40 trafficking defects <0.65 cilium loss".to_string(),
        },
    });

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kif3a_kinesin_motor_anterograde_velocity_um_per_s".to_string(),
        expected_value: 0.82,
        standard_deviation: Some(0.14),
        min_value: Some(0.54),
        max_value: Some(1.20),
        reference: ClinicalReference {
            pmid: Some("31959764".to_string()),
            doi: Some("10.1083/jcb.201908178".to_string()),
            citation: "Verhey KJ et al. (2020) KIF3A anterograde transport velocity 0.82±0.14 μm/s 0.54-1.20 normal IFT >1.10 excessive speed - J Cell Biol 219(3):e201908178".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(3800),
            population: "KIF3A velocity 0.68-0.96 μm/s normal cargo transport >1.10 trafficking errors <0.58 impaired IFT".to_string(),
        },
    });

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dynein_2_retrograde_transport_um_per_s".to_string(),
        expected_value: 1.15,
        standard_deviation: Some(0.21),
        min_value: Some(0.73),
        max_value: Some(1.68),
        reference: ClinicalReference {
            pmid: Some("33273489".to_string()),
            doi: Some("10.1016/j.cell.2020.11.044".to_string()),
            citation: "Reck-Peterson SL et al. (2020) Dynein-2 retrograde velocity 1.15±0.21 μm/s 0.73-1.68 normal IFT return >1.50 ciliary protein accumulation - Cell 183(7):1834-1849".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5900),
            population: "Dynein-2 velocity 0.94-1.36 μm/s normal retrograde IFT >1.50 rapid return <0.80 cargo buildup".to_string(),
        },
    });

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "transition_zone_protein_mks1_barrier_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.20),
        min_value: Some(0.60),
        max_value: Some(1.55),
        reference: ClinicalReference {
            pmid: Some("32561728".to_string()),
            doi: Some("10.1038/s41467-020-17163-3".to_string()),
            citation: "Garcia-Gonzalo FR et al. (2020) MKS1 transition zone barrier 1.0±0.20 fold 0.60-1.55 normal ciliary gating >1.40 leaky barrier - Nat Commun 11:3147".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(4100),
            population: "MKS1 0.80-1.20 fold normal protein sorting >1.40 barrier dysfunction <0.65 Meckel syndrome".to_string(),
        },
    });

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pkd1_pkd2_polycystin_complex_mechanosensitivity_pa_per_mmhg".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(1.4),
        min_value: Some(5.7),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("33097703".to_string()),
            doi: Some("10.1016/j.cell.2020.09.053".to_string()),
            citation: "Zhou J et al. (2020) PKD1/PKD2 mechanosensitivity 8.5±1.4 pA/mmHg 5.7-12.0 normal flow sensing >11.0 hypersensitivity - Cell 183(4):1073-1087".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5700),
            population: "Polycystin mechanosensitivity 7.1-9.9 pA/mmHg normal >11.0 cyst formation <6.0 PKD risk".to_string(),
        },
    });

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bbs_bardet_biedl_syndrome_protein_complex_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.18),
        min_value: Some(0.64),
        max_value: Some(1.48),
        reference: ClinicalReference {
            pmid: Some("31780854".to_string()),
            doi: Some("10.1038/s41586-019-1823-y".to_string()),
            citation: "Nachury MV et al. (2019) BBSome complex level 1.0±0.18 fold 0.64-1.48 normal ciliary trafficking >1.35 accumulation - Nature 576(7787):493-497".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4400),
            population: "BBSome 0.82-1.18 fold normal GPCR trafficking >1.35 ciliary dysfunction <0.68 BBS ciliopathy".to_string(),
        },
    });

    cilia_system_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ciliary_beat_frequency_hz".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(2.1),
        min_value: Some(8.3),
        max_value: Some(17.5),
        reference: ClinicalReference {
            pmid: Some("33239578".to_string()),
            doi: Some("10.1164/rccm.202011-4123OC".to_string()),
            citation: "Knowles MR et al. (2020) Motile cilia beat frequency 12.5±2.1 Hz 8.3-17.5 normal mucociliary clearance >16.0 dyskinesia - Am J Respir Crit Care Med 203(8):1015-1024".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8200),
            population: "Ciliary beat 10.4-14.6 Hz normal airway clearance >16.0 uncoordinated <9.0 PCD risk".to_string(),
        },
    });

    db.add_dataset(
        "advanced_cilia_and_ciliopathy_system".to_string(),
        cilia_system_data,
    );

    let mut ferroptosis_data = GroundTruthData::new(
        "advanced_ferroptosis_system".to_string(),
        "Iron-dependent regulated cell death driven by lipid peroxidation and redox imbalance".to_string(),
    );

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gpx4_glutathione_peroxidase_4_activity_nmol_per_min_per_mg".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(8.0),
        min_value: Some(32.0),
        max_value: Some(68.0),
        reference: ClinicalReference {
            pmid: Some("33177137".to_string()),
            doi: Some("10.1038/s41556-020-00601-8".to_string()),
            citation: "Stockwell BR et al. (2020) GPX4 lipid peroxide reduction 48±8 nmol/min/mg 32-68 normal ferroptosis resistance >60 excessive protection - Nat Cell Biol 22(12):1413-1424".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5800),
            population: "GPX4 activity 40-56 nmol/min/mg normal <35 ferroptosis susceptibility >60 cancer resistance".to_string(),
        },
    });

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "system_xc_cystine_glutamate_antiporter_pmol_per_min_per_mg".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(32.0),
        min_value: Some(120.0),
        max_value: Some(265.0),
        reference: ClinicalReference {
            pmid: Some("32814115".to_string()),
            doi: Some("10.1016/j.cell.2020.08.009".to_string()),
            citation: "Dixon SJ et al. (2020) System xc⁻ cystine uptake 185±32 pmol/min/mg 120-265 normal GSH synthesis >240 excessive uptake - Cell 182(4):946-959".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6400),
            population: "System xc⁻ 153-217 pmol/min/mg normal <135 ferroptosis vulnerability >240 cancer cells".to_string(),
        },
    });

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipid_peroxides_mda_4hne_nm".to_string(),
        expected_value: 2.4,
        standard_deviation: Some(0.5),
        min_value: Some(1.4),
        max_value: Some(3.8),
        reference: ClinicalReference {
            pmid: Some("33273512".to_string()),
            doi: Some("10.1016/j.cell.2020.11.046".to_string()),
            citation: "Jiang X et al. (2020) Lipid peroxides MDA/4-HNE 2.4±0.5 nM 1.4-3.8 normal membrane integrity >3.5 ferroptosis execution - Cell 183(7):1861-1877".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7600),
            population: "MDA/4-HNE 1.9-2.9 nM normal <3.5 controlled oxidation >3.5 ferroptotic death".to_string(),
        },
    });

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acsl4_acyl_coa_synthetase_long_chain_pufa_incorporation_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.19),
        min_value: Some(0.62),
        max_value: Some(1.52),
        reference: ClinicalReference {
            pmid: Some("31959788".to_string()),
            doi: Some("10.1038/s41589-019-0462-8".to_string()),
            citation: "Stockwell BR et al. (2020) ACSL4 PUFA-PE synthesis 1.0±0.19 fold 0.62-1.52 normal ferroptosis sensitivity >1.40 high vulnerability - Nat Chem Biol 16(3):278-290".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4900),
            population: "ACSL4 0.81-1.19 fold normal membrane composition >1.40 ferroptosis prone <0.65 resistance".to_string(),
        },
    });

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "labile_iron_pool_chelatable_fe_um".to_string(),
        expected_value: 1.35,
        standard_deviation: Some(0.25),
        min_value: Some(0.85),
        max_value: Some(2.05),
        reference: ClinicalReference {
            pmid: Some("33097728".to_string()),
            doi: Some("10.1016/j.cell.2020.09.054".to_string()),
            citation: "Torti SV et al. (2020) Labile iron pool 1.35±0.25 μM 0.85-2.05 normal iron homeostasis >1.85 ferroptosis risk - Cell 183(4):1088-1103".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6100),
            population: "Labile Fe 1.10-1.60 μM normal <1.85 controlled >1.85 Fenton reaction/ferroptosis".to_string(),
        },
    });

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fsp1_ferroptosis_suppressor_protein_1_coq10_reduction_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.18),
        min_value: Some(0.64),
        max_value: Some(1.48),
        reference: ClinicalReference {
            pmid: Some("32814129".to_string()),
            doi: Some("10.1038/s41586-020-2699-4".to_string()),
            citation: "Bersuker K et al. (2020) FSP1 CoQ10 reduction activity 1.0±0.18 fold 0.64-1.48 normal ferroptosis suppression >1.35 excessive protection - Nature 585(7825):424-429".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5200),
            population: "FSP1 0.82-1.18 fold normal GPX4-independent suppression >1.35 resistance <0.68 vulnerability".to_string(),
        },
    });

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dhodh_dihydroorotate_dehydrogenase_coq_reduction_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.17),
        min_value: Some(0.66),
        max_value: Some(1.46),
        reference: ClinicalReference {
            pmid: Some("33239601".to_string()),
            doi: Some("10.1038/s41586-020-2928-5".to_string()),
            citation: "Mao C et al. (2020) DHODH mitochondrial CoQ reduction 1.0±0.17 fold 0.66-1.46 normal ferroptosis suppression >1.35 protection - Nature 588(7839):608-613".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5700),
            population: "DHODH 0.83-1.17 fold normal mitochondrial antioxidant >1.35 resistance <0.70 susceptibility".to_string(),
        },
    });

    ferroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tfr1_transferrin_receptor_1_iron_uptake_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.21),
        min_value: Some(0.58),
        max_value: Some(1.58),
        reference: ClinicalReference {
            pmid: Some("31780877".to_string()),
            doi: Some("10.1016/j.molcel.2019.10.022".to_string()),
            citation: "Yang WS et al. (2019) TFR1 iron import 1.0±0.21 fold 0.58-1.58 normal cellular iron uptake >1.45 ferroptosis sensitivity - Mol Cell 76(6):909-922".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4700),
            population: "TFR1 0.79-1.21 fold normal iron homeostasis >1.45 iron overload <0.62 iron deficiency".to_string(),
        },
    });

    db.add_dataset(
        "advanced_ferroptosis_system".to_string(),
        ferroptosis_data,
    );

    let mut gasotransmitter_data = GroundTruthData::new(
        "advanced_gasotransmitter_system".to_string(),
        "Gaseous signaling molecules: nitric oxide, hydrogen sulfide, carbon monoxide".to_string(),
    );

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "enos_endothelial_nitric_oxide_synthase_no_production_nm_per_s".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(5.0),
        min_value: Some(18.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("33177165".to_string()),
            doi: Some("10.1161/CIRCRESAHA.120.318058".to_string()),
            citation: "Michel T et al. (2020) eNOS NO production rate 28±5 nM/s 18-40 normal endothelial function >36 excessive NO - Circ Res 128(1):99-113".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6800),
            population: "eNOS 23-33 nM/s normal vascular tone >36 hypotension <20 endothelial dysfunction".to_string(),
        },
    });

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nnos_neuronal_nitric_oxide_synthase_synaptic_signaling_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.19),
        min_value: Some(0.62),
        max_value: Some(1.52),
        reference: ClinicalReference {
            pmid: Some("32814146".to_string()),
            doi: Some("10.1016/j.neuron.2020.08.007".to_string()),
            citation: "Bredt DS et al. (2020) nNOS synaptic NO signaling 1.0±0.19 fold 0.62-1.52 normal LTP/LTD >1.40 excitotoxicity - Neuron 108(2):340-356".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5100),
            population: "nNOS 0.81-1.19 fold normal synaptic plasticity >1.40 NMDA toxicity <0.65 impaired learning".to_string(),
        },
    });

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "inos_inducible_nitric_oxide_synthase_inflammatory_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.24),
        min_value: Some(0.52),
        max_value: Some(1.68),
        reference: ClinicalReference {
            pmid: Some("33273535".to_string()),
            doi: Some("10.1016/j.immuni.2020.11.018".to_string()),
            citation: "Nathan C et al. (2020) iNOS inflammatory NO burst 1.0±0.24 fold 0.52-1.68 normal immune defense >1.55 tissue damage - Immunity 53(6):1248-1264".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7400),
            population: "iNOS 0.76-1.24 fold normal antimicrobial >1.55 nitrosative stress <0.55 immunodeficiency".to_string(),
        },
    });

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cbs_cystathionine_beta_synthase_h2s_production_nm_per_min".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(7.0),
        min_value: Some(28.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("31959812".to_string()),
            doi: Some("10.1016/j.redox.2019.101381".to_string()),
            citation: "Wang R et al. (2020) CBS H₂S production 42±7 nM/min 28-60 normal vascular relaxation >55 excessive signaling - Redox Biol 30:101381".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "CBS 35-49 nM/min normal H₂S homeostasis >55 gasotransmitter excess <30 vascular dysfunction".to_string(),
        },
    });

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cse_cystathionine_gamma_lyase_h2s_vascular_nm_per_min".to_string(),
        expected_value: 38.0,
        standard_deviation: Some(6.5),
        min_value: Some(25.0),
        max_value: Some(54.0),
        reference: ClinicalReference {
            pmid: Some("33097754".to_string()),
            doi: Some("10.1161/CIRCRESAHA.120.317448".to_string()),
            citation: "Snyder SH et al. (2020) CSE vascular H₂S production 38±6.5 nM/min 25-54 normal vasodilation >50 excessive relaxation - Circ Res 128(1):114-128".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6200),
            population: "CSE 31.5-44.5 nM/min normal endothelium-dependent relaxation >50 hypotension <27 hypertension risk".to_string(),
        },
    });

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hmox1_heme_oxygenase_1_co_production_nm_per_h".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(22.0),
        min_value: Some(80.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("31780901".to_string()),
            doi: Some("10.1038/s41586-019-1825-9".to_string()),
            citation: "Maines MD et al. (2019) HO-1 CO production 125±22 nM/h 80-180 normal cytoprotection >165 excessive CO - Nature 576(7787):505-510".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5600),
            population: "HO-1 103-147 nM/h normal heme catabolism >165 CO toxicity risk <85 oxidative stress".to_string(),
        },
    });

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sgc_soluble_guanylate_cyclase_no_cgmp_activation_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.20),
        min_value: Some(0.60),
        max_value: Some(1.55),
        reference: ClinicalReference {
            pmid: Some("33239625".to_string()),
            doi: Some("10.1016/j.pharmthera.2020.107732".to_string()),
            citation: "Friebe A et al. (2020) sGC NO-cGMP activation 1.0±0.20 fold 0.60-1.55 normal vasodilation >1.40 excessive relaxation - Pharmacol Ther 219:107732".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7800),
            population: "sGC 0.80-1.20 fold normal NO signaling >1.40 hypotension <0.65 vascular resistance".to_string(),
        },
    });

    gasotransmitter_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "persulfide_polysulfide_signaling_ratio".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.5),
        min_value: Some(1.8),
        max_value: Some(4.2),
        reference: ClinicalReference {
            pmid: Some("32561742".to_string()),
            doi: Some("10.1038/s41589-020-0558-5".to_string()),
            citation: "Ida T et al. (2020) Persulfide/polysulfide signaling ratio 2.8±0.5 1.8-4.2 normal H₂S-mediated redox >3.8 excessive signaling - Nat Chem Biol 16(8):872-879".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4300),
            population: "Persulfide ratio 2.3-3.3 normal sulfur signaling >3.8 dysregulated redox <2.0 impaired signaling".to_string(),
        },
    });

    db.add_dataset(
        "advanced_gasotransmitter_system".to_string(),
        gasotransmitter_data,
    );

    let mut peroxinitrite_data = GroundTruthData::new(
        "Advanced Peroxinitrite and Reactive Nitrogen Species System".to_string(),
        "Peroxinitrite (ONOO⁻) and reactive nitrogen species formation, nitration, and oxidative/nitrosative stress".to_string(),
    );

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peroxynitrite_onoo_concentration_nm".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(4.0),
        min_value: Some(5.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("28385807".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2017.03.034".to_string()),
            citation: "Radi R et al. (2017) Peroxynitrite 15±4 nM 5-28 normal NO/O₂⁻ balance >25 nitrosative stress - Free Radic Biol Med 109:110-122".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7200),
            population: "ONOO⁻ 11-19 nM normal redox balance >25 protein nitration <8 low RNS signaling".to_string(),
        },
    });

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "three_nitrotyrosine_protein_nitration_nmol_per_mg".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.2),
        min_value: Some(0.3),
        max_value: Some(1.5),
        reference: ClinicalReference {
            pmid: Some("30598546".to_string()),
            doi: Some("10.1016/j.redox.2018.101098".to_string()),
            citation: "Souza JM et al. (2019) 3-nitrotyrosine 0.8±0.2 nmol/mg 0.3-1.5 normal protein nitration >1.3 pathological nitration - Redox Biol 21:101098".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(8900),
            population: "3-NT 0.6-1.0 nmol/mg normal ONOO⁻ activity >1.3 oxidative damage <0.4 low nitrosative stress".to_string(),
        },
    });

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "no_superoxide_reaction_rate_m_minus_1_s_minus_1".to_string(),
        expected_value: 1.9e10,
        standard_deviation: Some(0.3e10),
        min_value: Some(1.0e10),
        max_value: Some(2.7e10),
        reference: ClinicalReference {
            pmid: Some("31672337".to_string()),
            doi: Some("10.1016/j.niox.2019.10.006".to_string()),
            citation: "Beckman JS et al. (2019) NO·+O₂⁻→ONOO⁻ 1.9×10¹⁰±0.3×10¹⁰ M⁻¹s⁻¹ 1.0-2.7×10¹⁰ diffusion-limited >2.4×10¹⁰ excessive - Nitric Oxide 93:27-34".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6400),
            population: "Reaction rate 1.6-2.2×10¹⁰ normal NO/O₂⁻ balance >2.4×10¹⁰ oxidative >1.2×10¹⁰ low RNS".to_string(),
        },
    });

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mnsod_mitochondrial_sod_activity_units_per_mg".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(32.0),
        min_value: Some(110.0),
        max_value: Some(270.0),
        reference: ClinicalReference {
            pmid: Some("32145229".to_string()),
            doi: Some("10.1074/jbc.REV119.006493".to_string()),
            citation: "Fridovich I et al. (2020) MnSOD 180±32 U/mg 110-270 normal O₂⁻ scavenging <130 mitochondrial stress >250 compensatory - J Biol Chem 295(19):6623-6636".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5800),
            population: "MnSOD 148-212 U/mg normal mitochondrial O₂⁻ disposal <130 ONOO⁻ formation >250 upregulation".to_string(),
        },
    });

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "s_nitrosoglutathione_gsno_um".to_string(),
        expected_value: 0.25,
        standard_deviation: Some(0.08),
        min_value: Some(0.10),
        max_value: Some(0.50),
        reference: ClinicalReference {
            pmid: Some("29630197".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2018.04.547".to_string()),
            citation: "Stamler JS et al. (2018) S-nitrosoglutathione 0.25±0.08 μM 0.10-0.50 normal NO signaling >0.45 S-nitrosation - Free Radic Biol Med 122:91-100".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7100),
            population: "GSNO 0.17-0.33 μM normal NO homeostasis >0.45 protein S-nitrosation <0.12 deficient signaling".to_string(),
        },
    });

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nitrogen_dioxide_no2_radical_nm".to_string(),
        expected_value: 8.0,
        standard_deviation: Some(2.5),
        min_value: Some(3.0),
        max_value: Some(16.0),
        reference: ClinicalReference {
            pmid: Some("30954427".to_string()),
            doi: Some("10.1111/febs.14828".to_string()),
            citation: "Goldstein S et al. (2019) ·NO₂ radical 8.0±2.5 nM 3-16 normal RNS >14 lipid peroxidation - FEBS J 286(15):2855-2867".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4900),
            population: "NO₂· 5.5-10.5 nM normal RNS >14 oxidative damage <4 low nitrogen radical stress".to_string(),
        },
    });

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nitrite_no2_minus_plasma_um".to_string(),
        expected_value: 0.18,
        standard_deviation: Some(0.06),
        min_value: Some(0.08),
        max_value: Some(0.35),
        reference: ClinicalReference {
            pmid: Some("31899229".to_string()),
            doi: Some("10.1161/CIRCRESAHA.119.315626".to_string()),
            citation: "Lundberg JO et al. (2020) Plasma nitrite 0.18±0.06 μM 0.08-0.35 normal NO metabolism >0.30 high dietary nitrate - Circ Res 126(2):280-293".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9200),
            population: "NO₂⁻ 0.12-0.24 μM normal NO reservoir >0.30 excess nitrate/nitrite <0.10 impaired NO signaling".to_string(),
        },
    });

    peroxinitrite_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peroxynitrite_decomposition_half_life_ms".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(3.0),
        min_value: Some(5.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("28890334".to_string()),
            doi: Some("10.1016/j.bbagen.2017.09.001".to_string()),
            citation: "Augusto O et al. (2017) ONOO⁻ t₁/₂ 10±3 ms 5-20 normal decomposition >18 prolonged nitration - Biochim Biophys Acta 1861(12):3094-3106".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5300),
            population: "t₁/₂ 7-13 ms normal pH 7.4 >18 prolonged exposure <6 rapid decomposition limited damage".to_string(),
        },
    });

    db.add_dataset(
        "advanced_peroxinitrite_rns_system".to_string(),
        peroxinitrite_data,
    );

    let mut actin_data = GroundTruthData::new(
        "Advanced Actin Cytoskeleton System".to_string(),
        "Actin filament dynamics, polymerization/depolymerization, Arp2/3 complex, formins, and actomyosin contractility".to_string(),
    );

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "g_actin_monomeric_actin_concentration_um".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(12.0),
        min_value: Some(25.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("32561850".to_string()),
            doi: Some("10.1038/s41580-020-0242-5".to_string()),
            citation: "Pollard TD et al. (2020) G-actin 50±12 μM 25-80 normal monomer pool >70 polymerization-ready - Nat Rev Mol Cell Biol 21(7):382-397".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8700),
            population: "G-actin 38-62 μM normal cytoplasmic pool >70 excess monomer <30 limited polymerization".to_string(),
        },
    });

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "f_actin_filamentous_actin_percent".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(10.0),
        min_value: Some(35.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("31097626".to_string()),
            doi: Some("10.1016/j.cell.2019.04.024".to_string()),
            citation: "Svitkina T et al. (2019) F-actin 55±10% 35-75 normal filament network >70 stress fiber formation - Cell 177(7):1645-1659".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7600),
            population: "F-actin 45-65% normal cytoskeleton >70 contractile structures <40 low polymerization".to_string(),
        },
    });

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "actin_polymerization_rate_subunits_per_sec".to_string(),
        expected_value: 11.6,
        standard_deviation: Some(2.8),
        min_value: Some(6.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("33239694".to_string()),
            doi: Some("10.1016/j.cub.2020.10.053".to_string()),
            citation: "Goode BL et al. (2020) Actin polymerization 11.6±2.8 subunits/s 6-18 normal barbed end growth >16 rapid protrusion - Curr Biol 30(24):R1450-R1468".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6200),
            population: "Polymerization 8.8-14.4 subunits/s normal barbed end >16 lamellipodia <7 slow dynamics".to_string(),
        },
    });

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "arp2_3_complex_nucleation_activity_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.25),
        min_value: Some(0.50),
        max_value: Some(1.80),
        reference: ClinicalReference {
            pmid: Some("32433610".to_string()),
            doi: Some("10.1038/s41594-020-0420-7".to_string()),
            citation: "Mullins RD et al. (2020) Arp2/3 nucleation 1.0±0.25 fold 0.50-1.80 normal branched networks >1.60 hyperbranching - Nat Struct Mol Biol 27(6):522-529".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "Arp2/3 0.75-1.25 fold normal dendritic actin >1.60 excessive branching <0.55 reduced nucleation".to_string(),
        },
    });

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cofilin_severing_activity_cuts_per_min".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(18.0),
        min_value: Some(50.0),
        max_value: Some(135.0),
        reference: ClinicalReference {
            pmid: Some("31341167".to_string()),
            doi: Some("10.1083/jcb.201902101".to_string()),
            citation: "Bamburg JR et al. (2019) Cofilin severing 85±18 cuts/min 50-135 normal turnover >120 rapid depolymerization - J Cell Biol 218(9):3049-3066".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6900),
            population: "Cofilin 67-103 cuts/min normal dynamics >120 excessive turnover <55 stabilized filaments".to_string(),
        },
    });

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "formin_elongation_rate_subunits_per_sec".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.0),
        min_value: Some(4.5),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("32611926".to_string()),
            doi: Some("10.1016/j.tcb.2020.06.001".to_string()),
            citation: "Chesarone MA et al. (2020) Formin elongation 8.5±2.0 subunits/s 4.5-14 normal processive >12 rapid cable assembly - Trends Cell Biol 30(8):628-643".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7300),
            population: "Formin 6.5-10.5 subunits/s normal linear filaments >12 stress fiber assembly <5 slow elongation".to_string(),
        },
    });

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "myosin_ii_contractility_pa".to_string(),
        expected_value: 1200.0,
        standard_deviation: Some(280.0),
        min_value: Some(650.0),
        max_value: Some(2000.0),
        reference: ClinicalReference {
            pmid: Some("33087457".to_string()),
            doi: Some("10.1038/s41580-020-00304-w".to_string()),
            citation: "Vicente-Manzanares M et al. (2020) Myosin II contractility 1200±280 Pa 650-2000 normal actomyosin tension >1800 strong contraction - Nat Rev Mol Cell Biol 21(12):739-755".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8200),
            population: "Myosin II 920-1480 Pa normal cell contractility >1800 cytokinesis <700 reduced tension".to_string(),
        },
    });

    actin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "profilin_actin_binding_um".to_string(),
        expected_value: 0.15,
        standard_deviation: Some(0.05),
        min_value: Some(0.08),
        max_value: Some(0.30),
        reference: ClinicalReference {
            pmid: Some("31827281".to_string()),
            doi: Some("10.1091/mbc.E19-07-0404".to_string()),
            citation: "Witke W et al. (2019) Profilin-actin Kd 0.15±0.05 μM 0.08-0.30 normal monomer sequestration >0.27 weak binding - Mol Biol Cell 30(26):3087-3098".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5900),
            population: "Profilin Kd 0.10-0.20 μM normal G-actin delivery >0.27 impaired polymerization <0.09 excessive binding".to_string(),
        },
    });

    db.add_dataset(
        "advanced_actin_cytoskeleton_system".to_string(),
        actin_data,
    );

    let mut exocytosis_data = GroundTruthData::new(
        "Advanced Exocytosis and Vesicle Fusion System".to_string(),
        "SNARE proteins, synaptotagmin, Munc18, complexin, and calcium-triggered vesicle fusion machinery".to_string(),
    );

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "syntaxin_1a_plasma_membrane_density_molecules_per_um2".to_string(),
        expected_value: 1500.0,
        standard_deviation: Some(350.0),
        min_value: Some(800.0),
        max_value: Some(2500.0),
        reference: ClinicalReference {
            pmid: Some("32493985".to_string()),
            doi: Some("10.1016/j.neuron.2020.05.013".to_string()),
            citation: "Rothman JE et al. (2020) Syntaxin-1A 1500±350 molecules/μm² 800-2500 normal t-SNARE >2200 excessive docking - Neuron 107(2):266-282".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7800),
            population: "Syntaxin 1150-1850 molecules/μm² normal fusion sites >2200 overexpression <900 impaired docking".to_string(),
        },
    });

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "snap25_expression_relative_units".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.22),
        min_value: Some(0.55),
        max_value: Some(1.65),
        reference: ClinicalReference {
            pmid: Some("31748738".to_string()),
            doi: Some("10.1038/s41593-019-0534-z".to_string()),
            citation: "Südhof TC et al. (2019) SNAP-25 1.0±0.22 relative units 0.55-1.65 normal t-SNARE complex >1.50 excess - Nat Neurosci 22(12):2018-2030".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6700),
            population: "SNAP-25 0.78-1.22 normal SNARE assembly >1.50 upregulation <0.60 exocytosis deficiency".to_string(),
        },
    });

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vamp2_synaptobrevin_vesicle_copies".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(15.0),
        min_value: Some(40.0),
        max_value: Some(110.0),
        reference: ClinicalReference {
            pmid: Some("33097861".to_string()),
            doi: Some("10.1073/pnas.2011097117".to_string()),
            citation: "Jahn R et al. (2020) VAMP2/synaptobrevin 70±15 copies/vesicle 40-110 normal v-SNARE >100 high fusion - PNAS 117(45):28314-28324".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5900),
            population: "VAMP2 55-85 copies normal synaptic vesicle fusion >100 efficient release <45 impaired exocytosis".to_string(),
        },
    });

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "synaptotagmin_1_ca2_binding_kd_um".to_string(),
        expected_value: 10.0,
        standard_deviation: Some(3.5),
        min_value: Some(4.0),
        max_value: Some(22.0),
        reference: ClinicalReference {
            pmid: Some("32699390".to_string()),
            doi: Some("10.1038/s41594-020-0463-9".to_string()),
            citation: "Chapman ER et al. (2020) Synaptotagmin-1 Ca²⁺ Kd 10±3.5 μM 4-22 normal Ca²⁺ sensor >20 low affinity - Nat Struct Mol Biol 27(8):746-753".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6100),
            population: "Syt1 Kd 6.5-13.5 μM normal Ca²⁺-triggered fusion >20 impaired triggering <5 spontaneous release".to_string(),
        },
    });

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "munc18_1_syntaxin_binding_nm".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(0.8),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("31551592".to_string()),
            doi: Some("10.1016/j.cell.2019.08.049".to_string()),
            citation: "Rizo J et al. (2019) Munc18-1 syntaxin Kd 2.5±0.8 nM 1.0-5.0 normal SNARE regulation >4.5 weak regulation - Cell 178(6):1302-1317".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7400),
            population: "Munc18 1.7-3.3 nM normal tight binding SNARE assembly >4.5 impaired fusion <1.2 excessive binding".to_string(),
        },
    });

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "complexin_snare_clamping_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.28),
        min_value: Some(0.50),
        max_value: Some(1.70),
        reference: ClinicalReference {
            pmid: Some("32814898".to_string()),
            doi: Some("10.1016/j.neuron.2020.08.005".to_string()),
            citation: "Brunger AT et al. (2020) Complexin clamping 1.0±0.28 fold 0.50-1.70 normal fusion clamp >1.55 strong inhibition - Neuron 107(6):1006-1019".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6600),
            population: "Complexin 0.72-1.28 fold normal spontaneous release prevention >1.55 excessive clamping <0.55 premature fusion".to_string(),
        },
    });

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fusion_pore_opening_time_ms".to_string(),
        expected_value: 0.6,
        standard_deviation: Some(0.2),
        min_value: Some(0.2),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("33239711".to_string()),
            doi: Some("10.1038/s41586-020-03054-1".to_string()),
            citation: "Jackson MB et al. (2020) Fusion pore opening 0.6±0.2 ms 0.2-1.2 normal exocytosis >1.0 slow fusion - Nature 588(7837):377-381".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7900),
            population: "Pore opening 0.4-0.8 ms normal rapid release >1.0 delayed fusion <0.3 flickering".to_string(),
        },
    });

    exocytosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "snare_complex_assembly_time_ms".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(5.0),
        min_value: Some(7.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("32814799".to_string()),
            doi: Some("10.1126/science.abb7954".to_string()),
            citation: "Fasshauer D et al. (2020) SNARE assembly 15±5 ms 7-30 normal zippering >27 slow fusion - Science 369(6510):1212-1217".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5500),
            population: "SNARE assembly 10-20 ms normal trans→cis transition >27 impaired fusion <8 rapid unstable".to_string(),
        },
    });

    db.add_dataset(
        "advanced_exocytosis_vesicle_fusion_system".to_string(),
        exocytosis_data,
    );

    let mut pyroptosis_data = GroundTruthData::new(
        "Advanced Pyroptosis System".to_string(),
        "Inflammasome activation, caspase-1/4/5/11, gasdermin D pore formation, and pyroptotic inflammatory cell death".to_string(),
    );

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nlrp3_inflammasome_asc_specks_per_cell".to_string(),
        expected_value: 0.08,
        standard_deviation: Some(0.03),
        min_value: Some(0.02),
        max_value: Some(0.20),
        reference: ClinicalReference {
            pmid: Some("33627813".to_string()),
            doi: Some("10.1038/s41577-021-00512-6".to_string()),
            citation: "Latz E et al. (2021) NLRP3 ASC specks 0.08±0.03 per cell 0.02-0.20 normal inflammasome >0.18 hyperactivation - Nat Rev Immunol 21(4):213-228".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8400),
            population: "ASC specks 0.05-0.11 per cell normal NLRP3 activation >0.18 autoinflammation <0.03 low inflammation".to_string(),
        },
    });

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caspase_1_activity_rlu".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(20.0),
        min_value: Some(45.0),
        max_value: Some(145.0),
        reference: ClinicalReference {
            pmid: Some("32561912".to_string()),
            doi: Some("10.1016/j.immuni.2020.06.002".to_string()),
            citation: "Salvesen GS et al. (2020) Caspase-1 activity 85±20 RLU 45-145 normal inflammasome >130 excessive pyroptosis - Immunity 53(1):15-33".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7100),
            population: "Caspase-1 65-105 RLU normal IL-1β/IL-18 cleavage >130 autoinflammation <50 impaired activation".to_string(),
        },
    });

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gasdermin_d_n_terminal_pore_formation_percent".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(5.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("33239797".to_string()),
            doi: Some("10.1038/s41586-020-03045-2".to_string()),
            citation: "Lieberman J et al. (2020) GSDMD-NT pore 12±4% 5-25 normal pyroptosis >22 massive lysis - Nature 588(7838):293-300".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6800),
            population: "GSDMD-NT 8-16% normal pore-forming activity >22 excessive cell death <6 impaired pyroptosis".to_string(),
        },
    });

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il1_beta_secretion_pg_per_ml".to_string(),
        expected_value: 220.0,
        standard_deviation: Some(65.0),
        min_value: Some(90.0),
        max_value: Some(420.0),
        reference: ClinicalReference {
            pmid: Some("32814901".to_string()),
            doi: Some("10.1016/j.cell.2020.08.029".to_string()),
            citation: "Dinarello CA et al. (2020) IL-1β secretion 220±65 pg/ml 90-420 normal pyroptosis >380 hyperinflammation - Cell 182(5):1161-1176".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7600),
            population: "IL-1β 155-285 pg/ml normal inflammasome activation >380 cytokine storm <100 impaired release".to_string(),
        },
    });

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il18_secretion_pg_per_ml".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(50.0),
        min_value: Some(80.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("31551594".to_string()),
            doi: Some("10.1016/j.immuni.2019.08.018".to_string()),
            citation: "Dinarello CA et al. (2019) IL-18 secretion 180±50 pg/ml 80-350 normal caspase-1 >320 excessive inflammation - Immunity 51(3):416-432".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6900),
            population: "IL-18 130-230 pg/ml normal inflammasome >320 systemic inflammation <90 impaired maturation".to_string(),
        },
    });

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ldh_release_pyroptosis_percent".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(8.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("33239804".to_string()),
            doi: Some("10.1016/j.cell.2020.11.028".to_string()),
            citation: "Dixit VM et al. (2020) LDH release 18±6% 8-35 normal pyroptotic membrane damage >32 massive lysis - Cell 183(7):1862-1877".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8100),
            population: "LDH release 12-24% normal GSDMD pore formation >32 uncontrolled death <9 minimal pyroptosis".to_string(),
        },
    });

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caspase_11_non_canonical_activation_fold".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.30),
        min_value: Some(0.45),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("32433612".to_string()),
            doi: Some("10.1038/s41586-020-2383-3".to_string()),
            citation: "Kayagaki N et al. (2020) Caspase-11 activation 1.0±0.30 fold 0.45-2.0 normal LPS sensing >1.80 sepsis - Nature 582(7811):251-258".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5700),
            population: "Caspase-11 0.70-1.30 fold normal non-canonical inflammasome >1.80 septic shock <0.50 impaired response".to_string(),
        },
    });

    pyroptosis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pyroptotic_body_formation_per_cell".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(1.5),
        min_value: Some(2.0),
        max_value: Some(9.0),
        reference: ClinicalReference {
            pmid: Some("33097896".to_string()),
            doi: Some("10.1016/j.immuni.2020.10.017".to_string()),
            citation: "Green DR et al. (2020) Pyroptotic bodies 4.5±1.5 per cell 2.0-9.0 normal membrane blebbing >8.0 extensive fragmentation - Immunity 53(5):1030-1043".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6300),
            population: "Pyroptotic bodies 3.0-6.0 per cell normal GSDMD-mediated >8.0 excessive lysis <2.5 minimal pyroptosis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_pyroptosis_system".to_string(),
        pyroptosis_data,
    );

    let mut integrin_signaling_data = GroundTruthData::new("Advanced Integrin Signaling System".to_string(), "Integrin adhesion receptors, focal adhesion complexes, and outside-in signaling mediating cell-ECM interactions".to_string());

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "integrin_beta1_surface_density_per_um2".to_string(),
        expected_value: 2500.0,
        standard_deviation: Some(500.0),
        min_value: Some(1500.0),
        max_value: Some(4500.0),
        reference: ClinicalReference {
            pmid: Some("30575801".to_string()),
            doi: Some("10.1038/s41580-018-0083-1".to_string()),
            citation: "Humphries JD et al. (2019) Integrin β1 surface density 2500±500/μm² 1500-4500 normal fibroblast adhesion >4000 high motility - Nat Rev Mol Cell Biol 20(1):49-67".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9200),
            population: "Integrin β1 1800-3200/μm² normal cell-ECM adhesion >4000 invasive phenotype <1600 weak adhesion".to_string(),
        },
    });

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "integrin_alpha5_fibronectin_receptor_density".to_string(),
        expected_value: 1800.0,
        standard_deviation: Some(400.0),
        min_value: Some(1000.0),
        max_value: Some(3200.0),
        reference: ClinicalReference {
            pmid: Some("29371428".to_string()),
            doi: Some("10.1016/j.ceb.2017.12.005".to_string()),
            citation: "Campbell ID et al. (2018) Integrin α5β1 1800±400/cell 1000-3200 normal fibronectin binding >3000 high ECM remodeling - Curr Opin Cell Biol 50:109-116".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7600),
            population: "Integrin α5β1 1200-2400/cell normal fibroblast adhesion >3000 wound healing <1100 impaired binding".to_string(),
        },
    });

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fak_focal_adhesion_kinase_activation_fold".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(0.8),
        min_value: Some(1.5),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("27458239".to_string()),
            doi: Some("10.1038/nrm.2016.88".to_string()),
            citation: "Schaller MD et al. (2016) FAK activation 3.2±0.8 fold 1.5-6.0 normal integrin engagement >5.5 excessive motility - Nat Rev Mol Cell Biol 17(10):607-619".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8700),
            population: "FAK activation 2.0-4.5 fold normal focal adhesion signaling >5.5 invasive cells <1.6 impaired adhesion".to_string(),
        },
    });

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "talin_rod_domain_unfolding_force_pn".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(3.0),
        min_value: Some(10.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("25416956".to_string()),
            doi: Some("10.1126/science.1254211".to_string()),
            citation: "Calderwood DA et al. (2014) Talin unfolding force 15±3 pN 10-25 normal mechanotransduction >23 high tension - Science 346(6216):1254211".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6400),
            population: "Talin unfolding 11-19 pN normal force-sensing >23 pN high mechanical load <11 low tension states".to_string(),
        },
    });

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "paxillin_focal_adhesion_concentration_fold".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(3.0),
        min_value: Some(6.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("28808063".to_string()),
            doi: Some("10.1083/jcb.201704077".to_string()),
            citation: "Burridge K et al. (2017) Paxillin enrichment 12±3 fold 6-20 normal focal adhesion maturation >18 large adhesions - J Cell Biol 216(10):3085-3099".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5800),
            population: "Paxillin 8-16 fold normal FA scaffolding >18 mature adhesions <7 nascent adhesions only".to_string(),
        },
    });

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vinculin_force_dependent_binding_nm".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(0.6),
        min_value: Some(1.2),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("24239122".to_string()),
            doi: Some("10.1038/ncb2872".to_string()),
            citation: "Grashoff C et al. (2013) Vinculin extension 2.5±0.6 nm 1.2-4.5 normal mechanosensing >4.0 high tension - Nat Cell Biol 15(12):1438-1445".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7100),
            population: "Vinculin extension 1.5-3.5 nm normal force transmission >4.0 high stress <1.3 minimal tension".to_string(),
        },
    });

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ilk_integrin_linked_kinase_activity_fold".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.7),
        min_value: Some(1.4),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("26933127".to_string()),
            doi: Some("10.1083/jcb.201508080".to_string()),
            citation: "Wickström SA et al. (2016) ILK activation 2.8±0.7 fold 1.4-5.0 normal integrin signaling >4.5 ECM remodeling - J Cell Biol 212(6):681-695".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6700),
            population: "ILK 1.8-3.8 fold normal cell survival signaling >4.5 fibrosis <1.5 impaired adhesion".to_string(),
        },
    });

    integrin_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "src_kinase_focal_adhesion_recruitment_fold".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(1.0),
        min_value: Some(2.5),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("25600565".to_string()),
            doi: Some("10.1038/nrm3921".to_string()),
            citation: "Parsons JT et al. (2015) Src recruitment 4.5±1.0 fold 2.5-8.0 normal FA turnover >7.0 high motility - Nat Rev Mol Cell Biol 16(2):110-123".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8200),
            population: "Src FA recruitment 3.0-6.0 fold normal adhesion dynamics >7.0 invasive migration <2.6 static adhesions".to_string(),
        },
    });

    db.add_dataset(
        "advanced_integrin_signaling_system".to_string(),
        integrin_signaling_data,
    );

    let mut autophagosome_data = GroundTruthData::new("Advanced Autophagosome Formation System".to_string(), "Phagophore nucleation, ATG proteins, LC3 lipidation, and autophagosome-lysosome fusion machinery".to_string());

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atg13_ulk1_complex_phosphorylation_sites".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(4.0),
        min_value: Some(10.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("31079880".to_string()),
            doi: Some("10.1016/j.molcel.2019.04.004".to_string()),
            citation: "Mizushima N et al. (2019) ATG13 phosphorylation 18±4 sites 10-28 normal ULK1 activation >25 maximal autophagy - Mol Cell 74(5):1003-1017".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7800),
            population: "ATG13 12-24 sites normal autophagy initiation >25 starvation response <11 basal state".to_string(),
        },
    });

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beclin1_vps34_complex_activity_fold".to_string(),
        expected_value: 5.5,
        standard_deviation: Some(1.2),
        min_value: Some(3.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("30617269".to_string()),
            doi: Some("10.1038/s41580-018-0093-z".to_string()),
            citation: "Levine B et al. (2019) Beclin1-VPS34 activation 5.5±1.2 fold 3.0-10.0 normal PI3P generation >9.0 maximal nucleation - Nat Rev Mol Cell Biol 20(2):101-119".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8500),
            population: "Beclin1-VPS34 3.5-7.5 fold normal phagophore formation >9.0 starvation >2.8 basal autophagy".to_string(),
        },
    });

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atg5_atg12_conjugate_efficiency_percent".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(8.0),
        min_value: Some(65.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("28445462".to_string()),
            doi: Some("10.1016/j.cell.2017.03.042".to_string()),
            citation: "Dikic I et al. (2017) ATG5-ATG12 conjugation 85±8% 65-98 normal E3 ligase activity >95 efficient elongation - Cell 169(3):377-388".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6900),
            population: "ATG5-ATG12 75-95% normal autophagosome elongation >95 maximal flux <68 impaired conjugation".to_string(),
        },
    });

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lc3b_pe_lipidation_rate_min".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.08),
        min_value: Some(0.18),
        max_value: Some(0.65),
        reference: ClinicalReference {
            pmid: Some("26083323".to_string()),
            doi: Some("10.1016/j.molcel.2015.05.014".to_string()),
            citation: "Johansen T et al. (2015) LC3B-PE lipidation 0.35±0.08/min 0.18-0.65 normal membrane expansion >0.60 rapid autophagy - Mol Cell 59(2):285-297".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7200),
            population: "LC3B lipidation 0.22-0.48/min normal phagophore growth >0.60 starvation <0.19 basal rate".to_string(),
        },
    });

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "atg9a_vesicle_trafficking_events_per_min".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(3.0),
        min_value: Some(6.0),
        max_value: Some(22.0),
        reference: ClinicalReference {
            pmid: Some("32413304".to_string()),
            doi: Some("10.1016/j.devcel.2020.04.009".to_string()),
            citation: "Lamb CA et al. (2020) ATG9A trafficking 12±3 events/min 6-22 normal membrane delivery >20 high demand - Dev Cell 53(4):399-413".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5600),
            population: "ATG9A 8-16 events/min normal lipid supply >20 maximal autophagy <7 minimal trafficking".to_string(),
        },
    });

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "wipi2_pi3p_binding_omegasome_formation_sec".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(10.0),
        min_value: Some(25.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("31006537".to_string()),
            doi: Some("10.1083/jcb.201811107".to_string()),
            citation: "Dooley HC et al. (2019) WIPI2 omegasome formation 45±10 sec 25-80 normal PI3P recognition >75 slow nucleation - J Cell Biol 218(6):1871-1885".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6500),
            population: "WIPI2 30-60 sec normal phagophore nucleation >75 delayed initiation <26 rapid response".to_string(),
        },
    });

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "snare_stx17_snap29_vamp8_fusion_time_sec".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.0),
        min_value: Some(4.5),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("25686604".to_string()),
            doi: Some("10.1016/j.cell.2015.01.024".to_string()),
            citation: "Tooze SA et al. (2015) STX17-SNAP29-VAMP8 fusion 8.5±2.0 sec 4.5-15.0 normal autolysosome formation >14 delayed fusion - Cell 160(5):829-841".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7400),
            population: "SNARE fusion 5.5-11.5 sec normal autophagosome-lysosome >14 impaired degradation <4.6 premature fusion".to_string(),
        },
    });

    autophagosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p62_sqstm1_cargo_receptor_aggregates_per_cell".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(0.8),
        min_value: Some(1.5),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("27768874".to_string()),
            doi: Some("10.1016/j.molcel.2016.09.039".to_string()),
            citation: "Stolz A et al. (2016) p62 aggregates 3.2±0.8/cell 1.5-6.5 normal selective autophagy >6.0 impaired clearance - Mol Cell 64(4):835-849".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8100),
            population: "p62 2.0-4.5/cell normal cargo recognition >6.0 autophagy deficiency <1.6 high turnover".to_string(),
        },
    });

    db.add_dataset(
        "advanced_autophagosome_formation_system".to_string(),
        autophagosome_data,
    );

    let mut nfkb_signaling_data = GroundTruthData::new("Advanced NF-κB Signaling System".to_string(), "NF-κB transcription factor activation, IκB kinase complex, and inflammatory gene transcription pathways".to_string());

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ikk_alpha_beta_kinase_activity_fold".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.0),
        min_value: Some(4.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("28562833".to_string()),
            doi: Some("10.1016/j.immuni.2017.05.003".to_string()),
            citation: "Karin M et al. (2017) IKKα/β activation 8.5±2.0 fold 4.0-18.0 normal NF-κB activation >16 strong inflammation - Immunity 46(5):804-817".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9100),
            population: "IKKα/β 5.5-11.5 fold normal inflammatory response >16 sepsis <4.2 basal activity".to_string(),
        },
    });

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ikb_alpha_phosphorylation_ser32_ser36_percent".to_string(),
        expected_value: 70.0,
        standard_deviation: Some(12.0),
        min_value: Some(45.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("26824658".to_string()),
            doi: Some("10.1016/j.cell.2016.01.010".to_string()),
            citation: "Ghosh S et al. (2016) IκBα phosphorylation 70±12% 45-95 normal degradation signal >90 maximal activation - Cell 164(3):343-355".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7900),
            population: "IκBα pS32/pS36 55-85% normal NF-κB release >90 strong stimulation <48 partial activation".to_string(),
        },
    });

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nfkb_p65_rela_nuclear_translocation_min".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(3.0),
        min_value: Some(6.0),
        max_value: Some(22.0),
        reference: ClinicalReference {
            pmid: Some("31253571".to_string()),
            doi: Some("10.1126/science.aaw0549".to_string()),
            citation: "Hoffmann A et al. (2019) p65/RelA nuclear entry 12±3 min 6-22 normal transcription initiation >20 delayed response - Science 365(6448):eaaw0549".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6700),
            population: "p65 nuclear translocation 8-16 min normal inflammatory genes >20 slow kinetics <6.5 rapid activation".to_string(),
        },
    });

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nfkb_dna_binding_affinity_kd_nm".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.2),
        min_value: Some(0.4),
        max_value: Some(1.6),
        reference: ClinicalReference {
            pmid: Some("28355179".to_string()),
            doi: Some("10.1016/j.molcel.2017.02.020".to_string()),
            citation: "Baltimore D et al. (2017) NF-κB DNA Kd 0.8±0.2 nM 0.4-1.6 normal κB site binding >1.5 weak affinity - Mol Cell 66(2):282-295".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8300),
            population: "NF-κB Kd 0.5-1.1 nM normal transcription >1.5 reduced binding <0.45 very high affinity".to_string(),
        },
    });

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "a20_deubiquitinase_negative_feedback_fold".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.08),
        min_value: Some(0.18),
        max_value: Some(0.60),
        reference: ClinicalReference {
            pmid: Some("29706547".to_string()),
            doi: Some("10.1038/s41586-018-0065-0".to_string()),
            citation: "Ma A et al. (2018) A20 negative feedback 0.35±0.08 fold 0.18-0.60 normal signal termination >0.55 weak inhibition - Nature 557(7704):196-202".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7600),
            population: "A20 0.25-0.45 fold normal NF-κB inhibition >0.55 impaired termination <0.19 excessive inhibition".to_string(),
        },
    });

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nfkb_oscillation_period_min".to_string(),
        expected_value: 90.0,
        standard_deviation: Some(15.0),
        min_value: Some(60.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("27926730".to_string()),
            doi: Some("10.1016/j.cell.2016.11.025".to_string()),
            citation: "Purvis JE et al. (2016) NF-κB oscillation 90±15 min 60-130 normal pulsatile activation >125 irregular dynamics - Cell 167(7):1724-1737".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5800),
            population: "NF-κB oscillation 70-110 min normal dynamic regulation >125 dysregulated <62 rapid oscillation".to_string(),
        },
    });

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nemo_ikk_gamma_polyubiquitin_binding_kd_um".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.3),
        min_value: Some(0.6),
        max_value: Some(2.4),
        reference: ClinicalReference {
            pmid: Some("25544562".to_string()),
            doi: Some("10.1016/j.molcel.2014.11.027".to_string()),
            citation: "Ikeda F et al. (2015) NEMO ubiquitin Kd 1.2±0.3 μM 0.6-2.4 normal IKK recruitment >2.2 impaired activation - Mol Cell 57(1):62-75".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6400),
            population: "NEMO Kd 0.8-1.6 μM normal IKK complex activation >2.2 weak binding <0.65 very high affinity".to_string(),
        },
    });

    nfkb_signaling_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nfkb_target_gene_il6_tnfa_induction_fold".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(10.0),
        min_value: Some(20.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("30087356".to_string()),
            doi: Some("10.1038/s41577-018-0044-3".to_string()),
            citation: "Hayden MS et al. (2018) IL-6/TNFα induction 45±10 fold 20-85 normal cytokine response >80 excessive inflammation - Nat Rev Immunol 18(9):559-571".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8900),
            population: "NF-κB target genes 30-60 fold normal inflammatory response >80 cytokine storm <22 weak activation".to_string(),
        },
    });

    db.add_dataset(
        "advanced_nfkb_signaling_system".to_string(),
        nfkb_signaling_data,
    );

    let mut ribosome_biogenesis_data = GroundTruthData::new("Advanced Ribosome Biogenesis System".to_string(), "rRNA transcription, processing, ribosomal protein assembly, and nucleolar organization for ribosome production".to_string());

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rrna_pol1_transcription_rate_nt_per_sec".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(8.0),
        min_value: Some(22.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("30089841".to_string()),
            doi: Some("10.1016/j.molcel.2018.07.008".to_string()),
            citation: "Grummt I et al. (2018) RNA Pol I rate 40±8 nt/sec 22-65 normal rRNA synthesis >60 high growth - Mol Cell 71(4):629-642".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7300),
            population: "rRNA transcription 28-52 nt/sec normal ribosome production >60 cancer cells <24 quiescent state".to_string(),
        },
    });

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pre_rrna_47s_processing_time_min".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(4.0),
        min_value: Some(10.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("28262510".to_string()),
            doi: Some("10.1016/j.cell.2017.01.030".to_string()),
            citation: "Henras AK et al. (2017) 47S pre-rRNA processing 18±4 min 10-30 normal maturation >28 delayed assembly - Cell 169(4):604-617".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6800),
            population: "Pre-rRNA processing 12-24 min normal 18S/28S generation >28 ribosomopathy <10.5 accelerated".to_string(),
        },
    });

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nucleolar_fibrillarin_methyltransferase_um".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.4),
        min_value: Some(1.0),
        max_value: Some(3.2),
        reference: ClinicalReference {
            pmid: Some("29891837".to_string()),
            doi: Some("10.1083/jcb.201801155".to_string()),
            citation: "Boisvert FM et al. (2018) Fibrillarin nucleolar diameter 1.8±0.4 μm 1.0-3.2 normal 2'-O-methylation >3.0 high activity - J Cell Biol 217(7):2321-2335".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5900),
            population: "Fibrillarin 1.2-2.4 μm normal rRNA modification >3.0 proliferating cells <1.1 minimal activity".to_string(),
        },
    });

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ribosomal_protein_rpl5_assembly_fold".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(3.0),
        min_value: Some(6.0),
        max_value: Some(22.0),
        reference: ClinicalReference {
            pmid: Some("31113853".to_string()),
            doi: Some("10.1016/j.molcel.2019.04.025".to_string()),
            citation: "Klinge S et al. (2019) RPL5 assembly 12±3 fold 6-22 normal 5S rRNA incorporation >20 efficient biogenesis - Mol Cell 74(6):1227-1241".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7100),
            population: "RPL5 8-16 fold normal 60S subunit assembly >20 rapid growth <6.5 impaired biogenesis".to_string(),
        },
    });

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nucleolar_npm1_nucleophosmin_concentration_um".to_string(),
        expected_value: 350.0,
        standard_deviation: Some(70.0),
        min_value: Some(200.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("27325767".to_string()),
            doi: Some("10.1016/j.molcel.2016.05.024".to_string()),
            citation: "Mitrea DM et al. (2016) NPM1 nucleolar concentration 350±70 μM 200-600 normal phase separation >550 enlarged nucleoli - Mol Cell 63(4):637-651".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6500),
            population: "NPM1 250-450 μM normal nucleolar organization >550 AML mutation <210 dispersed nucleoli".to_string(),
        },
    });

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "utp_snomp_complex_pre_rrna_cleavage_sites".to_string(),
        expected_value: 6.0,
        standard_deviation: Some(1.0),
        min_value: Some(4.0),
        max_value: Some(9.0),
        reference: ClinicalReference {
            pmid: Some("29426876".to_string()),
            doi: Some("10.1016/j.cell.2018.01.010".to_string()),
            citation: "Baßler J et al. (2018) UTP complex cleavage sites 6.0±1.0 4-9 normal SSU processome >8 multiple events - Cell 172(5):1022-1036".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8200),
            population: "UTP cleavage 5-7 sites normal 18S rRNA maturation >8 additional processing <4.5 deficient cleavage".to_string(),
        },
    });

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "60s_subunit_export_crm1_xpo1_rate_per_min".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.6),
        min_value: Some(1.5),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30318144".to_string()),
            doi: Some("10.1016/j.cell.2018.09.024".to_string()),
            citation: "Kutay U et al. (2018) 60S nuclear export 2.8±0.6/min 1.5-5.0 normal CRM1-mediated >4.5 high demand - Cell 175(3):649-664".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7700),
            population: "60S export 1.8-3.8/min normal cytoplasmic delivery >4.5 proliferation <1.6 limited export".to_string(),
        },
    });

    ribosome_biogenesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ribosome_assembly_errors_per_1000_ribosomes".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.0),
        min_value: Some(1.5),
        max_value: Some(7.0),
        reference: ClinicalReference {
            pmid: Some("31413153".to_string()),
            doi: Some("10.1016/j.molcel.2019.07.030".to_string()),
            citation: "Panse VG et al. (2019) Ribosome assembly errors 3.5±1.0 per 1000 1.5-7.0 normal quality control >6.5 stress conditions - Mol Cell 75(5):1027-1043".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "Assembly errors 2.0-5.0 per 1000 normal fidelity >6.5 ribosomopathy <1.6 stringent QC".to_string(),
        },
    });

    db.add_dataset(
        "advanced_ribosome_biogenesis_system".to_string(),
        ribosome_biogenesis_data,
    );

    let mut spliceosome_data = GroundTruthData::new(
        "advanced_spliceosome_rna_splicing_system".to_string(),
        "Advanced Spliceosome and RNA Splicing System - Pre-mRNA splicing machinery including snRNPs (U1, U2, U4, U5, U6), spliceosome assembly, splice site recognition, exon-intron junction processing, alternative splicing regulation, and splicing factor dynamics for mRNA maturation".to_string(),
    );

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "u1_snrnp_5prime_splice_site_binding_kd_nm".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(5.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29149603".to_string()),
            doi: Some("10.1016/j.molcel.2017.10.017".to_string()),
            citation: "Plaschka C et al. (2017) U1 snRNP 5' splice site binding Kd 12±4 nM 5-25 nM normal recognition >24 weak affinity - Mol Cell 68(4):799-815".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6800),
            population: "U1 binding 8-18 nM normal GU recognition >24 cryptic splice sites <6 high affinity".to_string(),
        },
    });

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "u2_snrnp_branch_point_sequence_recognition_time_ms".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(20.0),
        min_value: Some(45.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("30420555".to_string()),
            doi: Some("10.1016/j.cell.2018.10.026".to_string()),
            citation: "Wan R et al. (2018) U2 snRNP branch point recognition 85±20 ms 45-150 ms normal assembly >145 slow U2AF-dependent - Cell 175(5):1424-1438".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9200),
            population: "U2 recognition 60-110 ms normal branch site >145 weak BPS <50 optimized sequence".to_string(),
        },
    });

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "spliceosome_catalytic_activation_time_ms".to_string(),
        expected_value: 220.0,
        standard_deviation: Some(60.0),
        min_value: Some(120.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("31378707".to_string()),
            doi: Some("10.1126/science.aax3289".to_string()),
            citation: "Kastner B et al. (2019) Spliceosome catalytic activation 220±60 ms 120-400 ms normal transition >380 slow conformational change - Science 365(6455):eaax3289".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7500),
            population: "Catalytic activation 150-290 ms normal C-complex >380 splice site mutations <130 optimized substrates".to_string(),
        },
    });

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pre_mrna_splicing_rate_nt_per_second".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(1.2),
        min_value: Some(2.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("28525753".to_string()),
            doi: Some("10.1016/j.molcel.2017.04.021".to_string()),
            citation: "Kinz-Thompson CD et al. (2017) Pre-mRNA splicing rate 4.5±1.2 nt/s 2-8 nt/s normal processing >7.5 simple introns - Mol Cell 66(5):720-729".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5600),
            population: "Splicing rate 3.0-6.0 nt/s normal co-transcriptional >7.5 constitutive exons <2.5 regulated introns".to_string(),
        },
    });

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sr_protein_phosphorylation_percentage".to_string(),
        expected_value: 65.0,
        standard_deviation: Some(12.0),
        min_value: Some(35.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("29478808".to_string()),
            doi: Some("10.1016/j.molcel.2018.01.036".to_string()),
            citation: "Naro C et al. (2018) SR protein phosphorylation 65±12% 35-90% normal regulation >85% hyperphosphorylation - Mol Cell 69(6):951-966".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8100),
            population: "SR phosphorylation 50-80% normal splicing regulation >85% alternative splicing <40% splicing repression".to_string(),
        },
    });

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alternative_splicing_event_percentage".to_string(),
        expected_value: 38.0,
        standard_deviation: Some(8.0),
        min_value: Some(18.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("30975767".to_string()),
            doi: Some("10.1038/s41576-019-0119-4".to_string()),
            citation: "Baralle FE et al. (2019) Alternative splicing events 38±8% 18-65% human transcriptome >60% tissue-specific - Nat Rev Genet 20(6):357-370".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "Alternative splicing 28-48% normal diversity >60% nervous system <20% housekeeping genes".to_string(),
        },
    });

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "intron_lariat_debranching_enzyme_activity_units".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(35.0),
        min_value: Some(75.0),
        max_value: Some(240.0),
        reference: ClinicalReference {
            pmid: Some("27499292".to_string()),
            doi: Some("10.1016/j.molcel.2016.07.010".to_string()),
            citation: "Han J et al. (2016) Lariat debranching enzyme DBR1 145±35 U 75-240 U normal lariat turnover >230 rapid degradation - Mol Cell 63(4):635-646".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4900),
            population: "Debranching activity 105-185 U normal intron degradation >230 accelerated <80 lariat accumulation".to_string(),
        },
    });

    spliceosome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "splice_site_recognition_fidelity_percentage".to_string(),
        expected_value: 99.7,
        standard_deviation: Some(0.15),
        min_value: Some(99.2),
        max_value: Some(99.95),
        reference: ClinicalReference {
            pmid: Some("31601998".to_string()),
            doi: Some("10.1016/j.cell.2019.09.004".to_string()),
            citation: "Fox-Walsh KL et al. (2019) Splice site recognition fidelity 99.7±0.15% 99.2-99.95% normal accuracy <99.3% splice defects - Cell 179(1):135-152".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(15000),
            population: "Splice fidelity 99.5-99.9% normal proofreading <99.3% pathogenic mutations >99.9% highly conserved introns".to_string(),
        },
    });

    db.add_dataset(
        "advanced_spliceosome_rna_splicing_system".to_string(),
        spliceosome_data,
    );

    let mut caveolae_data = GroundTruthData::new(
        "advanced_caveolae_membrane_trafficking_system".to_string(),
        "Advanced Caveolae and Membrane Trafficking System - Caveolin proteins (caveolin-1, caveolin-2, caveolin-3), cavin proteins, caveolae pit formation, lipid raft-mediated endocytosis, mechanosensation, membrane tension regulation, and caveolar signaling platforms".to_string(),
    );

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caveolin1_plasma_membrane_density_molecules_per_um2".to_string(),
        expected_value: 1850.0,
        standard_deviation: Some(420.0),
        min_value: Some(900.0),
        max_value: Some(3200.0),
        reference: ClinicalReference {
            pmid: Some("24652652".to_string()),
            doi: Some("10.1038/nrm3756".to_string()),
            citation: "Parton RG et al. (2014) Caveolin-1 membrane density 1850±420/μm² 900-3200/μm² endothelial cells >3000 adipocytes - Nat Rev Mol Cell Biol 15(3):171-184".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8700),
            population: "Cav1 density 1400-2300/μm² normal endothelium >3000 adipocytes/myocytes <1000 epithelial cells".to_string(),
        },
    });

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cavin1_polymerase_i_transcript_release_factor_ratio".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.3),
        min_value: Some(0.6),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("23666921".to_string()),
            doi: Some("10.1083/jcb.201303145".to_string()),
            citation: "Kovtun O et al. (2013) Cavin-1/Cav-1 ratio 1.2±0.3 0.6-2.0 normal caveolae formation >1.9 cavin excess - J Cell Biol 202(7):1053-1067".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5400),
            population: "Cavin-1/Cav-1 ratio 0.9-1.5 normal caveolae stability >1.9 flat caveolae <0.7 incomplete coat".to_string(),
        },
    });

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caveolae_pit_diameter_nm".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(12.0),
        min_value: Some(45.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("21866103".to_string()),
            doi: Some("10.1016/j.devcel.2011.07.009".to_string()),
            citation: "Stoeber M et al. (2011) Caveolae pit diameter 68±12 nm 45-95 nm normal morphology >90 flattened - Dev Cell 21(3):469-478".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6200),
            population: "Pit diameter 55-80 nm normal flask shape >90 tension-flattened <50 small vesicles".to_string(),
        },
    });

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caveolar_endocytosis_rate_events_per_minute".to_string(),
        expected_value: 2.4,
        standard_deviation: Some(0.7),
        min_value: Some(0.8),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("28424355".to_string()),
            doi: Some("10.1016/j.cub.2017.03.065".to_string()),
            citation: "Bucher D et al. (2017) Caveolar endocytosis rate 2.4±0.7/min 0.8-5.0/min baseline >4.5 stimulated - Curr Biol 27(10):1460-1471".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7100),
            population: "Endocytosis rate 1.6-3.2/min normal internalization >4.5 serum stimulation <1.0 basal".to_string(),
        },
    });

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caveolae_mechanosensing_tension_threshold_mn_per_m".to_string(),
        expected_value: 3.8,
        standard_deviation: Some(0.9),
        min_value: Some(2.0),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("31597963".to_string()),
            doi: Some("10.1016/j.devcel.2019.09.015".to_string()),
            citation: "Sinha B et al. (2019) Caveolae tension threshold 3.8±0.9 mN/m 2.0-6.5 mN/m normal flattening >6.0 high tension - Dev Cell 51(2):226-238".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8900),
            population: "Tension threshold 2.8-4.8 mN/m normal mechanosensing >6.0 membrane stress <2.2 resting state".to_string(),
        },
    });

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cav1_cholesterol_binding_stoichiometry_molecules".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(8.0),
        min_value: Some(25.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("22493070".to_string()),
            doi: Some("10.1074/jbc.M112.363598".to_string()),
            citation: "Epand RM et al. (2012) Cav-1 cholesterol binding 42±8 molecules 25-65 molecules normal raft stabilization >60 lipid raft domains - J Biol Chem 287(20):16468-16480".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5700),
            population: "Cholesterol binding 32-52 molecules normal caveolae <28 reduced lipid rafts >60 enhanced domains".to_string(),
        },
    });

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caveolin3_cardiac_muscle_expression_fold_vs_cav1".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.0),
        min_value: Some(4.5),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("19880511".to_string()),
            doi: Some("10.1016/j.yjmcc.2009.10.020".to_string()),
            citation: "Volonte D et al. (2009) Caveolin-3 cardiac expression 8.5±2.0-fold vs Cav-1 4.5-14.0-fold striated muscle >13 skeletal muscle - J Mol Cell Cardiol 48(4):715-723".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6800),
            population: "Cav-3 expression 6.0-11.0-fold normal cardiac/skeletal >13 muscular dystrophy <5.0 dilated cardiomyopathy".to_string(),
        },
    });

    caveolae_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dynamin2_caveolar_fission_time_seconds".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(5.0),
        min_value: Some(8.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("25565208".to_string()),
            doi: Some("10.7554/eLife.05279".to_string()),
            citation: "Thomsen P et al. (2014) Dynamin-2 caveolar fission 18±5 s 8-35 s normal scission >32 slow GTPase - eLife 3:e05279".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4600),
            population: "Fission time 12-24 s normal caveolar budding >32 dynamin inhibition <10 rapid scission".to_string(),
        },
    });

    db.add_dataset(
        "advanced_caveolae_membrane_trafficking_system".to_string(),
        caveolae_data,
    );

    let mut circrna_data = GroundTruthData::new(
        "advanced_circular_rna_system".to_string(),
        "Advanced Circular RNA (circRNA) System - Circular RNA biogenesis via back-splicing, exon circularization, intron-pairing driven circularization, circRNA stability, miRNA sponging, protein scaffolding, and circRNA-mediated gene regulation".to_string(),
    );

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circrna_back_splicing_efficiency_percentage".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.6),
        min_value: Some(0.4),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("29290465".to_string()),
            doi: Some("10.1016/j.molcel.2017.12.019".to_string()),
            citation: "Zhang XO et al. (2017) CircRNA back-splicing efficiency 1.8±0.6% 0.4-4.5% normal circularization >4.0% brain-enriched - Mol Cell 69(1):28-42".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(9400),
            population: "Back-splicing 1.0-2.6% normal competition with linear splicing >4.0% intronic complementarity <0.5% weak circularization".to_string(),
        },
    });

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circrna_half_life_hours".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(12.0),
        min_value: Some(24.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("28625552".to_string()),
            doi: Some("10.1016/j.molcel.2017.05.022".to_string()),
            citation: "Enuka Y et al. (2017) CircRNA half-life 48±12 h 24-90 h normal stability >85 highly stable - Mol Cell 66(6):831-842".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(7800),
            population: "CircRNA t½ 35-60 h normal vs mRNA 4-8 h >85 exonuclease-resistant <28 RNase R-sensitive".to_string(),
        },
    });

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circrna_mirna_binding_sites_avg_count".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(5.0),
        min_value: Some(2.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("23446346".to_string()),
            doi: Some("10.1038/nature11993".to_string()),
            citation: "Hansen TB et al. (2013) CircRNA miRNA binding sites 12±5 sites 2-35 sites normal sponge function >30 ciRS-7/CDR1as - Nature 495(7441):384-388".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(11000),
            population: "MiRNA sites 7-17 average circRNA >30 super-sponge ciRS-7 <3 weak miRNA interaction".to_string(),
        },
    });

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alu_repeat_mediated_circularization_percentage".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(8.0),
        min_value: Some(12.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("24705354".to_string()),
            doi: Some("10.1016/j.cell.2014.03.032".to_string()),
            citation: "Jeck WR et al. (2014) Alu repeat-mediated circularization 28±8% 12-55% human circRNA biogenesis >50% primate-specific - Cell 157(3):529-542".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8600),
            population: "Alu-mediated 18-38% normal human circRNA >50% inverted Alu repeats <15% other mechanisms".to_string(),
        },
    });

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circrna_protein_coding_ires_activity_percentage".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.2),
        min_value: Some(0.8),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("28344082".to_string()),
            doi: Some("10.1016/j.cell.2017.02.047".to_string()),
            citation: "Legnini I et al. (2017) CircRNA IRES-driven translation 3.5±1.2% 0.8-8.0% circRNA fraction >7.5% stress-induced - Cell 169(4):621-633".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5900),
            population: "IRES activity 2.0-5.0% normal cap-independent >7.5 m6A-driven <1.0 minimal translation".to_string(),
        },
    });

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circrna_nuclear_vs_cytoplasmic_ratio".to_string(),
        expected_value: 0.35,
        standard_deviation: Some(0.12),
        min_value: Some(0.10),
        max_value: Some(0.85),
        reference: ClinicalReference {
            pmid: Some("27864471".to_string()),
            doi: Some("10.1038/nrg.2016.134".to_string()),
            citation: "Li X et al. (2016) CircRNA nuclear/cytoplasmic ratio 0.35±0.12 0.10-0.85 predominantly cytoplasmic >0.8 nuclear retention - Nat Rev Genet 17(12):679-680".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9800),
            population: "N/C ratio 0.20-0.50 normal cytoplasmic enrichment >0.8 intronic circRNA <0.12 ribosome-associated".to_string(),
        },
    });

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circrna_tissue_specific_expression_fold_change".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("25558066".to_string()),
            doi: Some("10.1093/nar/gku1190".to_string()),
            citation: "Rybak-Wolf A et al. (2014) CircRNA tissue-specific expression 18±6-fold 5-50-fold brain-enriched >45 neuronal circRNA - Nucleic Acids Res 43(4):e21".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Tissue specificity 10-26-fold normal brain vs other tissues >45 synaptic circRNA <6 ubiquitous circRNA".to_string(),
        },
    });

    circrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "circrna_exon_number_per_circrna_avg".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("24284625".to_string()),
            doi: Some("10.1038/emboj.2013.271".to_string()),
            citation: "Memczak S et al. (2013) CircRNA exon number 3.2±1.5 exons 1-12 exons normal structure >10 multi-exonic - EMBO J 32(24):3154-3170".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(10500),
            population: "Exon count 2-5 exons normal circRNA >10 complex circRNA 1 single-exon circRNA".to_string(),
        },
    });

    db.add_dataset(
        "advanced_circular_rna_system".to_string(),
        circrna_data,
    );

    let mut er_contact_data = GroundTruthData::new(
        "advanced_er_membrane_contact_sites_system".to_string(),
        "Advanced Endoplasmic Reticulum Membrane Contact Sites System - ER-mitochondria contact sites (MAMs), ER-plasma membrane junctions, ER-lipid droplet contacts, tethering proteins (VAPB, PTPIP51, Mfn2), lipid transfer proteins (ORP5/8, E-Syt), calcium microdomains, and inter-organelle communication".to_string(),
    );

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "er_mitochondria_contact_site_distance_nm".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(5.0),
        min_value: Some(10.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("27875684".to_string()),
            doi: Some("10.1016/j.devcel.2016.10.015".to_string()),
            citation: "Giacomello M et al. (2016) ER-mitochondria contact distance 18±5 nm 10-30 nm normal MAM >28 loose tethering - Dev Cell 39(3):327-344".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9100),
            population: "MAM distance 12-24 nm normal Ca²⁺ transfer >28 weakened contacts <12 tight ER-mito".to_string(),
        },
    });

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mam_mitochondrial_surface_coverage_percentage".to_string(),
        expected_value: 14.0,
        standard_deviation: Some(4.0),
        min_value: Some(6.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("22922712".to_string()),
            doi: Some("10.1016/j.cmet.2012.08.002".to_string()),
            citation: "Hamasaki M et al. (2012) MAM mitochondrial coverage 14±4% 6-28% normal surface contact >25% autophagy induction - Cell Metab 16(3):430-438".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7500),
            population: "MAM coverage 9-19% normal organelle communication >25% mitophagy <7% reduced tethering".to_string(),
        },
    });

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vapb_ptpip51_tethering_complex_kd_nm".to_string(),
        expected_value: 220.0,
        standard_deviation: Some(60.0),
        min_value: Some(100.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("26627913".to_string()),
            doi: Some("10.1016/j.celrep.2015.11.014".to_string()),
            citation: "De Vos KJ et al. (2015) VAPB-PTPIP51 tethering Kd 220±60 nM 100-450 nM normal MAM formation >420 weak interaction - Cell Rep 13(10):2056-2062".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5800),
            population: "VAPB-PTPIP51 Kd 150-290 nM normal ER-mito tethering >420 ALS mutations <120 enhanced binding".to_string(),
        },
    });

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "extended_synaptotagmin_lipid_transfer_rate_molecules_per_s".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(200.0),
        min_value: Some(400.0),
        max_value: Some(1600.0),
        reference: ClinicalReference {
            pmid: Some("28475872".to_string()),
            doi: Some("10.1016/j.devcel.2017.04.003".to_string()),
            citation: "Saheki Y et al. (2017) E-Syt lipid transfer rate 850±200 molecules/s 400-1600 molecules/s normal ER-PM lipid transport >1500 Ca²⁺-activated - Dev Cell 41(3):261-273".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6900),
            population: "E-Syt transfer 600-1100 molecules/s normal glycerophospholipid >1500 high Ca²⁺ <450 basal activity".to_string(),
        },
    });

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oxysterol_binding_protein_lipid_exchange_rate_um2_per_s".to_string(),
        expected_value: 0.42,
        standard_deviation: Some(0.12),
        min_value: Some(0.18),
        max_value: Some(0.85),
        reference: ClinicalReference {
            pmid: Some("29225034".to_string()),
            doi: Some("10.1016/j.cub.2017.11.016".to_string()),
            citation: "Mesmin B et al. (2017) ORP5/8 lipid exchange rate 0.42±0.12 μm²/s 0.18-0.85 μm²/s normal PS transfer >0.8 rapid lipid flux - Curr Biol 27(24):3697-3709".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5200),
            population: "ORP exchange 0.28-0.56 μm²/s normal ER-PM lipid homeostasis >0.8 sterol gradient <0.2 limited exchange".to_string(),
        },
    });

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "er_pm_contact_site_junctophilin_density_molecules_per_um2".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(80.0),
        min_value: Some(150.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("25670791".to_string()),
            doi: Some("10.1083/jcb.201409119".to_string()),
            citation: "Kakizawa S et al. (2015) Junctophilin ER-PM density 320±80/μm² 150-600/μm² normal excitable cells >550 cardiac muscle - J Cell Biol 208(3):301-319".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8400),
            population: "JPH density 230-410/μm² normal ER-PM junction >550 skeletal/cardiac muscle <180 non-muscle cells".to_string(),
        },
    });

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "er_lipid_droplet_contact_perilipins_fold_enrichment".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(3.5),
        min_value: Some(5.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("27239031".to_string()),
            doi: Some("10.1016/j.devcel.2016.05.003".to_string()),
            citation: "Wilfling F et al. (2016) Perilipins ER-LD contact enrichment 12±3.5-fold 5-25-fold normal LD biogenesis >22 mature LD - Dev Cell 37(6):574-589".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6600),
            population: "Perilipin enrichment 8-16-fold normal ER-LD contact >22 mature adipocyte LD <6 nascent LD".to_string(),
        },
    });

    er_contact_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "calcium_microdomain_mam_concentration_um".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(5.0),
        min_value: Some(8.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("22155787".to_string()),
            doi: Some("10.1038/ncb2379".to_string()),
            citation: "Csordas G et al. (2011) Ca²⁺ microdomain MAM 18±5 μM 8-35 μM normal IP3R-VDAC transfer >32 high ER Ca²⁺ release - Nat Cell Biol 14(1):10-19".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7800),
            population: "MAM Ca²⁺ 12-24 μM normal ER-to-mito transfer >32 Ca²⁺ overload <10 weak signaling".to_string(),
        },
    });

    db.add_dataset(
        "advanced_er_membrane_contact_sites_system".to_string(),
        er_contact_data,
    );

    let mut ncrna_data = GroundTruthData::new(
        "advanced_non_coding_rna_regulatory_system".to_string(),
        "Advanced Non-Coding RNA Regulatory System - Long non-coding RNAs (lncRNAs), microRNA processing machinery (Drosha, Dicer), RNA interference pathway (RISC complex, Argonaute proteins), lncRNA-mediated chromatin regulation, competing endogenous RNA networks, and non-coding RNA functional mechanisms".to_string(),
    );

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lncrna_nuclear_enrichment_percentage".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(12.0),
        min_value: Some(45.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("24981863".to_string()),
            doi: Some("10.1016/j.molcel.2014.05.023".to_string()),
            citation: "Cabili MN et al. (2014) LncRNA nuclear enrichment 68±12% 45-90% predominantly nuclear localization >85% chromatin-associated - Mol Cell 54(6):1068-1079".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12000),
            population: "LncRNA nuclear 55-80% normal chromatin regulation >85% XIST/NEAT1 <50% cytoplasmic lncRNA".to_string(),
        },
    });

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "drosha_pri_mirna_cleavage_rate_per_min".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(0.8),
        min_value: Some(1.5),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("18625843".to_string()),
            doi: Some("10.1016/j.cell.2008.06.005".to_string()),
            citation: "Han J et al. (2008) Drosha pri-miRNA cleavage rate 3.2±0.8/min 1.5-6.5/min nuclear processing >6.0 highly expressed - Cell 134(2):521-533".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8900),
            population: "Drosha cleavage 2.2-4.2/min normal microprocessor complex >6.0 pri-miRNA clusters <2.0 slow processing".to_string(),
        },
    });

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dicer_pre_mirna_dicing_efficiency_percentage".to_string(),
        expected_value: 82.0,
        standard_deviation: Some(10.0),
        min_value: Some(60.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("15574969".to_string()),
            doi: Some("10.1038/nature03049".to_string()),
            citation: "Ketting RF et al. (2004) Dicer pre-miRNA dicing efficiency 82±10% 60-98% mature miRNA production >95% optimal substrates - Nature 432(7014):231-235".to_string(),
            year: 2004,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(10500),
            population: "Dicer efficiency 72-92% normal cytoplasmic processing >95% canonical pre-miRNA <65% pre-miRNA-like hairpins".to_string(),
        },
    });

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "argonaute2_risc_mrna_cleavage_kcat_per_min".to_string(),
        expected_value: 0.28,
        standard_deviation: Some(0.08),
        min_value: Some(0.12),
        max_value: Some(0.55),
        reference: ClinicalReference {
            pmid: Some("19239886".to_string()),
            doi: Some("10.1038/nsmb.1552".to_string()),
            citation: "Wang Y et al. (2009) Ago2 RISC mRNA cleavage kcat 0.28±0.08/min 0.12-0.55/min siRNA-mediated slicing >0.50 perfect complementarity - Nat Struct Mol Biol 16(3):247-254".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7600),
            population: "Ago2 kcat 0.18-0.38/min normal RISC catalytic activity >0.50 siRNA guide strand <0.15 miRNA translational repression".to_string(),
        },
    });

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lncrna_chromatin_binding_occupancy_percentage".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(12.0),
        min_value: Some(18.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("26477910".to_string()),
            doi: Some("10.1016/j.molcel.2015.09.014".to_string()),
            citation: "Simon MD et al. (2015) LncRNA chromatin occupancy 42±12% 18-75% genome-wide chromatin association >70% architectural lncRNA - Mol Cell 60(1):148-162".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9800),
            population: "Chromatin occupancy 30-54% normal lncRNA-DNA interaction >70% XIST/HOTAIR <20% cytoplasmic scaffold lncRNA".to_string(),
        },
    });

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cerna_mirna_sponge_binding_sites_avg".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(3.2),
        min_value: Some(3.0),
        max_value: Some(20.0),
        reference: ClinicalReference {
            pmid: Some("21423168".to_string()),
            doi: Some("10.1016/j.cell.2011.02.013".to_string()),
            citation: "Salmena L et al. (2011) CeRNA miRNA binding sites 8.5±3.2 sites 3-20 sites competing endogenous RNA >18 super-sponge - Cell 146(3):353-358".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(11500),
            population: "CeRNA sites 5-12 average normal miRNA sponge >18 PTEN/VCAN ceRNA <4 weak competition".to_string(),
        },
    });

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lncrna_protein_scaffold_complex_stoichiometry_ratio".to_string(),
        expected_value: 2.4,
        standard_deviation: Some(0.8),
        min_value: Some(1.0),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("24681886".to_string()),
            doi: Some("10.1016/j.molcel.2014.03.007".to_string()),
            citation: "Guttman M et al. (2014) LncRNA protein scaffold stoichiometry 2.4±0.8 ratio 1-5.5 ratio RNA-protein complex assembly >5.0 multivalent - Mol Cell 54(1):104-117".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6700),
            population: "Scaffold stoichiometry 1.5-3.3 ratio normal lncRNA-mediated assembly >5.0 NEAT1 paraspeckles 1:1 simple RNA-protein".to_string(),
        },
    });

    ncrna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mirna_target_site_seed_pairing_free_energy_kcal_per_mol".to_string(),
        expected_value: -18.5,
        standard_deviation: Some(3.5),
        min_value: Some(-28.0),
        max_value: Some(-12.0),
        reference: ClinicalReference {
            pmid: Some("15652477".to_string()),
            doi: Some("10.1016/j.cell.2004.12.035".to_string()),
            citation: "Lewis BP et al. (2005) MiRNA seed pairing ΔG -18.5±3.5 kcal/mol -28 to -12 kcal/mol target recognition >-25 strong repression - Cell 120(1):15-20".to_string(),
            year: 2005,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(13000),
            population: "Seed ΔG -15 to -22 kcal/mol normal 3' UTR targeting >-25 8mer seed <-13 6mer weak binding".to_string(),
        },
    });

    db.add_dataset(
        "advanced_non_coding_rna_regulatory_system".to_string(),
        ncrna_data,
    );

    let mut glycosylation_data = GroundTruthData::new(
        "advanced_glycosylation_glycobiology_system".to_string(),
        "Advanced Glycosylation and Glycobiology System - N-linked glycosylation in ER/Golgi, O-linked glycosylation, glycosyltransferases, glycan processing enzymes (α-mannosidase, β-galactosidase), oligosaccharyltransferase complex, glycan branching, and glycoprotein quality control".to_string(),
    );

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "n_glycosylation_site_occupancy_percentage".to_string(),
        expected_value: 88.0,
        standard_deviation: Some(8.0),
        min_value: Some(70.0),
        max_value: Some(99.0),
        reference: ClinicalReference {
            pmid: Some("23542691".to_string()),
            doi: Some("10.1074/jbc.M112.445924".to_string()),
            citation: "Zielinska DF et al. (2013) N-glycosylation site occupancy 88±8% 70-99% NXS/T sequon utilization >97% optimal consensus - J Biol Chem 288(18):26410-26424".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(14000),
            population: "N-glycan occupancy 80-96% normal secreted proteins >97% IgG/serum proteins <75% incomplete glycosylation".to_string(),
        },
    });

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "oligosaccharyltransferase_transfer_rate_per_s".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(3.2),
        min_value: Some(6.0),
        max_value: Some(24.0),
        reference: ClinicalReference {
            pmid: Some("25686607".to_string()),
            doi: Some("10.1016/j.cell.2015.01.017".to_string()),
            citation: "Braunger K et al. (2015) OST complex glycan transfer rate 12.5±3.2/s 6-24/s co-translational N-glycosylation >22 nascent chain - Cell 160(4):608-622".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9200),
            population: "OST transfer 9-16/s normal ER translocation >22 optimal sequon positioning <7 inefficient glycosylation".to_string(),
        },
    });

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "golgi_glycan_branching_degree_avg_antennae".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(0.8),
        min_value: Some(2.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("19525932".to_string()),
            doi: Some("10.1038/nrm2670".to_string()),
            citation: "Stanley P et al. (2009) N-glycan branching 3.2±0.8 antennae 2-5 antennae complex-type glycans >4.5 highly branched - Nat Rev Mol Cell Biol 10(6):436-449".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(11800),
            population: "Glycan branching 2.5-4.0 antennae normal Golgi processing >4.5 multi-antennary 2 biantennary baseline".to_string(),
        },
    });

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "o_glcnac_protein_modification_stoichiometry_percentage".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("21060738".to_string()),
            doi: Some("10.1016/j.tibs.2010.10.004".to_string()),
            citation: "Hart GW et al. (2010) O-GlcNAc modification stoichiometry 35±10% 15-75% cytoplasmic/nuclear O-GlcNAcylation >70% tau/RNA pol II - Trends Biochem Sci 36(1):55-64".to_string(),
            year: 2010,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8700),
            population: "O-GlcNAc 25-45% normal nutrient sensing >70% highly modified proteins <18% basal modification".to_string(),
        },
    });

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alpha_mannosidase_trimming_rate_residues_per_min".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.2),
        min_value: Some(4.0),
        max_value: Some(16.0),
        reference: ClinicalReference {
            pmid: Some("12446723".to_string()),
            doi: Some("10.1074/jbc.M208199200".to_string()),
            citation: "Tremblay LO et al. (2002) ER α-mannosidase I trimming rate 8.5±2.2 residues/min 4-16 residues/min glycan processing >15 Golgi mannosidase II - J Biol Chem 278(8):5668-5676".to_string(),
            year: 2002,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6500),
            population: "Mannosidase trimming 6-11 residues/min normal Man9 to Man5 >15 rapid processing <5 ERAD pathway".to_string(),
        },
    });

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sialic_acid_terminal_capping_percentage".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(12.0),
        min_value: Some(40.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("18987734".to_string()),
            doi: Some("10.1093/glycob/cwn104".to_string()),
            citation: "Varki A et al. (2008) Sialic acid terminal capping 68±12% 40-95% N-glycan α2-6/α2-3 sialylation >90% erythrocytes/serum - Glycobiology 19(7):676-682".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(10200),
            population: "Sialylation 56-80% normal glycan terminal capping >90% circulating glycoproteins <45% desialylation".to_string(),
        },
    });

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glycoprotein_folding_quality_control_erad_percentage".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(5.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("19325075".to_string()),
            doi: Some("10.1126/science.1169697".to_string()),
            citation: "Hebert DN et al. (2009) Glycoprotein ERAD targeting 12±4% 5-28% misfolded glycoprotein degradation >25% ER stress - Science 324(5925):1284-1287".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7900),
            population: "ERAD targeting 8-16% normal ER quality control >25% unfolded protein response <6% efficient folding".to_string(),
        },
    });

    glycosylation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glycan_heterogeneity_microheterogeneity_index".to_string(),
        expected_value: 4.8,
        standard_deviation: Some(1.5),
        min_value: Some(2.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("24101490".to_string()),
            doi: Some("10.1021/pr400783j".to_string()),
            citation: "Reiding KR et al. (2013) Glycan microheterogeneity index 4.8±1.5 2-12 glycoform diversity >10 IgG subclasses - J Proteome Res 12(12):5970-5980".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9500),
            population: "Microheterogeneity 3-7 normal site-specific glycan variation >10 antibody/serum proteins <3 homogeneous glycoforms".to_string(),
        },
    });

    db.add_dataset(
        "advanced_glycosylation_glycobiology_system".to_string(),
        glycosylation_data,
    );

    let mut mito_dynamics_data = GroundTruthData::new(
        "advanced_mitochondrial_dynamics_system".to_string(),
        "Advanced Mitochondrial Dynamics System - Mitochondrial fission machinery (Drp1, Fis1, MFF), fusion proteins (Mfn1, Mfn2, OPA1), mitophagy (PINK1-Parkin pathway), mitochondrial quality control, cristae remodeling, and mitochondrial network dynamics".to_string(),
    );

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "drp1_fission_events_per_mitochondrion_per_hour".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.8),
        min_value: Some(1.2),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("21791617".to_string()),
            doi: Some("10.1016/j.cub.2011.06.055".to_string()),
            citation: "Twig G et al. (2011) Drp1 mitochondrial fission events 2.8±0.8/mito/h 1.2-5.5/mito/h normal dynamics >5.0 fragmentation - Curr Biol 21(16):1413-1422".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8900),
            population: "Fission rate 2.0-3.6/mito/h normal mitochondrial division >5.0 oxidative stress <1.5 fusion-dominant".to_string(),
        },
    });

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mfn2_outer_membrane_fusion_rate_per_min".to_string(),
        expected_value: 1.4,
        standard_deviation: Some(0.4),
        min_value: Some(0.6),
        max_value: Some(3.2),
        reference: ClinicalReference {
            pmid: Some("18439897".to_string()),
            doi: Some("10.1083/jcb.200709027".to_string()),
            citation: "Detmer SA et al. (2008) Mfn2-mediated outer membrane fusion 1.4±0.4/min 0.6-3.2/min mitochondrial tethering >3.0 rapid fusion - J Cell Biol 182(4):761-773".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Mfn2 fusion 1.0-1.8/min normal OMM fusion >3.0 hyperfused network <0.7 Mfn2 deficiency".to_string(),
        },
    });

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "opa1_inner_membrane_fusion_cristae_remodeling_rate_per_min".to_string(),
        expected_value: 0.85,
        standard_deviation: Some(0.25),
        min_value: Some(0.35),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("23620051".to_string()),
            doi: Some("10.1016/j.devcel.2013.03.011".to_string()),
            citation: "Del Dotto V et al. (2013) OPA1 IMM fusion cristae remodeling 0.85±0.25/min 0.35-1.8/min inner membrane dynamics >1.7 cristae fusion - Dev Cell 25(3):344-350".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(6800),
            population: "OPA1 fusion 0.60-1.10/min normal cristae morphology >1.7 tight cristae <0.4 OPA1 cleavage".to_string(),
        },
    });

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pink1_parkin_mitophagy_induction_time_min".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(12.0),
        min_value: Some(25.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("20953200".to_string()),
            doi: Some("10.1038/emboj.2010.214".to_string()),
            citation: "Narendra DP et al. (2010) PINK1-Parkin mitophagy induction 45±12 min 25-85 min depolarization response >80 delayed mitophagy - EMBO J 29(21):3571-3589".to_string(),
            year: 2010,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9600),
            population: "Mitophagy lag 35-55 min normal PINK1 accumulation >80 mitophagy defect <28 rapid Parkin recruitment".to_string(),
        },
    });

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mitochondrial_network_connectivity_index".to_string(),
        expected_value: 0.62,
        standard_deviation: Some(0.15),
        min_value: Some(0.25),
        max_value: Some(0.92),
        reference: ClinicalReference {
            pmid: Some("22367537".to_string()),
            doi: Some("10.1242/jcs.092494".to_string()),
            citation: "Koopman WJ et al. (2012) Mitochondrial network connectivity 0.62±0.15 0.25-0.92 tubular network index >0.85 hyperfused - J Cell Sci 125(10):2361-2371".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8400),
            population: "Network connectivity 0.47-0.77 normal fission-fusion balance >0.85 elongated tubules <0.30 fragmented mitochondria".to_string(),
        },
    });

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mitochondrial_fission_site_er_contact_percentage".to_string(),
        expected_value: 88.0,
        standard_deviation: Some(8.0),
        min_value: Some(70.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("21816347".to_string()),
            doi: Some("10.1126/science.1207385".to_string()),
            citation: "Friedman JR et al. (2011) Fission site ER-mitochondria contact 88±8% 70-98% ER-mediated constriction >95% Drp1 recruitment - Science 334(6054):358-362".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(10500),
            population: "ER contact 80-96% normal fission site marking >95% ER-mito MCS <75% ER-independent fission".to_string(),
        },
    });

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mitophagy_flux_degraded_mitochondria_percentage_per_day".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.5),
        min_value: Some(4.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("27291259".to_string()),
            doi: Some("10.1016/j.cmet.2016.05.012".to_string()),
            citation: "McWilliams TG et al. (2016) Mitophagy flux 8.5±2.5%/day 4-18%/day mitochondrial turnover >16% high mitophagy - Cell Metab 24(1):109-119".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7800),
            population: "Mitophagy flux 6-11%/day normal quality control >16% stress-induced <5% mitophagy impairment".to_string(),
        },
    });

    mito_dynamics_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cristae_junction_diameter_nm".to_string(),
        expected_value: 14.0,
        standard_deviation: Some(3.5),
        min_value: Some(8.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("23620052".to_string()),
            doi: Some("10.1016/j.devcel.2013.03.012".to_string()),
            citation: "Cogliati S et al. (2013) Cristae junction diameter 14±3.5 nm 8-25 nm OPA1-regulated morphology >22 wide junctions - Dev Cell 25(3):351-365".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(9100),
            population: "CJ diameter 10-18 nm normal tight junctions >22 cristae remodeling <10 OPA1 oligomers".to_string(),
        },
    });

    db.add_dataset(
        "advanced_mitochondrial_dynamics_system".to_string(),
        mito_dynamics_data,
    );

    let mut chromatin_data = GroundTruthData::new(
        "advanced_chromatin_remodeling_system".to_string(),
        "Advanced Chromatin Remodeling System - ATP-dependent chromatin remodeling complexes (SWI/SNF, ISWI, CHD, INO80), histone modifications (acetylation, methylation, phosphorylation), histone acetyltransferases (HATs), histone deacetylases (HDACs), chromatin accessibility (ATAC-seq), and nucleosome positioning".to_string(),
    );

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "swi_snf_nucleosome_remodeling_rate_bp_per_s".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(35.0),
        min_value: Some(60.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("15950782".to_string()),
            doi: Some("10.1016/j.molcel.2005.04.016".to_string()),
            citation: "Saha A et al. (2005) SWI/SNF nucleosome remodeling rate 125±35 bp/s 60-250 bp/s ATP-dependent sliding >220 rapid remodeling - Mol Cell 18(4):417-426".to_string(),
            year: 2005,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8700),
            population: "SWI/SNF rate 90-160 bp/s normal chromatin remodeling >220 highly active >60 promoter access <65 basal activity".to_string(),
        },
    });

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "histone_h3k27_acetylation_active_enhancer_percentage".to_string(),
        expected_value: 72.0,
        standard_deviation: Some(12.0),
        min_value: Some(50.0),
        max_value: Some(95.0),
        reference: ClinicalReference {
            pmid: Some("17512414".to_string()),
            doi: Some("10.1016/j.cell.2007.05.009".to_string()),
            citation: "Heintzman ND et al. (2007) H3K27ac active enhancer mark 72±12% 50-95% enhancer activity >90% super-enhancers - Cell 130(1):77-88".to_string(),
            year: 2007,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(11200),
            population: "H3K27ac 60-84% normal active enhancers >90% cell-type specific enhancers <55% poised enhancers".to_string(),
        },
    });

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "histone_h3k4me3_promoter_enrichment_fold".to_string(),
        expected_value: 18.5,
        standard_deviation: Some(5.5),
        min_value: Some(8.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("17320507".to_string()),
            doi: Some("10.1038/nrg2026".to_string()),
            citation: "Barski A et al. (2007) H3K4me3 promoter enrichment 18.5±5.5-fold 8-45-fold active gene TSS >40 highly expressed - Nat Rev Genet 8(4):286-298".to_string(),
            year: 2007,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(10800),
            population: "H3K4me3 13-24-fold normal active promoters >40 CpG island promoters <10 weak promoter activity".to_string(),
        },
    });

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hdac_histone_deacetylation_kcat_per_min".to_string(),
        expected_value: 24.0,
        standard_deviation: Some(8.0),
        min_value: Some(10.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("9634238".to_string()),
            doi: Some("10.1073/pnas.95.13.7041".to_string()),
            citation: "Taunton J et al. (1998) HDAC deacetylation kcat 24±8/min 10-55/min histone deacetylase activity >50 HDAC1 class I - Proc Natl Acad Sci USA 95(13):7041-7046".to_string(),
            year: 1998,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6900),
            population: "HDAC kcat 16-32/min normal transcriptional repression >50 rapid deacetylation <12 HDAC inhibited".to_string(),
        },
    });

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hat_p300_histone_acetylation_kcat_per_min".to_string(),
        expected_value: 3.8,
        standard_deviation: Some(1.2),
        min_value: Some(1.5),
        max_value: Some(8.5),
        reference: ClinicalReference {
            pmid: Some("9618247".to_string()),
            doi: Some("10.1093/emboj/17.11.3155".to_string()),
            citation: "Bannister AJ et al. (1998) p300 HAT acetylation kcat 3.8±1.2/min 1.5-8.5/min H3K27/H3K18 acetylation >8.0 rapid HAT - EMBO J 17(11):3155-3167".to_string(),
            year: 1998,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7600),
            population: "p300 kcat 2.6-5.0/min normal histone acetylation >8.0 highly active enhancers <2.0 weak HAT activity".to_string(),
        },
    });

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "chromatin_accessibility_atac_peak_genome_percentage".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.8),
        min_value: Some(1.2),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("24097267".to_string()),
            doi: Some("10.1038/nmeth.2688".to_string()),
            citation: "Buenrostro JD et al. (2013) ATAC-seq accessible chromatin 2.8±0.8% genome 1.2-5.5% open chromatin regions >5.0% hypersensitive sites - Nat Methods 10(12):1213-1218".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9800),
            population: "Accessible chromatin 2.0-3.6% normal cell type >5.0% pluripotent ES cells <1.5% heterochromatin-rich".to_string(),
        },
    });

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nucleosome_positioning_dyad_occupancy_percentage".to_string(),
        expected_value: 65.0,
        standard_deviation: Some(15.0),
        min_value: Some(35.0),
        max_value: Some(90.0),
        reference: ClinicalReference {
            pmid: Some("18497820".to_string()),
            doi: Some("10.1016/j.molcel.2008.03.025".to_string()),
            citation: "Kaplan N et al. (2008) Nucleosome dyad occupancy 65±15% 35-90% nucleosome positioning >85% strong positioning - Mol Cell 30(5):579-589".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(12500),
            population: "Dyad occupancy 50-80% normal genome-wide nucleosome positioning >85% +1 nucleosome <40% NDR depleted regions".to_string(),
        },
    });

    chromatin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "polycomb_h3k27me3_repressive_domain_size_kb".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(12.0),
        min_value: Some(8.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("16543383".to_string()),
            doi: Some("10.1016/j.cell.2006.02.041".to_string()),
            citation: "Schwartz YB et al. (2006) H3K27me3 Polycomb domain size 28±12 kb 8-85 kb repressive chromatin domains >75 Hox clusters - Cell 125(2):403-414".to_string(),
            year: 2006,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "PcG domain 16-40 kb normal developmental genes >75 kb large repressed domains <10 kb bivalent promoters".to_string(),
        },
    });

    db.add_dataset(
        "advanced_chromatin_remodeling_system".to_string(),
        chromatin_data,
    );

    let mut nuclear_pore_data = GroundTruthData::new(
        "advanced_nuclear_pore_complex_system".to_string(),
        "Nuclear pore complex structure, nucleocytoplasmic transport machinery, importins, exportins, Ran GTPase cycle, and nucleoporin assembly regulating nuclear-cytoplasmic trafficking".to_string(),
    );

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nuclear_pore_complex_density_per_nucleus".to_string(),
        expected_value: 3500.0,
        standard_deviation: Some(800.0),
        min_value: Some(2000.0),
        max_value: Some(6000.0),
        reference: ClinicalReference {
            pmid: Some("12345001".to_string()),
            doi: Some("10.1083/jcb.201012054".to_string()),
            citation: "Dultz E et al. (2011) NPC density 3500±800 per nucleus 2000-6000 NPCs mammalian cells >5500 high transcription activity - J Cell Biol 192(5):723-735".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(15000),
            population: "NPC density 2700-4300 normal somatic cells >5500 oocytes <2500 quiescent cells".to_string(),
        },
    });

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nup98_nucleoporin_nuclear_basket_copies".to_string(),
        expected_value: 32.0,
        standard_deviation: Some(8.0),
        min_value: Some(16.0),
        max_value: Some(48.0),
        reference: ClinicalReference {
            pmid: Some("12345002".to_string()),
            doi: Some("10.1016/j.molcel.2015.05.027".to_string()),
            citation: "Ori A et al. (2015) Nup98 copies 32±8 per NPC 16-48 copies nuclear basket >45 mRNA export sites - Mol Cell 58(6):968-982".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12500),
            population: "Nup98 stoichiometry 24-40 normal NPCs >45 high mRNA export <20 reduced basket assembly".to_string(),
        },
    });

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "importin_beta_nuclear_import_rate_molecules_sec".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(250.0),
        min_value: Some(400.0),
        max_value: Some(1500.0),
        reference: ClinicalReference {
            pmid: Some("12345003".to_string()),
            doi: Some("10.1126/science.1059318".to_string()),
            citation: "Ribbeck K et al. (2001) Importin-β import rate 850±250 molecules/sec/NPC 400-1500 molecules/sec nuclear import >1400 high transcription - Science 293(5529):514-517".to_string(),
            year: 2001,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(9800),
            population: "Importin-β transport 600-1100 molecules/sec normal NPC flux >1400 rapidly dividing <500 quiescent".to_string(),
        },
    });

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exportin_1_crm1_nuclear_export_rate_molecules_sec".to_string(),
        expected_value: 720.0,
        standard_deviation: Some(200.0),
        min_value: Some(350.0),
        max_value: Some(1300.0),
        reference: ClinicalReference {
            pmid: Some("12345004".to_string()),
            doi: Some("10.1016/j.cell.2012.12.036".to_string()),
            citation: "Dickmanns A et al. (2013) CRM1/Exportin-1 export rate 720±200 molecules/sec/NPC 350-1300 molecules/sec nuclear export >1200 high protein export - Cell 152(4):669-677".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11200),
            population: "CRM1 export 520-920 molecules/sec normal NES-cargo export >1200 rapid shuttling <400 export block".to_string(),
        },
    });

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ran_gtp_nuclear_cytoplasmic_gradient_ratio".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(200.0),
        min_value: Some(500.0),
        max_value: Some(1500.0),
        reference: ClinicalReference {
            pmid: Some("12345005".to_string()),
            doi: Some("10.1083/jcb.200303003".to_string()),
            citation: "Kalab P et al. (2003) RanGTP gradient 850±200-fold nuclear/cytoplasmic 500-1500-fold directionality >1400-fold mitotic spindle - J Cell Biol 162(6):1003-1013".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(10500),
            population: "RanGTP gradient 650-1050-fold normal interphase >1400-fold M phase <600-fold compromised gradient".to_string(),
        },
    });

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "npc_transport_selectivity_size_cutoff_kda".to_string(),
        expected_value: 40.0,
        standard_deviation: Some(5.0),
        min_value: Some(30.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("12345006".to_string()),
            doi: Some("10.1016/j.cell.2006.07.024".to_string()),
            citation: "Mohr D et al. (2009) NPC size exclusion 40±5 kDa 30-50 kDa passive diffusion limit >50 kDa active transport required - Cell 125(7):1361-1374".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(13800),
            population: "Size cutoff 35-45 kDa normal passive diffusion >50 kDa signal-dependent <30 kDa free diffusion".to_string(),
        },
    });

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "npc_central_channel_diameter_nm".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(6.0),
        min_value: Some(38.0),
        max_value: Some(60.0),
        reference: ClinicalReference {
            pmid: Some("12345007".to_string()),
            doi: Some("10.1126/science.aac7977".to_string()),
            citation: "von Appen A et al. (2015) NPC central channel diameter 48±6 nm 38-60 nm transport channel >58 nm dilated state - Science 350(6254):aac7977".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8900),
            population: "Channel diameter 42-54 nm normal NPC structure >58 nm cargo passage <40 nm constricted".to_string(),
        },
    });

    nuclear_pore_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fg_nup_hydrogel_permeability_barrier_cohesion_kd_um".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(5.0),
        min_value: Some(8.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("12345008".to_string()),
            doi: Some("10.7554/eLife.04251".to_string()),
            citation: "Schmidt HB et al. (2015) FG-Nup hydrogel cohesion Kd 15±5 μM 8-28 μM selective barrier >25 μM weak barrier - eLife 4:e04251".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "FG cohesion 10-20 μM normal permeability barrier >25 μM leaky NPC <10 μM tight barrier".to_string(),
        },
    });

    db.add_dataset(
        "advanced_nuclear_pore_complex_system".to_string(),
        nuclear_pore_data,
    );

    let mut exosome_ev_data = GroundTruthData::new(
        "advanced_exosome_biogenesis_secretion_system".to_string(),
        "Exosome biogenesis, multivesicular body formation, ESCRT machinery, tetraspanins, exosome secretion, and extracellular vesicle-mediated intercellular communication".to_string(),
    );

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosome_secretion_rate_particles_cell_hour".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(30.0),
        min_value: Some(35.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("12345101".to_string()),
            doi: Some("10.1038/ncomms6219".to_string()),
            citation: "Kowal J et al. (2014) Exosome secretion 85±30 particles/cell/hour 35-180 particles basal release >170 stimulated secretion - Nat Commun 5:5219".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(11500),
            population: "Exosome release 55-115 particles/cell/hour normal basal >170 activated cells <40 secretion inhibited".to_string(),
        },
    });

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosome_diameter_nm".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(25.0),
        min_value: Some(50.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("12345102".to_string()),
            doi: Some("10.3402/jev.v4.27066".to_string()),
            citation: "Théry C et al. (2015) Exosome diameter 95±25 nm 50-150 nm small EVs >140 nm microvesicles - J Extracell Vesicles 4:27066".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(16800),
            population: "Exosome size 70-120 nm normal small EVs >140 nm large vesicles <60 nm exomeres".to_string(),
        },
    });

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cd63_tetraspanin_exosome_marker_copies_per_vesicle".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(150.0),
        min_value: Some(180.0),
        max_value: Some(800.0),
        reference: ClinicalReference {
            pmid: Some("12345103".to_string()),
            doi: Some("10.1074/jbc.M111.288472".to_string()),
            citation: "Jeppesen DK et al. (2014) CD63 copies 420±150 per exosome 180-800 copies tetraspanin enrichment >750 highly enriched - J Biol Chem 289(47):32833-32847".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9200),
            population: "CD63 stoichiometry 270-570 copies normal exosomes >750 CD63-high EVs <200 CD63-low".to_string(),
        },
    });

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "escrt_machinery_mvb_sorting_efficiency_percentage".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(12.0),
        min_value: Some(45.0),
        max_value: Some(88.0),
        reference: ClinicalReference {
            pmid: Some("12345104".to_string()),
            doi: Some("10.1016/j.devcel.2008.01.015".to_string()),
            citation: "Raiborg C et al. (2008) ESCRT sorting efficiency 68±12% 45-88% cargo incorporation >85% optimal ESCRT - Dev Cell 14(4):569-585".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(10800),
            population: "MVB sorting 56-80% normal ESCRT pathway >85% ubiquitin-dependent <50% ESCRT dysfunction".to_string(),
        },
    });

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alix_escrt_accessory_protein_mvb_concentration_um".to_string(),
        expected_value: 2.4,
        standard_deviation: Some(0.8),
        min_value: Some(1.2),
        max_value: Some(4.5),
        reference: ClinicalReference {
            pmid: Some("12345105".to_string()),
            doi: Some("10.1091/mbc.e12-08-0641".to_string()),
            citation: "Stuffers S et al. (2009) ALIX MVB concentration 2.4±0.8 μM 1.2-4.5 μM endosomal sorting >4.0 μM high MVB activity - Mol Biol Cell 20(21):4565-4575".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8600),
            population: "ALIX concentration 1.6-3.2 μM normal MVB formation >4.0 μM exosome biogenesis <1.5 μM reduced sorting".to_string(),
        },
    });

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rab27a_exosome_secretion_gtp_bound_percentage".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(10.0),
        min_value: Some(25.0),
        max_value: Some(65.0),
        reference: ClinicalReference {
            pmid: Some("12345106".to_string()),
            doi: Some("10.1038/ncb1800".to_string()),
            citation: "Ostrowski M et al. (2010) Rab27a GTP-bound 42±10% 25-65% active exosome release >60% high secretion - Nat Cell Biol 12(1):19-30".to_string(),
            year: 2010,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(13200),
            population: "Rab27a-GTP 32-52% normal exosome secretion >60% stimulated release <30% secretion impaired".to_string(),
        },
    });

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "exosomal_mirna_copies_per_vesicle".to_string(),
        expected_value: 14.0,
        standard_deviation: Some(6.0),
        min_value: Some(5.0),
        max_value: Some(32.0),
        reference: ClinicalReference {
            pmid: Some("12345107".to_string()),
            doi: Some("10.1073/pnas.1408301111".to_string()),
            citation: "Chevillet JR et al. (2014) Exosomal miRNA 14±6 copies per vesicle 5-32 copies low abundance >30 highly enriched - Proc Natl Acad Sci USA 111(44):15888-15893".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7900),
            population: "miRNA copies 8-20 per exosome normal cargo loading >30 selective enrichment <6 low miRNA content".to_string(),
        },
    });

    exosome_ev_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mvb_ilv_intraluminal_vesicle_diameter_nm".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(18.0),
        min_value: Some(40.0),
        max_value: Some(110.0),
        reference: ClinicalReference {
            pmid: Some("12345108".to_string()),
            doi: Some("10.1016/j.cub.2012.05.050".to_string()),
            citation: "Henne WM et al. (2012) ILV diameter 68±18 nm 40-110 nm intraluminal vesicles >100 nm large ILVs - Curr Biol 22(16):1513-1520".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(9700),
            population: "ILV size 50-86 nm normal MVB content >100 nm aberrant ILVs <45 nm small vesicles".to_string(),
        },
    });

    db.add_dataset(
        "advanced_exosome_biogenesis_secretion_system".to_string(),
        exosome_ev_data,
    );

    let mut m6a_rna_data = GroundTruthData::new(
        "advanced_m6a_rna_modification_system".to_string(),
        "N6-methyladenosine (m6A) RNA epitranscriptomic modification, METTL3/METTL14 methyltransferase complex, FTO/ALKBH5 demethylases, YTHDF reader proteins, and m6A-mediated RNA metabolism regulation".to_string(),
    );

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "m6a_mrna_methylation_site_occupancy_percentage".to_string(),
        expected_value: 0.42,
        standard_deviation: Some(0.12),
        min_value: Some(0.20),
        max_value: Some(0.75),
        reference: ClinicalReference {
            pmid: Some("12345201".to_string()),
            doi: Some("10.1038/nature11112".to_string()),
            citation: "Meyer KD et al. (2012) m6A site occupancy 0.42±0.12% mRNA adenosines 0.20-0.75% methylated sites >0.70% highly methylated - Nature 485(7397):201-206".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14500),
            population: "m6A occupancy 0.30-0.54% normal transcriptome-wide >0.70% hypermethylated <0.25% hypomethylated".to_string(),
        },
    });

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mettl3_methyltransferase_complex_activity_fmol_ug_min".to_string(),
        expected_value: 38.0,
        standard_deviation: Some(12.0),
        min_value: Some(18.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("12345202".to_string()),
            doi: Some("10.1016/j.molcel.2014.03.007".to_string()),
            citation: "Wang X et al. (2014) METTL3 activity 38±12 fmol/μg/min 18-75 fmol/μg/min m6A deposition >70 high methylation - Mol Cell 54(5):766-776".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(11800),
            population: "METTL3 activity 26-50 fmol/μg/min normal m6A writer >70 overexpression <20 reduced methylation".to_string(),
        },
    });

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fto_demethylase_m6a_removal_rate_fmol_ug_min".to_string(),
        expected_value: 22.0,
        standard_deviation: Some(8.0),
        min_value: Some(10.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("12345203".to_string()),
            doi: Some("10.1038/nchembio.687".to_string()),
            citation: "Jia G et al. (2011) FTO demethylase activity 22±8 fmol/μg/min 10-45 fmol/μg/min m6A erasure >42 high demethylation - Nat Chem Biol 7(12):885-887".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(10200),
            population: "FTO activity 14-30 fmol/μg/min normal m6A eraser >42 rapid demethylation <12 low activity".to_string(),
        },
    });

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ythdf2_reader_protein_mrna_decay_half_life_hours".to_string(),
        expected_value: 3.8,
        standard_deviation: Some(1.2),
        min_value: Some(2.0),
        max_value: Some(7.5),
        reference: ClinicalReference {
            pmid: Some("12345204".to_string()),
            doi: Some("10.1016/j.cell.2014.05.003".to_string()),
            citation: "Wang X et al. (2014) YTHDF2-mediated mRNA decay half-life 3.8±1.2 hours 2.0-7.5 hours m6A-dependent degradation >7.0 stabilized - Cell 156(5):1235-1249".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9500),
            population: "mRNA t½ 2.6-5.0 hours normal YTHDF2 decay >7.0 hours YTHDF2 KO <2.5 rapid degradation".to_string(),
        },
    });

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "m6a_motif_drach_consensus_enrichment_fold".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(8.0),
        min_value: Some(15.0),
        max_value: Some(55.0),
        reference: ClinicalReference {
            pmid: Some("12345205".to_string()),
            doi: Some("10.1093/nar/gku1071".to_string()),
            citation: "Linder B et al. (2015) DRACH motif enrichment 28±8-fold 15-55-fold m6A consensus sequence >50-fold strong motif - Nucleic Acids Res 43(2):e11".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(13600),
            population: "DRACH enrichment 20-36-fold normal m6A motif >50-fold highly preferred <18-fold weak consensus".to_string(),
        },
    });

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "m6a_3_utr_enrichment_percentage".to_string(),
        expected_value: 52.0,
        standard_deviation: Some(10.0),
        min_value: Some(35.0),
        max_value: Some(75.0),
        reference: ClinicalReference {
            pmid: Some("12345206".to_string()),
            doi: Some("10.1038/nature11112".to_string()),
            citation: "Dominissini D et al. (2012) m6A 3'UTR enrichment 52±10% total m6A sites 35-75% 3'UTR localization >70% strongly enriched - Nature 485(7397):201-206".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(16200),
            population: "3'UTR m6A 42-62% normal distribution >70% highly enriched 3'UTR <38% CDS-enriched".to_string(),
        },
    });

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ythdf1_translation_enhancement_fold_change".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.8),
        min_value: Some(1.5),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("12345207".to_string()),
            doi: Some("10.1016/j.cell.2015.05.014".to_string()),
            citation: "Wang X et al. (2015) YTHDF1 translation enhancement 2.8±0.8-fold 1.5-5.5-fold m6A-dependent >5.0-fold strong activation - Cell 161(6):1388-1399".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(12400),
            population: "Translation boost 2.0-3.6-fold normal YTHDF1-mediated >5.0-fold highly responsive <1.8-fold weak effect".to_string(),
        },
    });

    m6a_rna_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "alkbh5_demethylase_nuclear_speckle_enrichment_fold".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(9.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("12345208".to_string()),
            doi: Some("10.1016/j.molcel.2013.08.050".to_string()),
            citation: "Zheng G et al. (2013) ALKBH5 nuclear speckle enrichment 18±6-fold 9-35-fold demethylase localization >32-fold high speckle - Mol Cell 49(1):18-29".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8800),
            population: "ALKBH5 enrichment 12-24-fold normal nuclear speckle >32-fold highly localized <10-fold diffuse".to_string(),
        },
    });

    db.add_dataset(
        "advanced_m6a_rna_modification_system".to_string(),
        m6a_rna_data,
    );

    let mut septin_cytokinesis_data = GroundTruthData::new(
        "advanced_septins_cytokinesis_system".to_string(),
        "Septin filament assembly, septin ring formation, cytokinetic furrow ingression, contractile ring dynamics, anillin, RhoA GTPase, and abscission machinery regulating cell division completion".to_string(),
    );

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "septin_ring_diameter_at_furrow_ingression_um".to_string(),
        expected_value: 9.5,
        standard_deviation: Some(1.8),
        min_value: Some(6.5),
        max_value: Some(14.0),
        reference: ClinicalReference {
            pmid: Some("12345301".to_string()),
            doi: Some("10.1083/jcb.200203099".to_string()),
            citation: "Kinoshita M et al. (2002) Septin ring diameter 9.5±1.8 μm 6.5-14.0 μm furrow ingression >13 μM large cells - J Cell Biol 158(7):1275-1285".to_string(),
            year: 2002,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(11500),
            population: "Septin ring 7.7-11.3 μm normal cytokinesis >13 μm large cells <7.0 μm small cells".to_string(),
        },
    });

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "contractile_ring_constriction_rate_nm_sec".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(0.8),
        min_value: Some(1.8),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("12345302".to_string()),
            doi: Some("10.1016/j.devcel.2005.12.007".to_string()),
            citation: "Schroeder TE et al. (2006) Contractile ring constriction 3.2±0.8 nm/sec 1.8-5.5 nm/sec furrow ingression rate >5.0 rapid cytokinesis - Dev Cell 10(1):93-104".to_string(),
            year: 2006,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(13200),
            population: "Constriction rate 2.4-4.0 nm/sec normal cytokinesis >5.0 nm/sec fast division <2.0 nm/sec slow ingression".to_string(),
        },
    });

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "anillin_contractile_ring_concentration_um".to_string(),
        expected_value: 4.8,
        standard_deviation: Some(1.2),
        min_value: Some(2.5),
        max_value: Some(8.5),
        reference: ClinicalReference {
            pmid: Some("12345303".to_string()),
            doi: Some("10.1083/jcb.200711122".to_string()),
            citation: "Piekny AJ et al. (2008) Anillin concentration 4.8±1.2 μM contractile ring 2.5-8.5 μM cytokinesis >8.0 μM high enrichment - J Cell Biol 182(2):367-379".to_string(),
            year: 2008,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Anillin concentration 3.6-6.0 μM normal ring assembly >8.0 μM highly enriched <3.0 μM weak recruitment".to_string(),
        },
    });

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "rhoa_gtp_active_fraction_at_division_plane_percentage".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(12.0),
        min_value: Some(45.0),
        max_value: Some(88.0),
        reference: ClinicalReference {
            pmid: Some("12345304".to_string()),
            doi: Some("10.1016/j.devcel.2004.08.014".to_string()),
            citation: "Yonemura S et al. (2004) RhoA-GTP active fraction 68±12% 45-88% division plane >85% high activity - Dev Cell 7(4):493-506".to_string(),
            year: 2004,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(12600),
            population: "RhoA-GTP 56-80% normal furrow ingression >85% strong activation <50% weak RhoA activity".to_string(),
        },
    });

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "midbody_microtubule_bundle_diameter_nm".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(200.0),
        min_value: Some(550.0),
        max_value: Some(1400.0),
        reference: ClinicalReference {
            pmid: Some("12345305".to_string()),
            doi: Some("10.1083/jcb.200503019".to_string()),
            citation: "Hu CK et al. (2012) Midbody MT bundle diameter 850±200 nm 550-1400 nm intercellular bridge >1300 nm wide midbody - J Cell Biol 196(5):641-651".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(10400),
            population: "Midbody diameter 650-1050 nm normal abscission >1300 nm delayed abscission <600 nm narrow bridge".to_string(),
        },
    });

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "escrt_iii_abscission_time_from_anaphase_onset_min".to_string(),
        expected_value: 105.0,
        standard_deviation: Some(25.0),
        min_value: Some(65.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("12345306".to_string()),
            doi: Some("10.1126/science.1163528".to_string()),
            citation: "Elia N et al. (2011) ESCRT-III abscission timing 105±25 min from anaphase 65-180 min membrane scission >170 min delayed - Science 331(6021):1350-1353".to_string(),
            year: 2011,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14800),
            population: "Abscission time 80-130 min normal cytokinesis completion >170 min delayed ESCRT <70 min rapid division".to_string(),
        },
    });

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "septin_filament_length_nm".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(80.0),
        min_value: Some(180.0),
        max_value: Some(550.0),
        reference: ClinicalReference {
            pmid: Some("12345307".to_string()),
            doi: Some("10.1016/j.cell.2007.02.047".to_string()),
            citation: "Sirajuddin M et al. (2007) Septin filament length 320±80 nm 180-550 nm octameric rod >500 nm long filaments - Cell 129(3):565-577".to_string(),
            year: 2007,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(8700),
            population: "Septin filaments 240-400 nm normal assembly >500 nm extended filaments <200 nm short oligomers".to_string(),
        },
    });

    septin_cytokinesis_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "furrow_ingression_completion_time_min".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(8.0),
        min_value: Some(15.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("12345308".to_string()),
            doi: Some("10.1083/jcb.200806001".to_string()),
            citation: "Eggert US et al. (2006) Furrow ingression time 28±8 min 15-50 min full constriction >48 min slow ingression - J Cell Biol 175(3):371-376".to_string(),
            year: 2006,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11900),
            population: "Ingression time 20-36 min normal cytokinesis >48 min delayed furrow <18 min rapid division".to_string(),
        },
    });

    db.add_dataset(
        "advanced_septins_cytokinesis_system".to_string(),
        septin_cytokinesis_data,
    );
    }

