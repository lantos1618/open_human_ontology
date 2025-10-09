use crate::biology::BiologyResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardiacNode {
    SinoatrialNode,
    AtrioventricularNode,
    BundleOfHis,
    LeftBundleBranch,
    RightBundleBranch,
    PurkinjeFibers,
}

impl CardiacNode {
    pub fn intrinsic_rate_bpm(&self) -> f64 {
        match self {
            CardiacNode::SinoatrialNode => 60.0,
            CardiacNode::AtrioventricularNode => 45.0,
            CardiacNode::BundleOfHis => 40.0,
            CardiacNode::LeftBundleBranch => 35.0,
            CardiacNode::RightBundleBranch => 35.0,
            CardiacNode::PurkinjeFibers => 30.0,
        }
    }

    pub fn conduction_velocity_m_per_s(&self) -> f64 {
        match self {
            CardiacNode::SinoatrialNode => 0.05,
            CardiacNode::AtrioventricularNode => 0.05,
            CardiacNode::BundleOfHis => 2.0,
            CardiacNode::LeftBundleBranch => 2.5,
            CardiacNode::RightBundleBranch => 2.5,
            CardiacNode::PurkinjeFibers => 4.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionPotentialPhase {
    Phase0Depolarization,
    Phase1EarlyRepolarization,
    Phase2Plateau,
    Phase3Repolarization,
    Phase4RestingPotential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPotential {
    pub membrane_potential_mv: f64,
    pub phase: ActionPotentialPhase,
    pub duration_ms: f64,
    pub sodium_current_na: f64,
    pub calcium_current_ca: f64,
    pub potassium_current_k: f64,
}

impl ActionPotential {
    pub fn new_ventricular() -> Self {
        ActionPotential {
            membrane_potential_mv: -90.0,
            phase: ActionPotentialPhase::Phase4RestingPotential,
            duration_ms: 300.0,
            sodium_current_na: 0.0,
            calcium_current_ca: 0.0,
            potassium_current_k: 0.0,
        }
    }

    pub fn new_atrial() -> Self {
        ActionPotential {
            membrane_potential_mv: -90.0,
            phase: ActionPotentialPhase::Phase4RestingPotential,
            duration_ms: 200.0,
            sodium_current_na: 0.0,
            calcium_current_ca: 0.0,
            potassium_current_k: 0.0,
        }
    }

    pub fn new_pacemaker() -> Self {
        ActionPotential {
            membrane_potential_mv: -60.0,
            phase: ActionPotentialPhase::Phase4RestingPotential,
            duration_ms: 150.0,
            sodium_current_na: 0.0,
            calcium_current_ca: 0.0,
            potassium_current_k: 0.0,
        }
    }

    pub fn depolarize(&mut self) -> BiologyResult<()> {
        self.phase = ActionPotentialPhase::Phase0Depolarization;
        self.membrane_potential_mv = 20.0;
        self.sodium_current_na = 100.0;
        Ok(())
    }

    pub fn advance_phase(&mut self) -> BiologyResult<()> {
        self.phase = match self.phase {
            ActionPotentialPhase::Phase0Depolarization => {
                self.membrane_potential_mv = 10.0;
                self.sodium_current_na = 0.0;
                self.potassium_current_k = 30.0;
                ActionPotentialPhase::Phase1EarlyRepolarization
            }
            ActionPotentialPhase::Phase1EarlyRepolarization => {
                self.membrane_potential_mv = 0.0;
                self.calcium_current_ca = 50.0;
                ActionPotentialPhase::Phase2Plateau
            }
            ActionPotentialPhase::Phase2Plateau => {
                self.membrane_potential_mv = -60.0;
                self.calcium_current_ca = 0.0;
                self.potassium_current_k = 80.0;
                ActionPotentialPhase::Phase3Repolarization
            }
            ActionPotentialPhase::Phase3Repolarization => {
                self.membrane_potential_mv = -90.0;
                self.potassium_current_k = 0.0;
                ActionPotentialPhase::Phase4RestingPotential
            }
            ActionPotentialPhase::Phase4RestingPotential => {
                ActionPotentialPhase::Phase4RestingPotential
            }
        };

        Ok(())
    }

    pub fn is_refractory(&self) -> bool {
        matches!(
            self.phase,
            ActionPotentialPhase::Phase0Depolarization
                | ActionPotentialPhase::Phase1EarlyRepolarization
                | ActionPotentialPhase::Phase2Plateau
        )
    }

    pub fn effective_refractory_period_ms(&self) -> f64 {
        self.duration_ms * 0.8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IonChannel {
    SodiumFast,
    SodiumSlow,
    CalciumLType,
    CalciumTType,
    PotassiumDelayed,
    PotassiumInward,
    PotassiumATP,
    FunnyChannel,
}

impl IonChannel {
    pub fn conductance_ms(&self) -> f64 {
        match self {
            IonChannel::SodiumFast => 120.0,
            IonChannel::SodiumSlow => 8.0,
            IonChannel::CalciumLType => 0.5,
            IonChannel::CalciumTType => 0.1,
            IonChannel::PotassiumDelayed => 36.0,
            IonChannel::PotassiumInward => 0.3,
            IonChannel::PotassiumATP => 0.4,
            IonChannel::FunnyChannel => 0.2,
        }
    }

    pub fn reversal_potential_mv(&self) -> f64 {
        match self {
            IonChannel::SodiumFast | IonChannel::SodiumSlow => 60.0,
            IonChannel::CalciumLType | IonChannel::CalciumTType => 130.0,
            IonChannel::PotassiumDelayed
            | IonChannel::PotassiumInward
            | IonChannel::PotassiumATP => -90.0,
            IonChannel::FunnyChannel => -30.0,
        }
    }

    pub fn activation_voltage_mv(&self) -> f64 {
        match self {
            IonChannel::SodiumFast => -55.0,
            IonChannel::SodiumSlow => -45.0,
            IonChannel::CalciumLType => -40.0,
            IonChannel::CalciumTType => -60.0,
            IonChannel::PotassiumDelayed => -30.0,
            IonChannel::PotassiumInward => -80.0,
            IonChannel::PotassiumATP => -50.0,
            IonChannel::FunnyChannel => -60.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ECG {
    pub p_wave_duration_ms: f64,
    pub pr_interval_ms: f64,
    pub qrs_duration_ms: f64,
    pub qt_interval_ms: f64,
    pub st_segment_elevation_mv: f64,
    pub heart_rate_bpm: f64,
}

impl ECG {
    pub fn new_normal(heart_rate_bpm: f64) -> Self {
        ECG {
            p_wave_duration_ms: 80.0,
            pr_interval_ms: 160.0,
            qrs_duration_ms: 100.0,
            qt_interval_ms: 380.0,
            st_segment_elevation_mv: 0.0,
            heart_rate_bpm,
        }
    }

    pub fn corrected_qt_interval(&self) -> f64 {
        let rr_interval_s = 60.0 / self.heart_rate_bpm;
        self.qt_interval_ms / rr_interval_s.sqrt()
    }

    pub fn has_prolonged_qt(&self) -> bool {
        self.corrected_qt_interval() > 440.0
    }

    pub fn has_bundle_branch_block(&self) -> bool {
        self.qrs_duration_ms > 120.0
    }

    pub fn has_first_degree_av_block(&self) -> bool {
        self.pr_interval_ms > 200.0
    }

    pub fn has_st_elevation(&self) -> bool {
        self.st_segment_elevation_mv > 0.1
    }

    pub fn assess_rhythm(&self) -> BiologyResult<CardiacRhythm> {
        if self.heart_rate_bpm < 60.0 {
            Ok(CardiacRhythm::SinusBradycardia)
        } else if self.heart_rate_bpm > 100.0 {
            Ok(CardiacRhythm::SinusTachycardia)
        } else if self.p_wave_duration_ms == 0.0 {
            Ok(CardiacRhythm::AtrialFibrillation)
        } else {
            Ok(CardiacRhythm::NormalSinus)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardiacRhythm {
    NormalSinus,
    SinusBradycardia,
    SinusTachycardia,
    AtrialFibrillation,
    AtrialFlutter,
    VentricularTachycardia,
    VentricularFibrillation,
    FirstDegreeAVBlock,
    SecondDegreeAVBlock,
    ThirdDegreeAVBlock,
}

impl CardiacRhythm {
    pub fn is_life_threatening(&self) -> bool {
        matches!(
            self,
            CardiacRhythm::VentricularTachycardia
                | CardiacRhythm::VentricularFibrillation
                | CardiacRhythm::ThirdDegreeAVBlock
        )
    }

    pub fn requires_treatment(&self) -> bool {
        matches!(
            self,
            CardiacRhythm::AtrialFibrillation
                | CardiacRhythm::AtrialFlutter
                | CardiacRhythm::VentricularTachycardia
                | CardiacRhythm::VentricularFibrillation
                | CardiacRhythm::SecondDegreeAVBlock
                | CardiacRhythm::ThirdDegreeAVBlock
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConductionSystem {
    pub sa_node: CardiacNode,
    pub av_node: CardiacNode,
    pub bundle_of_his: CardiacNode,
    pub left_bundle: CardiacNode,
    pub right_bundle: CardiacNode,
    pub purkinje_fibers: CardiacNode,
    pub av_node_delay_ms: f64,
    pub total_conduction_time_ms: f64,
}

impl ConductionSystem {
    pub fn new() -> Self {
        ConductionSystem {
            sa_node: CardiacNode::SinoatrialNode,
            av_node: CardiacNode::AtrioventricularNode,
            bundle_of_his: CardiacNode::BundleOfHis,
            left_bundle: CardiacNode::LeftBundleBranch,
            right_bundle: CardiacNode::RightBundleBranch,
            purkinje_fibers: CardiacNode::PurkinjeFibers,
            av_node_delay_ms: 100.0,
            total_conduction_time_ms: 220.0,
        }
    }

    pub fn calculate_total_conduction_time(&self) -> f64 {
        let atrial_time = 30.0;
        let av_delay = self.av_node_delay_ms;
        let his_bundle_time = 40.0;
        let purkinje_time = 50.0;

        atrial_time + av_delay + his_bundle_time + purkinje_time
    }

    pub fn has_conduction_block(&self) -> bool {
        self.av_node_delay_ms > 200.0
    }

    pub fn pacemaker_hierarchy(&self) -> Vec<(CardiacNode, f64)> {
        vec![
            (self.sa_node, self.sa_node.intrinsic_rate_bpm()),
            (self.av_node, self.av_node.intrinsic_rate_bpm()),
            (self.bundle_of_his, self.bundle_of_his.intrinsic_rate_bpm()),
            (self.purkinje_fibers, self.purkinje_fibers.intrinsic_rate_bpm()),
        ]
    }
}

impl Default for ConductionSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arrhythmia {
    pub rhythm_type: CardiacRhythm,
    pub rate_bpm: f64,
    pub regularity: f64,
    pub origin: ArrhythmiaOrigin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArrhythmiaOrigin {
    Supraventricular,
    Ventricular,
    Junctional,
}

impl Arrhythmia {
    pub fn new_afib(rate_bpm: f64) -> Self {
        Arrhythmia {
            rhythm_type: CardiacRhythm::AtrialFibrillation,
            rate_bpm,
            regularity: 0.0,
            origin: ArrhythmiaOrigin::Supraventricular,
        }
    }

    pub fn new_vtach(rate_bpm: f64) -> Self {
        Arrhythmia {
            rhythm_type: CardiacRhythm::VentricularTachycardia,
            rate_bpm,
            regularity: 1.0,
            origin: ArrhythmiaOrigin::Ventricular,
        }
    }

    pub fn stroke_risk_chadsvasc(&self, age: u32, has_htn: bool, has_dm: bool, has_chf: bool, has_stroke_hx: bool, has_vascular_disease: bool, is_female: bool) -> u32 {
        if self.rhythm_type != CardiacRhythm::AtrialFibrillation {
            return 0;
        }

        let mut score = 0;

        if has_chf {
            score += 1;
        }
        if has_htn {
            score += 1;
        }
        if age >= 75 {
            score += 2;
        } else if age >= 65 {
            score += 1;
        }
        if has_dm {
            score += 1;
        }
        if has_stroke_hx {
            score += 2;
        }
        if has_vascular_disease {
            score += 1;
        }
        if is_female {
            score += 1;
        }

        score
    }

    pub fn requires_anticoagulation(&self, chadsvasc_score: u32) -> bool {
        self.rhythm_type == CardiacRhythm::AtrialFibrillation && chadsvasc_score >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardiac_node_rates() {
        let sa = CardiacNode::SinoatrialNode;
        assert_eq!(sa.intrinsic_rate_bpm(), 60.0);

        let av = CardiacNode::AtrioventricularNode;
        assert_eq!(av.intrinsic_rate_bpm(), 45.0);
    }

    #[test]
    fn test_action_potential_phases() {
        let mut ap = ActionPotential::new_ventricular();
        assert_eq!(ap.phase, ActionPotentialPhase::Phase4RestingPotential);

        ap.depolarize().unwrap();
        assert_eq!(ap.phase, ActionPotentialPhase::Phase0Depolarization);
        assert!(ap.is_refractory());

        ap.advance_phase().unwrap();
        assert_eq!(ap.phase, ActionPotentialPhase::Phase1EarlyRepolarization);
    }

    #[test]
    fn test_ion_channels() {
        let na_channel = IonChannel::SodiumFast;
        assert_eq!(na_channel.conductance_ms(), 120.0);
        assert_eq!(na_channel.reversal_potential_mv(), 60.0);

        let k_channel = IonChannel::PotassiumDelayed;
        assert_eq!(k_channel.reversal_potential_mv(), -90.0);
    }

    #[test]
    fn test_ecg_normal() {
        let ecg = ECG::new_normal(75.0);
        assert_eq!(ecg.heart_rate_bpm, 75.0);
        assert!(!ecg.has_prolonged_qt());
        assert!(!ecg.has_bundle_branch_block());
        assert!(!ecg.has_first_degree_av_block());
    }

    #[test]
    fn test_ecg_qt_prolongation() {
        let mut ecg = ECG::new_normal(60.0);
        ecg.qt_interval_ms = 500.0;
        assert!(ecg.has_prolonged_qt());
    }

    #[test]
    fn test_ecg_rhythm_assessment() {
        let mut ecg = ECG::new_normal(50.0);
        let rhythm = ecg.assess_rhythm().unwrap();
        assert_eq!(rhythm, CardiacRhythm::SinusBradycardia);

        ecg.heart_rate_bpm = 120.0;
        let rhythm = ecg.assess_rhythm().unwrap();
        assert_eq!(rhythm, CardiacRhythm::SinusTachycardia);
    }

    #[test]
    fn test_conduction_system() {
        let cs = ConductionSystem::new();
        assert_eq!(cs.av_node_delay_ms, 100.0);
        assert!(!cs.has_conduction_block());

        let total_time = cs.calculate_total_conduction_time();
        assert!(total_time > 200.0);
    }

    #[test]
    fn test_pacemaker_hierarchy() {
        let cs = ConductionSystem::new();
        let hierarchy = cs.pacemaker_hierarchy();
        assert_eq!(hierarchy.len(), 4);
        assert!(hierarchy[0].1 > hierarchy[1].1);
    }

    #[test]
    fn test_cardiac_rhythms() {
        assert!(CardiacRhythm::VentricularFibrillation.is_life_threatening());
        assert!(CardiacRhythm::AtrialFibrillation.requires_treatment());
        assert!(!CardiacRhythm::NormalSinus.is_life_threatening());
    }

    #[test]
    fn test_arrhythmia_afib() {
        let afib = Arrhythmia::new_afib(120.0);
        assert_eq!(afib.rhythm_type, CardiacRhythm::AtrialFibrillation);
        assert_eq!(afib.origin, ArrhythmiaOrigin::Supraventricular);

        let chads2vasc = afib.stroke_risk_chadsvasc(75, true, true, false, false, false, true);
        assert!(chads2vasc >= 5);
        assert!(afib.requires_anticoagulation(chads2vasc));
    }

    #[test]
    fn test_arrhythmia_vtach() {
        let vtach = Arrhythmia::new_vtach(180.0);
        assert_eq!(vtach.rhythm_type, CardiacRhythm::VentricularTachycardia);
        assert_eq!(vtach.origin, ArrhythmiaOrigin::Ventricular);
        assert!(vtach.rhythm_type.is_life_threatening());
    }

    #[test]
    fn test_action_potential_duration() {
        let ventricular = ActionPotential::new_ventricular();
        let atrial = ActionPotential::new_atrial();
        let pacemaker = ActionPotential::new_pacemaker();

        assert_eq!(ventricular.duration_ms, 300.0);
        assert_eq!(atrial.duration_ms, 200.0);
        assert_eq!(pacemaker.duration_ms, 150.0);
    }

    #[test]
    fn test_refractory_period() {
        let ap = ActionPotential::new_ventricular();
        let erp = ap.effective_refractory_period_ms();
        assert_eq!(erp, 240.0);
    }
}
