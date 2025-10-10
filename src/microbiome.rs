use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Microbiome {
    pub gut: GutMicrobiome,
    pub skin: SkinMicrobiome,
    pub oral: OralMicrobiome,
    pub respiratory: RespiratoryMicrobiome,
    pub urogenital: UrogenitalMicrobiome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutMicrobiome {
    pub bacterial_composition: HashMap<String, f64>,
    pub diversity_index: f64,
    pub total_bacterial_count_cfu_g: f64,
    pub dominant_phyla: Vec<Phylum>,
    pub metabolic_capacity: MetabolicCapacity,
    pub dysbiosis_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Phylum {
    Firmicutes,
    Bacteroidetes,
    Actinobacteria,
    Proteobacteria,
    Verrucomicrobia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicCapacity {
    pub short_chain_fatty_acid_production_mmol_day: f64,
    pub butyrate_production_mmol_day: f64,
    pub vitamin_k_synthesis_mcg_day: f64,
    pub vitamin_b12_synthesis_mcg_day: f64,
    pub bile_acid_metabolism: f64,
    pub neurotransmitter_precursor_production: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinMicrobiome {
    pub bacterial_composition: HashMap<String, f64>,
    pub dominant_genera: Vec<String>,
    pub diversity_index: f64,
    pub ph_regulation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OralMicrobiome {
    pub bacterial_composition: HashMap<String, f64>,
    pub plaque_forming_bacteria_percent: f64,
    pub cariogenic_bacteria_percent: f64,
    pub periodontal_pathogen_load: PathogenLoad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathogenLoad {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryMicrobiome {
    pub bacterial_composition: HashMap<String, f64>,
    pub diversity_index: f64,
    pub pathogen_colonization: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrogenitalMicrobiome {
    pub bacterial_composition: HashMap<String, f64>,
    pub lactobacillus_dominance: bool,
    pub ph: f64,
    pub dysbiosis_score: f64,
}

impl Microbiome {
    pub fn new_healthy_adult() -> Self {
        Self {
            gut: GutMicrobiome::new_healthy(),
            skin: SkinMicrobiome::new_healthy(),
            oral: OralMicrobiome::new_healthy(),
            respiratory: RespiratoryMicrobiome::new_healthy(),
            urogenital: UrogenitalMicrobiome::new_healthy(),
        }
    }

    pub fn overall_health_score(&self) -> f64 {
        let gut_score = (1.0 - self.gut.dysbiosis_score) * 0.4;
        let oral_score = match self.oral.periodontal_pathogen_load {
            PathogenLoad::Low => 0.3,
            PathogenLoad::Moderate => 0.15,
            PathogenLoad::High => 0.0,
        };
        let urogenital_score = (1.0 - self.urogenital.dysbiosis_score) * 0.2;
        let skin_score = (self.skin.diversity_index / 5.0).min(1.0) * 0.1;

        gut_score + oral_score + urogenital_score + skin_score
    }
}

impl GutMicrobiome {
    pub fn new_healthy() -> Self {
        let mut composition = HashMap::new();
        composition.insert("Bacteroides".to_string(), 25.0);
        composition.insert("Faecalibacterium".to_string(), 15.0);
        composition.insert("Bifidobacterium".to_string(), 10.0);
        composition.insert("Lactobacillus".to_string(), 8.0);
        composition.insert("Akkermansia".to_string(), 5.0);
        composition.insert("Roseburia".to_string(), 7.0);
        composition.insert("Eubacterium".to_string(), 6.0);
        composition.insert("Other".to_string(), 24.0);

        let mut neurotransmitters = HashMap::new();
        neurotransmitters.insert("Serotonin".to_string(), 0.5);
        neurotransmitters.insert("GABA".to_string(), 0.3);
        neurotransmitters.insert("Dopamine".to_string(), 0.2);

        Self {
            bacterial_composition: composition,
            diversity_index: 3.5,
            total_bacterial_count_cfu_g: 1e11,
            dominant_phyla: vec![Phylum::Firmicutes, Phylum::Bacteroidetes],
            metabolic_capacity: MetabolicCapacity {
                short_chain_fatty_acid_production_mmol_day: 100.0,
                butyrate_production_mmol_day: 20.0,
                vitamin_k_synthesis_mcg_day: 80.0,
                vitamin_b12_synthesis_mcg_day: 2.0,
                bile_acid_metabolism: 1.0,
                neurotransmitter_precursor_production: neurotransmitters,
            },
            dysbiosis_score: 0.0,
        }
    }

    pub fn firmicutes_bacteroidetes_ratio(&self) -> f64 {
        let firmicutes: f64 = self
            .bacterial_composition
            .iter()
            .filter(|(name, _)| {
                name.contains("Faecalibacterium")
                    || name.contains("Roseburia")
                    || name.contains("Eubacterium")
                    || name.contains("Lactobacillus")
            })
            .map(|(_, pct)| pct)
            .sum();

        let bacteroidetes: f64 = self
            .bacterial_composition
            .get("Bacteroides")
            .copied()
            .unwrap_or(0.0);

        if bacteroidetes > 0.0 {
            firmicutes / bacteroidetes
        } else {
            firmicutes
        }
    }

    pub fn assess_dysbiosis(&mut self) {
        let diversity_component = if self.diversity_index < 2.0 {
            0.3
        } else if self.diversity_index < 2.5 {
            0.15
        } else {
            0.0
        };

        let fb_ratio = self.firmicutes_bacteroidetes_ratio();
        let ratio_component = if fb_ratio < 0.5 || fb_ratio > 3.0 {
            0.3
        } else if fb_ratio < 0.8 || fb_ratio > 2.0 {
            0.15
        } else {
            0.0
        };

        let beneficial_bacteria = self
            .bacterial_composition
            .get("Faecalibacterium")
            .copied()
            .unwrap_or(0.0)
            + self
                .bacterial_composition
                .get("Bifidobacterium")
                .copied()
                .unwrap_or(0.0)
            + self
                .bacterial_composition
                .get("Akkermansia")
                .copied()
                .unwrap_or(0.0);

        let beneficial_component = if beneficial_bacteria < 15.0 {
            0.2
        } else if beneficial_bacteria < 25.0 {
            0.1
        } else {
            0.0
        };

        self.dysbiosis_score = (diversity_component + ratio_component + beneficial_component).min(1.0);
    }

    pub fn microbiome_type(&self) -> &str {
        let bacteroides = self
            .bacterial_composition
            .get("Bacteroides")
            .copied()
            .unwrap_or(0.0);
        let prevotella = self
            .bacterial_composition
            .get("Prevotella")
            .copied()
            .unwrap_or(0.0);
        let ruminococcus = self
            .bacterial_composition
            .get("Ruminococcus")
            .copied()
            .unwrap_or(0.0);

        if bacteroides > 20.0 {
            "Bacteroides-dominant (Type 1)"
        } else if prevotella > 15.0 {
            "Prevotella-dominant (Type 2)"
        } else if ruminococcus > 15.0 {
            "Ruminococcus-dominant (Type 3)"
        } else {
            "Mixed"
        }
    }
}

impl SkinMicrobiome {
    pub fn new_healthy() -> Self {
        let mut composition = HashMap::new();
        composition.insert("Cutibacterium".to_string(), 30.0);
        composition.insert("Staphylococcus".to_string(), 25.0);
        composition.insert("Corynebacterium".to_string(), 20.0);
        composition.insert("Micrococcus".to_string(), 10.0);
        composition.insert("Other".to_string(), 15.0);

        Self {
            bacterial_composition: composition,
            dominant_genera: vec![
                "Cutibacterium".to_string(),
                "Staphylococcus".to_string(),
                "Corynebacterium".to_string(),
            ],
            diversity_index: 2.8,
            ph_regulation: 5.5,
        }
    }
}

impl OralMicrobiome {
    pub fn new_healthy() -> Self {
        let mut composition = HashMap::new();
        composition.insert("Streptococcus".to_string(), 30.0);
        composition.insert("Veillonella".to_string(), 15.0);
        composition.insert("Prevotella".to_string(), 12.0);
        composition.insert("Neisseria".to_string(), 10.0);
        composition.insert("Fusobacterium".to_string(), 8.0);
        composition.insert("Other".to_string(), 25.0);

        Self {
            bacterial_composition: composition,
            plaque_forming_bacteria_percent: 15.0,
            cariogenic_bacteria_percent: 5.0,
            periodontal_pathogen_load: PathogenLoad::Low,
        }
    }
}

impl RespiratoryMicrobiome {
    pub fn new_healthy() -> Self {
        let mut composition = HashMap::new();
        composition.insert("Prevotella".to_string(), 25.0);
        composition.insert("Streptococcus".to_string(), 20.0);
        composition.insert("Veillonella".to_string(), 15.0);
        composition.insert("Haemophilus".to_string(), 10.0);
        composition.insert("Other".to_string(), 30.0);

        Self {
            bacterial_composition: composition,
            diversity_index: 3.0,
            pathogen_colonization: Vec::new(),
        }
    }
}

impl UrogenitalMicrobiome {
    pub fn new_healthy() -> Self {
        let mut composition = HashMap::new();
        composition.insert("Lactobacillus_crispatus".to_string(), 50.0);
        composition.insert("Lactobacillus_iners".to_string(), 30.0);
        composition.insert("Lactobacillus_jensenii".to_string(), 10.0);
        composition.insert("Other".to_string(), 10.0);

        Self {
            bacterial_composition: composition,
            lactobacillus_dominance: true,
            ph: 4.0,
            dysbiosis_score: 0.0,
        }
    }

    pub fn assess_bacterial_vaginosis_risk(&self) -> f64 {
        let lactobacillus_total: f64 = self
            .bacterial_composition
            .iter()
            .filter(|(name, _)| name.contains("Lactobacillus"))
            .map(|(_, pct)| pct)
            .sum();

        if lactobacillus_total > 80.0 {
            0.1
        } else if lactobacillus_total > 60.0 {
            0.3
        } else if lactobacillus_total > 40.0 {
            0.5
        } else {
            0.8
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrobiomeIntervention {
    pub intervention_type: InterventionType,
    pub targeted_bacteria: Vec<String>,
    pub expected_effects: Vec<String>,
    pub duration_days: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionType {
    Probiotic,
    Prebiotic,
    Synbiotic,
    AntimicrobialTherapy,
    DietaryModification,
    FecalMicrobiotaTransplant,
}

impl MicrobiomeIntervention {
    pub fn probiotic_lactobacillus() -> Self {
        Self {
            intervention_type: InterventionType::Probiotic,
            targeted_bacteria: vec!["Lactobacillus".to_string()],
            expected_effects: vec![
                "Improve gut barrier function".to_string(),
                "Reduce inflammation".to_string(),
                "Enhance immune response".to_string(),
            ],
            duration_days: 30,
        }
    }

    pub fn prebiotic_fiber() -> Self {
        Self {
            intervention_type: InterventionType::Prebiotic,
            targeted_bacteria: vec![
                "Bifidobacterium".to_string(),
                "Faecalibacterium".to_string(),
            ],
            expected_effects: vec![
                "Increase SCFA production".to_string(),
                "Improve metabolic health".to_string(),
                "Enhance beneficial bacteria".to_string(),
            ],
            duration_days: 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_microbiome_creation() {
        let microbiome = Microbiome::new_healthy_adult();
        assert!(microbiome.gut.diversity_index > 3.0);
        assert_eq!(microbiome.gut.dysbiosis_score, 0.0);
    }

    #[test]
    fn test_gut_microbiome() {
        let gut = GutMicrobiome::new_healthy();
        assert!(gut.diversity_index > 0.0);
        assert!(gut.total_bacterial_count_cfu_g > 1e10);
        assert!(!gut.bacterial_composition.is_empty());
    }

    #[test]
    fn test_fb_ratio() {
        let gut = GutMicrobiome::new_healthy();
        let ratio = gut.firmicutes_bacteroidetes_ratio();
        assert!(ratio > 0.0);
        assert!(ratio < 10.0);
    }

    #[test]
    fn test_dysbiosis_assessment() {
        let mut gut = GutMicrobiome::new_healthy();
        gut.assess_dysbiosis();
        assert!(gut.dysbiosis_score < 0.3);

        gut.diversity_index = 1.5;
        gut.assess_dysbiosis();
        assert!(gut.dysbiosis_score > 0.2);
    }

    #[test]
    fn test_microbiome_type() {
        let gut = GutMicrobiome::new_healthy();
        let mb_type = gut.microbiome_type();
        assert!(mb_type.contains("Type") || mb_type == "Mixed");
    }

    #[test]
    fn test_skin_microbiome() {
        let skin = SkinMicrobiome::new_healthy();
        assert!(!skin.dominant_genera.is_empty());
        assert!(skin.ph_regulation > 4.0 && skin.ph_regulation < 7.0);
    }

    #[test]
    fn test_oral_microbiome() {
        let oral = OralMicrobiome::new_healthy();
        assert_eq!(oral.periodontal_pathogen_load, PathogenLoad::Low);
        assert!(oral.cariogenic_bacteria_percent < 10.0);
    }

    #[test]
    fn test_urogenital_microbiome() {
        let ug = UrogenitalMicrobiome::new_healthy();
        assert!(ug.lactobacillus_dominance);
        assert!(ug.ph < 5.0);
        assert_eq!(ug.dysbiosis_score, 0.0);
    }

    #[test]
    fn test_bv_risk_assessment() {
        let ug = UrogenitalMicrobiome::new_healthy();
        let risk = ug.assess_bacterial_vaginosis_risk();
        assert!(risk < 0.3);
    }

    #[test]
    fn test_overall_health_score() {
        let microbiome = Microbiome::new_healthy_adult();
        let score = microbiome.overall_health_score();
        assert!(score > 0.5);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_metabolic_capacity() {
        let gut = GutMicrobiome::new_healthy();
        assert!(gut.metabolic_capacity.butyrate_production_mmol_day > 0.0);
        assert!(gut.metabolic_capacity.vitamin_k_synthesis_mcg_day > 0.0);
        assert!(!gut
            .metabolic_capacity
            .neurotransmitter_precursor_production
            .is_empty());
    }

    #[test]
    fn test_probiotic_intervention() {
        let intervention = MicrobiomeIntervention::probiotic_lactobacillus();
        assert_eq!(intervention.intervention_type, InterventionType::Probiotic);
        assert!(!intervention.expected_effects.is_empty());
    }

    #[test]
    fn test_prebiotic_intervention() {
        let intervention = MicrobiomeIntervention::prebiotic_fiber();
        assert_eq!(intervention.intervention_type, InterventionType::Prebiotic);
        assert!(intervention.duration_days > 0);
    }
}
