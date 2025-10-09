use serde::{Deserialize, Serialize};
use crate::biology::microbiome::{Microbiome, ShortChainFattyAcids};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutBrainAxis {
    pub vagus_nerve_signaling: f64,
    pub hpa_axis_activity: f64,
    pub serotonin_production_gut: f64,
    pub dopamine_production_gut: f64,
    pub gaba_production_gut: f64,
    pub microbial_metabolites: MicrobialMetabolites,
    pub intestinal_permeability: f64,
    pub inflammation_level: f64,
}

impl GutBrainAxis {
    pub fn new() -> Self {
        GutBrainAxis {
            vagus_nerve_signaling: 1.0,
            hpa_axis_activity: 1.0,
            serotonin_production_gut: 0.95,
            dopamine_production_gut: 0.5,
            gaba_production_gut: 0.3,
            microbial_metabolites: MicrobialMetabolites::new(),
            intestinal_permeability: 0.1,
            inflammation_level: 0.1,
        }
    }

    pub fn assess_from_microbiome(&mut self, microbiome: &Microbiome) {
        if microbiome.has_dysbiosis() {
            self.hpa_axis_activity *= 1.5;
            self.inflammation_level += 0.3;
            self.intestinal_permeability += 0.2;
            self.serotonin_production_gut *= 0.7;
        }

        let pathogenic_load = microbiome.pathogenic_load();
        if pathogenic_load > 10.0 {
            self.vagus_nerve_signaling *= 0.8;
            self.inflammation_level += pathogenic_load * 0.05;
        }
    }

    pub fn is_leaky_gut(&self) -> bool {
        self.intestinal_permeability > 0.3
    }

    pub fn neuroinflammation_risk(&self) -> f64 {
        self.inflammation_level * (1.0 + self.intestinal_permeability)
    }

    pub fn mood_regulation_capacity(&self) -> f64 {
        let serotonin_factor = self.serotonin_production_gut;
        let dopamine_factor = self.dopamine_production_gut;
        let inflammation_penalty = 1.0 - self.inflammation_level.min(0.5);

        (serotonin_factor + dopamine_factor) * inflammation_penalty / 2.0
    }
}

impl Default for GutBrainAxis {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrobialMetabolites {
    pub short_chain_fatty_acids: ShortChainFattyAcids,
    pub tryptophan_metabolites: TryptophanMetabolites,
    pub bile_acid_metabolites: BileAcidMetabolites,
    pub trimethylamine_n_oxide: f64,
    pub lipopolysaccharide: f64,
    pub indole_derivatives: f64,
}

impl MicrobialMetabolites {
    pub fn new() -> Self {
        MicrobialMetabolites {
            short_chain_fatty_acids: ShortChainFattyAcids::from_healthy_microbiome(),
            tryptophan_metabolites: TryptophanMetabolites::new(),
            bile_acid_metabolites: BileAcidMetabolites::new(),
            trimethylamine_n_oxide: 3.0,
            lipopolysaccharide: 0.1,
            indole_derivatives: 10.0,
        }
    }

    pub fn cardiovascular_risk(&self) -> f64 {
        if self.trimethylamine_n_oxide > 6.0 {
            (self.trimethylamine_n_oxide - 3.0) * 0.2
        } else {
            0.0
        }
    }

    pub fn anti_inflammatory_capacity(&self) -> f64 {
        self.short_chain_fatty_acids.butyrate_mm * 0.05 +
        self.indole_derivatives * 0.01
    }
}

impl Default for MicrobialMetabolites {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryptophanMetabolites {
    pub serotonin_gut_um: f64,
    pub kynurenine_um: f64,
    pub indole_um: f64,
    pub indole_3_propionic_acid_um: f64,
}

impl TryptophanMetabolites {
    pub fn new() -> Self {
        TryptophanMetabolites {
            serotonin_gut_um: 100.0,
            kynurenine_um: 2.0,
            indole_um: 5.0,
            indole_3_propionic_acid_um: 1.5,
        }
    }

    pub fn kynurenine_serotonin_ratio(&self) -> f64 {
        if self.serotonin_gut_um > 0.0 {
            self.kynurenine_um / self.serotonin_gut_um
        } else {
            f64::INFINITY
        }
    }

    pub fn neuroprotective_metabolites(&self) -> f64 {
        self.indole_um + self.indole_3_propionic_acid_um
    }
}

impl Default for TryptophanMetabolites {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BileAcidMetabolites {
    pub primary_bile_acids_um: f64,
    pub secondary_bile_acids_um: f64,
    pub lithocholic_acid_um: f64,
    pub deoxycholic_acid_um: f64,
}

impl BileAcidMetabolites {
    pub fn new() -> Self {
        BileAcidMetabolites {
            primary_bile_acids_um: 100.0,
            secondary_bile_acids_um: 50.0,
            lithocholic_acid_um: 10.0,
            deoxycholic_acid_um: 40.0,
        }
    }

