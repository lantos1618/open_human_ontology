use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulmonologyProfile {
    pub conditions: Vec<PulmonaryCondition>,
    pub pulmonary_function: PulmonaryFunctionTests,
    pub gas_exchange: GasExchangeStatus,
    pub imaging_findings: Vec<ImagingFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulmonaryCondition {
    pub diagnosis: PulmonaryDiagnosis,
    pub severity: PulmonarySeverity,
    pub exacerbations_per_year: u32,
    pub treatment_adherence: TreatmentAdherence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PulmonaryDiagnosis {
    Asthma,
    COPD,
    InterstitialLungDisease,
    PulmonaryFibrosis,
    Bronchiectasis,
    SleepApnea,
    PulmonaryHypertension,
    Sarcoidosis,
    Pneumonia,
    PleuralEffusion,
    Pneumothorax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PulmonarySeverity {
    Mild,
    Moderate,
    Severe,
    VerySevre,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreatmentAdherence {
    Excellent,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PulmonaryFunctionTests {
    pub fev1_liters: f64,
    pub fvc_liters: f64,
    pub fev1_fvc_ratio: f64,
    pub dlco_percent_predicted: f64,
    pub tlc_percent_predicted: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GasExchangeStatus {
    pub pao2_mmhg: f64,
    pub paco2_mmhg: f64,
    pub ph: f64,
    pub sao2_percent: f64,
    pub a_a_gradient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagingFinding {
    pub finding_type: ImagingFindingType,
    pub location: LungLocation,
    pub size_mm: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImagingFindingType {
    Nodule,
    Mass,
    Consolidation,
    GroundGlassOpacity,
    Infiltrate,
    Effusion,
    Pneumothorax,
    Fibrosis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LungLocation {
    RightUpperLobe,
    RightMiddleLobe,
    RightLowerLobe,
    LeftUpperLobe,
    Lingula,
    LeftLowerLobe,
}

impl PulmonologyProfile {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            pulmonary_function: PulmonaryFunctionTests::normal(),
            gas_exchange: GasExchangeStatus::normal(),
            imaging_findings: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: PulmonaryCondition) {
        self.conditions.push(condition);
    }

    pub fn add_imaging_finding(&mut self, finding: ImagingFinding) {
        self.imaging_findings.push(finding);
    }

    pub fn spirometry_pattern(&self) -> SpirometryPattern {
        if self.pulmonary_function.fev1_fvc_ratio < 0.70 {
            SpirometryPattern::Obstructive
        } else if self.pulmonary_function.fvc_liters < 0.80 {
            SpirometryPattern::Restrictive
        } else {
            SpirometryPattern::Normal
        }
    }

    pub fn asthma_control(&self) -> Option<AsthmaControl> {
        let asthma = self.conditions.iter()
            .find(|c| c.diagnosis == PulmonaryDiagnosis::Asthma)?;

        let control = match (asthma.exacerbations_per_year, asthma.severity) {
            (0, PulmonarySeverity::Mild) => AsthmaControl::WellControlled,
            (1..=2, _) => AsthmaControl::PartiallyControlled,
            (_, PulmonarySeverity::Severe | PulmonarySeverity::VerySevre) => AsthmaControl::Uncontrolled,
            _ => AsthmaControl::PartiallyControlled,
        };

        Some(control)
    }

    pub fn gold_copd_stage(&self) -> Option<GOLDStage> {
        let has_copd = self.conditions.iter()
            .any(|c| c.diagnosis == PulmonaryDiagnosis::COPD);

        if !has_copd {
            return None;
        }

        let fev1_percent = self.pulmonary_function.fev1_percent_predicted();

        let stage = match fev1_percent {
            x if x >= 80.0 => GOLDStage::Gold1,
            x if x >= 50.0 => GOLDStage::Gold2,
            x if x >= 30.0 => GOLDStage::Gold3,
            _ => GOLDStage::Gold4,
        };

        Some(stage)
    }

    pub fn hypoxemia_present(&self) -> bool {
        self.gas_exchange.pao2_mmhg < 60.0 || self.gas_exchange.sao2_percent < 88.0
    }

    pub fn respiratory_failure_type(&self) -> Option<RespiratoryFailureType> {
        if self.gas_exchange.pao2_mmhg >= 60.0 {
            return None;
        }

        if self.gas_exchange.paco2_mmhg > 50.0 {
            Some(RespiratoryFailureType::Type2)
        } else {
            Some(RespiratoryFailureType::Type1)
        }
    }

    pub fn nodule_malignancy_risk(&self) -> NoduleRisk {
        for finding in &self.imaging_findings {
            if let ImagingFindingType::Nodule = finding.finding_type {
                if let Some(size) = finding.size_mm {
                    return match size {
                        x if x < 6.0 => NoduleRisk::Low,
                        x if x < 15.0 => NoduleRisk::Moderate,
                        _ => NoduleRisk::High,
                    };
                }
            }
        }
        NoduleRisk::Low
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpirometryPattern {
    Normal,
    Obstructive,
    Restrictive,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsthmaControl {
    WellControlled,
    PartiallyControlled,
    Uncontrolled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GOLDStage {
    Gold1,
    Gold2,
    Gold3,
    Gold4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RespiratoryFailureType {
    Type1,
    Type2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoduleRisk {
    Low,
    Moderate,
    High,
}

impl Default for PulmonologyProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl PulmonaryFunctionTests {
    pub fn normal() -> Self {
        Self {
            fev1_liters: 3.5,
            fvc_liters: 4.5,
            fev1_fvc_ratio: 0.78,
            dlco_percent_predicted: 90.0,
            tlc_percent_predicted: 95.0,
        }
    }

    pub fn fev1_percent_predicted(&self) -> f64 {
        (self.fev1_liters / 3.5) * 100.0
    }

    pub fn is_obstructive(&self) -> bool {
        self.fev1_fvc_ratio < 0.70
    }

    pub fn is_restrictive(&self) -> bool {
        self.fev1_fvc_ratio >= 0.70 && self.fvc_liters < 3.6
    }
}

impl GasExchangeStatus {
    pub fn normal() -> Self {
        Self {
            pao2_mmhg: 95.0,
            paco2_mmhg: 40.0,
            ph: 7.40,
            sao2_percent: 98.0,
            a_a_gradient: 10.0,
        }
    }

    pub fn is_acidotic(&self) -> bool {
        self.ph < 7.35
    }

    pub fn is_alkalotic(&self) -> bool {
        self.ph > 7.45
    }

    pub fn respiratory_acidosis(&self) -> bool {
        self.is_acidotic() && self.paco2_mmhg > 45.0
    }

    pub fn respiratory_alkalosis(&self) -> bool {
        self.is_alkalotic() && self.paco2_mmhg < 35.0
    }
}

impl LungLocation {
    pub fn is_upper_lobe(&self) -> bool {
        matches!(self, LungLocation::RightUpperLobe | LungLocation::LeftUpperLobe)
    }

    pub fn is_lower_lobe(&self) -> bool {
        matches!(self, LungLocation::RightLowerLobe | LungLocation::LeftLowerLobe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pulmonology_profile_creation() {
        let profile = PulmonologyProfile::new();
        assert!(profile.conditions.is_empty());
        assert_eq!(profile.pulmonary_function.fev1_fvc_ratio, 0.78);
    }

    #[test]
    fn test_spirometry_pattern() {
        let mut profile = PulmonologyProfile::new();
        profile.pulmonary_function.fev1_fvc_ratio = 0.65;

        assert_eq!(profile.spirometry_pattern(), SpirometryPattern::Obstructive);
    }

    #[test]
    fn test_asthma_control() {
        let mut profile = PulmonologyProfile::new();

        profile.add_condition(PulmonaryCondition {
            diagnosis: PulmonaryDiagnosis::Asthma,
            severity: PulmonarySeverity::Mild,
            exacerbations_per_year: 0,
            treatment_adherence: TreatmentAdherence::Excellent,
        });

        let control = profile.asthma_control();
        assert_eq!(control, Some(AsthmaControl::WellControlled));
    }

    #[test]
    fn test_gold_copd_staging() {
        let mut profile = PulmonologyProfile::new();

        profile.add_condition(PulmonaryCondition {
            diagnosis: PulmonaryDiagnosis::COPD,
            severity: PulmonarySeverity::Moderate,
            exacerbations_per_year: 2,
            treatment_adherence: TreatmentAdherence::Good,
        });

        profile.pulmonary_function.fev1_liters = 2.1;

        let stage = profile.gold_copd_stage();
        assert_eq!(stage, Some(GOLDStage::Gold2));
    }

    #[test]
    fn test_hypoxemia_detection() {
        let mut profile = PulmonologyProfile::new();
        profile.gas_exchange.pao2_mmhg = 55.0;

        assert!(profile.hypoxemia_present());
    }

    #[test]
    fn test_respiratory_failure_type() {
        let mut profile = PulmonologyProfile::new();
        profile.gas_exchange.pao2_mmhg = 55.0;
        profile.gas_exchange.paco2_mmhg = 55.0;

        assert_eq!(profile.respiratory_failure_type(), Some(RespiratoryFailureType::Type2));
    }

    #[test]
    fn test_nodule_risk_assessment() {
        let mut profile = PulmonologyProfile::new();

        profile.add_imaging_finding(ImagingFinding {
            finding_type: ImagingFindingType::Nodule,
            location: LungLocation::RightUpperLobe,
            size_mm: Some(20.0),
        });

        assert_eq!(profile.nodule_malignancy_risk(), NoduleRisk::High);
    }

    #[test]
    fn test_acid_base_status() {
        let mut gas_exchange = GasExchangeStatus::normal();
        gas_exchange.ph = 7.25;
        gas_exchange.paco2_mmhg = 55.0;

        assert!(gas_exchange.respiratory_acidosis());
    }
}
