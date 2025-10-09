use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Toxin {
    HeavyMetals(HeavyMetal),
    OrganicCompound(OrganicToxin),
    Pesticide,
    IndustrialChemical,
    Mycotoxin,
    BacterialToxin,
    PlantToxin,
    MarineToxin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeavyMetal {
    Lead,
    Mercury,
    Cadmium,
    Arsenic,
    Chromium,
    Nickel,
    Aluminum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganicToxin {
    Benzene,
    Formaldehyde,
    Toluene,
    Xylene,
    PolychlorinatedBiphenyls,
    Dioxins,
    VinylChloride,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CytochromeP450 {
    CYP1A2,
    CYP2C9,
    CYP2C19,
    CYP2D6,
    CYP2E1,
    CYP3A4,
    CYP3A5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DetoxificationPhase {
    PhaseI,
    PhaseII,
    PhaseIII,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetoxificationEnzyme {
    pub name: String,
    pub phase: DetoxificationPhase,
    pub activity_level: f64,
    pub polymorphism: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxinMetabolism {
    pub toxin: String,
    pub metabolizing_enzymes: Vec<String>,
    pub half_life_hours: f64,
    pub primary_elimination_route: EliminationRoute,
    pub toxic_metabolites: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EliminationRoute {
    Renal,
    Hepatic,
    Biliary,
    Respiratory,
    Cutaneous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicityProfile {
    pub substance: String,
    pub lethal_dose_50: Option<f64>,
    pub target_organs: Vec<TargetOrgan>,
    pub mechanism_of_toxicity: String,
    pub bioaccumulation_potential: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetOrgan {
    Liver,
    Kidney,
    Brain,
    Heart,
    Lung,
    BoneMarrow,
    PeripheralNervousSystem,
    ReproductiveSystem,
    ImmuneSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlutathioneSystem {
    pub glutathione_level_umol_per_l: f64,
    pub glutathione_s_transferase_activity: f64,
    pub gst_genotypes: HashMap<String, String>,
}

impl GlutathioneSystem {
    pub fn new() -> Self {
        Self {
            glutathione_level_umol_per_l: 1000.0,
            glutathione_s_transferase_activity: 1.0,
            gst_genotypes: HashMap::new(),
        }
    }

    pub fn detoxification_capacity(&self) -> f64 {
        (self.glutathione_level_umol_per_l / 1000.0) * self.glutathione_s_transferase_activity
    }

    pub fn is_depleted(&self) -> bool {
        self.glutathione_level_umol_per_l < 500.0
    }
}

impl Default for GlutathioneSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugMetabolismProfile {
    pub cyp450_genotypes: HashMap<CytochromeP450, String>,
    pub metabolizer_status: HashMap<CytochromeP450, MetabolizerStatus>,
    pub phase_ii_enzymes: Vec<DetoxificationEnzyme>,
    pub transporter_genotypes: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetabolizerStatus {
    Poor,
    Intermediate,
    Normal,
    Rapid,
    Ultrarapid,
}

impl DrugMetabolismProfile {
    pub fn new() -> Self {
        Self {
            cyp450_genotypes: HashMap::new(),
            metabolizer_status: HashMap::new(),
            phase_ii_enzymes: Vec::new(),
            transporter_genotypes: HashMap::new(),
        }
    }

    pub fn predict_drug_response(&self, drug: &str, enzyme: CytochromeP450) -> DrugResponse {
        let status = self.metabolizer_status.get(&enzyme)
            .copied()
            .unwrap_or(MetabolizerStatus::Normal);

        match status {
            MetabolizerStatus::Poor => DrugResponse {
                expected_efficacy: 0.5,
                adverse_event_risk: 1.5,
                dose_adjustment: DoseAdjustment::Reduce(0.5),
                recommendation: format!("Consider alternative to {} or reduce dose", drug),
            },
            MetabolizerStatus::Ultrarapid => DrugResponse {
                expected_efficacy: 0.6,
                adverse_event_risk: 0.8,
                dose_adjustment: DoseAdjustment::Increase(1.5),
                recommendation: format!("May need higher dose of {} or alternative", drug),
            },
            _ => DrugResponse {
                expected_efficacy: 1.0,
                adverse_event_risk: 1.0,
                dose_adjustment: DoseAdjustment::Standard,
                recommendation: format!("Standard dosing appropriate for {}", drug),
            },
        }
    }
}

impl Default for DrugMetabolismProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrugResponse {
    pub expected_efficacy: f64,
    pub adverse_event_risk: f64,
    pub dose_adjustment: DoseAdjustment,
    pub recommendation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DoseAdjustment {
    Standard,
    Reduce(f64),
    Increase(f64),
    Avoid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicExposure {
    pub substance: String,
    pub exposure_route: ExposureRoute,
    pub dose_mg: f64,
    pub duration_hours: f64,
    pub biomarkers: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExposureRoute {
    Inhalation,
    Ingestion,
    Dermal,
    Intravenous,
    Intramuscular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToxicokineticsModel {
    pub absorption_rate: f64,
    pub distribution_volume_l: f64,
    pub metabolism_rate: f64,
    pub elimination_half_life_hours: f64,
    pub clearance_ml_per_min: f64,
}

impl ToxicokineticsModel {
    pub fn concentration_at_time(&self, dose_mg: f64, time_hours: f64) -> f64 {
        let k = 0.693 / self.elimination_half_life_hours;
        (dose_mg / self.distribution_volume_l) * (-k * time_hours).exp()
    }

    pub fn time_to_elimination(&self, dose_mg: f64, threshold_mg_per_l: f64) -> f64 {
        let initial = dose_mg / self.distribution_volume_l;
        if initial <= threshold_mg_per_l {
            return 0.0;
        }
        let k = 0.693 / self.elimination_half_life_hours;
        (initial / threshold_mg_per_l).ln() / k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glutathione_system() {
        let gst = GlutathioneSystem::new();
        assert!(!gst.is_depleted());
        assert_eq!(gst.detoxification_capacity(), 1.0);
    }

    #[test]
    fn test_drug_metabolism() {
        let mut profile = DrugMetabolismProfile::new();
        profile.metabolizer_status.insert(CytochromeP450::CYP2D6, MetabolizerStatus::Poor);

        let response = profile.predict_drug_response("codeine", CytochromeP450::CYP2D6);
        assert_eq!(response.expected_efficacy, 0.5);
        assert_eq!(response.adverse_event_risk, 1.5);
    }

    #[test]
    fn test_toxicokinetics() {
        let model = ToxicokineticsModel {
            absorption_rate: 0.8,
            distribution_volume_l: 50.0,
            metabolism_rate: 0.5,
            elimination_half_life_hours: 4.0,
            clearance_ml_per_min: 100.0,
        };

        let conc = model.concentration_at_time(100.0, 0.0);
        assert_eq!(conc, 2.0);

        let conc_later = model.concentration_at_time(100.0, 4.0);
        assert!(conc_later < conc);
    }

    #[test]
    fn test_time_to_elimination() {
        let model = ToxicokineticsModel {
            absorption_rate: 0.8,
            distribution_volume_l: 50.0,
            metabolism_rate: 0.5,
            elimination_half_life_hours: 4.0,
            clearance_ml_per_min: 100.0,
        };

        let time = model.time_to_elimination(100.0, 0.1);
        assert!(time > 0.0);
    }
}
