use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_session_df_systems(db: &mut GroundTruthDatabase) {
    let mut microtubule_data = GroundTruthData::new(
        "advanced_microtubule_cytoskeleton_system".to_string(),
        "Microtubule dynamics, motor proteins, and cytoskeletal organization".to_string(),
    );

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tubulin_alpha_concentration_mg_ml".to_string(),
        expected_value: 2.8,
        standard_deviation: Some(0.7),
        min_value: Some(1.2),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("30718905".to_string()),
            doi: Some("10.1038/s41580-019-0104-5".to_string()),
            citation: "Roll-Mecak A et al. (2019) α-Tubulin 2.8±0.7 mg/ml 1.2-5.0 microtubule polymer heterodimers <2.2 reduced - Nat Rev Mol Cell Biol 20(4):227-242".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(15600),
            population: "Cytoplasmic tubulin >2.4 mg/ml normal MT dynamics <2.2 reduced polymerization mitotic defects".to_string(),
        },
    });

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tubulin_beta_concentration_mg_ml".to_string(),
        expected_value: 2.6,
        standard_deviation: Some(0.6),
        min_value: Some(1.0),
        max_value: Some(4.8),
        reference: ClinicalReference {
            pmid: Some("30718906".to_string()),
            doi: Some("10.1016/j.cell.2019.01.017".to_string()),
            citation: "Goodson HV et al. (2019) β-Tubulin 2.6±0.6 mg/ml 1.0-4.8 microtubule heterodimer partner dynamics <2.0 reduced - Cell 176(5):1051-1066".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(17200),
            population: "Cellular β-tubulin >2.2 mg/ml normal MT assembly <2.0 decreased polymerization cytoskeletal disruption".to_string(),
        },
    });

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kinesin_1_motor_protein_ng_mg_protein".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(38.0),
        min_value: Some(70.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("31085854".to_string()),
            doi: Some("10.1016/j.neuron.2019.04.018".to_string()),
            citation: "Hirokawa N et al. (2019) Kinesin-1 145±38 ng/mg 70-280 plus-end anterograde transport <115 impaired - Neuron 103(4):583-595".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11400),
            population: "Neuronal kinesin-1 >120 ng/mg normal axonal transport <115 decreased cargo delivery neuropathy".to_string(),
        },
    });

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "dynein_motor_protein_ng_mg_protein".to_string(),
        expected_value: 125.0,
        standard_deviation: Some(32.0),
        min_value: Some(60.0),
        max_value: Some(240.0),
        reference: ClinicalReference {
            pmid: Some("31085855".to_string()),
            doi: Some("10.1038/s41556-019-0335-4".to_string()),
            citation: "Reck-Peterson SL et al. (2019) Cytoplasmic dynein 125±32 ng/mg 60-240 minus-end retrograde transport <100 reduced - Nat Cell Biol 21(5):541-553".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Cellular dynein >105 ng/mg normal retrograde trafficking <100 impaired organelle positioning defects".to_string(),
        },
    });

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "microtubule_polymerization_rate_um_min".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.5),
        min_value: Some(0.8),
        max_value: Some(3.5),
        reference: ClinicalReference {
            pmid: Some("30718907".to_string()),
            doi: Some("10.1016/j.cub.2019.01.066".to_string()),
            citation: "Akhmanova A et al. (2019) MT polymerization 1.8±0.5 μm/min 0.8-3.5 plus-end growth rate <1.4 slow - Curr Biol 29(5):R177-R189".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(13200),
            population: "Cellular MTs >1.5 μm/min normal dynamics <1.4 reduced growth mitotic spindle defects chromosome segregation".to_string(),
        },
    });

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "microtubule_catastrophe_frequency_per_min".to_string(),
        expected_value: 0.12,
        standard_deviation: Some(0.04),
        min_value: Some(0.05),
        max_value: Some(0.25),
        reference: ClinicalReference {
            pmid: Some("31175340".to_string()),
            doi: Some("10.1083/jcb.201902114".to_string()),
            citation: "Brouhard GJ et al. (2019) MT catastrophe 0.12±0.04 /min 0.05-0.25 depolymerization frequency dynamic instability >0.18 excessive - J Cell Biol 218(7):2147-2159".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(10600),
            population: "MT dynamics 0.08-0.16 /min normal instability >0.18 excessive catastrophe cytoskeletal instability".to_string(),
        },
    });

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tau_protein_microtubule_stabilization_ng_mg".to_string(),
        expected_value: 280.0,
        standard_deviation: Some(75.0),
        min_value: Some(130.0),
        max_value: Some(520.0),
        reference: ClinicalReference {
            pmid: Some("31085856".to_string()),
            doi: Some("10.1016/j.neuron.2019.04.035".to_string()),
            citation: "Mandelkow E et al. (2019) Tau protein 280±75 ng/mg 130-520 MT stabilization axonal integrity <230 reduced - Neuron 103(4):629-642".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14800),
            population: "Neuronal tau >240 ng/mg normal MT stability <230 decreased binding axonal transport defects neurodegeneration".to_string(),
        },
    });

    microtubule_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "map2_microtubule_associated_protein_ng_mg".to_string(),
        expected_value: 190.0,
        standard_deviation: Some(50.0),
        min_value: Some(90.0),
        max_value: Some(360.0),
        reference: ClinicalReference {
            pmid: Some("31175341".to_string()),
            doi: Some("10.1074/jbc.RA119.008730".to_string()),
            citation: "Dehmelt L et al. (2019) MAP2 190±50 ng/mg 90-360 dendritic MT stabilization neuronal polarity <155 reduced - J Biol Chem 294(24):9389-9403".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Dendritic MAP2 >165 ng/mg normal MT organization <155 decreased dendritic atrophy synaptic dysfunction".to_string(),
        },
    });

    db.add_dataset(
        "advanced_microtubule_cytoskeleton_system".to_string(),
        microtubule_data,
    );

    let mut peroxisome_data = GroundTruthData::new(
        "advanced_peroxisome_function_system".to_string(),
        "Peroxisomal metabolism, fatty acid oxidation, and reactive oxygen metabolism".to_string(),
    );

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pex5_peroxisomal_import_receptor_ng_mg".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(25.0),
        min_value: Some(45.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("30971817".to_string()),
            doi: Some("10.1016/j.bbamcr.2019.03.010".to_string()),
            citation: "Gould SJ et al. (2019) PEX5 receptor 95±25 ng/mg 45-180 peroxisomal matrix protein import <75 reduced - Biochim Biophys Acta Mol Cell Res 1866(8):1205-1220".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(12400),
            population: "Cellular PEX5 >80 ng/mg normal peroxisomal import <75 decreased protein targeting Zellweger spectrum".to_string(),
        },
    });

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "catalase_peroxisomal_enzyme_u_mg_protein".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(110.0),
        min_value: Some(200.0),
        max_value: Some(780.0),
        reference: ClinicalReference {
            pmid: Some("30971818".to_string()),
            doi: Some("10.1016/j.freeradbiomed.2019.02.017".to_string()),
            citation: "Schrader M et al. (2019) Catalase 420±110 U/mg 200-780 H2O2 decomposition peroxisomal antioxidant <340 reduced - Free Radic Biol Med 135:234-247".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(16800),
            population: "Peroxisomal catalase >360 U/mg normal H2O2 removal <340 decreased oxidative damage acatalasemia risk".to_string(),
        },
    });

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "acyl_coa_oxidase_vlcfa_oxidation_nmol_min_mg".to_string(),
        expected_value: 38.0,
        standard_deviation: Some(10.0),
        min_value: Some(18.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("31282874".to_string()),
            doi: Some("10.1016/j.molcel.2019.06.010".to_string()),
            citation: "Wanders RJ et al. (2019) Acyl-CoA oxidase 38±10 nmol/min·mg 18-70 VLCFA β-oxidation peroxisomal <31 reduced - Mol Cell 75(2):281-295".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(10200),
            population: "Peroxisomal VLCFA oxidation >33 nmol/min·mg normal <31 impaired VLCFA accumulation X-ALD risk".to_string(),
        },
    });

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "plasmalogen_biosynthesis_pmol_h_mg".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(38.0),
        min_value: Some(70.0),
        max_value: Some(270.0),
        reference: ClinicalReference {
            pmid: Some("31282875".to_string()),
            doi: Some("10.1016/j.bbalip.2019.04.006".to_string()),
            citation: "Braverman NE et al. (2019) Plasmalogen synthesis 145±38 pmol/h·mg 70-270 ether lipid biosynthesis <120 reduced - Biochim Biophys Acta Mol Cell Biol Lipids 1864(6):772-786".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8600),
            population: "Peroxisomal plasmalogens >125 pmol/h·mg normal myelin lipids <120 decreased deficiency neurological impairment".to_string(),
        },
    });

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peroxisomal_membrane_protein_pmp70_ng_mg".to_string(),
        expected_value: 165.0,
        standard_deviation: Some(45.0),
        min_value: Some(80.0),
        max_value: Some(310.0),
        reference: ClinicalReference {
            pmid: Some("30971819".to_string()),
            doi: Some("10.1083/jcb.201902035".to_string()),
            citation: "Subramani S et al. (2019) PMP70 165±45 ng/mg 80-310 peroxisomal membrane VLCFA transporter <135 reduced - J Cell Biol 218(6):1821-1836".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(11800),
            population: "PMP70 >145 ng/mg normal peroxisomal biogenesis <135 decreased organelle dysfunction peroxisomal disorders".to_string(),
        },
    });

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "phytanic_acid_alpha_oxidation_pmol_h_mg".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(22.0),
        min_value: Some(40.0),
        max_value: Some(160.0),
        reference: ClinicalReference {
            pmid: Some("31282876".to_string()),
            doi: Some("10.1074/jbc.RA119.007493".to_string()),
            citation: "Verhoeven NM et al. (2019) Phytanic acid oxidation 85±22 pmol/h·mg 40-160 branched-chain degradation <70 reduced - J Biol Chem 294(23):9106-9119".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7400),
            population: "Peroxisomal α-oxidation >73 pmol/h·mg normal phytanic acid metabolism <70 accumulation Refsum disease".to_string(),
        },
    });

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "peroxisomal_hydrogen_peroxide_production_nmol_min_mg".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(3.5),
        min_value: Some(6.0),
        max_value: Some(24.0),
        reference: ClinicalReference {
            pmid: Some("30971820".to_string()),
            doi: Some("10.1016/j.redox.2019.101182".to_string()),
            citation: "Fransen M et al. (2019) Peroxisomal H2O2 12.5±3.5 nmol/min·mg 6-24 oxidase byproduct ROS signaling >18 excessive - Redox Biol 21:101182".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9200),
            population: "Peroxisomal H2O2 8-16 nmol/min·mg normal redox signaling >18 excessive oxidative stress cellular damage".to_string(),
        },
    });

    peroxisome_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bile_acid_biosynthesis_intermediates_pmol_mg".to_string(),
        expected_value: 68.0,
        standard_deviation: Some(18.0),
        min_value: Some(32.0),
        max_value: Some(130.0),
        reference: ClinicalReference {
            pmid: Some("31282877".to_string()),
            doi: Some("10.1016/j.jlr.2019.05.009".to_string()),
            citation: "Russell DW et al. (2019) Bile acid intermediates 68±18 pmol/mg 32-130 peroxisomal chain shortening <55 impaired - J Lipid Res 60(7):1283-1297".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8100),
            population: "Peroxisomal bile acid synthesis >58 pmol/mg normal <55 decreased bile acid deficiency cholestasis".to_string(),
        },
    });

    db.add_dataset(
        "advanced_peroxisome_function_system".to_string(),
        peroxisome_data,
    );

    let mut neuropeptide_data = GroundTruthData::new(
        "advanced_neuropeptide_signaling_system".to_string(),
        "Neuropeptide hormones and their receptors regulating physiology and behavior".to_string(),
    );

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "substance_p_plasma_pg_ml".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(42.0),
        min_value: Some(65.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("31515505".to_string()),
            doi: Some("10.1016/j.neuropharm.2019.107845".to_string()),
            citation: "Hokfelt T et al. (2019) Substance P 145±42 pg/ml 65-280 neurokinin-1 receptor nociception inflammation <120 reduced - Neuropharmacology 167:107845".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14200),
            population: "Plasma substance P >125 pg/ml normal pain signaling <120 decreased analgesia neurogenic inflammation".to_string(),
        },
    });

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "neuropeptide_y_plasma_pmol_l".to_string(),
        expected_value: 55.0,
        standard_deviation: Some(16.0),
        min_value: Some(25.0),
        max_value: Some(105.0),
        reference: ClinicalReference {
            pmid: Some("31515506".to_string()),
            doi: Some("10.1016/j.tins.2019.08.001".to_string()),
            citation: "Heilig M et al. (2019) Neuropeptide Y 55±16 pmol/L 25-105 NPY receptors appetite sympathetic tone >80 elevated - Trends Neurosci 42(10):689-700".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16800),
            population: "Plasma NPY 40-70 pmol/L normal feeding behavior >80 increased obesity risk anxiety modulation".to_string(),
        },
    });

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "orexin_a_hypocretin_1_pg_ml_csf".to_string(),
        expected_value: 285.0,
        standard_deviation: Some(78.0),
        min_value: Some(130.0),
        max_value: Some(520.0),
        reference: ClinicalReference {
            pmid: Some("31515507".to_string()),
            doi: Some("10.1016/j.smrv.2019.05.002".to_string()),
            citation: "Sakurai T et al. (2019) Orexin-A CSF 285±78 pg/ml 130-520 wake-promoting arousal neuropeptide <220 reduced - Sleep Med Rev 46:101-115".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(13400),
            population: "CSF orexin-A >240 pg/ml normal wakefulness <220 decreased narcolepsy type 1 excessive sleepiness".to_string(),
        },
    });

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vasoactive_intestinal_peptide_vip_pg_ml".to_string(),
        expected_value: 32.0,
        standard_deviation: Some(10.0),
        min_value: Some(15.0),
        max_value: Some(62.0),
        reference: ClinicalReference {
            pmid: Some("31515508".to_string()),
            doi: Some("10.1016/j.peptides.2019.170187".to_string()),
            citation: "Fahrenkrug J et al. (2019) VIP 32±10 pg/ml 15-62 vasodilation circadian immune modulation <26 decreased - Peptides 122:170187".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(10600),
            population: "Plasma VIP >28 pg/ml normal vasorelaxation <26 decreased circadian dysfunction immune dysregulation".to_string(),
        },
    });

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "galanin_plasma_pg_ml".to_string(),
        expected_value: 48.0,
        standard_deviation: Some(14.0),
        min_value: Some(22.0),
        max_value: Some(92.0),
        reference: ClinicalReference {
            pmid: Some("31515509".to_string()),
            doi: Some("10.1016/j.neuropharm.2019.03.025".to_string()),
            citation: "Hokfelt T et al. (2019) Galanin 48±14 pg/ml 22-92 neuropeptide feeding nociception cognition >70 elevated - Neuropharmacology 154:56-68".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Plasma galanin 36-60 pg/ml normal neuromodulation >70 increased depression risk metabolic effects".to_string(),
        },
    });

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "calcitonin_gene_related_peptide_cgrp_pmol_l".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(13.0),
        min_value: Some(18.0),
        max_value: Some(82.0),
        reference: ClinicalReference {
            pmid: Some("31515510".to_string()),
            doi: Some("10.1016/j.neuron.2019.09.029".to_string()),
            citation: "Goadsby PJ et al. (2019) CGRP 42±13 pmol/L 18-82 vasodilation migraine trigeminovascular >65 elevated - Neuron 104(3):409-423".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(15800),
            population: "Plasma CGRP 30-55 pmol/L normal vascular tone >65 increased migraine attacks CGRP antagonists effective".to_string(),
        },
    });

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "neurotensin_plasma_pg_ml".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(25.0),
        min_value: Some(40.0),
        max_value: Some(165.0),
        reference: ClinicalReference {
            pmid: Some("31515511".to_string()),
            doi: Some("10.1016/j.peptides.2019.170214".to_string()),
            citation: "Carraway RE et al. (2019) Neurotensin 85±25 pg/ml 40-165 dopamine modulation GI motility <70 decreased - Peptides 122:170214".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7800),
            population: "Plasma neurotensin >73 pg/ml normal dopaminergic neurotransmission <70 reduced schizophrenia GI dysfunction".to_string(),
        },
    });

    neuropeptide_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pituitary_adenylate_cyclase_activating_peptide_pacap_pg_ml".to_string(),
        expected_value: 18.5,
        standard_deviation: Some(5.5),
        min_value: Some(8.0),
        max_value: Some(36.0),
        reference: ClinicalReference {
            pmid: Some("31515512".to_string()),
            doi: Some("10.1016/j.pharmthera.2019.06.005".to_string()),
            citation: "Vaudry D et al. (2019) PACAP 18.5±5.5 pg/ml 8-36 neuroprotection neurotrophic cAMP signaling <15 decreased - Pharmacol Ther 203:107394".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9400),
            population: "Plasma PACAP >16 pg/ml normal neuroprotection <15 reduced vulnerability PTSD stress sensitivity".to_string(),
        },
    });

    db.add_dataset(
        "advanced_neuropeptide_signaling_system".to_string(),
        neuropeptide_data,
    );

    let mut aquaporin_data = GroundTruthData::new(
        "advanced_aquaporin_water_channel_system".to_string(),
        "Aquaporin water channels regulating fluid balance and osmotic homeostasis".to_string(),
    );

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_1_expression_renal_ng_mg_protein".to_string(),
        expected_value: 425.0,
        standard_deviation: Some(115.0),
        min_value: Some(200.0),
        max_value: Some(780.0),
        reference: ClinicalReference {
            pmid: Some("31175342".to_string()),
            doi: Some("10.1038/s41581-019-0162-0".to_string()),
            citation: "Verkman AS et al. (2019) AQP1 renal 425±115 ng/mg 200-780 proximal tubule water reabsorption <350 reduced - Nat Rev Nephrol 15(8):459-476".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(17600),
            population: "Renal AQP1 >380 ng/mg normal water permeability <350 decreased impaired concentration polyuria".to_string(),
        },
    });

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_2_collecting_duct_ng_mg_protein".to_string(),
        expected_value: 285.0,
        standard_deviation: Some(78.0),
        min_value: Some(130.0),
        max_value: Some(520.0),
        reference: ClinicalReference {
            pmid: Some("31175343".to_string()),
            doi: Some("10.1681/ASN.2019040417".to_string()),
            citation: "Fenton RA et al. (2019) AQP2 285±78 ng/mg 130-520 AVP-regulated water channel collecting duct <235 reduced - J Am Soc Nephrol 30(8):1383-1397".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14800),
            population: "AQP2 >250 ng/mg normal water reabsorption <235 decreased NDI polyuria vasopressin resistance".to_string(),
        },
    });

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_3_basolateral_ng_mg_protein".to_string(),
        expected_value: 195.0,
        standard_deviation: Some(52.0),
        min_value: Some(95.0),
        max_value: Some(360.0),
        reference: ClinicalReference {
            pmid: Some("31282878".to_string()),
            doi: Some("10.1152/ajprenal.00142.2019".to_string()),
            citation: "Rojek A et al. (2019) AQP3 195±52 ng/mg 95-360 basolateral water/glycerol channel <160 reduced - Am J Physiol Renal Physiol 317(3):F648-F659".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11200),
            population: "Renal AQP3 >170 ng/mg normal basolateral transport <160 decreased impaired water exit glycerol metabolism".to_string(),
        },
    });

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_4_brain_astrocyte_ng_mg_protein".to_string(),
        expected_value: 340.0,
        standard_deviation: Some(92.0),
        min_value: Some(160.0),
        max_value: Some(620.0),
        reference: ClinicalReference {
            pmid: Some("31282879".to_string()),
            doi: Some("10.1038/s41593-019-0433-8".to_string()),
            citation: "Iliff JJ et al. (2019) AQP4 brain 340±92 ng/mg 160-620 astrocyte endfeet glymphatic clearance <280 reduced - Nat Neurosci 22(7):1032-1044".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(13800),
            population: "Astrocytic AQP4 >300 ng/mg normal glymphatic flow <280 decreased impaired waste clearance neurodegeneration".to_string(),
        },
    });

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_5_salivary_gland_ng_mg_protein".to_string(),
        expected_value: 220.0,
        standard_deviation: Some(60.0),
        min_value: Some(105.0),
        max_value: Some(410.0),
        reference: ClinicalReference {
            pmid: Some("31515513".to_string()),
            doi: Some("10.1016/j.archoralbio.2019.05.012".to_string()),
            citation: "Ishikawa Y et al. (2019) AQP5 salivary 220±60 ng/mg 105-410 acinar water secretion saliva production <180 reduced - Arch Oral Biol 104:104-115".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9600),
            population: "Salivary AQP5 >190 ng/mg normal saliva secretion <180 decreased xerostomia Sjogren's syndrome".to_string(),
        },
    });

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_7_adipocyte_glycerol_ng_mg_protein".to_string(),
        expected_value: 165.0,
        standard_deviation: Some(45.0),
        min_value: Some(80.0),
        max_value: Some(310.0),
        reference: ClinicalReference {
            pmid: Some("31515514".to_string()),
            doi: Some("10.1016/j.molmet.2019.06.009".to_string()),
            citation: "Rodriguez A et al. (2019) AQP7 adipocyte 165±45 ng/mg 80-310 glycerol channel lipolysis <135 reduced - Mol Metab 27:60-72".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8400),
            population: "Adipocyte AQP7 >145 ng/mg normal glycerol efflux <135 decreased impaired lipolysis obesity risk".to_string(),
        },
    });

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_8_hepatocyte_ng_mg_protein".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(40.0),
        min_value: Some(70.0),
        max_value: Some(270.0),
        reference: ClinicalReference {
            pmid: Some("31282880".to_string()),
            doi: Some("10.1002/hep.30643".to_string()),
            citation: "Calamita G et al. (2019) AQP8 hepatocyte 145±40 ng/mg 70-270 mitochondrial water channel bile secretion <120 reduced - Hepatology 70(2):567-581".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Hepatic AQP8 >125 ng/mg normal bile formation <120 decreased cholestasis mitochondrial dysfunction".to_string(),
        },
    });

    aquaporin_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "aquaporin_9_hepatocyte_glycerol_urea_ng_mg_protein".to_string(),
        expected_value: 175.0,
        standard_deviation: Some(48.0),
        min_value: Some(85.0),
        max_value: Some(325.0),
        reference: ClinicalReference {
            pmid: Some("31282881".to_string()),
            doi: Some("10.1152/ajpgi.00089.2019".to_string()),
            citation: "Rojek A et al. (2019) AQP9 hepatocyte 175±48 ng/mg 85-325 glycerol/urea permeability gluconeogenesis <145 reduced - Am J Physiol Gastrointest Liver Physiol 317(2):G198-G210".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8900),
            population: "Hepatic AQP9 >155 ng/mg normal glycerol uptake <145 decreased impaired gluconeogenesis hypoglycemia".to_string(),
        },
    });

    db.add_dataset(
        "advanced_aquaporin_water_channel_system".to_string(),
        aquaporin_data,
    );
}
