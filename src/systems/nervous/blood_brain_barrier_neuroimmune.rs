use serde::{Deserialize, Serialize};

/// Blood-brain barrier neuroimmune system representing inflammatory
/// effects on CNS access and barrier function
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BloodBrainBarrierNeuroimmune {
    /// BBB permeability index (ml/min/g)
    pub bbb_permeability_index_ml_min_g: f64,

    /// Tight junction protein occludin expression (fold control)
    pub tight_junction_occludin_expression_fold_control: f64,

    /// Claudin-5 tight junction expression (fold control)
    pub claudin_5_tight_junction_expression_fold_control: f64,

    /// P-glycoprotein BBB transporter activity (fold basal)
    pub p_glycoprotein_bbb_transporter_activity_fold_basal: f64,

    /// Pericyte coverage of brain capillaries (%)
    pub pericyte_coverage_brain_capillaries_percent: f64,

    /// Astrocyte end-feet AQP4 polarization index
    pub astrocyte_aqp4_polarization_index: f64,

    /// Matrix metalloproteinase-9 BBB disruption (ng/ml)
    pub mmp9_bbb_disruption_ng_ml: f64,

    /// S100β serum levels - BBB injury marker (ng/ml)
    pub s100b_bbb_injury_marker_ng_ml: f64,
}

impl Default for BloodBrainBarrierNeuroimmune {
    fn default() -> Self {
        Self {
            bbb_permeability_index_ml_min_g: 0.008,
            tight_junction_occludin_expression_fold_control: 1.0,
            claudin_5_tight_junction_expression_fold_control: 1.0,
            p_glycoprotein_bbb_transporter_activity_fold_basal: 1.0,
            pericyte_coverage_brain_capillaries_percent: 88.0,
            astrocyte_aqp4_polarization_index: 0.82,
            mmp9_bbb_disruption_ng_ml: 3.8,
            s100b_bbb_injury_marker_ng_ml: 0.12,
        }
    }
}

impl BloodBrainBarrierNeuroimmune {
    /// Create new blood-brain barrier neuroimmune system
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate BBB integrity score
    pub fn bbb_integrity_score(&self) -> f64 {
        let permeability_component = (0.015 - self.bbb_permeability_index_ml_min_g).max(0.0) / 0.015;
        let occludin_component = (self.tight_junction_occludin_expression_fold_control).min(1.5) / 1.5;
        let claudin_component = (self.claudin_5_tight_junction_expression_fold_control).min(1.5) / 1.5;
        let pericyte_component = self.pericyte_coverage_brain_capillaries_percent / 100.0;

        (permeability_component + occludin_component + claudin_component + pericyte_component) / 4.0
    }

    /// Calculate neuroinflammation index
    pub fn neuroinflammation_index(&self) -> f64 {
        let mmp9_component = (self.mmp9_bbb_disruption_ng_ml - 2.0).max(0.0) / 10.0;
        let s100b_component = (self.s100b_bbb_injury_marker_ng_ml - 0.08).max(0.0) / 0.3;
        let permeability_component = (self.bbb_permeability_index_ml_min_g - 0.005).max(0.0) / 0.02;

        ((mmp9_component + s100b_component + permeability_component) / 3.0).min(1.0)
    }

    /// Calculate neuroprotective capacity
    pub fn neuroprotective_capacity(&self) -> f64 {
        let efflux_capacity = self.p_glycoprotein_bbb_transporter_activity_fold_basal.min(2.0) / 2.0;
        let glymphatic_function = self.astrocyte_aqp4_polarization_index;
        let structural_integrity = self.bbb_integrity_score();

        (efflux_capacity + glymphatic_function + structural_integrity) / 3.0
    }

    /// Assess overall BBB neuroimmune status
    pub fn bbb_neuroimmune_status(&self) -> BBBNeuroImmuneStatus {
        let integrity_score = self.bbb_integrity_score();
        let inflammation_index = self.neuroinflammation_index();
        let neuroprotection = self.neuroprotective_capacity();

        match (integrity_score, inflammation_index, neuroprotection) {
            (i, inf, _) if i < 0.4 || inf > 0.7 => BBBNeuroImmuneStatus::Severely_Compromised,
            (i, inf, _) if i < 0.6 || inf > 0.4 => BBBNeuroImmuneStatus::Moderately_Compromised,
            (i, inf, n) if i > 0.8 && inf < 0.2 && n > 0.8 => BBBNeuroImmuneStatus::Optimal,
            _ => BBBNeuroImmuneStatus::Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BBBNeuroImmuneStatus {
    Optimal,
    Normal,
    Moderately_Compromised,
    Severely_Compromised,
}

/// Represents the structural integrity of the blood-brain barrier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BBBStructuralIntegrity {
    /// Tight junction protein status
    pub tight_junction_status: TightJunctionStatus,

    /// Endothelial cell integrity
    pub endothelial_integrity: EndothelialIntegrity,

    /// Pericyte coverage status
    pub pericyte_status: PericyteStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TightJunctionStatus {
    Severely_Disrupted,
    Disrupted,
    Normal,
    Enhanced,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndothelialIntegrity {
    Compromised,
    Impaired,
    Normal,
    Robust,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PericyteStatus {
    Severely_Reduced,
    Reduced,
    Normal,
    Enhanced_Coverage,
}

/// Glymphatic system function for brain clearance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlymphaticFunction {
    /// AQP4 polarization status
    pub aqp4_polarization: AQP4Polarization,

    /// CSF flow dynamics
    pub csf_flow: CSFFlow,

    /// Clearance efficiency
    pub clearance_efficiency: ClearanceEfficiency,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AQP4Polarization {
    Severely_Impaired,
    Impaired,
    Normal,
    Optimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CSFFlow {
    Blocked,
    Reduced,
    Normal,
    Enhanced,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearanceEfficiency {
    Poor,
    Impaired,
    Normal,
    Efficient,
}

/// Neuroinflammatory status affecting BBB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuroinflammationStatus {
    /// Matrix metalloproteinase activity
    pub mmp_activity: MMPActivity,

    /// Astrocyte activation state
    pub astrocyte_activation: AstrocyteActivation,

    /// Inflammatory cytokine transport
    pub cytokine_transport: CytokineTransport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MMPActivity {
    Low,
    Normal,
    Elevated,
    Pathological,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstrocyteActivation {
    Quiescent,
    Mildly_Activated,
    Activated,
    Severely_Activated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CytokineTransport {
    Minimal,
    Low,
    Normal,
    Excessive,
}