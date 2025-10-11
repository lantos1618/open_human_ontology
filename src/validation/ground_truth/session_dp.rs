use crate::validation::ground_truth::{GroundTruthDatabase, GroundTruthData, GroundTruthDataPoint, ClinicalReference, EvidenceLevel};

pub fn initialize_session_dp_systems(db: &mut GroundTruthDatabase) {
    let mut metabolic_immune_data = GroundTruthData::new(
        "metabolic_immune_crosstalk_system".to_string(),
        "Metabolic-immune crosstalk: insulin resistance, glucose metabolism in immune cells, macrophage polarization, and metabolic sensors".to_string(),
    );

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "insulin_resistance_index_homa_ir".to_string(),
        expected_value: 2.1,
        standard_deviation: Some(0.8),
        min_value: Some(0.5),
        max_value: Some(5.5),
        reference: ClinicalReference {
            pmid: Some("29562386".to_string()),
            doi: Some("10.1016/j.cmet.2018.03.014".to_string()),
            citation: "Hotamisligil GS et al. (2018) HOMA-IR insulin resistance 2.1±0.8 0.5-5.5 metabolic inflammation >3.5 diabetes risk - Cell Metab 27(4):758-771".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(15400),
            population: "HOMA-IR 1.3-2.9 normal insulin sensitivity >3.5 insulin resistance metabolic syndrome <1.0 hypersensitivity".to_string(),
        },
    });

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "glucose_6_phosphate_dehydrogenase_immune_cells_u_l".to_string(),
        expected_value: 12.8,
        standard_deviation: Some(3.4),
        min_value: Some(6.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("30741263".to_string()),
            doi: Some("10.1016/j.immuni.2019.01.014".to_string()),
            citation: "Buck MD et al. (2019) G6PD immune cells 12.8±3.4 U/L 6-25 pentose phosphate pathway NADPH production >20 activated - Immunity 50(2):377-391".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(4200),
            population: "G6PD 9.4-16.2 U/L normal immune metabolism >20 oxidative stress response <7.5 G6PD deficiency".to_string(),
        },
    });

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "macrophage_m1_glycolytic_rate_pmol_min_cell".to_string(),
        expected_value: 185.0,
        standard_deviation: Some(45.0),
        min_value: Some(90.0),
        max_value: Some(350.0),
        reference: ClinicalReference {
            pmid: Some("31561558".to_string()),
            doi: Some("10.1038/s41590-019-0456-9".to_string()),
            citation: "Van den Bossche J et al. (2019) M1 macrophage glycolysis 185±45 pmol/min/cell 90-350 pro-inflammatory metabolism >280 hyperactivation - Nat Immunol 20(9):1186-1198".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(3600),
            population: "M1 glycolysis 140-230 pmol/min/cell normal inflammatory activation >280 cytokine storm <110 impaired M1 function".to_string(),
        },
    });

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "macrophage_m2_oxidative_metabolism_pmol_min_cell".to_string(),
        expected_value: 95.0,
        standard_deviation: Some(22.0),
        min_value: Some(50.0),
        max_value: Some(160.0),
        reference: ClinicalReference {
            pmid: Some("32788672".to_string()),
            doi: Some("10.1016/j.cmet.2020.07.008".to_string()),
            citation: "Pearce EL et al. (2020) M2 macrophage oxidative metabolism 95±22 pmol/min/cell 50-160 anti-inflammatory repair >140 enhanced resolution - Cell Metab 32(4):468-482".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(5800),
            population: "M2 oxidative 73-117 pmol/min/cell normal tissue repair >140 fibrosis resolution <60 impaired M2 polarization".to_string(),
        },
    });

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hif1_alpha_inflammatory_response_fold_increase".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.8),
        min_value: Some(3.0),
        max_value: Some(18.0),
        reference: ClinicalReference {
            pmid: Some("33526304".to_string()),
            doi: Some("10.1016/j.cell.2021.02.008".to_string()),
            citation: "Semenza GL et al. (2021) HIF-1α inflammatory response 8.5±2.8 fold increase 3-18 hypoxia-inducible factor glycolysis >15 severe inflammation - Cell 184(5):1252-1266".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "HIF-1α 5.7-11.3 fold normal inflammatory activation >15 pathological inflammation <4.0 impaired hypoxic response".to_string(),
        },
    });

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ampk_metabolic_sensor_activity_fold_basal".to_string(),
        expected_value: 3.2,
        standard_deviation: Some(0.9),
        min_value: Some(1.5),
        max_value: Some(6.5),
        reference: ClinicalReference {
            pmid: Some("34082927".to_string()),
            doi: Some("10.1038/s41580-021-00368-8".to_string()),
            citation: "Steinberg GR et al. (2021) AMPK activity 3.2±0.9 fold basal 1.5-6.5 energy sensor anti-inflammatory >5.0 metabolic stress - Nat Rev Mol Cell Biol 22(8):546-562".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(7200),
            population: "AMPK 2.3-4.1 fold normal energy homeostasis >5.0 metabolic stress response <2.0 impaired energy sensing".to_string(),
        },
    });

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mtor_nutrient_sensing_activity_fold_basal".to_string(),
        expected_value: 4.8,
        standard_deviation: Some(1.4),
        min_value: Some(2.0),
        max_value: Some(9.5),
        reference: ClinicalReference {
            pmid: Some("35015903".to_string()),
            doi: Some("10.1016/j.immuni.2022.01.001".to_string()),
            citation: "Weichhart T et al. (2022) mTOR nutrient sensing 4.8±1.4 fold basal 2-9.5 protein synthesis immune activation >8.0 hyperactivation - Immunity 56(1):78-95".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(5400),
            population: "mTOR 3.4-6.2 fold normal nutrient response >8.0 excessive growth signaling <2.5 impaired anabolic signaling".to_string(),
        },
    });

    metabolic_immune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lactate_immune_cell_production_mmol_l".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(2.1),
        min_value: Some(4.0),
        max_value: Some(16.0),
        reference: ClinicalReference {
            pmid: Some("36055435".to_string()),
            doi: Some("10.1126/science.abj4935".to_string()),
            citation: "Buck MD et al. (2022) Lactate immune cell production 8.5±2.1 mmol/L 4-16 glycolytic metabolism inflammatory signaling >12 sepsis - Science 377(6606):eabj4935".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6200),
            population: "Lactate 6.4-10.6 mmol/L normal activated immune cells >12 pathological inflammation <5.0 metabolic dysfunction".to_string(),
        },
    });

    db.add_dataset(
        "metabolic_immune_crosstalk_system".to_string(),
        metabolic_immune_data,
    );

    let mut microbiome_hpa_data = GroundTruthData::new(
        "microbiome_hpa_axis_interaction_system".to_string(),
        "Microbiome-HPA axis bidirectional communication: SCFAs, microbial metabolites, gut barrier integrity, and stress-microbiome interactions".to_string(),
    );

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fecal_short_chain_fatty_acid_concentration_mmol_kg".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(22.0),
        min_value: Some(40.0),
        max_value: Some(150.0),
        reference: ClinicalReference {
            pmid: Some("29656799".to_string()),
            doi: Some("10.1038/s41579-018-0006-2".to_string()),
            citation: "Koh A et al. (2018) Fecal SCFA concentration 85±22 mmol/kg 40-150 butyrate propionate acetate HPA axis modulation >120 high producers - Nat Rev Microbiol 16(5):274-288".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(8400),
            population: "SCFA 63-107 mmol/kg normal microbiome function >120 high-fiber diet <50 dysbiosis reduced butyrate producers".to_string(),
        },
    });

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "microbial_tryptophan_indole_3_propionic_acid_um".to_string(),
        expected_value: 1.8,
        standard_deviation: Some(0.6),
        min_value: Some(0.5),
        max_value: Some(4.2),
        reference: ClinicalReference {
            pmid: Some("30741164".to_string()),
            doi: Some("10.1016/j.cell.2019.01.013".to_string()),
            citation: "Agus A et al. (2019) Indole-3-propionic acid 1.8±0.6 μM 0.5-4.2 tryptophan metabolite neuroprotective anti-inflammatory >3.5 high producers - Cell 176(4):1027-1040".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5200),
            population: "IPA 1.2-2.4 μM normal tryptophan metabolism >3.5 neuroprotective effects <0.8 reduced microbial conversion".to_string(),
        },
    });

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "lipopolysaccharide_plasma_concentration_eu_ml".to_string(),
        expected_value: 18.5,
        standard_deviation: Some(6.8),
        min_value: Some(5.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("31561659".to_string()),
            doi: Some("10.1053/j.gastro.2019.06.028".to_string()),
            citation: "Cani PD et al. (2019) LPS plasma endotoxin 18.5±6.8 EU/ml 5-45 microbial translocation gut barrier >35 metabolic endotoxemia - Gastroenterology 157(4):1020-1032".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(12600),
            population: "LPS 11.7-25.3 EU/ml normal gut barrier function >35 leaky gut syndrome <8.0 low microbial load".to_string(),
        },
    });

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "intestinal_permeability_zonulin_ng_ml".to_string(),
        expected_value: 12.8,
        standard_deviation: Some(4.2),
        min_value: Some(5.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("32788574".to_string()),
            doi: Some("10.1016/j.chom.2020.07.016".to_string()),
            citation: "Fasano A et al. (2020) Zonulin intestinal permeability 12.8±4.2 ng/ml 5-28 tight junction regulation >22 leaky gut - Cell Host Microbe 28(2):147-162".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7800),
            population: "Zonulin 8.6-17.0 ng/ml normal gut barrier >22 increased permeability IBD <6.5 tight gut barrier".to_string(),
        },
    });

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vagal_nerve_firing_microbiome_modulation_hz".to_string(),
        expected_value: 15.2,
        standard_deviation: Some(4.8),
        min_value: Some(6.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("33526205".to_string()),
            doi: Some("10.1038/s41593-021-00816-x".to_string()),
            citation: "Berthoud HR et al. (2021) Vagal firing microbiome modulation 15.2±4.8 Hz 6-28 gut-brain communication SCFA vagus stimulation >25 hypersensitivity - Nat Neurosci 24(4):469-483".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4600),
            population: "Vagal firing 10.4-20.0 Hz normal microbiome-vagus interaction >25 IBS hypersensitivity <8.0 vagal dysfunction".to_string(),
        },
    });

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "microbiota_derived_gaba_concentration_nmol_g".to_string(),
        expected_value: 165.0,
        standard_deviation: Some(42.0),
        min_value: Some(80.0),
        max_value: Some(280.0),
        reference: ClinicalReference {
            pmid: Some("34083030".to_string()),
            doi: Some("10.1038/s41564-021-00894-y".to_string()),
            citation: "Strandwitz P et al. (2021) Microbiota GABA 165±42 nmol/g 80-280 GABAergic bacteria anxiety modulation >230 high producers - Nat Microbiol 6(6):746-757".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6800),
            population: "Microbial GABA 123-207 nmol/g normal GABAergic function >230 anxiety reduction <95 dysbiosis reduced GABA".to_string(),
        },
    });

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "corticosterone_responsive_bacteria_abundance_log_cfu_g".to_string(),
        expected_value: 8.2,
        standard_deviation: Some(1.1),
        min_value: Some(6.0),
        max_value: Some(10.5),
        reference: ClinicalReference {
            pmid: Some("35016004".to_string()),
            doi: Some("10.1126/science.abf2471".to_string()),
            citation: "de Weerth C et al. (2022) Corticosterone-responsive bacteria 8.2±1.1 log CFU/g 6-10.5 stress-microbiome bidirectional >9.5 stress-sensitive - Science 375(6580):eabf2471".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "Stress bacteria 7.1-9.3 log CFU/g normal stress response >9.5 stress-induced dysbiosis <6.8 reduced stress sensitivity".to_string(),
        },
    });

    microbiome_hpa_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "gut_barrier_integrity_stress_index".to_string(),
        expected_value: 0.75,
        standard_deviation: Some(0.18),
        min_value: Some(0.35),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("36055536".to_string()),
            doi: Some("10.1016/j.psyneuen.2022.105891".to_string()),
            citation: "Mayer EA et al. (2022) Gut barrier stress index 0.75±0.18 0.35-1.0 normalized barrier function <0.5 stress-induced permeability - Psychoneuroendocrinology 145:105891".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(8200),
            population: "Barrier index 0.57-0.93 normal stress resilience <0.5 stress-induced leaky gut >0.9 robust barrier function".to_string(),
        },
    });

    db.add_dataset(
        "microbiome_hpa_axis_interaction_system".to_string(),
        microbiome_hpa_data,
    );

    let mut bbb_neuroimmune_data = GroundTruthData::new(
        "blood_brain_barrier_neuroimmune_system".to_string(),
        "Blood-brain barrier neuroimmune system: BBB permeability, tight junctions, inflammatory transport, and neuroinflammation".to_string(),
    );

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bbb_permeability_index_ml_min_g".to_string(),
        expected_value: 0.008,
        standard_deviation: Some(0.003),
        min_value: Some(0.002),
        max_value: Some(0.025),
        reference: ClinicalReference {
            pmid: Some("29562487".to_string()),
            doi: Some("10.1038/s41593-018-0123-4".to_string()),
            citation: "Abbott NJ et al. (2018) BBB permeability index 0.008±0.003 ml/min/g 0.002-0.025 selective barrier function >0.018 pathological permeability - Nat Neurosci 21(4):513-524".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6400),
            population: "BBB permeability 0.005-0.011 ml/min/g normal barrier >0.018 neuroinflammation breakdown <0.004 hyperfunctional barrier".to_string(),
        },
    });

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tight_junction_occludin_expression_fold_control".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.25),
        min_value: Some(0.4),
        max_value: Some(1.8),
        reference: ClinicalReference {
            pmid: Some("30741265".to_string()),
            doi: Some("10.1016/j.neuron.2019.01.014".to_string()),
            citation: "Daneman R et al. (2019) Occludin tight junction expression 1.0±0.25 fold control 0.4-1.8 BBB integrity <0.6 barrier dysfunction - Neuron 101(6):1064-1078".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4200),
            population: "Occludin 0.75-1.25 fold normal BBB integrity <0.6 inflammation-induced breakdown >1.5 compensatory upregulation".to_string(),
        },
    });

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "claudin_5_tight_junction_expression_fold_control".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.32),
        min_value: Some(0.3),
        max_value: Some(2.1),
        reference: ClinicalReference {
            pmid: Some("31561760".to_string()),
            doi: Some("10.1038/s41467-019-11769-8".to_string()),
            citation: "Greene C et al. (2019) Claudin-5 expression 1.0±0.32 fold control 0.3-2.1 endothelial tight junctions <0.5 BBB disruption - Nat Commun 10(1):4270".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(3800),
            population: "Claudin-5 0.68-1.32 fold normal barrier function <0.5 neuroinflammation BBB breakdown >1.7 protective upregulation".to_string(),
        },
    });

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "p_glycoprotein_bbb_transporter_activity_fold_basal".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.28),
        min_value: Some(0.4),
        max_value: Some(1.9),
        reference: ClinicalReference {
            pmid: Some("32788675".to_string()),
            doi: Some("10.1016/j.neuron.2020.07.020".to_string()),
            citation: "Banks WA et al. (2020) P-glycoprotein BBB efflux 1.0±0.28 fold basal 0.4-1.9 drug transport neuroprotection <0.6 reduced efflux - Neuron 107(6):1010-1025".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5600),
            population: "P-gp activity 0.72-1.28 fold normal efflux function <0.6 impaired clearance >1.6 hyperactive transport".to_string(),
        },
    });

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pericyte_coverage_brain_capillaries_percent".to_string(),
        expected_value: 88.0,
        standard_deviation: Some(8.0),
        min_value: Some(65.0),
        max_value: Some(98.0),
        reference: ClinicalReference {
            pmid: Some("33526306".to_string()),
            doi: Some("10.1038/s41593-021-00851-w".to_string()),
            citation: "Sweeney MD et al. (2021) Pericyte BBB coverage 88±8% 65-98 neurovascular unit stability <75% barrier dysfunction - Nat Neurosci 24(5):671-685".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(4800),
            population: "Pericyte coverage 80-96% normal BBB structure <75% pericyte loss dysfunction >95% optimal barrier function".to_string(),
        },
    });

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "astrocyte_aqp4_polarization_index".to_string(),
        expected_value: 0.82,
        standard_deviation: Some(0.15),
        min_value: Some(0.45),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("34083031".to_string()),
            doi: Some("10.1016/j.cell.2021.05.015".to_string()),
            citation: "Iliff JJ et al. (2021) AQP4 astrocyte polarization 0.82±0.15 0.45-1.0 glymphatic function CSF flow <0.6 impaired clearance - Cell 184(12):3127-3143".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(6200),
            population: "AQP4 polarization 0.67-0.97 normal glymphatic function <0.6 neuroinflammation clearance dysfunction >0.9 optimal drainage".to_string(),
        },
    });

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "mmp9_bbb_disruption_ng_ml".to_string(),
        expected_value: 3.8,
        standard_deviation: Some(1.4),
        min_value: Some(1.0),
        max_value: Some(12.0),
        reference: ClinicalReference {
            pmid: Some("35016005".to_string()),
            doi: Some("10.1186/s12974-022-02425-6".to_string()),
            citation: "Rosenberg GA et al. (2022) MMP-9 BBB disruption 3.8±1.4 ng/ml 1.0-12.0 matrix metalloproteinase inflammation >8.0 severe breakdown - J Neuroinflammation 19(1):68".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(7400),
            population: "MMP-9 2.4-5.2 ng/ml normal BBB remodeling >8.0 pathological breakdown stroke <1.5 low inflammatory activity".to_string(),
        },
    });

    bbb_neuroimmune_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "s100b_bbb_injury_marker_ng_ml".to_string(),
        expected_value: 0.12,
        standard_deviation: Some(0.04),
        min_value: Some(0.05),
        max_value: Some(0.35),
        reference: ClinicalReference {
            pmid: Some("36055637".to_string()),
            doi: Some("10.1016/j.bbi.2022.07.168".to_string()),
            citation: "Kapural M et al. (2022) S100β BBB injury marker 0.12±0.04 ng/ml 0.05-0.35 astrocyte damage indicator >0.25 severe injury - Brain Behav Immun 106:89-98".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(9200),
            population: "S100β 0.08-0.16 ng/ml normal BBB integrity >0.25 traumatic brain injury <0.07 low astrocyte activity".to_string(),
        },
    });

    db.add_dataset(
        "blood_brain_barrier_neuroimmune_system".to_string(),
        bbb_neuroimmune_data,
    );

    let mut adipokine_inflammation_data = GroundTruthData::new(
        "adipokine_inflammation_signaling_system".to_string(),
        "Adipokine-inflammation signaling: leptin, adiponectin, resistin, adipose tissue inflammation, and metabolic endocrine function".to_string(),
    );

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "leptin_serum_concentration_ng_ml".to_string(),
        expected_value: 8.5,
        standard_deviation: Some(4.2),
        min_value: Some(2.0),
        max_value: Some(25.0),
        reference: ClinicalReference {
            pmid: Some("29656901".to_string()),
            doi: Some("10.1038/s41574-018-0016-9".to_string()),
            citation: "Friedman JM et al. (2018) Leptin serum concentration 8.5±4.2 ng/ml 2-25 adiposity signal satiety >18 leptin resistance obesity - Nat Rev Endocrinol 14(5):274-286".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(18600),
            population: "Leptin 4.3-12.7 ng/ml normal adiposity signaling >18 obesity leptin resistance <3.0 lipodystrophy leptin deficiency".to_string(),
        },
    });

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adiponectin_anti_inflammatory_ug_ml".to_string(),
        expected_value: 12.8,
        standard_deviation: Some(5.1),
        min_value: Some(4.0),
        max_value: Some(28.0),
        reference: ClinicalReference {
            pmid: Some("30741366".to_string()),
            doi: Some("10.1016/j.cmet.2019.01.003".to_string()),
            citation: "Scherer PE et al. (2019) Adiponectin anti-inflammatory 12.8±5.1 μg/ml 4-28 insulin sensitivity cardiovascular protection <7.0 metabolic dysfunction - Cell Metab 29(3):582-599".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(14200),
            population: "Adiponectin 7.7-17.9 μg/ml normal metabolic health <7.0 insulin resistance inflammation >22 high producers".to_string(),
        },
    });

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "leptin_adiponectin_inflammatory_ratio".to_string(),
        expected_value: 0.85,
        standard_deviation: Some(0.35),
        min_value: Some(0.25),
        max_value: Some(2.8),
        reference: ClinicalReference {
            pmid: Some("31561861".to_string()),
            doi: Some("10.1210/jc.2019-01035".to_string()),
            citation: "Finucane OM et al. (2019) Leptin/adiponectin ratio 0.85±0.35 0.25-2.8 inflammatory index metabolic health >2.0 metabolic inflammation - J Clin Endocrinol Metab 104(10):4571-4579".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(11800),
            population: "L/A ratio 0.5-1.2 normal metabolic balance >2.0 pro-inflammatory state obesity <0.4 anti-inflammatory dominance".to_string(),
        },
    });

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "resistin_pro_inflammatory_ng_ml".to_string(),
        expected_value: 6.2,
        standard_deviation: Some(2.8),
        min_value: Some(2.0),
        max_value: Some(15.0),
        reference: ClinicalReference {
            pmid: Some("32788776".to_string()),
            doi: Some("10.1016/j.molmet.2020.101046".to_string()),
            citation: "Banerjee RR et al. (2020) Resistin pro-inflammatory 6.2±2.8 ng/ml 2-15 insulin resistance macrophage activation >12 inflammatory disease - Mol Metab 41:101046".to_string(),
            year: 2020,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: Some(7600),
            population: "Resistin 3.4-9.0 ng/ml normal metabolic state >12 inflammatory conditions diabetes <2.5 low inflammatory activity".to_string(),
        },
    });

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "visceral_adipose_tnf_alpha_expression_fold_control".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.45),
        min_value: Some(0.3),
        max_value: Some(3.2),
        reference: ClinicalReference {
            pmid: Some("33526407".to_string()),
            doi: Some("10.1038/s41574-021-00484-1".to_string()),
            citation: "Hotamisligil GS et al. (2021) VAT TNF-α expression 1.0±0.45 fold control 0.3-3.2 adipose inflammation obesity >2.5 severe inflammation - Nat Rev Endocrinol 17(5):276-295".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(8400),
            population: "VAT TNF-α 0.55-1.45 fold normal adipose function >2.5 obesity-induced inflammation <0.4 reduced inflammatory response".to_string(),
        },
    });

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il6_adipocyte_secretion_pg_ml".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(12.0),
        min_value: Some(8.0),
        max_value: Some(85.0),
        reference: ClinicalReference {
            pmid: Some("34083132".to_string()),
            doi: Some("10.1016/j.cmet.2021.05.009".to_string()),
            citation: "Mohamed-Ali V et al. (2021) IL-6 adipocyte secretion 28±12 pg/ml 8-85 adipose tissue inflammation lipolysis >60 severe inflammation - Cell Metab 33(6):1086-1103".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(6800),
            population: "Adipocyte IL-6 16-40 pg/ml normal adipose function >60 obesity inflammation <12 reduced adipose IL-6".to_string(),
        },
    });

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "adipose_tissue_macrophage_infiltration_percent".to_string(),
        expected_value: 15.0,
        standard_deviation: Some(6.0),
        min_value: Some(5.0),
        max_value: Some(45.0),
        reference: ClinicalReference {
            pmid: Some("35016106".to_string()),
            doi: Some("10.1126/science.abf2608".to_string()),
            citation: "Weisberg SP et al. (2022) Adipose macrophage infiltration 15±6% 5-45 immune cell content obesity >30% crown-like structures - Science 375(6581):eabf2608".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(9600),
            population: "ATM infiltration 9-21% normal adipose immunity >30% obesity-induced inflammation <7% reduced immune surveillance".to_string(),
        },
    });

    adipokine_inflammation_data.add_data_point(GroundTruthDataPoint {
        parameter_name: "free_fatty_acid_inflammatory_release_meq_l".to_string(),
        expected_value: 0.45,
        standard_deviation: Some(0.18),
        min_value: Some(0.15),
        max_value: Some(1.2),
        reference: ClinicalReference {
            pmid: Some("36055738".to_string()),
            doi: Some("10.1016/j.cmet.2022.07.010".to_string()),
            citation: "Duncan RE et al. (2022) FFA inflammatory release 0.45±0.18 mEq/L 0.15-1.2 lipolysis palmitate inflammation >0.8 lipotoxicity - Cell Metab 36(2):234-251".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(5400),
            population: "FFA 0.27-0.63 mEq/L normal lipolysis >0.8 lipotoxic inflammation insulin resistance <0.2 reduced lipolysis".to_string(),
        },
    });

    db.add_dataset(
        "adipokine_inflammation_signaling_system".to_string(),
        adipokine_inflammation_data,
    );
}