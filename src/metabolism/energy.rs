use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBalance {
    pub energy_intake_kcal_day: f64,
    pub total_energy_expenditure_kcal_day: f64,
    pub energy_storage_kcal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasalMetabolicRate {
    pub bmr_kcal_day: f64,
    pub lean_body_mass_kg: f64,
    pub fat_mass_kg: f64,
    pub age_years: f64,
    pub biological_sex: BiologicalSex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalEnergyExpenditure {
    pub basal_metabolic_rate: f64,
    pub thermic_effect_food: f64,
    pub activity_energy_expenditure: f64,
    pub non_exercise_activity_thermogenesis: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtraActive,
}

impl EnergyBalance {
    pub fn new(intake: f64, expenditure: f64) -> Self {
        Self {
            energy_intake_kcal_day: intake,
            total_energy_expenditure_kcal_day: expenditure,
            energy_storage_kcal: 0.0,
        }
    }

    pub fn daily_balance(&self) -> f64 {
        self.energy_intake_kcal_day - self.total_energy_expenditure_kcal_day
    }

    pub fn is_in_deficit(&self) -> bool {
        self.daily_balance() < 0.0
    }

    pub fn is_in_surplus(&self) -> bool {
        self.daily_balance() > 0.0
    }

    pub fn is_balanced(&self) -> bool {
        self.daily_balance().abs() < 100.0
    }

    pub fn estimated_weight_change_kg_per_week(&self) -> f64 {
        (self.daily_balance() * 7.0) / 7700.0
    }
}

impl BasalMetabolicRate {
    pub fn calculate_mifflin_st_jeor(
        weight_kg: f64,
        height_cm: f64,
        age_years: f64,
        sex: BiologicalSex,
    ) -> f64 {
        let base = 10.0 * weight_kg + 6.25 * height_cm - 5.0 * age_years;
        match sex {
            BiologicalSex::Male => base + 5.0,
            BiologicalSex::Female => base - 161.0,
        }
    }

    pub fn calculate_harris_benedict(
        weight_kg: f64,
        height_cm: f64,
        age_years: f64,
        sex: BiologicalSex,
    ) -> f64 {
        match sex {
            BiologicalSex::Male => {
                88.362 + (13.397 * weight_kg) + (4.799 * height_cm) - (5.677 * age_years)
            }
            BiologicalSex::Female => {
                447.593 + (9.247 * weight_kg) + (3.098 * height_cm) - (4.330 * age_years)
            }
        }
    }

    pub fn calculate_katch_mcardle(lean_body_mass_kg: f64) -> f64 {
        370.0 + (21.6 * lean_body_mass_kg)
    }

    pub fn new(weight_kg: f64, height_cm: f64, age_years: f64, sex: BiologicalSex) -> Self {
        let bmr = Self::calculate_mifflin_st_jeor(weight_kg, height_cm, age_years, sex);
        let lean_mass = weight_kg * 0.75;
        let fat_mass = weight_kg * 0.25;

        Self {
            bmr_kcal_day: bmr,
            lean_body_mass_kg: lean_mass,
            fat_mass_kg: fat_mass,
            age_years,
            biological_sex: sex,
        }
    }

    pub fn energy_per_kg_lean_mass(&self) -> f64 {
        self.bmr_kcal_day / self.lean_body_mass_kg
    }
}

impl TotalEnergyExpenditure {
    pub fn new(bmr: f64, activity_level: ActivityLevel) -> Self {
        let activity_factor = match activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::LightlyActive => 1.375,
            ActivityLevel::ModeratelyActive => 1.55,
            ActivityLevel::VeryActive => 1.725,
            ActivityLevel::ExtraActive => 1.9,
        };

        let tdee = bmr * activity_factor;
        let tef = tdee * 0.10;
        let aee = tdee - bmr - tef;
        let neat = aee * 0.3;

        Self {
            basal_metabolic_rate: bmr,
            thermic_effect_food: tef,
            activity_energy_expenditure: aee,
            non_exercise_activity_thermogenesis: neat,
        }
    }

    pub fn total(&self) -> f64 {
        self.basal_metabolic_rate
            + self.thermic_effect_food
            + self.activity_energy_expenditure
            + self.non_exercise_activity_thermogenesis
    }

    pub fn with_exercise(bmr: f64, exercise_kcal: f64) -> Self {
        let tef = (bmr + exercise_kcal) * 0.10;
        let neat = bmr * 0.15;

        Self {
            basal_metabolic_rate: bmr,
            thermic_effect_food: tef,
            activity_energy_expenditure: exercise_kcal,
            non_exercise_activity_thermogenesis: neat,
        }
    }

    pub fn bmr_percentage(&self) -> f64 {
        (self.basal_metabolic_rate / self.total()) * 100.0
    }

    pub fn activity_percentage(&self) -> f64 {
        (self.activity_energy_expenditure / self.total()) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_balance() {
        let balance = EnergyBalance::new(2000.0, 2200.0);
        assert!(balance.is_in_deficit());
        assert!(!balance.is_in_surplus());
        assert_eq!(balance.daily_balance(), -200.0);
    }

    #[test]
    fn test_bmr_calculation() {
        let bmr = BasalMetabolicRate::new(75.0, 175.0, 30.0, BiologicalSex::Male);
        assert!(bmr.bmr_kcal_day > 1500.0);
        assert!(bmr.bmr_kcal_day < 2000.0);
    }

    #[test]
    fn test_tdee() {
        let tdee = TotalEnergyExpenditure::new(1700.0, ActivityLevel::ModeratelyActive);
        assert!(tdee.total() > 1700.0);
        assert!(tdee.bmr_percentage() > 50.0);
    }

    #[test]
    fn test_weight_change_estimation() {
        let deficit = EnergyBalance::new(1500.0, 2000.0);
        let change = deficit.estimated_weight_change_kg_per_week();
        assert!(change < 0.0);
    }
}
