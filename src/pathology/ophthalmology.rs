use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OphthalmologyProfile {
    pub visual_acuity: VisualAcuity,
    pub refractive_error: RefractiveError,
    pub intraocular_pressure: IntraocularPressure,
    pub conditions: Vec<EyeCondition>,
    pub retinal_health: RetinalHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualAcuity {
    pub right_eye: SnellenAcuity,
    pub left_eye: SnellenAcuity,
    pub binocular: SnellenAcuity,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SnellenAcuity {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefractiveError {
    pub right_eye: Refraction,
    pub left_eye: Refraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refraction {
    pub sphere: f64,
    pub cylinder: f64,
    pub axis: f64,
    pub error_type: RefractiveErrorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefractiveErrorType {
    Emmetropia,
    Myopia,
    Hyperopia,
    Astigmatism,
    Presbyopia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntraocularPressure {
    pub right_eye_mmhg: f64,
    pub left_eye_mmhg: f64,
    pub measurement_method: PressureMeasurementMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureMeasurementMethod {
    GoldmannApplanation,
    NonContact,
    Rebound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeCondition {
    pub name: String,
    pub affected_eye: AffectedEye,
    pub severity: OphthalmoSeverity,
    pub onset_age: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AffectedEye {
    Right,
    Left,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OphthalmoSeverity {
    Mild,
    Moderate,
    Severe,
    Profound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetinalHealth {
    pub cup_to_disc_ratio: CupToDiscRatio,
    pub macula_status: MaculaStatus,
    pub vascular_health: VascularHealth,
    pub diabetic_retinopathy: Option<DiabeticRetinopathyStage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CupToDiscRatio {
    pub right_eye: f64,
    pub left_eye: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaculaStatus {
    Normal,
    DrussenPresent,
    MacularDegeneration,
    MacularEdema,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VascularHealth {
    Normal,
    ArteriolarNarrowing,
    AVNicking,
    Hemorrhages,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiabeticRetinopathyStage {
    NoRetinopathy,
    MildNPDR,
    ModerateNPDR,
    SevereNPDR,
    PDR,
}

impl OphthalmologyProfile {
    pub fn new() -> Self {
        Self {
            visual_acuity: VisualAcuity::normal(),
            refractive_error: RefractiveError::emmetropic(),
            intraocular_pressure: IntraocularPressure::normal(),
            conditions: Vec::new(),
            retinal_health: RetinalHealth::normal(),
        }
    }

    pub fn add_condition(&mut self, condition: EyeCondition) {
        self.conditions.push(condition);
    }

    pub fn glaucoma_risk(&self) -> GlaucomaRisk {
        let mut risk_score = 0;

        if self.intraocular_pressure.right_eye_mmhg > 21.0
            || self.intraocular_pressure.left_eye_mmhg > 21.0
        {
            risk_score += 2;
        }

        if self.retinal_health.cup_to_disc_ratio.right_eye > 0.6
            || self.retinal_health.cup_to_disc_ratio.left_eye > 0.6
        {
            risk_score += 2;
        }

        match risk_score {
            0..=1 => GlaucomaRisk::Low,
            2..=3 => GlaucomaRisk::Moderate,
            _ => GlaucomaRisk::High,
        }
    }

    pub fn macular_degeneration_risk(&self) -> AMDRisk {
        match self.retinal_health.macula_status {
            MaculaStatus::Normal => AMDRisk::Low,
            MaculaStatus::DrussenPresent => AMDRisk::Moderate,
            MaculaStatus::MacularDegeneration => AMDRisk::High,
            MaculaStatus::MacularEdema => AMDRisk::High,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlaucomaRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AMDRisk {
    Low,
    Moderate,
    High,
}

impl Default for OphthalmologyProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualAcuity {
    pub fn normal() -> Self {
        Self {
            right_eye: SnellenAcuity {
                numerator: 20,
                denominator: 20,
            },
            left_eye: SnellenAcuity {
                numerator: 20,
                denominator: 20,
            },
            binocular: SnellenAcuity {
                numerator: 20,
                denominator: 15,
            },
        }
    }
}

impl SnellenAcuity {
    pub fn decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn logmar(&self) -> f64 {
        -(self.decimal().log10())
    }
}

impl RefractiveError {
    pub fn emmetropic() -> Self {
        Self {
            right_eye: Refraction::emmetropic(),
            left_eye: Refraction::emmetropic(),
        }
    }
}

impl Refraction {
    pub fn emmetropic() -> Self {
        Self {
            sphere: 0.0,
            cylinder: 0.0,
            axis: 0.0,
            error_type: RefractiveErrorType::Emmetropia,
        }
    }

    pub fn spherical_equivalent(&self) -> f64 {
        self.sphere + (self.cylinder / 2.0)
    }
}

impl IntraocularPressure {
    pub fn normal() -> Self {
        Self {
            right_eye_mmhg: 15.0,
            left_eye_mmhg: 15.0,
            measurement_method: PressureMeasurementMethod::GoldmannApplanation,
        }
    }

    pub fn is_elevated(&self) -> bool {
        self.right_eye_mmhg > 21.0 || self.left_eye_mmhg > 21.0
    }
}

impl RetinalHealth {
    pub fn normal() -> Self {
        Self {
            cup_to_disc_ratio: CupToDiscRatio {
                right_eye: 0.3,
                left_eye: 0.3,
            },
            macula_status: MaculaStatus::Normal,
            vascular_health: VascularHealth::Normal,
            diabetic_retinopathy: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ophthalmology_profile_creation() {
        let profile = OphthalmologyProfile::new();
        assert_eq!(profile.visual_acuity.right_eye.numerator, 20);
        assert_eq!(profile.visual_acuity.right_eye.denominator, 20);
    }

    #[test]
    fn test_snellen_acuity_conversions() {
        let acuity = SnellenAcuity {
            numerator: 20,
            denominator: 40,
        };
        assert_eq!(acuity.decimal(), 0.5);
        assert!((acuity.logmar() - 0.301).abs() < 0.01);
    }

    #[test]
    fn test_glaucoma_risk_assessment() {
        let mut profile = OphthalmologyProfile::new();
        profile.intraocular_pressure.right_eye_mmhg = 25.0;
        profile.retinal_health.cup_to_disc_ratio.right_eye = 0.7;

        let risk = profile.glaucoma_risk();
        assert_eq!(risk, GlaucomaRisk::High);
    }

    #[test]
    fn test_spherical_equivalent() {
        let refraction = Refraction {
            sphere: -2.0,
            cylinder: -1.0,
            axis: 90.0,
            error_type: RefractiveErrorType::Myopia,
        };

        assert_eq!(refraction.spherical_equivalent(), -2.5);
    }

    #[test]
    fn test_iop_elevation() {
        let iop = IntraocularPressure {
            right_eye_mmhg: 24.0,
            left_eye_mmhg: 16.0,
            measurement_method: PressureMeasurementMethod::GoldmannApplanation,
        };

        assert!(iop.is_elevated());
    }
}
