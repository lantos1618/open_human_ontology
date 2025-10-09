use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Oncogene {
    MYC,
    RAS,
    BRAF,
    HER2,
    EGFR,
    ALK,
    RET,
    NTRK,
    PIK3CA,
    AKT,
    KRAS,
    NRAS,
    HRAS,
}

impl Oncogene {
    pub fn normal_function(&self) -> &str {
        match self {
            Oncogene::MYC => "Cell cycle progression, transcription factor",
            Oncogene::RAS | Oncogene::KRAS | Oncogene::NRAS | Oncogene::HRAS =>
                "Signal transduction, GTPase",
            Oncogene::BRAF => "Serine/threonine kinase, MAPK pathway",
            Oncogene::HER2 | Oncogene::EGFR => "Receptor tyrosine kinase",
            Oncogene::ALK | Oncogene::RET | Oncogene::NTRK =>
                "Receptor tyrosine kinase, fusion protein",
            Oncogene::PIK3CA => "PI3K pathway, lipid kinase",
            Oncogene::AKT => "Serine/threonine kinase, survival signaling",
        }
    }

    pub fn common_mutation(&self) -> &str {
        match self {
            Oncogene::MYC => "Amplification, translocation",
            Oncogene::KRAS => "G12C, G12D, G12V, G13D",
            Oncogene::BRAF => "V600E, V600K",
            Oncogene::HER2 => "Amplification",
            Oncogene::EGFR => "L858R, Exon 19 deletion",
            Oncogene::PIK3CA => "H1047R, E545K",
            _ => "Various activating mutations",
        }
    }

