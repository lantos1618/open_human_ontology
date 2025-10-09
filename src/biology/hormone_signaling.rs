use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HormoneClass {
    Peptide,
    Steroid,
    Amine,
    Eicosanoid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hormone {
    pub name: String,
    pub class: HormoneClass,
    pub molecular_weight_da: f64,
    pub half_life_minutes: f64,
    pub secreting_organ: String,
    pub target_tissues: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptorType {
    GProteinCoupled,
    TyrosineKinase,
    SerineKinase,
    NuclearReceptor,
    IonChannelLinked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneReceptor {
    pub name: String,
    pub receptor_type: ReceptorType,
    pub location: ReceptorLocation,
    pub affinity_kd_nm: f64,
    pub downstream_pathways: Vec<SignalingPathway>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptorLocation {
    CellMembrane,
    Cytoplasm,
    Nucleus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalingPathway {
    CAMP_PKA,
    IP3_DAG_PKC,
    JAK_STAT,
    MAPK_ERK,
    PI3K_AKT,
    mTOR,
    AMPK,
    NFKB,
    Wnt_Beta_catenin,
    Hedgehog,
    Notch,
    TGF_beta_SMAD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondMessenger {
    pub name: String,
    pub basal_concentration_um: f64,
    pub activated_concentration_um: f64,
    pub clearance_rate_per_sec: f64,
}

impl SecondMessenger {
    pub fn fold_change(&self) -> f64 {
        self.activated_concentration_um / self.basal_concentration_um
    }

    pub fn half_life_seconds(&self) -> f64 {
        0.693 / self.clearance_rate_per_sec
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPCRSignaling {
    pub receptor: String,
    pub g_protein_type: GProteinType,
    pub second_messengers: Vec<SecondMessenger>,
    pub effector_proteins: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GProteinType {
    Gs,
    Gi,
    Gq,
    G12_13,
}

impl GPCRSignaling {
    pub fn activate(&self) -> Vec<CellularResponse> {
        match self.g_protein_type {
            GProteinType::Gs => vec![
                CellularResponse::IncreasedCAMP,
                CellularResponse::PKAActivation,
            ],
            GProteinType::Gi => vec![
                CellularResponse::DecreasedCAMP,
                CellularResponse::PKAInhibition,
            ],
            GProteinType::Gq => vec![
                CellularResponse::IncreasedIP3,
                CellularResponse::IncreasedDAG,
                CellularResponse::CalciumRelease,
            ],
            GProteinType::G12_13 => vec![
                CellularResponse::RhoActivation,
                CellularResponse::CytoskeletonRemodeling,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellularResponse {
    IncreasedCAMP,
    DecreasedCAMP,
    IncreasedIP3,
    IncreasedDAG,
    CalciumRelease,
    PKAActivation,
    PKAInhibition,
    PKCActivation,
    RhoActivation,
    CytoskeletonRemodeling,
    GeneTranscription,
    ProteinSynthesis,
    Metabolism,
    CellGrowth,
    CellDifferentiation,
    Apoptosis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteroidHormoneAction {
    pub hormone: String,
    pub receptor: String,
    pub hormone_response_elements: Vec<String>,
    pub target_genes: Vec<TargetGene>,
    pub genomic_lag_time_minutes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetGene {
    pub gene_name: String,
    pub regulation: GeneRegulation,
    pub fold_change: f64,
    pub time_to_peak_hours: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneRegulation {
    Upregulation,
    Downregulation,
}

impl SteroidHormoneAction {
    pub fn new(hormone: String, receptor: String) -> Self {
        Self {
            hormone,
            receptor,
            hormone_response_elements: Vec::new(),
            target_genes: Vec::new(),
            genomic_lag_time_minutes: 30.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneAxis {
    pub name: String,
    pub hypothalamic_hormone: Option<String>,
    pub pituitary_hormone: Option<String>,
    pub peripheral_hormone: String,
    pub negative_feedback_loops: Vec<FeedbackLoop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackLoop {
    pub sensor_location: String,
    pub target_location: String,
    pub feedback_type: FeedbackType,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedbackType {
    Negative,
    Positive,
}

impl HormoneAxis {
    pub fn hpa_axis() -> Self {
        Self {
            name: "Hypothalamic-Pituitary-Adrenal".to_string(),
            hypothalamic_hormone: Some("CRH".to_string()),
            pituitary_hormone: Some("ACTH".to_string()),
            peripheral_hormone: "Cortisol".to_string(),
            negative_feedback_loops: vec![
                FeedbackLoop {
                    sensor_location: "Hypothalamus".to_string(),
                    target_location: "Hypothalamus".to_string(),
                    feedback_type: FeedbackType::Negative,
                    sensitivity: 0.8,
                },
                FeedbackLoop {
                    sensor_location: "Pituitary".to_string(),
                    target_location: "Pituitary".to_string(),
                    feedback_type: FeedbackType::Negative,
                    sensitivity: 0.7,
                },
            ],
        }
    }

    pub fn hpt_axis() -> Self {
        Self {
            name: "Hypothalamic-Pituitary-Thyroid".to_string(),
            hypothalamic_hormone: Some("TRH".to_string()),
            pituitary_hormone: Some("TSH".to_string()),
            peripheral_hormone: "T3/T4".to_string(),
            negative_feedback_loops: vec![
                FeedbackLoop {
                    sensor_location: "Pituitary".to_string(),
                    target_location: "Pituitary".to_string(),
                    feedback_type: FeedbackType::Negative,
                    sensitivity: 0.9,
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhosphorylationCascade {
    pub initiating_signal: String,
    pub kinases: Vec<Kinase>,
    pub phosphatases: Vec<String>,
    pub final_targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kinase {
    pub name: String,
    pub activation_state: bool,
    pub substrates: Vec<String>,
    pub activity_units: f64,
}

impl PhosphorylationCascade {
    pub fn mapk_cascade() -> Self {
        Self {
            initiating_signal: "Growth factor".to_string(),
            kinases: vec![
                Kinase {
                    name: "RAF".to_string(),
                    activation_state: false,
                    substrates: vec!["MEK".to_string()],
                    activity_units: 0.0,
                },
                Kinase {
                    name: "MEK".to_string(),
                    activation_state: false,
                    substrates: vec!["ERK".to_string()],
                    activity_units: 0.0,
                },
                Kinase {
                    name: "ERK".to_string(),
                    activation_state: false,
                    substrates: vec!["Transcription factors".to_string()],
                    activity_units: 0.0,
                },
            ],
            phosphatases: vec!["MKP".to_string()],
            final_targets: vec!["c-Fos".to_string(), "c-Jun".to_string()],
        }
    }

    pub fn amplification_factor(&self) -> f64 {
        10_f64.powi(self.kinases.len() as i32)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalciumSignaling {
    pub cytosolic_calcium_nm: f64,
    pub er_calcium_um: f64,
    pub calcium_channels: Vec<CalciumChannel>,
    pub calcium_buffers: Vec<String>,
    pub calcium_effectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalciumChannel {
    pub name: String,
    pub location: String,
    pub open_probability: f64,
    pub conductance_ps: f64,
}

impl CalciumSignaling {
    pub fn new() -> Self {
        Self {
            cytosolic_calcium_nm: 100.0,
            er_calcium_um: 500.0,
            calcium_channels: Vec::new(),
            calcium_buffers: vec!["Calmodulin".to_string(), "Calbindin".to_string()],
            calcium_effectors: vec!["PKC".to_string(), "CaMKII".to_string()],
        }
    }

    pub fn calcium_wave(&mut self, amplitude_fold: f64) {
        self.cytosolic_calcium_nm *= amplitude_fold;
    }

    pub fn is_elevated(&self) -> bool {
        self.cytosolic_calcium_nm > 500.0
    }
}

impl Default for CalciumSignaling {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrosstalkInteraction {
    pub pathway1: SignalingPathway,
    pub pathway2: SignalingPathway,
    pub interaction_type: InteractionType,
    pub integration_node: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    Synergistic,
    Antagonistic,
    Convergent,
    Divergent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_messenger() {
        let messenger = SecondMessenger {
            name: "cAMP".to_string(),
            basal_concentration_um: 0.1,
            activated_concentration_um: 10.0,
            clearance_rate_per_sec: 0.1,
        };

        assert_eq!(messenger.fold_change(), 100.0);
        assert!((messenger.half_life_seconds() - 6.93).abs() < 0.01);
    }

    #[test]
    fn test_gpcr_signaling() {
        let gpcr = GPCRSignaling {
            receptor: "Beta-adrenergic".to_string(),
            g_protein_type: GProteinType::Gs,
            second_messengers: Vec::new(),
            effector_proteins: Vec::new(),
        };

        let responses = gpcr.activate();
        assert!(responses.contains(&CellularResponse::IncreasedCAMP));
    }

    #[test]
    fn test_steroid_hormone_action() {
        let action = SteroidHormoneAction::new(
            "Cortisol".to_string(),
            "Glucocorticoid receptor".to_string()
        );

        assert_eq!(action.genomic_lag_time_minutes, 30.0);
    }

    #[test]
    fn test_hormone_axis() {
        let hpa = HormoneAxis::hpa_axis();
        assert_eq!(hpa.peripheral_hormone, "Cortisol");
        assert_eq!(hpa.negative_feedback_loops.len(), 2);
    }

    #[test]
    fn test_phosphorylation_cascade() {
        let mapk = PhosphorylationCascade::mapk_cascade();
        assert_eq!(mapk.kinases.len(), 3);
        assert_eq!(mapk.amplification_factor(), 1000.0);
    }

    #[test]
    fn test_calcium_signaling() {
        let mut calcium = CalciumSignaling::new();
        assert!(!calcium.is_elevated());

        calcium.calcium_wave(10.0);
        assert!(calcium.is_elevated());
    }

    #[test]
    fn test_crosstalk() {
        let crosstalk = CrosstalkInteraction {
            pathway1: SignalingPathway::CAMP_PKA,
            pathway2: SignalingPathway::MAPK_ERK,
            interaction_type: InteractionType::Synergistic,
            integration_node: "CREB".to_string(),
        };

        assert_eq!(crosstalk.interaction_type, InteractionType::Synergistic);
    }
}
