use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscularPerformance {
    pub strength: StrengthMetrics,
    pub endurance: EnduranceMetrics,
    pub power: PowerMetrics,
    pub fatigue: FatigueState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthMetrics {
    pub one_rep_max_kg: f64,
    pub isometric_force_n: f64,
    pub eccentric_force_multiplier: f64,
    pub concentric_force_n: f64,
    pub grip_strength_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnduranceMetrics {
    pub vo2_max_ml_kg_min: f64,
    pub lactate_threshold_percentage: f64,
    pub time_to_exhaustion_min: f64,
    pub recovery_rate_percentage_per_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerMetrics {
    pub peak_power_watts: f64,
    pub sustained_power_watts: f64,
    pub power_to_weight_ratio: f64,
    pub rate_of_force_development_n_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatigueState {
    pub current_fatigue_percentage: f64,
    pub metabolic_fatigue: f64,
    pub neural_fatigue: f64,
    pub peripheral_fatigue: f64,
    pub central_fatigue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusculoskeletalPerformance {
    pub muscle_groups: Vec<MuscleGroupPerformance>,
    pub total_muscle_mass_kg: f64,
    pub lean_body_mass_kg: f64,
    pub body_fat_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleGroupPerformance {
    pub group_name: MuscleGroup,
    pub max_voluntary_contraction_n: f64,
    pub activation_percentage: f64,
    pub fiber_type_ratio: f64,
    pub cross_sectional_area_cm2: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MuscleGroup {
    Quadriceps,
    Hamstrings,
    Glutes,
    Calves,
    ChestPectorals,
    BackLatissimus,
    ShouldersDeltoids,
    BicepsBrachii,
    TricepsBrachii,
    CoreAbdominals,
}

impl MuscularPerformance {
    pub fn new_sedentary_adult(body_weight_kg: f64) -> Self {
        Self {
            strength: StrengthMetrics {
                one_rep_max_kg: body_weight_kg * 0.8,
                isometric_force_n: body_weight_kg * 9.81 * 1.2,
                eccentric_force_multiplier: 1.4,
                concentric_force_n: body_weight_kg * 9.81 * 1.0,
                grip_strength_kg: body_weight_kg * 0.5,
            },
            endurance: EnduranceMetrics {
                vo2_max_ml_kg_min: 35.0,
                lactate_threshold_percentage: 60.0,
                time_to_exhaustion_min: 15.0,
                recovery_rate_percentage_per_min: 5.0,
            },
            power: PowerMetrics {
                peak_power_watts: body_weight_kg * 8.0,
                sustained_power_watts: body_weight_kg * 3.0,
                power_to_weight_ratio: 3.0,
                rate_of_force_development_n_s: 1000.0,
            },
            fatigue: FatigueState::new(),
        }
    }

    pub fn new_trained_athlete(body_weight_kg: f64) -> Self {
        Self {
            strength: StrengthMetrics {
                one_rep_max_kg: body_weight_kg * 1.5,
                isometric_force_n: body_weight_kg * 9.81 * 2.0,
                eccentric_force_multiplier: 1.6,
                concentric_force_n: body_weight_kg * 9.81 * 1.5,
                grip_strength_kg: body_weight_kg * 0.8,
            },
            endurance: EnduranceMetrics {
                vo2_max_ml_kg_min: 60.0,
                lactate_threshold_percentage: 85.0,
                time_to_exhaustion_min: 45.0,
                recovery_rate_percentage_per_min: 10.0,
            },
            power: PowerMetrics {
                peak_power_watts: body_weight_kg * 15.0,
                sustained_power_watts: body_weight_kg * 6.0,
                power_to_weight_ratio: 6.0,
                rate_of_force_development_n_s: 3000.0,
            },
            fatigue: FatigueState::new(),
        }
    }

    pub fn apply_fatigue(&mut self, exercise_intensity_percentage: f64, duration_min: f64) {
        let fatigue_increment =
            (exercise_intensity_percentage / 100.0) * (duration_min / 60.0) * 30.0;
        self.fatigue.current_fatigue_percentage =
            (self.fatigue.current_fatigue_percentage + fatigue_increment).min(100.0);

        self.fatigue.metabolic_fatigue = fatigue_increment * 0.6;
        self.fatigue.neural_fatigue = fatigue_increment * 0.2;
        self.fatigue.peripheral_fatigue = fatigue_increment * 0.15;
        self.fatigue.central_fatigue = fatigue_increment * 0.05;
    }

    pub fn recover(&mut self, rest_duration_min: f64) {
        let recovery_amount = self.endurance.recovery_rate_percentage_per_min * rest_duration_min;
        self.fatigue.current_fatigue_percentage =
            (self.fatigue.current_fatigue_percentage - recovery_amount).max(0.0);
    }

    pub fn current_strength_capacity(&self) -> f64 {
        let fatigue_factor = 1.0 - (self.fatigue.current_fatigue_percentage / 100.0);
        self.strength.one_rep_max_kg * fatigue_factor
    }

    pub fn current_power_output(&self) -> f64 {
        let fatigue_factor = 1.0 - (self.fatigue.current_fatigue_percentage / 100.0);
        self.power.peak_power_watts * fatigue_factor
    }

    pub fn assess_overtraining(&self) -> bool {
        self.fatigue.current_fatigue_percentage > 80.0 && self.fatigue.central_fatigue > 15.0
    }
}

impl FatigueState {
    pub fn new() -> Self {
        Self {
            current_fatigue_percentage: 0.0,
            metabolic_fatigue: 0.0,
            neural_fatigue: 0.0,
            peripheral_fatigue: 0.0,
            central_fatigue: 0.0,
        }
    }

    pub fn is_fresh(&self) -> bool {
        self.current_fatigue_percentage < 20.0
    }

    pub fn is_moderately_fatigued(&self) -> bool {
        self.current_fatigue_percentage >= 20.0 && self.current_fatigue_percentage < 60.0
    }

    pub fn is_severely_fatigued(&self) -> bool {
        self.current_fatigue_percentage >= 60.0
    }
}

impl Default for FatigueState {
    fn default() -> Self {
        Self::new()
    }
}

impl MusculoskeletalPerformance {
    pub fn new_adult(body_weight_kg: f64, _height_cm: f64, biological_sex: &str) -> Self {
        let (muscle_mass_ratio, body_fat) = match biological_sex {
            "male" => (0.45, 15.0),
            "female" => (0.35, 25.0),
            _ => (0.40, 20.0),
        };

        let total_muscle_mass = body_weight_kg * muscle_mass_ratio;
        let lean_body_mass = body_weight_kg * (1.0 - body_fat / 100.0);

        Self {
            muscle_groups: vec![
                MuscleGroupPerformance::new(MuscleGroup::Quadriceps, 180.0),
                MuscleGroupPerformance::new(MuscleGroup::Hamstrings, 120.0),
                MuscleGroupPerformance::new(MuscleGroup::Glutes, 150.0),
                MuscleGroupPerformance::new(MuscleGroup::ChestPectorals, 100.0),
                MuscleGroupPerformance::new(MuscleGroup::BackLatissimus, 130.0),
            ],
            total_muscle_mass_kg: total_muscle_mass,
            lean_body_mass_kg: lean_body_mass,
            body_fat_percentage: body_fat,
        }
    }

    pub fn muscle_quality_index(&self) -> f64 {
        let total_mvc: f64 = self
            .muscle_groups
            .iter()
            .map(|g| g.max_voluntary_contraction_n)
            .sum();

        total_mvc / (self.total_muscle_mass_kg * 1000.0)
    }

    pub fn assess_sarcopenia(&self, age_years: f64) -> bool {
        let age_adjusted_threshold = 35.0 - (age_years - 30.0) * 0.3;
        self.total_muscle_mass_kg < age_adjusted_threshold
    }
}

impl MuscleGroupPerformance {
    pub fn new(group_name: MuscleGroup, mvc_n: f64) -> Self {
        let (fiber_ratio, csa) = match group_name {
            MuscleGroup::Quadriceps => (0.5, 80.0),
            MuscleGroup::Hamstrings => (0.6, 50.0),
            MuscleGroup::Glutes => (0.55, 60.0),
            MuscleGroup::Calves => (0.7, 40.0),
            MuscleGroup::ChestPectorals => (0.45, 55.0),
            MuscleGroup::BackLatissimus => (0.5, 70.0),
            MuscleGroup::ShouldersDeltoids => (0.48, 45.0),
            MuscleGroup::BicepsBrachii => (0.5, 30.0),
            MuscleGroup::TricepsBrachii => (0.48, 35.0),
            MuscleGroup::CoreAbdominals => (0.52, 50.0),
        };

        Self {
            group_name,
            max_voluntary_contraction_n: mvc_n,
            activation_percentage: 90.0,
            fiber_type_ratio: fiber_ratio,
            cross_sectional_area_cm2: csa,
        }
    }

    pub fn specific_tension(&self) -> f64 {
        self.max_voluntary_contraction_n / self.cross_sectional_area_cm2
    }

    pub fn predicted_power(&self, velocity_m_s: f64) -> f64 {
        let optimal_velocity = 0.3;
        let velocity_factor = 1.0
            - ((velocity_m_s - optimal_velocity) / optimal_velocity)
                .abs()
                .min(1.0);

        self.max_voluntary_contraction_n * velocity_m_s * velocity_factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muscular_performance_sedentary() {
        let perf = MuscularPerformance::new_sedentary_adult(75.0);
        assert!(perf.strength.one_rep_max_kg > 50.0);
        assert_eq!(perf.endurance.vo2_max_ml_kg_min, 35.0);
    }

    #[test]
    fn test_muscular_performance_athlete() {
        let perf = MuscularPerformance::new_trained_athlete(75.0);
        assert!(perf.strength.one_rep_max_kg > 100.0);
        assert_eq!(perf.endurance.vo2_max_ml_kg_min, 60.0);
    }

    #[test]
    fn test_fatigue_application() {
        let mut perf = MuscularPerformance::new_sedentary_adult(75.0);
        let initial_capacity = perf.current_strength_capacity();

        perf.apply_fatigue(80.0, 30.0);
        assert!(perf.fatigue.current_fatigue_percentage > 0.0);
        assert!(perf.current_strength_capacity() < initial_capacity);
    }

    #[test]
    fn test_recovery() {
        let mut perf = MuscularPerformance::new_sedentary_adult(75.0);
        perf.apply_fatigue(80.0, 30.0);

        let fatigue_before = perf.fatigue.current_fatigue_percentage;
        perf.recover(20.0);
        assert!(perf.fatigue.current_fatigue_percentage < fatigue_before);
    }

    #[test]
    fn test_overtraining_assessment() {
        let mut perf = MuscularPerformance::new_sedentary_adult(75.0);
        perf.fatigue.current_fatigue_percentage = 85.0;
        perf.fatigue.central_fatigue = 20.0;
        assert!(perf.assess_overtraining());
    }

    #[test]
    fn test_musculoskeletal_performance() {
        let perf = MusculoskeletalPerformance::new_adult(75.0, 175.0, "male");
        assert!(perf.total_muscle_mass_kg > 30.0);
        let quality = perf.muscle_quality_index();
        assert!(quality > 0.0);
    }

    #[test]
    fn test_muscle_group_performance() {
        let quad = MuscleGroupPerformance::new(MuscleGroup::Quadriceps, 180.0);
        let tension = quad.specific_tension();
        assert!(tension > 0.0);

        let power = quad.predicted_power(0.3);
        assert!(power > 0.0);
    }
}
