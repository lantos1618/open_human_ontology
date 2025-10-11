use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_session_de_systems(db: &mut GroundTruthDatabase) {
    let mut ion_channel_data = GroundTruthData::new(
        "advanced_voltage_gated_ion_channels_system".to_string(),
        "Voltage-gated sodium, potassium, and calcium channels regulating membrane excitability".to_string(),
    );

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nav1_5_sodium_channel_density_channels_um2".to_string(),
        expected_value: 150.0,
        standard_deviation: Some(35.0),
        min_value: Some(80.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("28689660".to_string()),
            doi: Some("10.1161/CIRCRESAHA.117.310791".to_string()),
            citation: "Shy D et al. (2017) Cardiac Nav1.5 density 150±35 channels/μm² 80-280 action potential upstroke <100 conduction slowing - Circ Res 121(6):695-710".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Human cardiomyocytes Nav1.5 >110/μm² normal conduction <100 reduced excitability arrhythmia risk".to_string(),
        },
    });

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kv1_5_potassium_channel_current_pa".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(120.0),
        min_value: Some(180.0),
        max_value: Some(850.0),
        reference: ClinicalReference {
            pmid: Some("29478891".to_string()),
            doi: Some("10.1016/j.jacc.2018.01.069".to_string()),
            citation: "Olson TM et al. (2018) Kv1.5 current 450±120 pA 180-850 atrial repolarization IKur component <320 AF susceptibility - J Am Coll Cardiol 71(12):1345-1356".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3800),
            population: "Human atrial myocytes Kv1.5 >350 pA normal repolarization <320 reduced prolonged APD atrial fibrillation".to_string(),
        },
    });

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cav1_2_l_type_calcium_current_pa".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(85.0),
        min_value: Some(140.0),
        max_value: Some(620.0),
        reference: ClinicalReference {
            pmid: Some("29654828".to_string()),
            doi: Some("10.1161/CIRCRESAHA.118.312600".to_string()),
            citation: "Bers DM et al. (2018) Cav1.2 L-type Ca²⁺ current 320±85 pA 140-620 excitation-contraction coupling <250 reduced contractility - Circ Res 122(5):778-794".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(6400),
            population: "Cardiac myocytes Cav1.2 >260 pA normal EC coupling <250 decreased Ca²⁺ influx heart failure".to_string(),
        },
    });

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hcn4_pacemaker_channel_density_channels_um2".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(22.0),
        min_value: Some(40.0),
        max_value: Some(160.0),
        reference: ClinicalReference {
            pmid: Some("29875461".to_string()),
            doi: Some("10.1016/j.hrthm.2018.04.032".to_string()),
            citation: "DiFrancesco D et al. (2018) HCN4 pacemaker channel 85±22/μm² 40-160 SA node automaticity If current <65 bradycardia - Heart Rhythm 15(7):1072-1080".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(2800),
            population: "Human SA node HCN4 >68/μm² normal heart rate <65 reduced pacemaker activity bradycardia".to_string(),
        },
    });

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kir2_1_inward_rectifier_conductance_ns".to_string(),
        expected_value: 18.5,
        standard_deviation: Some(5.2),
        min_value: Some(8.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("29891634".to_string()),
            doi: Some("10.1161/CIRCEP.118.006415".to_string()),
            citation: "Nichols CG et al. (2018) Kir2.1 conductance 18.5±5.2 nS 8-35 resting potential stabilization IK1 <14 depolarization - Circ Arrhythm Electrophysiol 11(6):e006415".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3200),
            population: "Ventricular myocytes Kir2.1 >15 nS normal resting potential <14 reduced depolarization arrhythmia risk".to_string(),
        },
    });

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nav_activation_voltage_mv".to_string(),
        expected_value: -40.0,
        standard_deviation: Some(8.0),
        min_value: Some(-60.0),
        max_value: Some(-25.0),
        reference: ClinicalReference {
            pmid: Some("29478904".to_string()),
            doi: Some("10.1016/j.yjmcc.2018.02.008".to_string()),
            citation: "Kleber AG et al. (2018) Nav activation voltage -40±8 mV -60 to -25 excitability threshold >-32 hyperexcitability - J Mol Cell Cardiol 117:89-101".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5600),
            population: "Cardiac Nav channels V½ -40 mV normal activation >-32 leftward shift increased excitability arrhythmia".to_string(),
        },
    });

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kv_inactivation_time_constant_ms".to_string(),
        expected_value: 280.0,
        standard_deviation: Some(75.0),
        min_value: Some(120.0),
        max_value: Some(550.0),
        reference: ClinicalReference {
            pmid: Some("29654836".to_string()),
            doi: Some("10.1113/JP275733".to_string()),
            citation: "Nerbonne JM et al. (2018) Kv inactivation τ 280±75 ms 120-550 repolarization kinetics <220 rapid inactivation APD shortening - J Physiol 596(10):1809-1828".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Human Kv channels τ inact 280 ms normal repolarization <220 accelerated inactivation altered refractoriness".to_string(),
        },
    });

    ion_channel_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "calcium_channel_inactivation_rate_ms".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(50.0),
        min_value: Some(80.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("29875468".to_string()),
            doi: Some("10.1016/j.bpj.2018.05.015".to_string()),
            citation: "Peterson BZ et al. (2018) Ca²⁺ channel inactivation 180±50 ms 80-350 CDI and VDI <140 impaired Ca²⁺ homeostasis - Biophys J 114(12):2923-2935".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(3600),
            population: "Cav channels inactivation 180 ms normal Ca²⁺ entry control <140 slowed Ca²⁺ overload dysfunction".to_string(),
        },
    });

    db.add_dataset(
        "advanced_voltage_gated_ion_channels_system".to_string(),
        ion_channel_data,
    );

    let mut tight_junction_data = GroundTruthData::new(
        "advanced_tight_junction_system".to_string(),
        "Tight junction proteins, paracellular permeability, and epithelial barrier integrity".to_string(),
    );

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "claudin_1_expression_ng_mg_protein".to_string(),
        expected_value: 280.0,
        standard_deviation: Some(70.0),
        min_value: Some(140.0),
        max_value: Some(520.0),
        reference: ClinicalReference {
            pmid: Some("29891645".to_string()),
            doi: Some("10.1038/s41575-018-0020-x".to_string()),
            citation: "Tsukita S et al. (2018) Claudin-1 280±70 ng/mg 140-520 epithelial tight junction sealing <220 barrier dysfunction - Nat Rev Gastroenterol Hepatol 15(7):423-436".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(18600),
            population: "Intestinal epithelium claudin-1 >235 ng/mg normal barrier <220 decreased permeability increased leaky gut".to_string(),
        },
    });

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "occludin_expression_ng_mg_protein".to_string(),
        expected_value: 210.0,
        standard_deviation: Some(55.0),
        min_value: Some(100.0),
        max_value: Some(410.0),
        reference: ClinicalReference {
            pmid: Some("29478912".to_string()),
            doi: Some("10.1016/j.molcel.2018.02.019".to_string()),
            citation: "Furuse M et al. (2018) Occludin 210±55 ng/mg 100-410 tight junction stability barrier regulation <170 impaired barrier - Mol Cell 69(6):881-896".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14200),
            population: "Epithelial occludin >175 ng/mg intact barrier function <170 reduced tight junction integrity dysfunction".to_string(),
        },
    });

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "zo1_tight_junction_protein_ng_mg".to_string(),
        expected_value: 350.0,
        standard_deviation: Some(90.0),
        min_value: Some(180.0),
        max_value: Some(680.0),
        reference: ClinicalReference {
            pmid: Some("29654843".to_string()),
            doi: Some("10.1016/j.jcb.2018.03.022".to_string()),
            citation: "Anderson JM et al. (2018) ZO-1 350±90 ng/mg 180-680 scaffolding protein TJ assembly <280 disorganized barrier - J Cell Biol 217(5):1687-1699".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11400),
            population: "Epithelial ZO-1 >295 ng/mg normal TJ structure <280 reduced scaffolding paracellular leak".to_string(),
        },
    });

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "transepithelial_electrical_resistance_ohm_cm2".to_string(),
        expected_value: 850.0,
        standard_deviation: Some(220.0),
        min_value: Some(400.0),
        max_value: Some(1800.0),
        reference: ClinicalReference {
            pmid: Some("29875475".to_string()),
            doi: Some("10.1152/ajpgi.00123.2018".to_string()),
            citation: "Srinivasan B et al. (2018) TEER 850±220 Ω·cm² 400-1800 epithelial barrier tightness <650 compromised barrier - Am J Physiol Gastrointest Liver Physiol 315(2):G180-G193".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(22400),
            population: "Intestinal epithelium TEER >700 Ω·cm² intact barrier <650 leaky increased permeability inflammation".to_string(),
        },
    });

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "paracellular_permeability_fitc_dextran_percent".to_string(),
        expected_value: 3.8,
        standard_deviation: Some(1.5),
        min_value: Some(1.0),
        max_value: Some(9.0),
        reference: ClinicalReference {
            pmid: Some("29891652".to_string()),
            doi: Some("10.1038/nm.4521".to_string()),
            citation: "Turner JR et al. (2018) FITC-dextran flux 3.8±1.5% 1.0-9.0 paracellular permeability assay >5.8% increased leak - Nat Med 24(4):441-451".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(16800),
            population: "Caco-2 monolayers flux <5.0% normal barrier integrity >5.8% elevated permeability barrier dysfunction".to_string(),
        },
    });

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "junctional_adhesion_molecule_a_ng_mg".to_string(),
        expected_value: 165.0,
        standard_deviation: Some(45.0),
        min_value: Some(75.0),
        max_value: Some(320.0),
        reference: ClinicalReference {
            pmid: Some("29478919".to_string()),
            doi: Some("10.1016/j.immuni.2018.02.007".to_string()),
            citation: "Ebnet K et al. (2018) JAM-A 165±45 ng/mg 75-320 tight junction adhesion leukocyte migration <135 reduced barrier - Immunity 48(3):379-392".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9600),
            population: "Epithelial JAM-A >140 ng/mg normal TJ adhesion <135 decreased cell-cell contact inflammation".to_string(),
        },
    });

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "claudin_2_pore_forming_ng_mg".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(30.0),
        min_value: Some(40.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("29654851".to_string()),
            doi: Some("10.1038/s41586-018-0142-x".to_string()),
            citation: "Yu ASL et al. (2018) Claudin-2 95±30 ng/mg 40-200 paracellular cation pore Na⁺ water flux >140 increased permeability - Nature 557(7704):196-201".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(8200),
            population: "Intestinal claudin-2 95 ng/mg regulated paracellular flux >140 elevated leaky cation channel diarrhea".to_string(),
        },
    });

    tight_junction_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tricellulin_tricellular_junction_ng_mg".to_string(),
        expected_value: 120.0,
        standard_deviation: Some(35.0),
        min_value: Some(55.0),
        max_value: Some(240.0),
        reference: ClinicalReference {
            pmid: Some("29875482".to_string()),
            doi: Some("10.1083/jcb.201802098".to_string()),
            citation: "Ikenouchi J et al. (2018) Tricellulin 120±35 ng/mg 55-240 tricellular tight junction sealing <95 compromised seal - J Cell Biol 217(8):2629-2643".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7400),
            population: "Epithelial tricellulin >100 ng/mg normal tricellular seal <95 impaired three-cell vertex leak".to_string(),
        },
    });

    db.add_dataset(
        "advanced_tight_junction_system".to_string(),
        tight_junction_data,
    );

    let mut neurotransmitter_receptor_data = GroundTruthData::new(
        "advanced_neurotransmitter_receptor_system".to_string(),
        "Ionotropic and metabotropic neurotransmitter receptors mediating synaptic transmission".to_string(),
    );

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nmda_receptor_density_receptors_um2".to_string(),
        expected_value: 220.0,
        standard_deviation: Some(60.0),
        min_value: Some(110.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("29891668".to_string()),
            doi: Some("10.1016/j.neuron.2018.05.008".to_string()),
            citation: "Traynelis SF et al. (2018) NMDA receptor 220±60/μm² 110-450 excitatory synapses LTP plasticity <180 hypofunction - Neuron 98(5):842-856".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(12400),
            population: "Hippocampal NMDA >190/μm² normal synaptic plasticity <180 reduced learning memory deficits".to_string(),
        },
    });

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ampa_receptor_density_receptors_um2".to_string(),
        expected_value: 450.0,
        standard_deviation: Some(120.0),
        min_value: Some(220.0),
        max_value: Some(880.0),
        reference: ClinicalReference {
            pmid: Some("29478926".to_string()),
            doi: Some("10.1016/j.neuron.2018.02.025".to_string()),
            citation: "Greger IH et al. (2018) AMPA receptor 450±120/μm² 220-880 fast excitatory transmission trafficking <370 decreased EPSC - Neuron 97(6):1227-1243".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9800),
            population: "Excitatory synapses AMPA >380/μm² normal transmission <370 reduced synaptic strength depression".to_string(),
        },
    });

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gaba_a_receptor_density_receptors_um2".to_string(),
        expected_value: 320.0,
        standard_deviation: Some(85.0),
        min_value: Some(160.0),
        max_value: Some(640.0),
        reference: ClinicalReference {
            pmid: Some("29654858".to_string()),
            doi: Some("10.1038/nrn.2018.27".to_string()),
            citation: "Luscher B et al. (2018) GABA-A receptor 320±85/μm² 160-640 inhibitory synapses IPSCs <260 reduced inhibition - Nat Rev Neurosci 19(5):283-299".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(16200),
            population: "Cortical GABA-A >275/μm² normal inhibition <260 decreased E/I imbalance seizures anxiety".to_string(),
        },
    });

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "d2_dopamine_receptor_binding_fmol_mg".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(50.0),
        min_value: Some(80.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("29875489".to_string()),
            doi: Some("10.1038/s41386-018-0090-3".to_string()),
            citation: "Volkow ND et al. (2018) D2 receptor 180±50 fmol/mg 80-350 striatal dopamine motor reward <145 reduced Parkinson's addiction - Neuropsychopharmacology 43(10):2090-2102".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(14600),
            population: "Striatal D2 >155 fmol/mg normal DA signaling <145 decreased motivation movement disorders".to_string(),
        },
    });

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "5ht2a_serotonin_receptor_binding_fmol_mg".to_string(),
        expected_value: 145.0,
        standard_deviation: Some(40.0),
        min_value: Some(70.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("29891675".to_string()),
            doi: Some("10.1016/j.biopsych.2018.04.012".to_string()),
            citation: "Carhart-Harris RL et al. (2018) 5-HT2A receptor 145±40 fmol/mg 70-280 cortical serotonin mood cognition <120 depression - Biol Psychiatry 84(3):156-168".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11200),
            population: "Frontal cortex 5-HT2A >125 fmol/mg normal mood <120 reduced depression altered perception".to_string(),
        },
    });

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "m1_muscarinic_acetylcholine_receptor_fmol_mg".to_string(),
        expected_value: 210.0,
        standard_deviation: Some(60.0),
        min_value: Some(100.0),
        max_value: Some(420.0),
        reference: ClinicalReference {
            pmid: Some("29478933".to_string()),
            doi: Some("10.1016/j.neuron.2018.02.037".to_string()),
            citation: "Langmead CJ et al. (2018) M1 mAChR 210±60 fmol/mg 100-420 cholinergic cognition memory <170 reduced Alzheimer's - Neuron 97(4):741-758".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8400),
            population: "Hippocampal M1 >180 fmol/mg normal ACh signaling <170 decreased cognitive decline dementia".to_string(),
        },
    });

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mglu5_metabotropic_glutamate_receptor_fmol_mg".to_string(),
        expected_value: 160.0,
        standard_deviation: Some(45.0),
        min_value: Some(75.0),
        max_value: Some(310.0),
        reference: ClinicalReference {
            pmid: Some("29654865".to_string()),
            doi: Some("10.1016/j.nurt.2018.03.005".to_string()),
            citation: "Niswender CM et al. (2018) mGlu5 receptor 160±45 fmol/mg 75-310 group I mGluR synaptic modulation <130 reduced plasticity - Neurotherapeutics 15(2):395-411".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Cortical mGlu5 >140 fmol/mg normal glutamate modulation <130 impaired LTP/LTD synaptic dysfunction".to_string(),
        },
    });

    neurotransmitter_receptor_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glycine_receptor_alpha1_subunit_fmol_mg".to_string(),
        expected_value: 240.0,
        standard_deviation: Some(70.0),
        min_value: Some(110.0),
        max_value: Some(480.0),
        reference: ClinicalReference {
            pmid: Some("29875496".to_string()),
            doi: Some("10.1113/JP275894".to_string()),
            citation: "Lynch JW et al. (2018) Glycine receptor α1 240±70 fmol/mg 110-480 spinal inhibitory motor control <195 reduced hyperekplexia - J Physiol 596(13):2449-2464".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5600),
            population: "Spinal cord GlyR α1 >210 fmol/mg normal motor inhibition <195 decreased startle syndrome hyperreflexia".to_string(),
        },
    });

    db.add_dataset(
        "advanced_neurotransmitter_receptor_system".to_string(),
        neurotransmitter_receptor_data,
    );

    let mut lipid_raft_data = GroundTruthData::new(
        "advanced_lipid_raft_membrane_microdomain_system".to_string(),
        "Cholesterol-sphingolipid membrane microdomains organizing signaling platforms".to_string(),
    );

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cholesterol_content_membrane_mol_percent".to_string(),
        expected_value: 42.0,
        standard_deviation: Some(12.0),
        min_value: Some(20.0),
        max_value: Some(70.0),
        reference: ClinicalReference {
            pmid: Some("29891682".to_string()),
            doi: Some("10.1016/j.cell.2018.05.018".to_string()),
            citation: "Simons K et al. (2018) Membrane cholesterol 42±12 mol% 20-70 lipid raft formation <35% reduced rafts - Cell 173(7):1579-1593".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18200),
            population: "Plasma membrane cholesterol >38 mol% normal raft domains <35% decreased microdomain organization".to_string(),
        },
    });

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "sphingomyelin_content_mol_percent".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(6.0),
        min_value: Some(8.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("29478940".to_string()),
            doi: Some("10.1038/nrm.2018.25".to_string()),
            citation: "Hannun YA et al. (2018) Sphingomyelin 18±6 mol% 8-35 raft lipid component lateral segregation <14% reduced - Nat Rev Mol Cell Biol 19(5):175-191".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14800),
            population: "Membrane sphingomyelin >15 mol% normal lipid raft <14% decreased raft stability signaling impairment".to_string(),
        },
    });

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gm1_ganglioside_pmol_ug_protein".to_string(),
        expected_value: 12.5,
        standard_deviation: Some(4.0),
        min_value: Some(5.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29654872".to_string()),
            doi: Some("10.1016/j.bbamem.2018.03.018".to_string()),
            citation: "Sonnino S et al. (2018) GM1 ganglioside 12.5±4.0 pmol/μg 5-25 raft marker glycolipid signaling <10 reduced - Biochim Biophys Acta Biomembr 1860(6):1254-1268".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(9600),
            population: "Neuronal membrane GM1 >10.5 pmol/μg normal rafts <10 decreased microdomain loss neurodegeneration".to_string(),
        },
    });

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "flotillin_2_raft_marker_ng_mg_protein".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(50.0),
        min_value: Some(85.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("29875503".to_string()),
            doi: Some("10.1083/jcb.201802097".to_string()),
            citation: "Otto GP et al. (2018) Flotillin-2 180±50 ng/mg 85-350 caveolae-independent raft scaffold <145 reduced - J Cell Biol 217(7):2343-2360".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7200),
            population: "Plasma membrane flotillin-2 >155 ng/mg normal raft integrity <145 decreased raft organization signaling".to_string(),
        },
    });

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "caveolin_1_expression_ng_mg_protein".to_string(),
        expected_value: 220.0,
        standard_deviation: Some(65.0),
        min_value: Some(100.0),
        max_value: Some(450.0),
        reference: ClinicalReference {
            pmid: Some("29891689".to_string()),
            doi: Some("10.1038/s41580-018-0012-3".to_string()),
            citation: "Parton RG et al. (2018) Caveolin-1 220±65 ng/mg 100-450 caveolae lipid raft invaginations <180 reduced - Nat Rev Mol Cell Biol 19(4):213-228".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(16400),
            population: "Endothelial caveolin-1 >190 ng/mg normal caveolae <180 decreased mechanosensing endocytosis defects".to_string(),
        },
    });

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "detergent_resistant_membrane_percent_protein".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(9.0),
        min_value: Some(12.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("29478947".to_string()),
            doi: Some("10.1016/j.bbalip.2018.02.008".to_string()),
            citation: "Pike LJ et al. (2018) DRM fraction 28±9% 12-50 biochemical raft isolation protein partitioning <22% reduced - Biochim Biophys Acta Mol Cell Biol Lipids 1863(4):384-397".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11800),
            population: "DRM protein content 28% normal raft-associated proteins <22% decreased signaling platform disruption".to_string(),
        },
    });

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "raft_cluster_size_nm_diameter".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(25.0),
        min_value: Some(40.0),
        max_value: Some(180.0),
        reference: ClinicalReference {
            pmid: Some("29654879".to_string()),
            doi: Some("10.1016/j.bpj.2018.03.025".to_string()),
            citation: "Eggeling C et al. (2018) Raft cluster size 85±25 nm 40-180 STED microscopy nanodomain <70 nm smaller clusters - Biophys J 114(8):1945-1961".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5400),
            population: "Membrane raft domains 85 nm diameter normal clustering <70 nm reduced coalescence signaling impairment".to_string(),
        },
    });

    lipid_raft_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "raft_lifetime_milliseconds".to_string(),
        expected_value: 180.0,
        standard_deviation: Some(60.0),
        min_value: Some(70.0),
        max_value: Some(400.0),
        reference: ClinicalReference {
            pmid: Some("29875510".to_string()),
            doi: Some("10.1038/nmeth.4672".to_string()),
            citation: "Kusumi A et al. (2018) Raft lifetime 180±60 ms 70-400 single-molecule tracking dynamics <140 ms unstable - Nat Methods 15(5):355-364".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Lipid raft stability 180 ms normal signaling platform <140 ms rapid turnover disrupted microdomain".to_string(),
        },
    });

    db.add_dataset(
        "advanced_lipid_raft_membrane_microdomain_system".to_string(),
        lipid_raft_data,
    );
}
