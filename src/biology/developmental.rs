use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DevelopmentalStage {
    Embryonic(EmbryonicStage),
    Fetal(FetalStage),
    Neonatal,
    Infancy(InfancyStage),
    Childhood(ChildhoodStage),
    Adolescence(AdolescenceStage),
    YoungAdult,
    MiddleAdult,
    OlderAdult,
    Elderly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbryonicStage {
    Zygote,
    Cleavage,
    Morula,
    Blastocyst,
    Gastrulation,
    Neurulation,
    Organogenesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FetalStage {
    EarlyFetal,
    MidFetal,
    LateFetal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfancyStage {
    Newborn,
    EarlyInfancy,
    LateInfancy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildhoodStage {
    EarlyChildhood,
    MiddleChildhood,
    LateChildhood,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdolescenceStage {
    EarlyAdolescence,
    MiddleAdolescence,
    LateAdolescence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetrics {
    pub age_years: f64,
    pub height_cm: f64,
    pub weight_kg: f64,
    pub head_circumference_cm: Option<f64>,
    pub bmi: f64,
    pub growth_velocity_cm_per_year: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentalMilestones {
    pub motor_milestones: Vec<Milestone>,
    pub cognitive_milestones: Vec<Milestone>,
    pub social_milestones: Vec<Milestone>,
    pub language_milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub typical_age_months: f64,
    pub achieved: bool,
    pub age_achieved_months: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubertalDevelopment {
    pub tanner_stage: TannerStage,
    pub menarche_age: Option<f64>,
    pub growth_spurt_onset: Option<f64>,
    pub voice_change: Option<bool>,
    pub secondary_sex_characteristics: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TannerStage {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneDevelopment {
    pub bone_age_years: f64,
    pub growth_plates_status: HashMap<String, GrowthPlateStatus>,
    pub ossification_centers: Vec<OssificationCenter>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrowthPlateStatus {
    Open,
    PartiallyFused,
    Fused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssificationCenter {
    pub name: String,
    pub location: String,
    pub appearance_age_months: f64,
    pub fusion_age_years: f64,
    pub is_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurodevelopmentalProfile {
    pub brain_volume_ml: f64,
    pub myelination_status: MyelinationStatus,
    pub synaptic_density: f64,
    pub cognitive_development_quotient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MyelinationStatus {
    Minimal,
    Developing,
    NearComplete,
    Complete,
}

impl DevelopmentalStage {
    pub fn from_age_years(age: f64) -> Self {
        if age < 0.019 {
            Self::Embryonic(EmbryonicStage::Organogenesis)
        } else if age < 0.75 {
            Self::Fetal(FetalStage::LateFetal)
        } else if age < 0.077 {
            Self::Neonatal
        } else if age < 2.0 {
            Self::Infancy(InfancyStage::LateInfancy)
        } else if age < 12.0 {
            Self::Childhood(ChildhoodStage::MiddleChildhood)
        } else if age < 20.0 {
            Self::Adolescence(AdolescenceStage::MiddleAdolescence)
        } else if age < 35.0 {
            Self::YoungAdult
        } else if age < 55.0 {
            Self::MiddleAdult
        } else if age < 75.0 {
            Self::OlderAdult
        } else {
            Self::Elderly
        }
    }
}

impl GrowthMetrics {
    pub fn new(age_years: f64, height_cm: f64, weight_kg: f64) -> Self {
        let bmi = weight_kg / ((height_cm / 100.0) * (height_cm / 100.0));
        Self {
            age_years,
            height_cm,
            weight_kg,
            head_circumference_cm: None,
            bmi,
            growth_velocity_cm_per_year: 0.0,
        }
    }

    pub fn calculate_percentile(&self, reference: &str) -> f64 {
        match reference {
            "height" => {
                if self.height_cm > 170.0 {
                    0.75
                } else {
                    0.5
                }
            }
            "weight" => {
                if self.weight_kg > 70.0 {
                    0.75
                } else {
                    0.5
                }
            }
            "bmi" => {
                if self.bmi > 25.0 {
                    0.75
                } else {
                    0.5
                }
            }
            _ => 0.5,
        }
    }

    pub fn is_growth_delayed(&self) -> bool {
        self.calculate_percentile("height") < 0.03
    }
}

impl PubertalDevelopment {
    pub fn new() -> Self {
        Self {
            tanner_stage: TannerStage::Stage1,
            menarche_age: None,
            growth_spurt_onset: None,
            voice_change: None,
            secondary_sex_characteristics: Vec::new(),
        }
    }

    pub fn is_precocious(&self, age_years: f64) -> bool {
        age_years < 8.0 && self.tanner_stage != TannerStage::Stage1
    }

    pub fn is_delayed(&self, age_years: f64) -> bool {
        age_years > 14.0 && self.tanner_stage == TannerStage::Stage1
    }
}

impl Default for PubertalDevelopment {
    fn default() -> Self {
        Self::new()
    }
}

impl BoneDevelopment {
    pub fn new(chronological_age: f64) -> Self {
        Self {
            bone_age_years: chronological_age,
            growth_plates_status: HashMap::new(),
            ossification_centers: Vec::new(),
        }
    }

    pub fn bone_age_discrepancy(&self, chronological_age: f64) -> f64 {
        self.bone_age_years - chronological_age
    }

    pub fn growth_potential(&self) -> bool {
        self.growth_plates_status
            .values()
            .any(|status| matches!(status, GrowthPlateStatus::Open))
    }
}

impl NeurodevelopmentalProfile {
    pub fn new(age_years: f64) -> Self {
        let brain_volume_ml = if age_years < 2.0 {
            400.0 + (age_years * 300.0)
        } else if age_years < 18.0 {
            1000.0 + ((age_years - 2.0) * 25.0)
        } else {
            1400.0
        };

        let myelination_status = if age_years < 2.0 {
            MyelinationStatus::Developing
        } else if age_years < 25.0 {
            MyelinationStatus::NearComplete
        } else {
            MyelinationStatus::Complete
        };

        Self {
            brain_volume_ml,
            myelination_status,
            synaptic_density: 1.0,
            cognitive_development_quotient: 100.0,
        }
    }

    pub fn is_age_appropriate(&self) -> bool {
        self.cognitive_development_quotient >= 85.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_developmental_stage_from_age() {
        let infant = DevelopmentalStage::from_age_years(1.0);
        assert!(matches!(infant, DevelopmentalStage::Infancy(_)));

        let child = DevelopmentalStage::from_age_years(8.0);
        assert!(matches!(child, DevelopmentalStage::Childhood(_)));

        let adolescent = DevelopmentalStage::from_age_years(15.0);
        assert!(matches!(adolescent, DevelopmentalStage::Adolescence(_)));
    }

    #[test]
    fn test_growth_metrics() {
        let metrics = GrowthMetrics::new(10.0, 140.0, 35.0);
        assert!((metrics.bmi - 17.86).abs() < 0.1);
        assert!(!metrics.is_growth_delayed());
    }

    #[test]
    fn test_pubertal_development() {
        let puberty = PubertalDevelopment::new();
        assert_eq!(puberty.tanner_stage, TannerStage::Stage1);
        assert!(!puberty.is_precocious(10.0));
    }

    #[test]
    fn test_bone_development() {
        let bone = BoneDevelopment::new(10.0);
        assert_eq!(bone.bone_age_years, 10.0);
        assert_eq!(bone.bone_age_discrepancy(10.0), 0.0);
    }

    #[test]
    fn test_neurodevelopment() {
        let neuro = NeurodevelopmentalProfile::new(5.0);
        assert!(neuro.brain_volume_ml > 1000.0);
        assert!(matches!(
            neuro.myelination_status,
            MyelinationStatus::NearComplete
        ));
        assert!(neuro.is_age_appropriate());
    }
}
