use serde::{Deserialize, Serialize};

/// Adipokine-inflammation signaling system representing fat tissue
/// as an endocrine-immune organ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdipokineInflammationSignaling {
    /// Leptin serum concentration (ng/ml)
    pub leptin_serum_concentration_ng_ml: f64,

    /// Adiponectin levels - anti-inflammatory (μg/ml)
    pub adiponectin_anti_inflammatory_ug_ml: f64,

    /// Leptin/adiponectin ratio - inflammatory index
    pub leptin_adiponectin_inflammatory_ratio: f64,

    /// Resistin - pro-inflammatory adipokine (ng/ml)
    pub resistin_pro_inflammatory_ng_ml: f64,

    /// Visceral adipose tissue TNF-α expression (fold control)
    pub visceral_adipose_tnf_alpha_expression_fold_control: f64,

    /// IL-6 secretion from adipocytes (pg/ml)
    pub il6_adipocyte_secretion_pg_ml: f64,

    /// Adipose tissue macrophage infiltration (%)
    pub adipose_tissue_macrophage_infiltration_percent: f64,

    /// Free fatty acid release and inflammation (mEq/L)
    pub free_fatty_acid_inflammatory_release_meq_l: f64,
}

impl Default for AdipokineInflammationSignaling {
    fn default() -> Self {
        Self {
            leptin_serum_concentration_ng_ml: 8.5,
            adiponectin_anti_inflammatory_ug_ml: 12.8,
            leptin_adiponectin_inflammatory_ratio: 0.85,
            resistin_pro_inflammatory_ng_ml: 6.2,
            visceral_adipose_tnf_alpha_expression_fold_control: 1.0,
            il6_adipocyte_secretion_pg_ml: 28.0,
            adipose_tissue_macrophage_infiltration_percent: 15.0,
            free_fatty_acid_inflammatory_release_meq_l: 0.45,
        }
    }
}

impl AdipokineInflammationSignaling {
    /// Create new adipokine-inflammation signaling system
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate adipose tissue inflammation index
    pub fn adipose_inflammation_index(&self) -> f64 {
        let tnf_component = (self.visceral_adipose_tnf_alpha_expression_fold_control - 0.5).max(0.0) / 2.5;
        let il6_component = (self.il6_adipocyte_secretion_pg_ml - 15.0).max(0.0) / 70.0;
        let macrophage_component = (self.adipose_tissue_macrophage_infiltration_percent - 8.0).max(0.0) / 37.0;
        let resistin_component = (self.resistin_pro_inflammatory_ng_ml - 3.0).max(0.0) / 12.0;

        ((tnf_component + il6_component + macrophage_component + resistin_component) / 4.0).min(1.0)
    }

    /// Calculate metabolic adipokine balance
    pub fn metabolic_adipokine_balance(&self) -> f64 {
        // Higher adiponectin and lower leptin/adiponectin ratio indicate better balance
        let adiponectin_score = (self.adiponectin_anti_inflammatory_ug_ml / 20.0).min(1.0);
        let ratio_score = (2.0 - self.leptin_adiponectin_inflammatory_ratio).max(0.0) / 2.0;

        (adiponectin_score + ratio_score) / 2.0
    }

    /// Calculate lipotoxicity index
    pub fn lipotoxicity_index(&self) -> f64 {
        let ffa_component = (self.free_fatty_acid_inflammatory_release_meq_l - 0.2).max(0.0) / 1.0;
        let leptin_component = (self.leptin_serum_concentration_ng_ml - 5.0).max(0.0) / 20.0;

        ((ffa_component + leptin_component) / 2.0).min(1.0)
    }

    /// Assess overall adipokine-inflammation status
    pub fn adipokine_inflammation_status(&self) -> AdipokineInflammationStatus {
        let inflammation_index = self.adipose_inflammation_index();
        let metabolic_balance = self.metabolic_adipokine_balance();
        let lipotoxicity = self.lipotoxicity_index();

        match (inflammation_index, metabolic_balance, lipotoxicity) {
            (i, _, l) if i > 0.7 || l > 0.7 => AdipokineInflammationStatus::SeverelyDisrupted,
            (i, b, l) if i > 0.4 || b < 0.4 || l > 0.4 => AdipokineInflammationStatus::ModeratelyDisrupted,
            (i, b, l) if i < 0.2 && b > 0.8 && l < 0.2 => AdipokineInflammationStatus::Optimal,
            _ => AdipokineInflammationStatus::Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdipokineInflammationStatus {
    Optimal,
    Normal,
    ModeratelyDisrupted,
    SeverelyDisrupted,
}

/// Represents the inflammatory state of adipose tissue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdiposeInflammatoryState {
    /// Macrophage infiltration level
    pub macrophage_infiltration: MacrophageInfiltration,

    /// Pro-inflammatory cytokine production
    pub proinflammatory_cytokines: ProInflammatoryCytokineProduction,

    /// Crown-like structure formation
    pub crown_like_structures: CrownLikeStructures,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MacrophageInfiltration {
    Minimal,
    Low,
    Moderate,
    High,
    Severe,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProInflammatoryCytokineProduction {
    Low,
    Normal,
    Elevated,
    High,
    Pathological,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrownLikeStructures {
    Absent,
    Few,
    Moderate,
    Numerous,
    Extensive,
}

/// Metabolic adipokine profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetabolicAdipokineProfile {
    /// Leptin signaling status
    pub leptin_signaling: LeptinSignaling,

    /// Adiponectin activity
    pub adiponectin_activity: AdiponectinActivity,

    /// Resistin inflammatory activity
    pub resistin_activity: ResistinActivity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LeptinSignaling {
    Deficient,
    Normal,
    Elevated,
    Resistant,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdiponectinActivity {
    Deficient,
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResistinActivity {
    Low,
    Normal,
    Elevated,
    High,
}

/// Lipotoxicity and metabolic stress indicators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LipotoxicityProfile {
    /// Free fatty acid release pattern
    pub ffa_release: FFARelease,

    /// Inflammatory lipid mediators
    pub inflammatory_lipids: InflammatoryLipids,

    /// Lipid-induced insulin resistance
    pub lipid_insulin_resistance: LipidInsulinResistance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FFARelease {
    Low,
    Normal,
    Elevated,
    Excessive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InflammatoryLipids {
    Low,
    Normal,
    Elevated,
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LipidInsulinResistance {
    Absent,
    Mild,
    Moderate,
    Severe,
}