    pub fn targeted_therapy(&self) -> Option<&str> {
        match self {
            Oncogene::HER2 => Some("Trastuzumab, Pertuzumab"),
            Oncogene::EGFR => Some("Osimertinib, Erlotinib"),
            Oncogene::BRAF => Some("Vemurafenib, Dabrafenib"),
            Oncogene::ALK => Some("Alectinib, Crizotinib"),
            Oncogene::KRAS => Some("Sotorasib (G12C)"),
            Oncogene::RET => Some("Selpercatinib"),
            Oncogene::NTRK => Some("Larotrectinib"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TumorSuppressorGene {
    TP53,
    BRCA1,
    BRCA2,
    PTEN,
    RB1,
    APC,
    VHL,
    NF1,
    NF2,
    CDKN2A,
    STK11,
    TSC1,
    TSC2,
    WT1,
    ATM,
}

impl TumorSuppressorGene {
    pub fn normal_function(&self) -> &str {
        match self {
            TumorSuppressorGene::TP53 => "Guardian of the genome, apoptosis, cell cycle arrest",
            TumorSuppressorGene::BRCA1 | TumorSuppressorGene::BRCA2 =>
                "DNA repair, homologous recombination",
            TumorSuppressorGene::PTEN => "PI3K pathway inhibitor, tumor suppressor",
            TumorSuppressorGene::RB1 => "Cell cycle regulation, G1/S checkpoint",
            TumorSuppressorGene::APC => "Wnt pathway inhibitor, colon development",
            TumorSuppressorGene::VHL => "HIF degradation, oxygen sensing",
            TumorSuppressorGene::NF1 | TumorSuppressorGene::NF2 => "Neurofibromatosis genes",
            TumorSuppressorGene::CDKN2A => "p16/p14ARF, cell cycle inhibitor",
            TumorSuppressorGene::STK11 => "LKB1, AMPK activation",
            TumorSuppressorGene::TSC1 | TumorSuppressorGene::TSC2 => "mTOR inhibition",
            TumorSuppressorGene::WT1 => "Wilms tumor suppressor",
            TumorSuppressorGene::ATM => "DNA damage response, checkpoint kinase",
        }
    }

    pub fn associated_cancer(&self) -> &str {
        match self {
            TumorSuppressorGene::TP53 => "50% of all cancers, Li-Fraumeni syndrome",
            TumorSuppressorGene::BRCA1 | TumorSuppressorGene::BRCA2 =>
                "Breast, ovarian, pancreatic, prostate",
            TumorSuppressorGene::PTEN => "Breast, prostate, thyroid (Cowden syndrome)",
            TumorSuppressorGene::RB1 => "Retinoblastoma, osteosarcoma",
            TumorSuppressorGene::APC => "Colorectal (FAP)",
            TumorSuppressorGene::VHL => "Renal cell carcinoma, hemangioblastoma",
            TumorSuppressorGene::NF1 => "Neurofibromas, gliomas",
            TumorSuppressorGene::NF2 => "Acoustic neuromas, meningiomas",
            TumorSuppressorGene::CDKN2A => "Melanoma, pancreatic",
            TumorSuppressorGene::STK11 => "Peutz-Jeghers syndrome",
            TumorSuppressorGene::TSC1 | TumorSuppressorGene::TSC2 => "Tuberous sclerosis",
            TumorSuppressorGene::WT1 => "Wilms tumor",
            TumorSuppressorGene::ATM => "Ataxia telangiectasia, breast cancer",
        }
    }

    pub fn loss_mechanism(&self) -> &str {
        "Loss of function, deletion, truncation, missense mutation"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DNARepairGene {
    MLH1,
    MSH2,
    MSH6,
    PMS2,
    BRCA1,
    BRCA2,
    ATM,
    CHEK2,
    PALB2,
    RAD51,
}

impl DNARepairGene {
    pub fn pathway(&self) -> &str {
        match self {
            DNARepairGene::MLH1 | DNARepairGene::MSH2 |
            DNARepairGene::MSH6 | DNARepairGene::PMS2 =>
                "Mismatch repair (MMR)",
            DNARepairGene::BRCA1 | DNARepairGene::BRCA2 |
            DNARepairGene::PALB2 | DNARepairGene::RAD51 =>
                "Homologous recombination (HR)",
            DNARepairGene::ATM | DNARepairGene::CHEK2 =>
                "DNA damage checkpoint",
        }
    }

    pub fn deficiency_syndrome(&self) -> &str {
        match self {
            DNARepairGene::MLH1 | DNARepairGene::MSH2 |
            DNARepairGene::MSH6 | DNARepairGene::PMS2 =>
                "Lynch syndrome (HNPCC)",
            DNARepairGene::BRCA1 | DNARepairGene::BRCA2 =>
                "Hereditary breast-ovarian cancer syndrome",
            DNARepairGene::ATM => "Ataxia telangiectasia",
            _ => "Increased cancer risk",
        }
    }

    pub fn therapeutic_target(&self) -> Option<&str> {
        match self {
            DNARepairGene::BRCA1 | DNARepairGene::BRCA2 =>
                Some("PARP inhibitors (olaparib, rucaparib)"),
            DNARepairGene::MLH1 | DNARepairGene::MSH2 |
            DNARepairGene::MSH6 | DNARepairGene::PMS2 =>
                Some("Immune checkpoint inhibitors (pembrolizumab)"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TumorMutationalBurden {
    pub mutations_per_megabase: f64,
    pub microsatellite_instability: bool,
    pub mismatch_repair_deficient: bool,
}

impl TumorMutationalBurden {
    pub fn new(mutations_per_mb: f64) -> Self {
        TumorMutationalBurden {
            mutations_per_megabase: mutations_per_mb,
            microsatellite_instability: false,
            mismatch_repair_deficient: false,
        }
    }

    pub fn is_high(&self) -> bool {
        self.mutations_per_megabase >= 10.0
    }

    pub fn immunotherapy_response_likely(&self) -> bool {
        self.is_high() || self.microsatellite_instability || self.mismatch_repair_deficient
    }

    pub fn classification(&self) -> &str {
        if self.mutations_per_megabase < 6.0 {
            "TMB-Low"
        } else if self.mutations_per_megabase < 10.0 {
            "TMB-Intermediate"
        } else if self.mutations_per_megabase < 20.0 {
            "TMB-High"
        } else {
            "TMB-Very High"
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerCell {
    pub oncogenes_activated: Vec<Oncogene>,
    pub tumor_suppressors_inactivated: Vec<TumorSuppressorGene>,
    pub dna_repair_defects: Vec<DNARepairGene>,
    pub proliferation_rate: f64,
    pub apoptosis_resistance: f64,
    pub angiogenesis_active: bool,
    pub metastatic_potential: f64,
    pub tmb: TumorMutationalBurden,
}

impl CancerCell {
    pub fn new() -> Self {
        CancerCell {
            oncogenes_activated: Vec::new(),
            tumor_suppressors_inactivated: Vec::new(),
            dna_repair_defects: Vec::new(),
            proliferation_rate: 1.0,
            apoptosis_resistance: 0.0,
            angiogenesis_active: false,
            metastatic_potential: 0.0,
            tmb: TumorMutationalBurden::new(1.0),
        }
    }

    pub fn activate_oncogene(&mut self, oncogene: Oncogene) {
        if !self.oncogenes_activated.contains(&oncogene) {
            self.oncogenes_activated.push(oncogene);
            self.proliferation_rate *= 1.5;
        }
    }

    pub fn inactivate_tumor_suppressor(&mut self, tsg: TumorSuppressorGene) {
        if !self.tumor_suppressors_inactivated.contains(&tsg) {
            self.tumor_suppressors_inactivated.push(tsg);
            self.apoptosis_resistance += 0.2;

            if tsg == TumorSuppressorGene::TP53 {
                self.apoptosis_resistance += 0.3;
            }
        }
    }

    pub fn hallmarks_count(&self) -> u32 {
        let mut count = 0;

        if self.proliferation_rate > 1.5 {
            count += 1;
        }
        if self.apoptosis_resistance > 0.5 {
            count += 1;
        }
        if self.angiogenesis_active {
            count += 1;
        }
        if self.metastatic_potential > 0.3 {
            count += 1;
        }
        if !self.oncogenes_activated.is_empty() {
            count += 1;
        }
        if !self.tumor_suppressors_inactivated.is_empty() {
            count += 1;
        }

        count
    }

    pub fn malignancy_score(&self) -> f64 {
        let oncogene_score = self.oncogenes_activated.len() as f64 * 0.15;
        let tsg_score = self.tumor_suppressors_inactivated.len() as f64 * 0.20;
        let prolif_score = (self.proliferation_rate - 1.0) * 0.10;
        let apoptosis_score = self.apoptosis_resistance * 0.20;
        let metastatic_score = self.metastatic_potential * 0.25;
        let angiogenesis_score = if self.angiogenesis_active { 0.10 } else { 0.0 };

        (oncogene_score + tsg_score + prolif_score + apoptosis_score + metastatic_score + angiogenesis_score).min(1.0)
    }
}

impl Default for CancerCell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetastasisStage {
    Local,
    Invasion,
    Intravasation,
    Circulation,
    Extravasation,
    Micrometastasis,
    Colonization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetastaticCascade {
    pub stage: MetastasisStage,
    pub epithelial_mesenchymal_transition: bool,
    pub matrix_metalloproteinases_active: bool,
    pub circulating_tumor_cells: u32,
    pub distant_sites_colonized: Vec<String>,
}

impl MetastaticCascade {
    pub fn new() -> Self {
        MetastaticCascade {
            stage: MetastasisStage::Local,
            epithelial_mesenchymal_transition: false,
            matrix_metalloproteinases_active: false,
            circulating_tumor_cells: 0,
            distant_sites_colonized: Vec::new(),
        }
    }

    pub fn initiate_emt(&mut self) {
        self.epithelial_mesenchymal_transition = true;
        self.stage = MetastasisStage::Invasion;
    }

    pub fn invade_local_tissue(&mut self) {
        self.matrix_metalloproteinases_active = true;
        self.stage = MetastasisStage::Invasion;
    }

    pub fn intravasate(&mut self, cell_count: u32) {
        self.stage = MetastasisStage::Intravasation;
        self.circulating_tumor_cells += cell_count;
    }

    pub fn extravasate(&mut self, organ: String) {
        self.stage = MetastasisStage::Extravasation;
        if !self.distant_sites_colonized.contains(&organ) {
            self.distant_sites_colonized.push(organ);
        }
    }

    pub fn metastatic_burden(&self) -> u32 {
        self.distant_sites_colonized.len() as u32
    }
}

impl Default for MetastaticCascade {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TumorMicroenvironment {
    pub cancer_associated_fibroblasts: f64,
    pub tumor_associated_macrophages: f64,
    pub regulatory_t_cells: f64,
    pub cytotoxic_t_cells: f64,
    pub hypoxia_level: f64,
    pub ph_level: f64,
    pub vegf_concentration: f64,
}

impl TumorMicroenvironment {
    pub fn new() -> Self {
        TumorMicroenvironment {
            cancer_associated_fibroblasts: 0.2,
            tumor_associated_macrophages: 0.1,
            regulatory_t_cells: 0.05,
            cytotoxic_t_cells: 0.1,
            hypoxia_level: 0.2,
            ph_level: 7.4,
            vegf_concentration: 0.0,
        }
    }

    pub fn immunosuppressive_index(&self) -> f64 {
        (self.regulatory_t_cells + self.tumor_associated_macrophages) /
        (self.cytotoxic_t_cells + 0.01)
    }

    pub fn is_immunosuppressive(&self) -> bool {
        self.immunosuppressive_index() > 1.0
    }

    pub fn angiogenic_potential(&self) -> f64 {
        self.vegf_concentration * (1.0 + self.hypoxia_level)
    }
}

impl Default for TumorMicroenvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oncogenes() {
        let kras = Oncogene::KRAS;
        assert_eq!(kras.common_mutation(), "G12C, G12D, G12V, G13D");
        assert!(kras.targeted_therapy().is_some());

        let her2 = Oncogene::HER2;
        assert!(her2.targeted_therapy().unwrap().contains("Trastuzumab"));
    }

    #[test]
    fn test_tumor_suppressors() {
        let tp53 = TumorSuppressorGene::TP53;
        assert!(tp53.associated_cancer().contains("Li-Fraumeni"));

        let brca1 = TumorSuppressorGene::BRCA1;
        assert!(brca1.normal_function().contains("DNA repair"));
    }

    #[test]
    fn test_dna_repair_genes() {
        let mlh1 = DNARepairGene::MLH1;
        assert_eq!(mlh1.pathway(), "Mismatch repair (MMR)");
        assert!(mlh1.deficiency_syndrome().contains("Lynch"));
        assert!(mlh1.therapeutic_target().unwrap().contains("pembrolizumab"));

        let brca2 = DNARepairGene::BRCA2;
        assert!(brca2.therapeutic_target().unwrap().contains("PARP"));
    }

    #[test]
    fn test_tumor_mutational_burden() {
        let low_tmb = TumorMutationalBurden::new(3.0);
        assert!(!low_tmb.is_high());
        assert_eq!(low_tmb.classification(), "TMB-Low");

        let high_tmb = TumorMutationalBurden::new(15.0);
        assert!(high_tmb.is_high());
        assert_eq!(high_tmb.classification(), "TMB-High");
    }

    #[test]
    fn test_cancer_cell() {
        let mut cell = CancerCell::new();
        assert_eq!(cell.hallmarks_count(), 0);

        cell.activate_oncogene(Oncogene::KRAS);
        cell.inactivate_tumor_suppressor(TumorSuppressorGene::TP53);

        assert!(cell.proliferation_rate > 1.0);
        assert!(cell.apoptosis_resistance > 0.0);
        assert!(cell.hallmarks_count() >= 2);
    }

    #[test]
    fn test_malignancy_score() {
        let mut cell = CancerCell::new();
        cell.activate_oncogene(Oncogene::KRAS);
        cell.activate_oncogene(Oncogene::MYC);
        cell.inactivate_tumor_suppressor(TumorSuppressorGene::TP53);
        cell.inactivate_tumor_suppressor(TumorSuppressorGene::PTEN);
        cell.angiogenesis_active = true;
        cell.metastatic_potential = 0.7;

        let score = cell.malignancy_score();
        assert!(score > 0.5);
    }

    #[test]
    fn test_metastatic_cascade() {
        let mut cascade = MetastaticCascade::new();
        assert_eq!(cascade.stage, MetastasisStage::Local);

        cascade.initiate_emt();
        assert!(cascade.epithelial_mesenchymal_transition);
        assert_eq!(cascade.stage, MetastasisStage::Invasion);

        cascade.intravasate(1000);
        assert_eq!(cascade.circulating_tumor_cells, 1000);

        cascade.extravasate("Liver".to_string());
        cascade.extravasate("Lung".to_string());
        assert_eq!(cascade.metastatic_burden(), 2);
    }

    #[test]
    fn test_tumor_microenvironment() {
        let mut tme = TumorMicroenvironment::new();

        tme.regulatory_t_cells = 0.3;
        tme.tumor_associated_macrophages = 0.5;
        tme.cytotoxic_t_cells = 0.05;

        assert!(tme.is_immunosuppressive());
        assert!(tme.immunosuppressive_index() > 1.0);
    }

    #[test]
    fn test_angiogenic_potential() {
        let mut tme = TumorMicroenvironment::new();
        tme.vegf_concentration = 100.0;
        tme.hypoxia_level = 0.5;

        let angiogenic = tme.angiogenic_potential();
        assert!(angiogenic > 100.0);
    }
}
