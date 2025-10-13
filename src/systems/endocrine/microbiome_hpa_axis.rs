use serde::{Deserialize, Serialize};

/// Microbiome-HPA axis interaction system representing bidirectional
/// gut-brain-stress communication pathways
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicrobiomeHPAAxis {
    /// Fecal short-chain fatty acid concentration (mmol/kg)
    pub fecal_short_chain_fatty_acid_concentration_mmol_kg: f64,

    /// Microbial tryptophan metabolite indole-3-propionic acid (μM)
    pub microbial_tryptophan_indole_3_propionic_acid_um: f64,

    /// Lipopolysaccharide plasma concentration - microbial translocation (EU/ml)
    pub lipopolysaccharide_plasma_concentration_eu_ml: f64,

    /// Intestinal permeability zonulin levels (ng/ml)
    pub intestinal_permeability_zonulin_ng_ml: f64,

    /// Vagal nerve firing rate modulated by microbiome (Hz)
    pub vagal_nerve_firing_microbiome_modulation_hz: f64,

    /// Microbiota-derived GABA concentration (nmol/g)
    pub microbiota_derived_gaba_concentration_nmol_g: f64,

    /// Corticosterone-responsive bacteria abundance (log CFU/g)
    pub corticosterone_responsive_bacteria_abundance_log_cfu_g: f64,

    /// Gut barrier integrity under stress (normalized index)
    pub gut_barrier_integrity_stress_index: f64,
}

impl Default for MicrobiomeHPAAxis {
    fn default() -> Self {
        Self {
            fecal_short_chain_fatty_acid_concentration_mmol_kg: 85.0,
            microbial_tryptophan_indole_3_propionic_acid_um: 1.8,
            lipopolysaccharide_plasma_concentration_eu_ml: 18.5,
            intestinal_permeability_zonulin_ng_ml: 12.8,
            vagal_nerve_firing_microbiome_modulation_hz: 15.2,
            microbiota_derived_gaba_concentration_nmol_g: 165.0,
            corticosterone_responsive_bacteria_abundance_log_cfu_g: 8.2,
            gut_barrier_integrity_stress_index: 0.75,
        }
    }
}

impl MicrobiomeHPAAxis {
    /// Create new microbiome-HPA axis system
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate gut barrier dysfunction score
    pub fn gut_barrier_dysfunction_score(&self) -> f64 {
        let permeability_component = (self.intestinal_permeability_zonulin_ng_ml - 8.0).max(0.0) / 20.0;
        let lps_component = (self.lipopolysaccharide_plasma_concentration_eu_ml - 15.0).max(0.0) / 30.0;
        let integrity_component = (1.0 - self.gut_barrier_integrity_stress_index).max(0.0);

        ((permeability_component + lps_component + integrity_component) / 3.0).min(1.0)
    }

    /// Calculate microbiome diversity index based on metabolite production
    pub fn microbiome_diversity_index(&self) -> f64 {
        let scfa_score = (self.fecal_short_chain_fatty_acid_concentration_mmol_kg / 120.0).min(1.0);
        let tryptophan_score = (self.microbial_tryptophan_indole_3_propionic_acid_um / 3.0).min(1.0);
        let gaba_score = (self.microbiota_derived_gaba_concentration_nmol_g / 200.0).min(1.0);

        (scfa_score + tryptophan_score + gaba_score) / 3.0
    }

    /// Calculate stress-microbiome interaction strength
    pub fn stress_microbiome_interaction(&self) -> f64 {
        let bacterial_responsiveness = self.corticosterone_responsive_bacteria_abundance_log_cfu_g / 10.0;
        let vagal_modulation = self.vagal_nerve_firing_microbiome_modulation_hz / 20.0;
        let barrier_integrity = self.gut_barrier_integrity_stress_index;

        (bacterial_responsiveness + vagal_modulation + barrier_integrity) / 3.0
    }

    /// Assess overall microbiome-HPA axis health
    pub fn microbiome_hpa_health(&self) -> MicrobiomeHPAHealth {
        let dysfunction_score = self.gut_barrier_dysfunction_score();
        let diversity_index = self.microbiome_diversity_index();
        let interaction_strength = self.stress_microbiome_interaction();

        match (dysfunction_score, diversity_index, interaction_strength) {
            (d, div, _) if d > 0.7 || div < 0.3 => MicrobiomeHPAHealth::SeverelyDisrupted,
            (d, div, _) if d > 0.4 || div < 0.5 => MicrobiomeHPAHealth::ModeratelyDisrupted,
            (d, div, i) if d < 0.2 && div > 0.8 && i > 0.7 => MicrobiomeHPAHealth::Optimal,
            _ => MicrobiomeHPAHealth::Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MicrobiomeHPAHealth {
    Optimal,
    Normal,
    ModeratelyDisrupted,
    SeverelyDisrupted,
}

/// Represents the state of gut microbiome metabolic activity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MicrobiomeMetabolicActivity {
    /// SCFA production capacity
    pub scfa_production: SCFAProduction,

    /// Tryptophan metabolism activity
    pub tryptophan_metabolism: TryptophanMetabolism,

    /// Neurotransmitter production
    pub neurotransmitter_production: NeurotransmitterProduction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SCFAProduction {
    SeverelyReduced,
    Reduced,
    Normal,
    Enhanced,
    HighProducer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TryptophanMetabolism {
    Impaired,
    Low,
    Normal,
    Enhanced,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NeurotransmitterProduction {
    Deficient,
    Low,
    Normal,
    High,
}

/// Gut barrier status under stress conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GutBarrierStatus {
    /// Tight junction integrity
    pub tight_junction_integrity: TightJunctionIntegrity,

    /// Microbial translocation level
    pub microbial_translocation: MicrobialTranslocation,

    /// Stress resilience
    pub stress_resilience: StressResilience,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TightJunctionIntegrity {
    SeverelyCompromised,
    Compromised,
    Normal,
    Robust,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MicrobialTranslocation {
    High,
    Moderate,
    Low,
    Minimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StressResilience {
    Poor,
    Fair,
    Good,
    Excellent,
}