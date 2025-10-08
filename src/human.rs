use serde::{Deserialize, Serialize};
use crate::systems::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Human {
    pub id: String,
    pub demographics: Demographics,
    pub body_metrics: BodyMetrics,
    pub systems: BodySystems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demographics {
    pub age_years: f64,
    pub biological_sex: BiologicalSex,
    pub ancestry: Vec<Ancestry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ancestry {
    pub population: String,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMetrics {
    pub height_cm: f64,
    pub weight_kg: f64,
    pub body_surface_area_m2: f64,
    pub blood_volume_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySystems {
    pub cardiovascular: CardiovascularSystem,
    pub respiratory: RespiratorySystem,
    pub nervous: NervousSystemIntegrated,
    pub digestive: DigestiveSystem,
    pub renal: RenalSystem,
    pub endocrine: EndocrineLandscape,
    pub muscular: MuscularSystem,
    pub skeletal: SkeletalSystem,
    pub integumentary: IntegumentarySystem,
    pub immune: ImmuneSystem,
    pub reproductive: ReproductiveSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularSystem {
    pub heart: Heart,
    pub blood_vessels: Vec<BloodVessel>,
    pub blood: Blood,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratorySystem {
    pub left_lung: Lung,
    pub right_lung: Lung,
    pub gas_exchange: GasExchange,
    pub breathing_pattern: BreathingPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NervousSystemIntegrated {
    pub central: CentralNervousSystem,
    pub peripheral: PeripheralNervousSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestiveSystem {
    pub gi_tract: GITract,
    pub nutrient_absorption: NutrientAbsorption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalSystem {
    pub left_kidney: Kidney,
    pub right_kidney: Kidney,
    pub filtration: Filtration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscularSystem {
    pub total_muscle_mass_kg: f64,
    pub fiber_type_distribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkeletalSystem {
    pub total_bone_mass_kg: f64,
    pub bone_density_g_cm3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegumentarySystem {
    pub skin: Skin,
    pub skin_type: integumentary::SkinType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneSystem {
    pub lymphatic: LymphaticSystem,
    pub wbc_count_per_ul: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReproductiveSystem {
    Male(MaleReproductiveSystem),
    Female(FemaleReproductiveSystem),
}

impl Human {
    pub fn new_adult_male(id: String, age_years: f64, height_cm: f64, weight_kg: f64) -> Self {
        let body_metrics = BodyMetrics::calculate(height_cm, weight_kg);

        Self {
            id,
            demographics: Demographics {
                age_years,
                biological_sex: BiologicalSex::Male,
                ancestry: vec![],
            },
            body_metrics,
            systems: BodySystems::new_adult_male(),
        }
    }

    pub fn new_adult_female(id: String, age_years: f64, height_cm: f64, weight_kg: f64) -> Self {
        let body_metrics = BodyMetrics::calculate(height_cm, weight_kg);

        Self {
            id,
            demographics: Demographics {
                age_years,
                biological_sex: BiologicalSex::Female,
                ancestry: vec![],
            },
            body_metrics,
            systems: BodySystems::new_adult_female(),
        }
    }

    pub fn bmi(&self) -> f64 {
        let height_m = self.body_metrics.height_cm / 100.0;
        self.body_metrics.weight_kg / (height_m * height_m)
    }

    pub fn cardiac_output_l_per_min(&self) -> f64 {
        self.systems.cardiovascular.heart.cardiac_output_l_min()
    }

    pub fn metabolic_rate_kcal_per_day(&self) -> f64 {
        let bmr = match self.demographics.biological_sex {
            BiologicalSex::Male => {
                10.0 * self.body_metrics.weight_kg
                    + 6.25 * self.body_metrics.height_cm
                    - 5.0 * self.demographics.age_years
                    + 5.0
            }
            BiologicalSex::Female => {
                10.0 * self.body_metrics.weight_kg
                    + 6.25 * self.body_metrics.height_cm
                    - 5.0 * self.demographics.age_years
                    - 161.0
            }
        };
        bmr
    }

    pub fn total_blood_volume_l(&self) -> f64 {
        self.body_metrics.blood_volume_l
    }

    pub fn gfr_ml_per_min(&self) -> f64 {
        (self.systems.renal.left_kidney.gfr_ml_per_min
            + self.systems.renal.right_kidney.gfr_ml_per_min)
            / 2.0
    }

    pub fn health_summary(&self) -> HealthSummary {
        HealthSummary {
            bmi: self.bmi(),
            cardiac_output: self.cardiac_output_l_per_min(),
            respiratory_rate: self.systems.respiratory.breathing_pattern.rate_bpm,
            gfr: self.gfr_ml_per_min(),
            metabolic_rate: self.metabolic_rate_kcal_per_day(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    pub bmi: f64,
    pub cardiac_output: f64,
    pub respiratory_rate: f64,
    pub gfr: f64,
    pub metabolic_rate: f64,
}

impl BodyMetrics {
    pub fn calculate(height_cm: f64, weight_kg: f64) -> Self {
        let bsa = ((height_cm * weight_kg) / 3600.0).sqrt();
        let blood_volume = weight_kg * 0.07;

        Self {
            height_cm,
            weight_kg,
            body_surface_area_m2: bsa,
            blood_volume_l: blood_volume,
        }
    }
}

impl BodySystems {
    pub fn new_adult_male() -> Self {
        Self {
            cardiovascular: CardiovascularSystem {
                heart: Heart::new(),
                blood_vessels: vec![],
                blood: Blood::new(cardiovascular::BloodType::OPositive),
            },
            respiratory: RespiratorySystem {
                left_lung: Lung::new_left(),
                right_lung: Lung::new_right(),
                gas_exchange: GasExchange::new_normal(),
                breathing_pattern: BreathingPattern::new_normal(),
            },
            nervous: NervousSystemIntegrated {
                central: CentralNervousSystem::new_adult(),
                peripheral: PeripheralNervousSystem::new(),
            },
            digestive: DigestiveSystem {
                gi_tract: GITract::new_adult(),
                nutrient_absorption: NutrientAbsorption::new_normal(),
            },
            renal: RenalSystem {
                left_kidney: Kidney::new_left(),
                right_kidney: Kidney::new_right(),
                filtration: Filtration::new_normal(),
            },
            endocrine: EndocrineLandscape::new_adult_male(),
            muscular: MuscularSystem {
                total_muscle_mass_kg: 35.0,
                fiber_type_distribution: 0.5,
            },
            skeletal: SkeletalSystem {
                total_bone_mass_kg: 4.5,
                bone_density_g_cm3: 1.3,
            },
            integumentary: IntegumentarySystem {
                skin: Skin::new_adult(1.8),
                skin_type: integumentary::SkinType::new_type_iii(),
            },
            immune: ImmuneSystem {
                lymphatic: LymphaticSystem::new_adult(),
                wbc_count_per_ul: 7000.0,
            },
            reproductive: ReproductiveSystem::Male(MaleReproductiveSystem::new_adult()),
        }
    }

    pub fn new_adult_female() -> Self {
        Self {
            cardiovascular: CardiovascularSystem {
                heart: Heart::new(),
                blood_vessels: vec![],
                blood: Blood::new(cardiovascular::BloodType::OPositive),
            },
            respiratory: RespiratorySystem {
                left_lung: Lung::new_left(),
                right_lung: Lung::new_right(),
                gas_exchange: GasExchange::new_normal(),
                breathing_pattern: BreathingPattern::new_normal(),
            },
            nervous: NervousSystemIntegrated {
                central: CentralNervousSystem::new_adult(),
                peripheral: PeripheralNervousSystem::new(),
            },
            digestive: DigestiveSystem {
                gi_tract: GITract::new_adult(),
                nutrient_absorption: NutrientAbsorption::new_normal(),
            },
            renal: RenalSystem {
                left_kidney: Kidney::new_left(),
                right_kidney: Kidney::new_right(),
                filtration: Filtration::new_normal(),
            },
            endocrine: EndocrineLandscape::new_adult_female(),
            muscular: MuscularSystem {
                total_muscle_mass_kg: 28.0,
                fiber_type_distribution: 0.5,
            },
            skeletal: SkeletalSystem {
                total_bone_mass_kg: 3.5,
                bone_density_g_cm3: 1.2,
            },
            integumentary: IntegumentarySystem {
                skin: Skin::new_adult(1.6),
                skin_type: integumentary::SkinType::new_type_iii(),
            },
            immune: ImmuneSystem {
                lymphatic: LymphaticSystem::new_adult(),
                wbc_count_per_ul: 7000.0,
            },
            reproductive: ReproductiveSystem::Female(FemaleReproductiveSystem::new_adult()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_human_male() {
        let human = Human::new_adult_male("test_001".to_string(), 30.0, 175.0, 75.0);
        assert_eq!(human.demographics.biological_sex, BiologicalSex::Male);
        assert_eq!(human.demographics.age_years, 30.0);
    }

    #[test]
    fn test_create_human_female() {
        let human = Human::new_adult_female("test_002".to_string(), 28.0, 165.0, 60.0);
        assert_eq!(human.demographics.biological_sex, BiologicalSex::Female);
    }

    #[test]
    fn test_bmi_calculation() {
        let human = Human::new_adult_male("test_003".to_string(), 25.0, 180.0, 80.0);
        let bmi = human.bmi();
        assert!(bmi > 20.0 && bmi < 30.0);
    }

    #[test]
    fn test_cardiac_output() {
        let human = Human::new_adult_male("test_004".to_string(), 30.0, 175.0, 75.0);
        let co = human.cardiac_output_l_per_min();
        assert!(co > 4.0 && co < 6.0);
    }

    #[test]
    fn test_metabolic_rate() {
        let human = Human::new_adult_male("test_005".to_string(), 30.0, 175.0, 75.0);
        let bmr = human.metabolic_rate_kcal_per_day();
        assert!(bmr > 1500.0 && bmr < 2500.0);
    }

    #[test]
    fn test_gfr() {
        let human = Human::new_adult_male("test_006".to_string(), 30.0, 175.0, 75.0);
        let gfr = human.gfr_ml_per_min();
        assert!(gfr > 50.0);
    }

    #[test]
    fn test_health_summary() {
        let human = Human::new_adult_female("test_007".to_string(), 25.0, 165.0, 60.0);
        let summary = human.health_summary();
        assert!(summary.bmi > 0.0);
        assert!(summary.cardiac_output > 0.0);
        assert!(summary.gfr > 0.0);
    }

    #[test]
    fn test_body_systems_male() {
        let systems = BodySystems::new_adult_male();
        assert!(matches!(
            systems.reproductive,
            ReproductiveSystem::Male(_)
        ));
        assert!(systems.muscular.total_muscle_mass_kg > 30.0);
    }

    #[test]
    fn test_body_systems_female() {
        let systems = BodySystems::new_adult_female();
        assert!(matches!(
            systems.reproductive,
            ReproductiveSystem::Female(_)
        ));
        assert!(systems.muscular.total_muscle_mass_kg < 30.0);
    }
}