    pub fn secondary_to_primary_ratio(&self) -> f64 {
        if self.primary_bile_acids_um > 0.0 {
            self.secondary_bile_acids_um / self.primary_bile_acids_um
        } else {
            f64::INFINITY
        }
    }

    pub fn is_dysregulated(&self) -> bool {
        let ratio = self.secondary_to_primary_ratio();
        ratio > 1.0 || self.lithocholic_acid_um > 20.0
    }
}

impl Default for BileAcidMetabolites {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntericNervousSystem {
    pub neuron_count: u64,
    pub neurotransmitters: EntericNeurotransmitters,
    pub peristalsis_frequency_per_min: f64,
    pub gut_motility_index: f64,
}

impl EntericNervousSystem {
    pub fn new() -> Self {
        EntericNervousSystem {
            neuron_count: 500_000_000,
            neurotransmitters: EntericNeurotransmitters::new(),
            peristalsis_frequency_per_min: 3.0,
            gut_motility_index: 1.0,
        }
    }

    pub fn is_functional(&self) -> bool {
        self.peristalsis_frequency_per_min > 1.0 &&
        self.gut_motility_index > 0.5
    }

    pub fn modulate_by_microbiome(&mut self, scfa_level: f64) {
        if scfa_level > 80.0 {
            self.gut_motility_index = 1.0;
            self.neurotransmitters.serotonin_nm *= 1.2;
        } else {
            self.gut_motility_index *= 0.8;
        }
    }
}

impl Default for EntericNervousSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntericNeurotransmitters {
    pub serotonin_nm: f64,
    pub dopamine_nm: f64,
    pub acetylcholine_nm: f64,
    pub gaba_nm: f64,
    pub norepinephrine_nm: f64,
}

impl EntericNeurotransmitters {
    pub fn new() -> Self {
        EntericNeurotransmitters {
            serotonin_nm: 1000.0,
            dopamine_nm: 100.0,
            acetylcholine_nm: 500.0,
            gaba_nm: 50.0,
            norepinephrine_nm: 30.0,
        }
    }

    pub fn excitatory_inhibitory_balance(&self) -> f64 {
        let excitatory = self.acetylcholine_nm + self.norepinephrine_nm;
        let inhibitory = self.gaba_nm;
        if inhibitory > 0.0 {
            excitatory / inhibitory
        } else {
            f64::INFINITY
        }
    }
}

impl Default for EntericNeurotransmitters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gut_brain_axis() {
        let gba = GutBrainAxis::new();
        assert_eq!(gba.vagus_nerve_signaling, 1.0);
        assert!(!gba.is_leaky_gut());
    }

    #[test]
    fn test_dysbiosis_effects() {
        let mut gba = GutBrainAxis::new();
        let mut microbiome = Microbiome::new_healthy_gut();

        microbiome.diversity_index = 1.0;
        gba.assess_from_microbiome(&microbiome);

        assert!(gba.hpa_axis_activity > 1.0);
    }

    #[test]
    fn test_mood_regulation() {
        let gba = GutBrainAxis::new();
        let mood_capacity = gba.mood_regulation_capacity();
        assert!(mood_capacity > 0.5);
    }

    #[test]
    fn test_microbial_metabolites() {
        let metabolites = MicrobialMetabolites::new();
        assert!(metabolites.short_chain_fatty_acids.is_healthy());
        assert!(metabolites.cardiovascular_risk() < 0.5);
    }

    #[test]
    fn test_tryptophan_metabolites() {
        let trp = TryptophanMetabolites::new();
        assert!(trp.kynurenine_serotonin_ratio() < 0.1);
        assert!(trp.neuroprotective_metabolites() > 5.0);
    }

    #[test]
    fn test_bile_acid_metabolites() {
        let ba = BileAcidMetabolites::new();
        assert!(!ba.is_dysregulated());
        assert!(ba.secondary_to_primary_ratio() < 1.0);
    }

    #[test]
    fn test_enteric_nervous_system() {
        let mut ens = EntericNervousSystem::new();
        assert!(ens.is_functional());

        ens.modulate_by_microbiome(100.0);
        assert_eq!(ens.gut_motility_index, 1.0);
    }

    #[test]
    fn test_enteric_neurotransmitters() {
        let nt = EntericNeurotransmitters::new();
        let balance = nt.excitatory_inhibitory_balance();
        assert!(balance > 0.0);
    }
}
