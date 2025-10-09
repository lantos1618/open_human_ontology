use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConductionNode {
    SinusNode,
    AtrioventricularNode,
    BundleOfHis,
    LeftBundle,
    RightBundle,
    PurkinjeFibers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConductionSystem {
    pub sinus_node: SinusNode,
    pub av_node: AVNode,
    pub bundle_branches: BundleBranches,
    pub purkinje_network: PurkinjeNetwork,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinusNode {
    pub rate_bpm: f64,
    pub location: NodeLocation,
    pub automaticity: f64,
    pub dysfunction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AVNode {
    pub conduction_delay_ms: f64,
    pub location: NodeLocation,
    pub refractory_period_ms: f64,
    pub block_degree: AVBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AVBlock {
    None,
    FirstDegree,
    SecondDegreeMobitzI,
    SecondDegreeMobitzII,
    ThirdDegree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleBranches {
    pub left_bundle: Bundle,
    pub right_bundle: Bundle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bundle {
    pub conduction_velocity_m_s: f64,
    pub is_blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurkinjeNetwork {
    pub fiber_density: f64,
    pub conduction_velocity_m_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeLocation {
    pub x_mm: f64,
    pub y_mm: f64,
    pub z_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ECG {
    pub p_wave_mv: f64,
    pub pr_interval_ms: f64,
    pub qrs_duration_ms: f64,
    pub qt_interval_ms: f64,
    pub st_segment_mv: f64,
    pub t_wave_mv: f64,
    pub heart_rate_bpm: f64,
}

impl ConductionSystem {
    pub fn new() -> Self {
        ConductionSystem {
            sinus_node: SinusNode::new(),
            av_node: AVNode::new(),
            bundle_branches: BundleBranches::new(),
            purkinje_network: PurkinjeNetwork::new(),
        }
    }

    pub fn generate_ecg(&self) -> ECG {
        let hr = self.sinus_node.rate_bpm;
        let pr_interval = 120.0 + self.av_node.conduction_delay_ms;

        let qrs_duration = if self.bundle_branches.left_bundle.is_blocked
            || self.bundle_branches.right_bundle.is_blocked {
            120.0
        } else {
            90.0
        };

        ECG {
            p_wave_mv: 0.15,
            pr_interval_ms: pr_interval,
            qrs_duration_ms: qrs_duration,
            qt_interval_ms: 400.0,
            st_segment_mv: 0.0,
            t_wave_mv: 0.3,
            heart_rate_bpm: hr,
        }
    }

    pub fn detect_arrhythmias(&self) -> Vec<Arrhythmia> {
        let mut arrhythmias = Vec::new();

        if self.sinus_node.rate_bpm > 100.0 {
            arrhythmias.push(Arrhythmia::SinusTachycardia);
        } else if self.sinus_node.rate_bpm < 60.0 {
            arrhythmias.push(Arrhythmia::SinusBradycardia);
        }

        if self.av_node.block_degree != AVBlock::None {
            arrhythmias.push(Arrhythmia::AVBlock(self.av_node.block_degree));
        }

        if self.bundle_branches.left_bundle.is_blocked {
            arrhythmias.push(Arrhythmia::LeftBundleBranchBlock);
        }

        if self.bundle_branches.right_bundle.is_blocked {
            arrhythmias.push(Arrhythmia::RightBundleBranchBlock);
        }

        arrhythmias
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Arrhythmia {
    SinusTachycardia,
    SinusBradycardia,
    AtrialFibrillation,
    AtrialFlutter,
    VentricularTachycardia,
    VentricularFibrillation,
    AVBlock(AVBlock),
    LeftBundleBranchBlock,
    RightBundleBranchBlock,
    PrematureVentricularContraction,
    PrematureAtrialContraction,
}

impl SinusNode {
    pub fn new() -> Self {
        SinusNode {
            rate_bpm: 70.0,
            location: NodeLocation {
                x_mm: 0.0,
                y_mm: 0.0,
                z_mm: 0.0,
            },
            automaticity: 1.0,
            dysfunction: false,
        }
    }
}

impl AVNode {
    pub fn new() -> Self {
        AVNode {
            conduction_delay_ms: 100.0,
            location: NodeLocation {
                x_mm: 0.0,
                y_mm: -20.0,
                z_mm: 0.0,
            },
            refractory_period_ms: 250.0,
            block_degree: AVBlock::None,
        }
    }
}

impl BundleBranches {
    pub fn new() -> Self {
        BundleBranches {
            left_bundle: Bundle {
                conduction_velocity_m_s: 2.0,
                is_blocked: false,
            },
            right_bundle: Bundle {
                conduction_velocity_m_s: 2.0,
                is_blocked: false,
            },
        }
    }
}

impl PurkinjeNetwork {
    pub fn new() -> Self {
        PurkinjeNetwork {
            fiber_density: 1.0,
            conduction_velocity_m_s: 4.0,
        }
    }
}

impl Default for ConductionSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SinusNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AVNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BundleBranches {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PurkinjeNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conduction_system_creation() {
        let system = ConductionSystem::new();
        assert_eq!(system.sinus_node.rate_bpm, 70.0);
        assert_eq!(system.av_node.block_degree, AVBlock::None);
    }

    #[test]
    fn test_ecg_generation() {
        let system = ConductionSystem::new();
        let ecg = system.generate_ecg();
        assert_eq!(ecg.heart_rate_bpm, 70.0);
        assert!(ecg.pr_interval_ms > 100.0);
        assert!(ecg.qrs_duration_ms < 100.0);
    }

    #[test]
    fn test_bundle_branch_block_detection() {
        let mut system = ConductionSystem::new();
        system.bundle_branches.left_bundle.is_blocked = true;

        let arrhythmias = system.detect_arrhythmias();
        assert!(arrhythmias.contains(&Arrhythmia::LeftBundleBranchBlock));
    }

    #[test]
    fn test_av_block_detection() {
        let mut system = ConductionSystem::new();
        system.av_node.block_degree = AVBlock::FirstDegree;

        let arrhythmias = system.detect_arrhythmias();
        assert!(arrhythmias.contains(&Arrhythmia::AVBlock(AVBlock::FirstDegree)));
    }

    #[test]
    fn test_tachycardia_detection() {
        let mut system = ConductionSystem::new();
        system.sinus_node.rate_bpm = 120.0;

        let arrhythmias = system.detect_arrhythmias();
        assert!(arrhythmias.contains(&Arrhythmia::SinusTachycardia));
    }

    #[test]
    fn test_bradycardia_detection() {
        let mut system = ConductionSystem::new();
        system.sinus_node.rate_bpm = 50.0;

        let arrhythmias = system.detect_arrhythmias();
        assert!(arrhythmias.contains(&Arrhythmia::SinusBradycardia));
    }
}
