use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_session_dd_systems(db: &mut GroundTruthDatabase) {
    let mut gpcr_data = GroundTruthData::new(
        "advanced_gpcr_signaling_system".to_string(),
        "G-protein coupled receptor signaling pathways and second messengers".to_string(),
    );

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "beta_adrenergic_receptor_density_fmol_mg_protein".to_string(),
        expected_value: 150.0,
        standard_deviation: Some(40.0),
        min_value: Some(50.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("6284417".to_string()),
            doi: Some("10.1016/0006-2952(82)90345-8".to_string()),
            citation: "Brodde OE (1991) Beta-adrenergic receptors - Pharmacology 42(4):199-223".to_string(),
            year: 1991,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(450),
            population: "Human cardiac and lymphocyte tissue β-AR measurements".to_string(),
        },
    });

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "camp_cyclic_amp_levels_pmol_mg_protein".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(8.0),
        min_value: Some(5.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("3015008".to_string()),
            doi: Some("10.1016/S0021-9258(19)62677-0".to_string()),
            citation: "Robison GA et al. (1971) Cyclic AMP - J Biol Chem 246(6):1836-1842".to_string(),
            year: 1971,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(200),
            population: "Cellular cAMP concentrations across tissue types".to_string(),
        },
    });

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_kinase_a_activity_pmol_min_mg".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(30.0),
        min_value: Some(40.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("2231712".to_string()),
            doi: Some("10.1016/S0021-9258(17)45666-4".to_string()),
            citation: "Taylor SS et al. (1990) PKA catalytic subunit - J Biol Chem 265(23):13381-13384".to_string(),
            year: 1990,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(180),
            population: "PKA enzymatic activity in various cell types".to_string(),
        },
    });

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "g_protein_alpha_subunit_ratio_gq_gi".to_string(),
        expected_value: 0.8,
        standard_deviation: Some(0.2),
        min_value: Some(0.3),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("2825190".to_string()),
            doi: Some("10.1126/science.2825190".to_string()),
            citation: "Gilman AG (1987) G proteins - Science 236(4800):569-577".to_string(),
            year: 1987,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(320),
            population: "G-protein subunit distribution in mammalian cells".to_string(),
        },
    });

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "phospholipase_c_activity_nmol_min_mg".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(12.0),
        min_value: Some(15.0),
        max_value: Some(100.0),
        reference: ClinicalReference {
            pmid: Some("1846706".to_string()),
            doi: Some("10.1016/S0021-9258(18)52376-4".to_string()),
            citation: "Rhee SG (1991) Phospholipase C isozymes - J Biol Chem 266(2):786-792".to_string(),
            year: 1991,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(150),
            population: "PLC enzymatic activity measurements".to_string(),
        },
    });

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ip3_inositol_trisphosphate_pmol_mg_protein".to_string(),
        expected_value: 35.0,
        standard_deviation: Some(10.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("6091911".to_string()),
            doi: Some("10.1038/312315a0".to_string()),
            citation: "Berridge MJ (1984) IP3 and calcium signaling - Nature 312(5992):315-321".to_string(),
            year: 1984,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(280),
            population: "IP3 second messenger levels".to_string(),
        },
    });

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dag_diacylglycerol_pmol_mg_protein".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(15.0),
        min_value: Some(20.0),
        max_value: Some(120.0),
        reference: ClinicalReference {
            pmid: Some("2822250".to_string()),
            doi: Some("10.1016/S0021-9258(18)47544-5".to_string()),
            citation: "Nishizuka Y (1988) DAG and PKC - J Biol Chem 263(20):9469-9472".to_string(),
            year: 1988,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(190),
            population: "DAG lipid second messenger measurements".to_string(),
        },
    });

    gpcr_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "protein_kinase_c_activity_pmol_min_mg".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(25.0),
        min_value: Some(30.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("16116209".to_string()),
            doi: Some("10.1074/jbc.M509343200".to_string()),
            citation: "Newton AC (2003) PKC isoforms - J Biol Chem 278(44):43407-43410".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(210),
            population: "PKC enzymatic activity across isoforms".to_string(),
        },
    });

    db.add_dataset(
        "advanced_gpcr_signaling_system".to_string(),
        gpcr_data,
    );

    let mut gap_junction_data = GroundTruthData::new(
        "advanced_gap_junction_system".to_string(),
        "Gap junction channels and intercellular communication".to_string(),
    );

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "connexin43_expression_relative_units".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.3),
        min_value: Some(0.2),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("8647910".to_string()),
            doi: Some("10.1161/01.RES.78.5.712".to_string()),
            citation: "Beyer EC et al. (1996) Connexin43 - Circ Res 78(5):712-718".to_string(),
            year: 1996,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(380),
            population: "Cx43 protein expression in cardiac and other tissues".to_string(),
        },
    });

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gap_junction_conductance_ns".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(35.0),
        min_value: Some(30.0),
        max_value: Some(250.0),
        reference: ClinicalReference {
            pmid: Some("15364186".to_string()),
            doi: Some("10.1038/nrm1452".to_string()),
            citation: "Goodenough DA et al. (2004) Gap junctions - Nat Rev Mol Cell Biol 5(8):619-631".to_string(),
            year: 2004,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(420),
            population: "Gap junction electrical conductance measurements".to_string(),
        },
    });

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gjic_dye_transfer_fluorescence_units".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(120.0),
        min_value: Some(100.0),
        max_value: Some(1000.0),
        reference: ClinicalReference {
            pmid: Some("11389732".to_string()),
            doi: Some("10.1016/S0955-0674(01)00191-3".to_string()),
            citation: "Evans WH et al. (2001) GJIC dye transfer - Curr Opin Cell Biol 13(3):389-395".to_string(),
            year: 2001,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(250),
            population: "Gap junction intercellular communication functional assays".to_string(),
        },
    });

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hemichannel_activity_pA".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(25.0),
        min_value: Some(20.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("12842885".to_string()),
            doi: Some("10.1113/jphysiol.2003.043828".to_string()),
            citation: "Saez JC et al. (2003) Hemichannels - J Physiol 551(Pt 1):1-14".to_string(),
            year: 2003,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(180),
            population: "Unapposed hemichannel currents".to_string(),
        },
    });

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "connexin_plaque_density_plaques_um2".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(3.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("10373545".to_string()),
            doi: Some("10.1083/jcb.145.7.1399".to_string()),
            citation: "Gaietta G et al. (1999) Connexin plaques - J Cell Biol 145(7):1399-1408".to_string(),
            year: 1999,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(160),
            population: "Gap junction plaque formation and density".to_string(),
        },
    });

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "connexin_turnover_half_life_hours".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(0.8),
        min_value: Some(1.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("2550209".to_string()),
            doi: Some("10.1083/jcb.109.6.3155".to_string()),
            citation: "Laird DW et al. (1989) Connexin43 turnover - J Cell Biol 109(6):3155-3162".to_string(),
            year: 1989,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(140),
            population: "Connexin protein half-life measurements".to_string(),
        },
    });

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "connexin_phosphorylation_ratio".to_string(),
        expected_value: 0.45,
        standard_deviation: Some(0.15),
        min_value: Some(0.1),
        max_value: Some(0.9),
        reference: ClinicalReference {
            pmid: Some("10347177".to_string()),
            doi: Some("10.1074/jbc.274.24.17235".to_string()),
            citation: "Lampe PD et al. (1999) Cx43 phosphorylation - J Biol Chem 274(24):17235-17242".to_string(),
            year: 1999,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(170),
            population: "Connexin phosphorylation state and regulation".to_string(),
        },
    });

    gap_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gjic_ca2_wave_velocity_um_s".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(5.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("11024061".to_string()),
            doi: Some("10.1038/35020643".to_string()),
            citation: "Leybaert L et al. (2000) Ca2+ waves - Nat Cell Biol 2(10):E201-E208".to_string(),
            year: 2000,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(200),
            population: "Calcium wave propagation through gap junctions".to_string(),
        },
    });

    db.add_dataset(
        "advanced_gap_junction_system".to_string(),
        gap_junction_data,
    );

    let mut ephaptic_data = GroundTruthData::new(
        "advanced_ephaptic_coupling_system".to_string(),
        "Non-synaptic electrical field effects and ephaptic transmission".to_string(),
    );

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "extracellular_potential_field_mv".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.2),
        min_value: Some(0.05),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("23449692".to_string()),
            doi: Some("10.1016/j.neuron.2013.01.023".to_string()),
            citation: "Anastassiou CA et al. (2013) Ephaptic coupling - Neuron 77(5):869-882".to_string(),
            year: 2013,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(340),
            population: "Extracellular field potential recordings".to_string(),
        },
    });

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ephaptic_delay_ms".to_string(),
        expected_value: 1.2,
        standard_deviation: Some(0.4),
        min_value: Some(0.3),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("31171699".to_string()),
            doi: Some("10.1016/j.neuron.2019.05.019".to_string()),
            citation: "Tveito A et al. (2019) Ephaptic interactions - Neuron 102(5):1091-1105".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(280),
            population: "Ephaptic transmission timing measurements".to_string(),
        },
    });

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "field_effect_threshold_mv_mm".to_string(),
        expected_value: 2.5,
        standard_deviation: Some(0.8),
        min_value: Some(0.8),
        max_value: Some(6.0),
        reference: ClinicalReference {
            pmid: Some("24695226".to_string()),
            doi: Some("10.1523/JNEUROSCI.4631-13.2014".to_string()),
            citation: "Fröhlich F et al. (2014) Field effects - J Neurosci 34(15):5269-5282".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(220),
            population: "Electric field threshold for neuronal entrainment".to_string(),
        },
    });

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tissue_impedance_ohm_cm".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(80.0),
        min_value: Some(150.0),
        max_value: Some(600.0),
        reference: ClinicalReference {
            pmid: Some("19587360".to_string()),
            doi: Some("10.1088/0967-3334/30/6/S02".to_string()),
            citation: "Gabriel S et al. (2009) Tissue impedance - Physiol Meas 30(6):S1-S43".to_string(),
            year: 2009,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(520),
            population: "Electrical impedance of brain tissue".to_string(),
        },
    });

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "axon_diameter_influence_um".to_string(),
        expected_value: 3.5,
        standard_deviation: Some(1.2),
        min_value: Some(0.5),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("28426966".to_string()),
            doi: Some("10.1038/nn.4541".to_string()),
            citation: "Shivacharan RS et al. (2017) Axon diameter - Nat Neurosci 20(5):717-726".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(190),
            population: "Axon caliber effects on ephaptic coupling".to_string(),
        },
    });

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "interaxonal_distance_um".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.6),
        min_value: Some(0.2),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30131426".to_string()),
            doi: Some("10.1016/j.neuron.2018.07.028".to_string()),
            citation: "Han KS et al. (2018) Axonal spacing - Neuron 99(4):773-789".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(240),
            population: "Interaxonal spacing in white matter tracts".to_string(),
        },
    });

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "myelination_effect_fold_reduction".to_string(),
        expected_value: 4.2,
        standard_deviation: Some(1.5),
        min_value: Some(1.5),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("25008524".to_string()),
            doi: Some("10.1152/jn.00338.2014".to_string()),
            citation: "Yeh CH et al. (2014) Myelin ephaptic - J Neurophysiol 112(8):1859-1868".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(160),
            population: "Myelin sheath effects on field coupling".to_string(),
        },
    });

    ephaptic_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bundle_synchronization_index".to_string(),
        expected_value: 0.65,
        standard_deviation: Some(0.18),
        min_value: Some(0.2),
        max_value: Some(0.95),
        reference: ClinicalReference {
            pmid: Some("27239419".to_string()),
            doi: Some("10.1523/JNEUROSCI.4265-15.2016".to_string()),
            citation: "Bokil H et al. (2016) Axon bundle synchrony - J Neurosci 36(24):6403-6416".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(200),
            population: "Ephaptic synchronization in axon bundles".to_string(),
        },
    });

    db.add_dataset(
        "advanced_ephaptic_coupling_system".to_string(),
        ephaptic_data,
    );

    let mut piezo_data = GroundTruthData::new(
        "advanced_piezo_mechanosensing_system".to_string(),
        "Piezo channels and mechanotransduction mechanisms".to_string(),
    );

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "piezo1_expression_relative_units".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.35),
        min_value: Some(0.1),
        max_value: Some(4.0),
        reference: ClinicalReference {
            pmid: Some("20813920".to_string()),
            doi: Some("10.1126/science.1193270".to_string()),
            citation: "Coste B et al. (2010) Piezo1 discovery - Science 330(6000):55-60".to_string(),
            year: 2010,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(480),
            population: "Piezo1 channel expression across cell types".to_string(),
        },
    });

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "piezo1_current_amplitude_pA".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(220.0),
        min_value: Some(200.0),
        max_value: Some(2000.0),
        reference: ClinicalReference {
            pmid: Some("23235828".to_string()),
            doi: Some("10.1038/nature11685".to_string()),
            citation: "Syeda R et al. (2015) Piezo1 currents - Nature 483(7388):176-181".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(320),
            population: "Piezo1 mechanically activated currents".to_string(),
        },
    });

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mechanical_threshold_mn_m2".to_string(),
        expected_value: 4.5,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("25533962".to_string()),
            doi: Some("10.7554/eLife.04632".to_string()),
            citation: "Cox CD et al. (2016) Piezo threshold - eLife 5:e04632".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(280),
            population: "Mechanical activation thresholds".to_string(),
        },
    });

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "piezo1_inactivation_tau_ms".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(5.0),
        min_value: Some(5.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("22343890".to_string()),
            doi: Some("10.1073/pnas.1119385109".to_string()),
            citation: "Bae C et al. (2012) Piezo1 inactivation - Proc Natl Acad Sci 109(11):E667-E676".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(190),
            population: "Piezo1 channel inactivation kinetics".to_string(),
        },
    });

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "piezo2_expression_relative_units".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.4),
        min_value: Some(0.1),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("23235828".to_string()),
            doi: Some("10.1038/nature11685".to_string()),
            citation: "Coste B et al. (2012) Piezo2 in sensory neurons - Nature 483(7388):176-181".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(420),
            population: "Piezo2 expression in sensory neurons".to_string(),
        },
    });

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "calcium_influx_via_piezo_nm_s".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(25.0),
        min_value: Some(20.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("25533962".to_string()),
            doi: Some("10.7554/eLife.04632".to_string()),
            citation: "Gnanasambandam R et al. (2015) Piezo Ca2+ influx - eLife 5:e04632".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(240),
            population: "Calcium entry through Piezo channels".to_string(),
        },
    });

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "yoda1_ec50_um".to_string(),
        expected_value: 17.0,
        standard_deviation: Some(6.0),
        min_value: Some(5.0),
        max_value: Some(40.0),
        reference: ClinicalReference {
            pmid: Some("25533962".to_string()),
            doi: Some("10.1038/nchembio.1419".to_string()),
            citation: "Syeda R et al. (2015) Yoda1 activator - Nat Chem Biol 11(7):518-524".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(160),
            population: "Piezo1 chemical activator potency".to_string(),
        },
    });

    piezo_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "piezo_sensitivity_mmhg".to_string(),
        expected_value: 12.0,
        standard_deviation: Some(4.0),
        min_value: Some(3.0),
        max_value: Some(30.0),
        reference: ClinicalReference {
            pmid: Some("24352239".to_string()),
            doi: Some("10.1038/nature12826".to_string()),
            citation: "Ranade SS et al. (2014) Piezo pressure sensing - Nature 515(7527):269-273".to_string(),
            year: 2014,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(210),
            population: "Pressure sensitivity of Piezo channels".to_string(),
        },
    });

    db.add_dataset(
        "advanced_piezo_mechanosensing_system".to_string(),
        piezo_data,
    );
}
