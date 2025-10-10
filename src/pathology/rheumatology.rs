use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RheumatologyProfile {
    pub diagnoses: Vec<RheumatologicDiagnosis>,
    pub joint_status: HashMap<Joint, JointStatus>,
    pub inflammatory_markers: InflammatoryMarkers,
    pub autoantibodies: AutoantibodyPanel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RheumatologicDiagnosis {
    pub condition: RheumatologicCondition,
    pub disease_activity: DiseaseActivity,
    pub onset_age: Option<f64>,
    pub duration_years: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RheumatologicCondition {
    RheumatoidArthritis,
    OsteoArthritis,
    PsoriaticArthritis,
    AnkylosingSpondylitis,
    Gout,
    Pseudogout,
    SystemicLupusErythematosus,
    Sjogrens,
    Scleroderma,
    PolymyalgiaRheumatica,
    FibromyalgiaSyndrome,
    ReactiveArthritis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Joint {
    RightShoulder,
    LeftShoulder,
    RightElbow,
    LeftElbow,
    RightWrist,
    LeftWrist,
    RightHip,
    LeftHip,
    RightKnee,
    LeftKnee,
    RightAnkle,
    LeftAnkle,
    CervicalSpine,
    LumbarSpine,
    ThoracicSpine,
    SacroiliacJoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointStatus {
    pub pain_score: u32,
    pub swelling: SwellingGrade,
    pub tenderness: bool,
    pub range_of_motion_percent: f64,
    pub deformity: Option<JointDeformity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwellingGrade {
    None,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JointDeformity {
    BoutonnierDeformity,
    SwanNeckDeformity,
    UlnarDeviation,
    Subluxation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiseaseActivity {
    Remission,
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InflammatoryMarkers {
    pub esr_mm_hr: f64,
    pub crp_mg_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoantibodyPanel {
    pub rheumatoid_factor: AntibodyResult,
    pub anti_ccp: AntibodyResult,
    pub ana: AntibodyResult,
    pub anti_dsdna: AntibodyResult,
    pub anti_sm: AntibodyResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AntibodyResult {
    pub positive: bool,
    pub titer: Option<f64>,
}

impl RheumatologyProfile {
    pub fn new() -> Self {
        Self {
            diagnoses: Vec::new(),
            joint_status: HashMap::new(),
            inflammatory_markers: InflammatoryMarkers::normal(),
            autoantibodies: AutoantibodyPanel::negative(),
        }
    }

    pub fn add_diagnosis(&mut self, diagnosis: RheumatologicDiagnosis) {
        self.diagnoses.push(diagnosis);
    }

    pub fn update_joint(&mut self, joint: Joint, status: JointStatus) {
        self.joint_status.insert(joint, status);
    }

    pub fn das28_score(&self) -> Option<f64> {
        if !self.has_ra() {
            return None;
        }

        let tender_joints = self.count_tender_joints();
        let swollen_joints = self.count_swollen_joints();
        let esr = self.inflammatory_markers.esr_mm_hr;

        let das28 = 0.56 * (tender_joints as f64).sqrt()
            + 0.28 * (swollen_joints as f64).sqrt()
            + 0.70 * esr.ln()
            + 0.014 * 50.0;

        Some(das28)
    }

    fn has_ra(&self) -> bool {
        self.diagnoses
            .iter()
            .any(|d| d.condition == RheumatologicCondition::RheumatoidArthritis)
    }

    fn count_tender_joints(&self) -> u32 {
        self.joint_status.values().filter(|s| s.tenderness).count() as u32
    }

    fn count_swollen_joints(&self) -> u32 {
        self.joint_status
            .values()
            .filter(|s| !matches!(s.swelling, SwellingGrade::None))
            .count() as u32
    }

    pub fn inflammatory_burden(&self) -> InflammatoryBurden {
        let esr_elevated = self.inflammatory_markers.esr_mm_hr > 20.0;
        let crp_elevated = self.inflammatory_markers.crp_mg_l > 10.0;

        match (esr_elevated, crp_elevated) {
            (true, true) => InflammatoryBurden::High,
            (true, false) | (false, true) => InflammatoryBurden::Moderate,
            (false, false) => InflammatoryBurden::Low,
        }
    }

    pub fn sle_diagnostic_criteria(&self) -> SLECriteriaScore {
        let mut score = 0;

        if self.autoantibodies.ana.positive {
            score += 1;
        }
        if self.autoantibodies.anti_dsdna.positive {
            score += 2;
        }
        if self.autoantibodies.anti_sm.positive {
            score += 2;
        }

        let arthritis_count = self
            .joint_status
            .values()
            .filter(|s| s.swelling != SwellingGrade::None)
            .count();

        if arthritis_count >= 2 {
            score += 1;
        }

        SLECriteriaScore { points: score }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InflammatoryBurden {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SLECriteriaScore {
    pub points: u32,
}

impl SLECriteriaScore {
    pub fn meets_criteria(&self) -> bool {
        self.points >= 4
    }
}

impl Default for RheumatologyProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl InflammatoryMarkers {
    pub fn normal() -> Self {
        Self {
            esr_mm_hr: 10.0,
            crp_mg_l: 3.0,
        }
    }

    pub fn is_elevated(&self) -> bool {
        self.esr_mm_hr > 20.0 || self.crp_mg_l > 10.0
    }
}

impl AutoantibodyPanel {
    pub fn negative() -> Self {
        Self {
            rheumatoid_factor: AntibodyResult {
                positive: false,
                titer: None,
            },
            anti_ccp: AntibodyResult {
                positive: false,
                titer: None,
            },
            ana: AntibodyResult {
                positive: false,
                titer: None,
            },
            anti_dsdna: AntibodyResult {
                positive: false,
                titer: None,
            },
            anti_sm: AntibodyResult {
                positive: false,
                titer: None,
            },
        }
    }
}

impl JointStatus {
    pub fn normal() -> Self {
        Self {
            pain_score: 0,
            swelling: SwellingGrade::None,
            tenderness: false,
            range_of_motion_percent: 100.0,
            deformity: None,
        }
    }

    pub fn is_inflamed(&self) -> bool {
        self.swelling != SwellingGrade::None || self.tenderness
    }
}

impl Joint {
    pub fn is_large_joint(&self) -> bool {
        matches!(
            self,
            Joint::RightShoulder
                | Joint::LeftShoulder
                | Joint::RightHip
                | Joint::LeftHip
                | Joint::RightKnee
                | Joint::LeftKnee
        )
    }

    pub fn is_small_joint(&self) -> bool {
        matches!(
            self,
            Joint::RightWrist | Joint::LeftWrist | Joint::RightAnkle | Joint::LeftAnkle
        )
    }

    pub fn is_axial(&self) -> bool {
        matches!(
            self,
            Joint::CervicalSpine
                | Joint::LumbarSpine
                | Joint::ThoracicSpine
                | Joint::SacroiliacJoint
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rheumatology_profile_creation() {
        let profile = RheumatologyProfile::new();
        assert!(profile.diagnoses.is_empty());
        assert!(!profile.inflammatory_markers.is_elevated());
    }

    #[test]
    fn test_joint_status() {
        let status = JointStatus {
            pain_score: 7,
            swelling: SwellingGrade::Moderate,
            tenderness: true,
            range_of_motion_percent: 60.0,
            deformity: Some(JointDeformity::UlnarDeviation),
        };

        assert!(status.is_inflamed());
    }

    #[test]
    fn test_das28_calculation() {
        let mut profile = RheumatologyProfile::new();

        profile.add_diagnosis(RheumatologicDiagnosis {
            condition: RheumatologicCondition::RheumatoidArthritis,
            disease_activity: DiseaseActivity::High,
            onset_age: Some(45.0),
            duration_years: Some(5.0),
        });

        profile.update_joint(
            Joint::RightWrist,
            JointStatus {
                pain_score: 6,
                swelling: SwellingGrade::Moderate,
                tenderness: true,
                range_of_motion_percent: 70.0,
                deformity: None,
            },
        );

        profile.update_joint(
            Joint::LeftWrist,
            JointStatus {
                pain_score: 6,
                swelling: SwellingGrade::Moderate,
                tenderness: true,
                range_of_motion_percent: 70.0,
                deformity: None,
            },
        );

        profile.inflammatory_markers.esr_mm_hr = 30.0;

        let das28 = profile.das28_score();
        assert!(das28.is_some());
        assert!(das28.unwrap() > 2.6);
    }

    #[test]
    fn test_inflammatory_burden() {
        let mut profile = RheumatologyProfile::new();
        profile.inflammatory_markers.esr_mm_hr = 50.0;
        profile.inflammatory_markers.crp_mg_l = 25.0;

        assert_eq!(profile.inflammatory_burden(), InflammatoryBurden::High);
    }

    #[test]
    fn test_sle_criteria() {
        let mut profile = RheumatologyProfile::new();

        profile.autoantibodies.ana.positive = true;
        profile.autoantibodies.anti_dsdna.positive = true;
        profile.autoantibodies.anti_sm.positive = true;

        let criteria = profile.sle_diagnostic_criteria();
        assert!(criteria.meets_criteria());
    }

    #[test]
    fn test_joint_classification() {
        assert!(Joint::RightKnee.is_large_joint());
        assert!(Joint::RightWrist.is_small_joint());
        assert!(Joint::LumbarSpine.is_axial());
    }
}
