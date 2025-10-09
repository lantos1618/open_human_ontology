use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToothType {
    Incisor,
    Canine,
    Premolar,
    Molar,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToothPosition {
    UpperRight,
    UpperLeft,
    LowerRight,
    LowerLeft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooth {
    pub tooth_type: ToothType,
    pub position: ToothPosition,
    pub number: u8,
    pub present: bool,
    pub health: ToothHealth,
    pub enamel_thickness_mm: f64,
    pub dentin_thickness_mm: f64,
    pub pulp_vitality: PulpVitality,
    pub root_structure: RootStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToothHealth {
    Healthy,
    CavityPresent,
    Filled,
    RootCanal,
    Crowned,
    Implant,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PulpVitality {
    Vital,
    Necrotic,
    RootCanalTreated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootStructure {
    pub root_count: u8,
    pub root_length_mm: f64,
    pub periodontal_ligament_health: f64,
    pub bone_support_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gums {
    pub health: GumHealth,
    pub pocket_depth_mm: f64,
    pub bleeding_on_probing: bool,
    pub recession_mm: f64,
    pub inflammation_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GumHealth {
    Healthy,
    Gingivitis,
    PeriodontitisMild,
    PeriodonditisModerate,
    PeriodonditisSevere,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OralMicrobiome {
    pub bacterial_diversity: f64,
    pub pathogenic_bacteria_level: PathogenLevel,
    pub beneficial_bacteria_level: BeneficialLevel,
    pub ph_level: f64,
    pub dominant_species: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PathogenLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BeneficialLevel {
    Low,
    Moderate,
    High,
    Optimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalivaProfile {
    pub flow_rate_ml_per_min: f64,
    pub ph: f64,
    pub buffering_capacity: f64,
    pub iga_concentration: f64,
    pub antimicrobial_proteins: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DentalSystem {
    pub teeth: Vec<Tooth>,
    pub gums: Gums,
    pub microbiome: OralMicrobiome,
    pub saliva: SalivaProfile,
    pub jaw_alignment: JawAlignment,
    pub bite_force_newtons: f64,
    pub tmj_health: TMJHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JawAlignment {
    Normal,
    Overbite,
    Underbite,
    Crossbite,
    OpenBite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMJHealth {
    pub left_joint_function: f64,
    pub right_joint_function: f64,
    pub clicking_present: bool,
    pub pain_level: f64,
    pub range_of_motion_mm: f64,
}

impl DentalSystem {
    pub fn new_adult() -> Self {
        Self {
            teeth: Self::create_adult_teeth(),
            gums: Gums::healthy(),
            microbiome: OralMicrobiome::healthy(),
            saliva: SalivaProfile::normal(),
            jaw_alignment: JawAlignment::Normal,
            bite_force_newtons: 600.0,
            tmj_health: TMJHealth::healthy(),
        }
    }

    fn create_adult_teeth() -> Vec<Tooth> {
        let mut teeth = Vec::new();

        for quadrant in [
            ToothPosition::UpperRight,
            ToothPosition::UpperLeft,
            ToothPosition::LowerRight,
            ToothPosition::LowerLeft,
        ] {
            for i in 1..=8 {
                let tooth_type = match i {
                    1 | 2 => ToothType::Incisor,
                    3 => ToothType::Canine,
                    4 | 5 => ToothType::Premolar,
                    6 | 7 | 8 => ToothType::Molar,
                    _ => unreachable!(),
                };

                teeth.push(Tooth::healthy(tooth_type, quadrant.clone(), i));
            }
        }

        teeth
    }

    pub fn tooth_count(&self) -> usize {
        self.teeth.iter().filter(|t| t.present).count()
    }

    pub fn cavity_count(&self) -> usize {
        self.teeth
            .iter()
            .filter(|t| t.health == ToothHealth::CavityPresent)
            .count()
    }

    pub fn periodontal_disease_present(&self) -> bool {
        matches!(
            self.gums.health,
            GumHealth::PeriodontitisMild
                | GumHealth::PeriodonditisModerate
                | GumHealth::PeriodonditisSevere
        )
    }

    pub fn overall_oral_health_score(&self) -> f64 {
        let tooth_health = self.tooth_count() as f64 / 32.0 * 40.0;

        let gum_score = match self.gums.health {
            GumHealth::Healthy => 30.0,
            GumHealth::Gingivitis => 20.0,
            GumHealth::PeriodontitisMild => 15.0,
            GumHealth::PeriodonditisModerate => 10.0,
            GumHealth::PeriodonditisSevere => 5.0,
        };

        let microbiome_score = match self.microbiome.pathogenic_bacteria_level {
            PathogenLevel::Low => 20.0,
            PathogenLevel::Moderate => 15.0,
            PathogenLevel::High => 10.0,
            PathogenLevel::VeryHigh => 5.0,
        };

        let saliva_score = if self.saliva.flow_rate_ml_per_min > 0.3 && self.saliva.ph > 6.5 {
            10.0
        } else if self.saliva.flow_rate_ml_per_min > 0.2 {
            7.0
        } else {
            3.0
        };

        tooth_health + gum_score + microbiome_score + saliva_score
    }

    pub fn caries_risk_score(&self) -> f64 {
        let mut risk = 0.0;

        if self.saliva.flow_rate_ml_per_min < 0.3 {
            risk += 2.0;
        }

        if self.saliva.ph < 6.5 {
            risk += 2.5;
        }

        match self.microbiome.pathogenic_bacteria_level {
            PathogenLevel::VeryHigh => risk += 3.0,
            PathogenLevel::High => risk += 2.0,
            PathogenLevel::Moderate => risk += 1.0,
            PathogenLevel::Low => {}
        }

        if self.saliva.buffering_capacity < 0.5 {
            risk += 1.5;
        }

        risk
    }

    pub fn periodontal_risk_assessment(&self) -> String {
        if self.gums.pocket_depth_mm > 6.0 {
            "Severe risk: pocket depth >6mm, immediate treatment needed".to_string()
        } else if self.gums.pocket_depth_mm > 4.0 {
            "High risk: pocket depth 4-6mm, periodontal therapy recommended".to_string()
        } else if self.gums.pocket_depth_mm > 3.0 {
            "Moderate risk: pocket depth 3-4mm, improved hygiene needed".to_string()
        } else if self.gums.bleeding_on_probing {
            "Low risk: bleeding present, improve brushing and flossing".to_string()
        } else {
            "Low risk: maintain good oral hygiene".to_string()
        }
    }
}

impl Tooth {
    fn healthy(tooth_type: ToothType, position: ToothPosition, number: u8) -> Self {
        let (enamel, dentin, root_count, root_length) = match tooth_type {
            ToothType::Incisor => (0.9, 2.5, 1, 13.0),
            ToothType::Canine => (1.0, 2.8, 1, 16.0),
            ToothType::Premolar => (1.1, 3.0, 1, 14.0),
            ToothType::Molar => (1.2, 3.5, 2, 13.0),
        };

        Self {
            tooth_type,
            position,
            number,
            present: true,
            health: ToothHealth::Healthy,
            enamel_thickness_mm: enamel,
            dentin_thickness_mm: dentin,
            pulp_vitality: PulpVitality::Vital,
            root_structure: RootStructure {
                root_count,
                root_length_mm: root_length,
                periodontal_ligament_health: 1.0,
                bone_support_percentage: 100.0,
            },
        }
    }
}

impl Gums {
    fn healthy() -> Self {
        Self {
            health: GumHealth::Healthy,
            pocket_depth_mm: 2.0,
            bleeding_on_probing: false,
            recession_mm: 0.0,
            inflammation_score: 0.0,
        }
    }
}

impl OralMicrobiome {
    fn healthy() -> Self {
        Self {
            bacterial_diversity: 0.8,
            pathogenic_bacteria_level: PathogenLevel::Low,
            beneficial_bacteria_level: BeneficialLevel::High,
            ph_level: 7.0,
            dominant_species: vec![
                "Streptococcus salivarius".to_string(),
                "Streptococcus mitis".to_string(),
                "Veillonella parvula".to_string(),
            ],
        }
    }
}

impl SalivaProfile {
    fn normal() -> Self {
        Self {
            flow_rate_ml_per_min: 0.4,
            ph: 6.8,
            buffering_capacity: 0.7,
            iga_concentration: 20.0,
            antimicrobial_proteins: 1.0,
        }
    }
}

impl TMJHealth {
    fn healthy() -> Self {
        Self {
            left_joint_function: 1.0,
            right_joint_function: 1.0,
            clicking_present: false,
            pain_level: 0.0,
            range_of_motion_mm: 50.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adult_dental_system() {
        let dental = DentalSystem::new_adult();
        assert_eq!(dental.tooth_count(), 32);
    }

    #[test]
    fn test_healthy_tooth_structure() {
        let tooth = Tooth::healthy(ToothType::Molar, ToothPosition::UpperRight, 6);
        assert_eq!(tooth.health, ToothHealth::Healthy);
        assert_eq!(tooth.pulp_vitality, PulpVitality::Vital);
        assert!(tooth.enamel_thickness_mm > 1.0);
    }

    #[test]
    fn test_oral_health_score() {
        let dental = DentalSystem::new_adult();
        let score = dental.overall_oral_health_score();
        assert!(score > 90.0);
    }

    #[test]
    fn test_caries_risk_low_saliva() {
        let mut dental = DentalSystem::new_adult();
        dental.saliva.flow_rate_ml_per_min = 0.2;
        dental.saliva.ph = 6.0;

        let risk = dental.caries_risk_score();
        assert!(risk > 3.0);
    }

    #[test]
    fn test_periodontal_disease_detection() {
        let mut dental = DentalSystem::new_adult();
        dental.gums.health = GumHealth::PeriodontitisMild;

        assert!(dental.periodontal_disease_present());
    }

    #[test]
    fn test_periodontal_risk_severe() {
        let mut dental = DentalSystem::new_adult();
        dental.gums.pocket_depth_mm = 7.0;

        let risk = dental.periodontal_risk_assessment();
        assert!(risk.contains("Severe"));
    }

    #[test]
    fn test_tooth_types() {
        let dental = DentalSystem::new_adult();
        let incisors = dental
            .teeth
            .iter()
            .filter(|t| t.tooth_type == ToothType::Incisor)
            .count();
        let molars = dental
            .teeth
            .iter()
            .filter(|t| t.tooth_type == ToothType::Molar)
            .count();

        assert_eq!(incisors, 8);
        assert_eq!(molars, 12);
    }

    #[test]
    fn test_microbiome_health_impact() {
        let mut dental = DentalSystem::new_adult();
        dental.microbiome.pathogenic_bacteria_level = PathogenLevel::VeryHigh;

        let score = dental.overall_oral_health_score();
        assert!(score < 90.0);
    }
}
