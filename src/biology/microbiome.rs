use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Microbiome {
    pub location: MicrobiomeLocation,
    pub bacteria: Vec<Bacteria>,
    pub diversity_index: f64,
    pub total_count_cfu_ml: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MicrobiomeLocation {
    Gut,
    Skin,
    OralCavity,
    Vaginal,
    Respiratory,
    UrinaryTract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bacteria {
    pub species: String,
    pub phylum: BacterialPhylum,
    pub relative_abundance_percent: f64,
    pub metabolic_functions: Vec<MetabolicFunction>,
    pub is_pathogenic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BacterialPhylum {
    Firmicutes,
    Bacteroidetes,
    Actinobacteria,
    Proteobacteria,
    Verrucomicrobia,
    Fusobacteria,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicFunction {
    ShortChainFattyAcidProduction,
    VitaminSynthesis,
    BileAcidMetabolism,
    CarbohydrateDigestion,
    ProteinDigestion,
    ImmuneMudulation,
    PathogenInhibition,
}

impl Microbiome {
    pub fn new_healthy_gut() -> Self {
        Self {
            location: MicrobiomeLocation::Gut,
            bacteria: vec![
                Bacteria::new_bacteroides_fragilis(),
                Bacteria::new_faecalibacterium_prausnitzii(),
                Bacteria::new_akkermansia_muciniphila(),
                Bacteria::new_lactobacillus_acidophilus(),
                Bacteria::new_bifidobacterium_longum(),
                Bacteria::new_escherichia_coli(),
            ],
            diversity_index: 3.5,
            total_count_cfu_ml: 1e11,
        }
    }

    pub fn firmicutes_bacteroidetes_ratio(&self) -> f64 {
        let firmicutes: f64 = self
            .bacteria
            .iter()
            .filter(|b| b.phylum == BacterialPhylum::Firmicutes)
            .map(|b| b.relative_abundance_percent)
            .sum();

        let bacteroidetes: f64 = self
            .bacteria
            .iter()
            .filter(|b| b.phylum == BacterialPhylum::Bacteroidetes)
            .map(|b| b.relative_abundance_percent)
            .sum();

        if bacteroidetes > 0.0 {
            firmicutes / bacteroidetes
        } else {
            f64::INFINITY
        }
    }

    pub fn has_dysbiosis(&self) -> bool {
        let fb_ratio = self.firmicutes_bacteroidetes_ratio();
        !(0.1..=10.0).contains(&fb_ratio) || self.diversity_index < 2.0
    }

    pub fn pathogenic_load(&self) -> f64 {
        self.bacteria
            .iter()
            .filter(|b| b.is_pathogenic)
            .map(|b| b.relative_abundance_percent)
            .sum()
    }

    pub fn beneficial_bacteria_count(&self) -> usize {
        self.bacteria
            .iter()
            .filter(|b| !b.is_pathogenic && b.relative_abundance_percent > 1.0)
            .count()
    }
}

impl Bacteria {
    pub fn new_bacteroides_fragilis() -> Self {
        Self {
            species: "Bacteroides fragilis".to_string(),
            phylum: BacterialPhylum::Bacteroidetes,
            relative_abundance_percent: 25.0,
            metabolic_functions: vec![
                MetabolicFunction::ShortChainFattyAcidProduction,
                MetabolicFunction::VitaminSynthesis,
            ],
            is_pathogenic: false,
        }
    }

    pub fn new_faecalibacterium_prausnitzii() -> Self {
        Self {
            species: "Faecalibacterium prausnitzii".to_string(),
            phylum: BacterialPhylum::Firmicutes,
            relative_abundance_percent: 20.0,
            metabolic_functions: vec![
                MetabolicFunction::ShortChainFattyAcidProduction,
                MetabolicFunction::ImmuneMudulation,
            ],
            is_pathogenic: false,
        }
    }

    pub fn new_akkermansia_muciniphila() -> Self {
        Self {
            species: "Akkermansia muciniphila".to_string(),
            phylum: BacterialPhylum::Verrucomicrobia,
            relative_abundance_percent: 5.0,
            metabolic_functions: vec![
                MetabolicFunction::ImmuneMudulation,
                MetabolicFunction::PathogenInhibition,
            ],
            is_pathogenic: false,
        }
    }

    pub fn new_lactobacillus_acidophilus() -> Self {
        Self {
            species: "Lactobacillus acidophilus".to_string(),
            phylum: BacterialPhylum::Firmicutes,
            relative_abundance_percent: 10.0,
            metabolic_functions: vec![
                MetabolicFunction::VitaminSynthesis,
                MetabolicFunction::PathogenInhibition,
            ],
            is_pathogenic: false,
        }
    }

    pub fn new_bifidobacterium_longum() -> Self {
        Self {
            species: "Bifidobacterium longum".to_string(),
            phylum: BacterialPhylum::Actinobacteria,
            relative_abundance_percent: 15.0,
            metabolic_functions: vec![
                MetabolicFunction::CarbohydrateDigestion,
                MetabolicFunction::VitaminSynthesis,
            ],
            is_pathogenic: false,
        }
    }

    pub fn new_escherichia_coli() -> Self {
        Self {
            species: "Escherichia coli".to_string(),
            phylum: BacterialPhylum::Proteobacteria,
            relative_abundance_percent: 1.0,
            metabolic_functions: vec![MetabolicFunction::VitaminSynthesis],
            is_pathogenic: false,
        }
    }

    pub fn new_clostridium_difficile() -> Self {
        Self {
            species: "Clostridium difficile".to_string(),
            phylum: BacterialPhylum::Firmicutes,
            relative_abundance_percent: 0.1,
            metabolic_functions: vec![],
            is_pathogenic: true,
        }
    }

    pub fn new_helicobacter_pylori() -> Self {
        Self {
            species: "Helicobacter pylori".to_string(),
            phylum: BacterialPhylum::Proteobacteria,
            relative_abundance_percent: 0.5,
            metabolic_functions: vec![],
            is_pathogenic: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortChainFattyAcids {
    pub butyrate_mm: f64,
    pub acetate_mm: f64,
    pub propionate_mm: f64,
}

impl ShortChainFattyAcids {
    pub fn from_healthy_microbiome() -> Self {
        Self {
            butyrate_mm: 20.0,
            acetate_mm: 60.0,
            propionate_mm: 25.0,
        }
    }

    pub fn total_scfa(&self) -> f64 {
        self.butyrate_mm + self.acetate_mm + self.propionate_mm
    }

    pub fn butyrate_ratio(&self) -> f64 {
        self.butyrate_mm / self.total_scfa()
    }

    pub fn is_healthy(&self) -> bool {
        let total = self.total_scfa();
        total > 80.0 && self.butyrate_ratio() > 0.15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microbiome_creation() {
        let gut = Microbiome::new_healthy_gut();
        assert_eq!(gut.location, MicrobiomeLocation::Gut);
        assert!(!gut.bacteria.is_empty());
    }

    #[test]
    fn test_fb_ratio() {
        let gut = Microbiome::new_healthy_gut();
        let ratio = gut.firmicutes_bacteroidetes_ratio();
        assert!(ratio > 0.0 && ratio < 10.0);
    }

    #[test]
    fn test_dysbiosis() {
        let gut = Microbiome::new_healthy_gut();
        assert!(!gut.has_dysbiosis());
    }

    #[test]
    fn test_scfa() {
        let scfa = ShortChainFattyAcids::from_healthy_microbiome();
        assert!(scfa.is_healthy());
        assert!(scfa.butyrate_ratio() > 0.15);
    }
}
