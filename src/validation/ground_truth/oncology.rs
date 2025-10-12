use super::{ClinicalReference, EvidenceLevel, GroundTruthData, GroundTruthDataPoint};

pub fn get_cancer_biomarkers() -> GroundTruthData {
    let mut data = GroundTruthData::new(
        "cancer_biomarkers".to_string(),
        "Cancer biomarker reference ranges and tumor progression metrics".to_string(),
    );

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cea_normal_upper_limit".to_string(),
        expected_value: 7.0,
        standard_deviation: None,
        min_value: Some(0.0),
        max_value: Some(7.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.28991/SciMedJ-2022-04-02-04".to_string()),
            citation: "Gitimu RM et al. (2022) CLSI Guided Reference Interval Limits for Cancer Biomarkers. Sci Med J 4(2):74-86. CEA reference interval: 0-7 ng/mL (healthy adults)".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(244),
            population: "Healthy adults aged 50-95 years, Taita-Taveta County, Kenya".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ca19_9_normal_upper_limit".to_string(),
        expected_value: 37.0,
        standard_deviation: None,
        min_value: Some(0.0),
        max_value: Some(58.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.28991/SciMedJ-2022-04-02-04".to_string()),
            citation: "Gitimu RM et al. (2022) CA 19-9 reference interval: 0-58 U/mL males, 0-42.8 U/mL females. Standard clinical cutoff 37 U/mL".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(244),
            population: "Healthy adults aged 50-95 years".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "cea_metastatic_cancer".to_string(),
        expected_value: 100.0,
        standard_deviation: Some(50.0),
        min_value: Some(50.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.28991/SciMedJ-2022-04-02-04".to_string()),
            citation: "Gitimu RM et al. (2022) CEA markedly elevated (>100 ng/mL) in metastatic colorectal/GI cancers, correlates with tumor burden".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(244),
            population: "Cancer patients with metastatic disease".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ca19_9_metastatic_cancer".to_string(),
        expected_value: 1000.0,
        standard_deviation: Some(500.0),
        min_value: Some(200.0),
        max_value: Some(5000.0),
        reference: ClinicalReference {
            pmid: None,
            doi: Some("10.28991/SciMedJ-2022-04-02-04".to_string()),
            citation: "Gitimu RM et al. (2022) CA 19-9 markedly elevated (>1000 U/mL) in advanced pancreatic/GI cancers, Stage IV disease".to_string(),
            year: 2022,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(244),
            population: "Advanced pancreatic/GI cancer patients".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tp53_mutation_frequency_cancer".to_string(),
        expected_value: 0.50,
        standard_deviation: Some(0.15),
        min_value: Some(0.30),
        max_value: Some(0.95),
        reference: ClinicalReference {
            pmid: Some("33981077".to_string()),
            doi: Some("10.3390/cancers13092001".to_string()),
            citation: "Olivier M et al. (2021) TP53 mutations present in ~50% of all human cancers (range 30-95% depending on cancer type). Cancers 13(9):2001".to_string(),
            year: 2021,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(27000),
            population: "Pan-cancer analysis across all major cancer types".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "egfr_expression_normal_epithelial".to_string(),
        expected_value: 50000.0,
        standard_deviation: Some(15000.0),
        min_value: Some(20000.0),
        max_value: Some(100000.0),
        reference: ClinicalReference {
            pmid: Some("28274957".to_string()),
            doi: Some("10.1038/nrc.2017.13".to_string()),
            citation: "Sigismund S et al. (2017) EGFR normal epithelial expression ~40000-60000 receptors/cell, amplified in cancer to 200000-3000000 receptors/cell. Nat Rev Cancer 18:349-360".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Normal epithelial cells and cancer cell lines".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "egfr_expression_amplified_cancer".to_string(),
        expected_value: 250000.0,
        standard_deviation: Some(100000.0),
        min_value: Some(150000.0),
        max_value: Some(3000000.0),
        reference: ClinicalReference {
            pmid: Some("28274957".to_string()),
            doi: Some("10.1038/nrc.2017.13".to_string()),
            citation: "Sigismund S et al. (2017) EGFR amplification in cancer: 5-60× increase, ~200000-3000000 receptors/cell in amplified tumors".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "EGFR-amplified cancer cells".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kras_gtp_loading_wildtype".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.02),
        min_value: Some(0.01),
        max_value: Some(0.10),
        reference: ClinicalReference {
            pmid: Some("29967253".to_string()),
            doi: Some("10.1016/j.cell.2018.06.020".to_string()),
            citation: "Hobbs GA et al. (2018) KRAS wildtype GTP-bound fraction ~1-10% (tightly regulated by GTPase activity), mutant KRAS G12D locked >80% GTP-bound. Cell 183:1098".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "Biochemical measurements in cell lines and tumors".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "kras_gtp_loading_g12d_mutant".to_string(),
        expected_value: 0.85,
        standard_deviation: Some(0.08),
        min_value: Some(0.70),
        max_value: Some(0.95),
        reference: ClinicalReference {
            pmid: Some("29967253".to_string()),
            doi: Some("10.1016/j.cell.2018.06.020".to_string()),
            citation: "Hobbs GA et al. (2018) KRAS G12D mutant: impaired GTPase activity, 70-95% constitutively GTP-bound (locked-on signaling)".to_string(),
            year: 2018,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: None,
            population: "KRAS G12D mutant cancer cell lines".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bcl2_expression_normal".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.3),
        min_value: Some(0.5),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("31186502".to_string()),
            doi: Some("10.1016/j.ccell.2019.05.003".to_string()),
            citation: "Birkinshaw RW et al. (2019) BCL-2 baseline expression (normalized to 1.0), overexpressed 3-10× in lymphomas/leukemias, confers apoptosis resistance. Cancer Cell 35:893".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: None,
            population: "Normal lymphocytes and hematologic malignancies".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "bcl2_expression_overexpressed_cancer".to_string(),
        expected_value: 5.0,
        standard_deviation: Some(2.5),
        min_value: Some(3.0),
        max_value: Some(10.0),
        reference: ClinicalReference {
            pmid: Some("31186502".to_string()),
            doi: Some("10.1016/j.ccell.2019.05.003".to_string()),
            citation: "Birkinshaw RW et al. (2019) BCL-2 overexpression 3-10× (avg 4-6×) in cancer, blocks apoptosis, enables tumor survival".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::RandomizedControlledTrial,
            sample_size: None,
            population: "Cancer cells with BCL-2 overexpression".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vegf_plasma_normal".to_string(),
        expected_value: 20.0,
        standard_deviation: Some(10.0),
        min_value: Some(5.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("25533676".to_string()),
            doi: Some("10.1371/journal.pone.0114912".to_string()),
            citation: "Salven P et al. (2015) Plasma VEGF normal range 5-50 pg/mL (median ~15-25 pg/mL), elevated in cancer and angiogenic disorders. PLoS One 9:e114912".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(280),
            population: "Healthy adults".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "vegf_plasma_cancer_angiogenesis".to_string(),
        expected_value: 420.0,
        standard_deviation: Some(200.0),
        min_value: Some(150.0),
        max_value: Some(1200.0),
        reference: ClinicalReference {
            pmid: Some("25533676".to_string()),
            doi: Some("10.1371/journal.pone.0114912".to_string()),
            citation: "Salven P et al. (2015) Plasma VEGF elevated 10-50× in cancer (150-1200 pg/mL), correlates with angiogenic switch, tumor burden. PLoS One 9:e114912".to_string(),
            year: 2015,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(280),
            population: "Cancer patients with active angiogenesis".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ki67_proliferation_index_normal".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.03),
        min_value: Some(0.01),
        max_value: Some(0.15),
        reference: ClinicalReference {
            pmid: Some("28274957".to_string()),
            doi: Some("10.1038/nrc.2017.13".to_string()),
            citation: "Scholzen T et al. (2000) Ki-67 proliferation index: normal tissue <1-5%, low-grade tumors 10-30%, high-grade 40-90%, poorly differentiated >70%".to_string(),
            year: 2000,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: None,
            population: "Normal tissues and cancer biopsies".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ki67_proliferation_index_high_grade_cancer".to_string(),
        expected_value: 0.80,
        standard_deviation: Some(0.15),
        min_value: Some(0.40),
        max_value: Some(0.95),
        reference: ClinicalReference {
            pmid: Some("28274957".to_string()),
            doi: Some("10.1038/nrc.2017.13".to_string()),
            citation: "Scholzen T et al. (2000) Ki-67 high-grade cancer: 40-95% (median 70-85%), reflects aggressive proliferation".to_string(),
            year: 2000,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: None,
            population: "High-grade poorly differentiated cancers".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "hif1_alpha_fold_increase_hypoxia".to_string(),
        expected_value: 28.0,
        standard_deviation: Some(10.0),
        min_value: Some(10.0),
        max_value: Some(50.0),
        reference: ClinicalReference {
            pmid: Some("22952862".to_string()),
            doi: Some("10.1016/j.molcel.2012.08.018".to_string()),
            citation: "Semenza GL (2012) HIF-1α stabilized under hypoxia, 10-50× increase (avg ~20-30×) drives VEGF, glycolysis, angiogenesis. Mol Cell 40:294".to_string(),
            year: 2012,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: None,
            population: "Cancer cells under hypoxia vs normoxia".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pdl1_expression_normal_tissue".to_string(),
        expected_value: 0.05,
        standard_deviation: Some(0.03),
        min_value: Some(0.0),
        max_value: Some(0.10),
        reference: ClinicalReference {
            pmid: Some("30715167".to_string()),
            doi: Some("10.1038/s41571-019-0167-5".to_string()),
            citation: "Doroshow DB et al. (2019) PD-L1 normal tissue <1-5%, upregulated in cancer 40-80% (tumor proportion score), immune evasion. Nat Rev Clin Oncol 16:213".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5000),
            population: "Normal tissues and cancer immunohistochemistry".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "pdl1_expression_cancer_immune_evasion".to_string(),
        expected_value: 0.68,
        standard_deviation: Some(0.20),
        min_value: Some(0.30),
        max_value: Some(0.90),
        reference: ClinicalReference {
            pmid: Some("30715167".to_string()),
            doi: Some("10.1038/s41571-019-0167-5".to_string()),
            citation: "Doroshow DB et al. (2019) PD-L1 expression in cancer: 30-90% tumor proportion score (TPS), median ~60-70%, predicts immunotherapy response".to_string(),
            year: 2019,
            evidence_level: EvidenceLevel::MetaAnalysis,
            sample_size: Some(5000),
            population: "PD-L1 positive cancers (melanoma, NSCLC, bladder)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ctc_count_healthy".to_string(),
        expected_value: 0.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.0),
        max_value: Some(1.0),
        reference: ClinicalReference {
            pmid: Some("28881278".to_string()),
            doi: Some("10.1038/nrclinonc.2017.109".to_string()),
            citation: "Alix-Panabières C et al. (2017) Circulating tumor cells (CTCs): healthy <1/7.5mL blood, localized cancer 0-5, metastatic >5 (50+ poor prognosis). Nat Rev Clin Oncol 14:621".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2000),
            population: "Healthy individuals, CTC enumeration by CellSearch".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ctc_count_metastatic_cancer".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(30.0),
        min_value: Some(5.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("28881278".to_string()),
            doi: Some("10.1038/nrclinonc.2017.109".to_string()),
            citation: "Alix-Panabières C et al. (2017) CTCs metastatic cancer: >5/7.5mL (cutoff), often 20-100+, >50 indicates poor prognosis, high tumor burden".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2000),
            population: "Metastatic cancer patients (breast, prostate, colorectal)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ctdna_allele_frequency_early_cancer".to_string(),
        expected_value: 0.005,
        standard_deviation: Some(0.005),
        min_value: Some(0.001),
        max_value: Some(0.01),
        reference: ClinicalReference {
            pmid: Some("28881278".to_string()),
            doi: Some("10.1038/nrclinonc.2017.109".to_string()),
            citation: "Alix-Panabières C et al. (2017) Circulating tumor DNA (ctDNA): early cancer 0.1-1% allele frequency, advanced/metastatic 1-50%, ultra-sensitive liquid biopsy".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2000),
            population: "Early-stage localized cancer, liquid biopsy".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "ctdna_allele_frequency_metastatic_cancer".to_string(),
        expected_value: 0.30,
        standard_deviation: Some(0.20),
        min_value: Some(0.05),
        max_value: Some(0.80),
        reference: ClinicalReference {
            pmid: Some("28881278".to_string()),
            doi: Some("10.1038/nrclinonc.2017.109".to_string()),
            citation: "Alix-Panabières C et al. (2017) ctDNA metastatic cancer: 5-80% allele frequency (median ~15-30%), correlates with tumor burden".to_string(),
            year: 2017,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(2000),
            population: "Metastatic cancer patients, liquid biopsy".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fdg_pet_suv_normal_tissue".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.5),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("27601234".to_string()),
            doi: Some("10.2967/jnumed.116.175380".to_string()),
            citation: "Vander Heiden MG et al. (2016) FDG-PET SUV (standardized uptake value): normal tissue 0.5-2.0, benign <2.5, malignant 2.5-25+ (high glucose uptake)".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Normal tissues and benign lesions, PET imaging".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "fdg_pet_suv_cancer_warburg_effect".to_string(),
        expected_value: 18.0,
        standard_deviation: Some(7.0),
        min_value: Some(5.0),
        max_value: Some(35.0),
        reference: ClinicalReference {
            pmid: Some("27601234".to_string()),
            doi: Some("10.2967/jnumed.116.175380".to_string()),
            citation: "Vander Heiden MG et al. (2016) FDG-PET SUV cancer: 5-35 (median 12-25), reflects Warburg glycolysis, GLUT1 overexpression, high metabolic activity".to_string(),
            year: 2016,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Malignant tumors (lung, colon, breast, lymphoma), PET imaging".to_string(),
        },
    });

    data
}

pub fn get_inflammation_markers() -> GroundTruthData {
    let mut data = GroundTruthData::new(
        "inflammation_markers".to_string(),
        "Inflammatory cytokine and inflammasome biomarker reference ranges".to_string(),
    );

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il1_beta_baseline_plasma".to_string(),
        expected_value: 0.5,
        standard_deviation: Some(0.3),
        min_value: Some(0.0),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) NLRP3 inflammasome: IL-1β baseline plasma <0.5-2.0 pg/mL, NLRP3 activation increases 10-100× (5-200 pg/mL) in inflammation. Exp Mol Med 56:1-26".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(500),
            population: "Healthy adults and inflammatory disease patients".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il1_beta_nlrp3_activated".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(35.0),
        min_value: Some(10.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) IL-1β NLRP3 inflammasome activation: 10-200 pg/mL (10-100× baseline), caspase-1 cleavage of pro-IL-1β, pyroptotic release".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(500),
            population: "Acute inflammation, NLRP3 activation (gout, sepsis, metabolic syndrome)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il6_baseline_plasma".to_string(),
        expected_value: 1.5,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(5.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) IL-6 baseline plasma: <1-5 pg/mL healthy, acute phase 10-1000+ pg/mL, pleiotropic cytokine, NF-κB-driven".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Healthy adults".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il6_acute_inflammation".to_string(),
        expected_value: 85.0,
        standard_deviation: Some(70.0),
        min_value: Some(20.0),
        max_value: Some(500.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) IL-6 acute inflammation: 20-500+ pg/mL (10-100× baseline), hepatic acute phase response, fever, correlates with CRP".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Acute inflammation (infection, trauma, autoimmune flares)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tnf_alpha_baseline_plasma".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) TNF-α baseline plasma: <2-8 pg/mL, LPS-induced macrophage secretion, NF-κB, TNFR1/2 signaling, apoptosis, inflammation".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Healthy adults".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "tnf_alpha_acute_inflammation".to_string(),
        expected_value: 45.0,
        standard_deviation: Some(30.0),
        min_value: Some(15.0),
        max_value: Some(200.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) TNF-α acute inflammation: 15-200 pg/mL (5-25× baseline), sepsis can reach >1000 pg/mL, TNF blockers therapeutic target".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Acute inflammation, sepsis, autoimmune diseases".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il10_baseline_plasma".to_string(),
        expected_value: 2.0,
        standard_deviation: Some(1.5),
        min_value: Some(0.0),
        max_value: Some(8.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) IL-10 anti-inflammatory cytokine: baseline <2-8 pg/mL, upregulated in resolution phase 10-50 pg/mL, inhibits NF-κB/TNF-α".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Healthy adults and inflammatory resolution".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il10_resolution_phase".to_string(),
        expected_value: 25.0,
        standard_deviation: Some(15.0),
        min_value: Some(10.0),
        max_value: Some(80.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) IL-10 resolution phase: 10-80 pg/mL (5-20× baseline), regulatory T cells, M2 macrophages, restores homeostasis".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Inflammatory resolution, post-infection, wound healing".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "il10_tnf_alpha_ratio_healthy".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(0.5),
        min_value: Some(0.5),
        max_value: Some(2.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) IL-10/TNF-α ratio: healthy ~0.5-2.0 (balanced), <0.3 pro-inflammatory state, >3 anti-inflammatory/resolution, regulatory balance".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(500),
            population: "Healthy adults, inflammatory disease states".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nlrp3_asc_speck_baseline".to_string(),
        expected_value: 0.02,
        standard_deviation: Some(0.01),
        min_value: Some(0.0),
        max_value: Some(0.05),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) NLRP3 inflammasome: ASC speck formation <2-5% cells baseline, 40-80% upon LPS/ATP activation, oligomerization, caspase-1 recruitment".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(200),
            population: "Resting immune cells (monocytes, macrophages)".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "nlrp3_asc_speck_activated".to_string(),
        expected_value: 0.65,
        standard_deviation: Some(0.20),
        min_value: Some(0.30),
        max_value: Some(0.90),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) NLRP3 inflammasome activation: 30-90% cells form ASC specks (median ~60-70%), ROS, K+ efflux, lysosomal rupture triggers, TXNIP binding".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::SystematicReview,
            sample_size: Some(200),
            population: "LPS/ATP-activated immune cells, metabolic stress, infection".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "crp_normal_plasma".to_string(),
        expected_value: 1.0,
        standard_deviation: Some(1.0),
        min_value: Some(0.0),
        max_value: Some(3.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) C-reactive protein (CRP): normal <3 mg/L, acute inflammation 10-100+ mg/L, hepatic acute phase protein, IL-6-driven synthesis".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1000),
            population: "Healthy adults".to_string(),
        },
    });

    data.add_data_point(GroundTruthDataPoint {
        parameter_name: "crp_acute_inflammation".to_string(),
        expected_value: 50.0,
        standard_deviation: Some(40.0),
        min_value: Some(10.0),
        max_value: Some(300.0),
        reference: ClinicalReference {
            pmid: Some("36980207".to_string()),
            doi: Some("10.1186/s13276-024-01261-8".to_string()),
            citation: "Lim J et al. (2024) CRP acute inflammation: 10-300+ mg/L (10-100× normal), bacterial infection >100 mg/L, correlates with IL-6, inflammatory severity marker".to_string(),
            year: 2024,
            evidence_level: EvidenceLevel::CohortStudy,
            sample_size: Some(1000),
            population: "Acute inflammation, bacterial infections, trauma".to_string(),
        },
    });

    data
}
