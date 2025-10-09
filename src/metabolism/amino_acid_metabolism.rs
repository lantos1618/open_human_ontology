use serde::{Deserialize, Serialize};
use crate::biology::AminoAcid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AminoAcidMetabolism {
    pub essential_amino_acids: EssentialAminoAcidStatus,
    pub non_essential_amino_acids: NonEssentialAminoAcidStatus,
    pub urea_cycle: UreaCycle,
    pub amino_acid_disorders: Vec<AminoAcidDisorder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssentialAminoAcidStatus {
    pub histidine_umol_l: f64,
    pub isoleucine_umol_l: f64,
    pub leucine_umol_l: f64,
    pub lysine_umol_l: f64,
    pub methionine_umol_l: f64,
    pub phenylalanine_umol_l: f64,
    pub threonine_umol_l: f64,
    pub tryptophan_umol_l: f64,
    pub valine_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonEssentialAminoAcidStatus {
    pub alanine_umol_l: f64,
    pub arginine_umol_l: f64,
    pub asparagine_umol_l: f64,
    pub aspartate_umol_l: f64,
    pub cysteine_umol_l: f64,
    pub glutamate_umol_l: f64,
    pub glutamine_umol_l: f64,
    pub glycine_umol_l: f64,
    pub proline_umol_l: f64,
    pub serine_umol_l: f64,
    pub tyrosine_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UreaCycle {
    pub ammonia_umol_l: f64,
    pub urea_mmol_l: f64,
    pub carbamoyl_phosphate_synthetase_activity: f64,
    pub ornithine_transcarbamylase_activity: f64,
    pub argininosuccinate_synthetase_activity: f64,
    pub argininosuccinate_lyase_activity: f64,
    pub arginase_activity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AminoAcidDisorder {
    Phenylketonuria,
    MapleSyrupUrineDisease,
    Homocystinuria,
    Tyrosinemia,
    AlkaptonuriaOchronosis,
    Cystinuria,
    Hyperammonemia,
    CitrullinemiaTypeI,
    CitrullinemiaTypeII,
    ArgininosuccinicAciduria,
    NonketotichyperglycinemiaGlycineEncephalopathy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchedChainAminoAcidMetabolism {
    pub leucine_umol_l: f64,
    pub isoleucine_umol_l: f64,
    pub valine_umol_l: f64,
    pub bckdh_activity: f64,
    pub alpha_ketoisocaproic_acid_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AromaticAminoAcidMetabolism {
    pub phenylalanine_umol_l: f64,
    pub tyrosine_umol_l: f64,
    pub tryptophan_umol_l: f64,
    pub pah_activity: f64,
    pub phe_tyr_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SulfurAminoAcidMetabolism {
    pub methionine_umol_l: f64,
    pub cysteine_umol_l: f64,
    pub homocysteine_umol_l: f64,
    pub cystathionine_umol_l: f64,
    pub taurine_umol_l: f64,
    pub glutathione_umol_l: f64,
}

impl EssentialAminoAcidStatus {
    pub fn new_normal() -> Self {
        Self {
            histidine_umol_l: 80.0,
            isoleucine_umol_l: 65.0,
            leucine_umol_l: 120.0,
            lysine_umol_l: 180.0,
            methionine_umol_l: 25.0,
            phenylalanine_umol_l: 60.0,
            threonine_umol_l: 120.0,
            tryptophan_umol_l: 50.0,
            valine_umol_l: 220.0,
        }
    }

    pub fn is_deficient(&self, amino_acid: &str) -> bool {
        match amino_acid.to_lowercase().as_str() {
            "histidine" => self.histidine_umol_l < 50.0,
            "isoleucine" => self.isoleucine_umol_l < 40.0,
            "leucine" => self.leucine_umol_l < 70.0,
            "lysine" => self.lysine_umol_l < 100.0,
            "methionine" => self.methionine_umol_l < 15.0,
            "phenylalanine" => self.phenylalanine_umol_l < 35.0,
            "threonine" => self.threonine_umol_l < 80.0,
            "tryptophan" => self.tryptophan_umol_l < 30.0,
            "valine" => self.valine_umol_l < 150.0,
            _ => false,
        }
    }

    pub fn total_bcaa(&self) -> f64 {
        self.leucine_umol_l + self.isoleucine_umol_l + self.valine_umol_l
    }

    pub fn total_aromatic_aa(&self) -> f64 {
        self.phenylalanine_umol_l + self.tryptophan_umol_l
    }
}

impl NonEssentialAminoAcidStatus {
    pub fn new_normal() -> Self {
        Self {
            alanine_umol_l: 350.0,
            arginine_umol_l: 80.0,
            asparagine_umol_l: 45.0,
            aspartate_umol_l: 20.0,
            cysteine_umol_l: 35.0,
            glutamate_umol_l: 60.0,
            glutamine_umol_l: 550.0,
            glycine_umol_l: 240.0,
            proline_umol_l: 180.0,
            serine_umol_l: 110.0,
            tyrosine_umol_l: 65.0,
        }
    }

    pub fn fischer_ratio(&self, bcaa_umol_l: f64) -> f64 {
        let aromatic = self.tyrosine_umol_l;
        if aromatic > 0.0 {
            bcaa_umol_l / aromatic
        } else {
            0.0
        }
    }

    pub fn glutamine_glutamate_ratio(&self) -> f64 {
        if self.glutamate_umol_l > 0.0 {
            self.glutamine_umol_l / self.glutamate_umol_l
        } else {
            0.0
        }
    }
}

impl UreaCycle {
    pub fn new_normal() -> Self {
        Self {
            ammonia_umol_l: 30.0,
            urea_mmol_l: 4.5,
            carbamoyl_phosphate_synthetase_activity: 1.0,
            ornithine_transcarbamylase_activity: 1.0,
            argininosuccinate_synthetase_activity: 1.0,
            argininosuccinate_lyase_activity: 1.0,
            arginase_activity: 1.0,
        }
    }

    pub fn has_hyperammonemia(&self) -> bool {
        self.ammonia_umol_l > 50.0
    }

    pub fn detect_urea_cycle_defect(&self) -> Option<String> {
        if self.carbamoyl_phosphate_synthetase_activity < 0.2 {
            Some("CPS1 deficiency".to_string())
        } else if self.ornithine_transcarbamylase_activity < 0.2 {
            Some("OTC deficiency".to_string())
        } else if self.argininosuccinate_synthetase_activity < 0.2 {
            Some("Citrullinemia type I".to_string())
        } else if self.argininosuccinate_lyase_activity < 0.2 {
            Some("Argininosuccinic aciduria".to_string())
        } else if self.arginase_activity < 0.2 {
            Some("Arginase deficiency".to_string())
        } else {
            None
        }
    }

    pub fn overall_cycle_efficiency(&self) -> f64 {
        let min_activity = self.carbamoyl_phosphate_synthetase_activity
            .min(self.ornithine_transcarbamylase_activity)
            .min(self.argininosuccinate_synthetase_activity)
            .min(self.argininosuccinate_lyase_activity)
            .min(self.arginase_activity);
        min_activity
    }
}

impl BranchedChainAminoAcidMetabolism {
    pub fn new_normal() -> Self {
        Self {
            leucine_umol_l: 120.0,
            isoleucine_umol_l: 65.0,
            valine_umol_l: 220.0,
            bckdh_activity: 1.0,
            alpha_ketoisocaproic_acid_umol_l: 5.0,
        }
    }

    pub fn has_maple_syrup_urine_disease(&self) -> bool {
        self.bckdh_activity < 0.3 &&
        self.leucine_umol_l > 400.0 &&
        self.alpha_ketoisocaproic_acid_umol_l > 20.0
    }

    pub fn total_bcaa(&self) -> f64 {
        self.leucine_umol_l + self.isoleucine_umol_l + self.valine_umol_l
    }

    pub fn leucine_ratio(&self) -> f64 {
        let total = self.total_bcaa();
        if total > 0.0 {
            self.leucine_umol_l / total
        } else {
            0.0
        }
    }
}

impl AromaticAminoAcidMetabolism {
    pub fn new_normal() -> Self {
        Self {
            phenylalanine_umol_l: 60.0,
            tyrosine_umol_l: 65.0,
            tryptophan_umol_l: 50.0,
            pah_activity: 1.0,
            phe_tyr_ratio: 0.92,
        }
    }

    pub fn has_phenylketonuria(&self) -> bool {
        self.phenylalanine_umol_l > 1200.0 && self.phe_tyr_ratio > 3.0
    }

    pub fn has_hyperphenylalaninemia(&self) -> bool {
        self.phenylalanine_umol_l > 120.0 && !self.has_phenylketonuria()
    }

    pub fn calculate_phe_tyr_ratio(&mut self) {
        if self.tyrosine_umol_l > 0.0 {
            self.phe_tyr_ratio = self.phenylalanine_umol_l / self.tyrosine_umol_l;
        }
    }
}

impl SulfurAminoAcidMetabolism {
    pub fn new_normal() -> Self {
        Self {
            methionine_umol_l: 25.0,
            cysteine_umol_l: 35.0,
            homocysteine_umol_l: 8.0,
            cystathionine_umol_l: 1.0,
            taurine_umol_l: 50.0,
            glutathione_umol_l: 900.0,
        }
    }

    pub fn has_hyperhomocysteinemia(&self) -> bool {
        self.homocysteine_umol_l > 15.0
    }

    pub fn has_severe_hyperhomocysteinemia(&self) -> bool {
        self.homocysteine_umol_l > 100.0
    }

    pub fn has_homocystinuria(&self) -> bool {
        self.homocysteine_umol_l > 100.0 && self.methionine_umol_l > 50.0
    }

    pub fn transsulfuration_pathway_flux(&self) -> f64 {
        if self.methionine_umol_l > 0.0 {
            self.cysteine_umol_l / self.methionine_umol_l
        } else {
            0.0
        }
    }

    pub fn redox_capacity(&self) -> f64 {
        self.glutathione_umol_l + self.cysteine_umol_l
    }
}

impl AminoAcidMetabolism {
    pub fn new_normal() -> Self {
        Self {
            essential_amino_acids: EssentialAminoAcidStatus::new_normal(),
            non_essential_amino_acids: NonEssentialAminoAcidStatus::new_normal(),
            urea_cycle: UreaCycle::new_normal(),
            amino_acid_disorders: Vec::new(),
        }
    }

    pub fn assess_protein_status(&self) -> ProteinStatus {
        let total_essential = self.essential_amino_acids.histidine_umol_l
            + self.essential_amino_acids.isoleucine_umol_l
            + self.essential_amino_acids.leucine_umol_l
            + self.essential_amino_acids.lysine_umol_l
            + self.essential_amino_acids.methionine_umol_l
            + self.essential_amino_acids.phenylalanine_umol_l
            + self.essential_amino_acids.threonine_umol_l
            + self.essential_amino_acids.tryptophan_umol_l
            + self.essential_amino_acids.valine_umol_l;

        if total_essential < 500.0 {
            ProteinStatus::Deficient
        } else if total_essential < 700.0 {
            ProteinStatus::Insufficient
        } else if total_essential > 1500.0 {
            ProteinStatus::Excess
        } else {
            ProteinStatus::Adequate
        }
    }

    pub fn detect_metabolic_disorders(&mut self) {
        self.amino_acid_disorders.clear();

        if self.essential_amino_acids.phenylalanine_umol_l > 1200.0 {
            self.amino_acid_disorders.push(AminoAcidDisorder::Phenylketonuria);
        }

        let bcaa_total = self.essential_amino_acids.total_bcaa();
        if bcaa_total > 800.0 {
            self.amino_acid_disorders.push(AminoAcidDisorder::MapleSyrupUrineDisease);
        }

        if self.urea_cycle.has_hyperammonemia() {
            self.amino_acid_disorders.push(AminoAcidDisorder::Hyperammonemia);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProteinStatus {
    Deficient,
    Insufficient,
    Adequate,
    Excess,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_essential_amino_acids_normal() {
        let eaa = EssentialAminoAcidStatus::new_normal();
        assert!(!eaa.is_deficient("leucine"));
        assert!(eaa.total_bcaa() > 350.0);
    }

    #[test]
    fn test_urea_cycle_normal() {
        let urea = UreaCycle::new_normal();
        assert!(!urea.has_hyperammonemia());
        assert!(urea.detect_urea_cycle_defect().is_none());
        assert!(urea.overall_cycle_efficiency() > 0.9);
    }

    #[test]
    fn test_bcaa_metabolism() {
        let bcaa = BranchedChainAminoAcidMetabolism::new_normal();
        assert!(!bcaa.has_maple_syrup_urine_disease());
        assert!(bcaa.total_bcaa() > 350.0);
    }

    #[test]
    fn test_aromatic_aa_metabolism() {
        let mut aromatic = AromaticAminoAcidMetabolism::new_normal();
        assert!(!aromatic.has_phenylketonuria());
        aromatic.calculate_phe_tyr_ratio();
        assert!(aromatic.phe_tyr_ratio < 2.0);
    }

    #[test]
    fn test_sulfur_aa_metabolism() {
        let sulfur = SulfurAminoAcidMetabolism::new_normal();
        assert!(!sulfur.has_hyperhomocysteinemia());
        assert!(!sulfur.has_homocystinuria());
        assert!(sulfur.redox_capacity() > 500.0);
    }

    #[test]
    fn test_amino_acid_metabolism() {
        let mut aa_metabolism = AminoAcidMetabolism::new_normal();
        assert_eq!(aa_metabolism.assess_protein_status(), ProteinStatus::Adequate);
        aa_metabolism.detect_metabolic_disorders();
        assert!(aa_metabolism.amino_acid_disorders.is_empty());
    }

    #[test]
    fn test_pku_detection() {
        let mut aromatic = AromaticAminoAcidMetabolism::new_normal();
        aromatic.phenylalanine_umol_l = 1500.0;
        aromatic.tyrosine_umol_l = 30.0;
        aromatic.calculate_phe_tyr_ratio();
        assert!(aromatic.has_phenylketonuria());
    }
}
