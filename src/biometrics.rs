use serde::{Deserialize, Serialize};
use crate::human::BiologicalSex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricProfile {
    pub anthropometric: AnthropometricMeasures,
    pub body_composition: BodyComposition,
    pub vital_signs: VitalSigns,
    pub fitness_metrics: FitnessMetrics,
    pub age_years: f64,
    pub biological_age: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropometricMeasures {
    pub height_cm: f64,
    pub weight_kg: f64,
    pub bmi: f64,
    pub waist_circumference_cm: f64,
    pub hip_circumference_cm: f64,
    pub waist_hip_ratio: f64,
    pub neck_circumference_cm: f64,
    pub head_circumference_cm: f64,
    pub arm_span_cm: f64,
    pub sitting_height_cm: f64,
    pub leg_length_cm: f64,
}

impl AnthropometricMeasures {
    pub fn new(height_cm: f64, weight_kg: f64) -> Self {
        let bmi = weight_kg / (height_cm / 100.0).powi(2);

        Self {
            height_cm,
            weight_kg,
            bmi,
            waist_circumference_cm: 0.0,
            hip_circumference_cm: 0.0,
            waist_hip_ratio: 0.0,
            neck_circumference_cm: 0.0,
            head_circumference_cm: 0.0,
            arm_span_cm: height_cm,
            sitting_height_cm: height_cm * 0.52,
            leg_length_cm: height_cm * 0.48,
        }
    }

    pub fn calculate_bmi(&mut self) {
        self.bmi = self.weight_kg / (self.height_cm / 100.0).powi(2);
    }

    pub fn calculate_waist_hip_ratio(&mut self) {
        if self.hip_circumference_cm > 0.0 {
            self.waist_hip_ratio = self.waist_circumference_cm / self.hip_circumference_cm;
        }
    }

    pub fn bmi_category(&self) -> BMICategory {
        match self.bmi {
            bmi if bmi < 16.0 => BMICategory::SeverelyUnderweight,
            bmi if bmi < 18.5 => BMICategory::Underweight,
            bmi if bmi < 25.0 => BMICategory::Normal,
            bmi if bmi < 30.0 => BMICategory::Overweight,
            bmi if bmi < 35.0 => BMICategory::ObesityClass1,
            bmi if bmi < 40.0 => BMICategory::ObesityClass2,
            _ => BMICategory::ObesityClass3,
        }
    }

    pub fn metabolic_risk_from_waist(&self, sex: BiologicalSex) -> MetabolicRisk {
        match sex {
            BiologicalSex::Male => {
                if self.waist_circumference_cm > 102.0 {
                    MetabolicRisk::High
                } else if self.waist_circumference_cm > 94.0 {
                    MetabolicRisk::Increased
                } else {
                    MetabolicRisk::Normal
                }
            }
            BiologicalSex::Female => {
                if self.waist_circumference_cm > 88.0 {
                    MetabolicRisk::High
                } else if self.waist_circumference_cm > 80.0 {
                    MetabolicRisk::Increased
                } else {
                    MetabolicRisk::Normal
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BMICategory {
    SeverelyUnderweight,
    Underweight,
    Normal,
    Overweight,
    ObesityClass1,
    ObesityClass2,
    ObesityClass3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicRisk {
    Normal,
    Increased,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyComposition {
    pub body_fat_percentage: f64,
    pub lean_body_mass_kg: f64,
    pub fat_mass_kg: f64,
    pub skeletal_muscle_mass_kg: f64,
    pub bone_mineral_density_g_cm2: f64,
    pub total_body_water_l: f64,
    pub intracellular_water_l: f64,
    pub extracellular_water_l: f64,
    pub visceral_fat_rating: f64,
}

impl BodyComposition {
    pub fn estimate_from_anthropometrics(
        height_cm: f64,
        weight_kg: f64,
        age: f64,
        sex: BiologicalSex,
    ) -> Self {
        let bmi = weight_kg / (height_cm / 100.0).powi(2);

        let body_fat_percentage = match sex {
            BiologicalSex::Male => {
                1.20 * bmi + 0.23 * age - 16.2
            }
            BiologicalSex::Female => {
                1.20 * bmi + 0.23 * age - 5.4
            }
        };

        let body_fat_percentage = body_fat_percentage.max(3.0).min(60.0);

        let fat_mass_kg = weight_kg * (body_fat_percentage / 100.0);
        let lean_body_mass_kg = weight_kg - fat_mass_kg;
        let skeletal_muscle_mass_kg = lean_body_mass_kg * 0.45;

        let total_body_water_l = match sex {
            BiologicalSex::Male => weight_kg * 0.6,
            BiologicalSex::Female => weight_kg * 0.55,
        };

        let intracellular_water_l = total_body_water_l * 0.65;
        let extracellular_water_l = total_body_water_l * 0.35;

        let visceral_fat_rating = (body_fat_percentage - 10.0) / 5.0;

        Self {
            body_fat_percentage,
            lean_body_mass_kg,
            fat_mass_kg,
            skeletal_muscle_mass_kg,
            bone_mineral_density_g_cm2: 1.2,
            total_body_water_l,
            intracellular_water_l,
            extracellular_water_l,
            visceral_fat_rating: visceral_fat_rating.max(1.0),
        }
    }

    pub fn body_fat_category(&self, sex: BiologicalSex, age: f64) -> BodyFatCategory {
        let percentage = self.body_fat_percentage;

        match sex {
            BiologicalSex::Male => {
                if age < 40.0 {
                    match percentage {
                        p if p < 8.0 => BodyFatCategory::Essential,
                        p if p < 14.0 => BodyFatCategory::Athletic,
                        p if p < 18.0 => BodyFatCategory::Fitness,
                        p if p < 25.0 => BodyFatCategory::Average,
                        _ => BodyFatCategory::Obese,
                    }
                } else {
                    match percentage {
                        p if p < 8.0 => BodyFatCategory::Essential,
                        p if p < 17.0 => BodyFatCategory::Athletic,
                        p if p < 21.0 => BodyFatCategory::Fitness,
                        p if p < 28.0 => BodyFatCategory::Average,
                        _ => BodyFatCategory::Obese,
                    }
                }
            }
            BiologicalSex::Female => {
                if age < 40.0 {
                    match percentage {
                        p if p < 14.0 => BodyFatCategory::Essential,
                        p if p < 21.0 => BodyFatCategory::Athletic,
                        p if p < 25.0 => BodyFatCategory::Fitness,
                        p if p < 32.0 => BodyFatCategory::Average,
                        _ => BodyFatCategory::Obese,
                    }
                } else {
                    match percentage {
                        p if p < 14.0 => BodyFatCategory::Essential,
                        p if p < 24.0 => BodyFatCategory::Athletic,
                        p if p < 28.0 => BodyFatCategory::Fitness,
                        p if p < 35.0 => BodyFatCategory::Average,
                        _ => BodyFatCategory::Obese,
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyFatCategory {
    Essential,
    Athletic,
    Fitness,
    Average,
    Obese,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSigns {
    pub resting_heart_rate_bpm: f64,
    pub systolic_bp_mmhg: f64,
    pub diastolic_bp_mmhg: f64,
    pub respiratory_rate_per_min: f64,
    pub body_temperature_celsius: f64,
    pub oxygen_saturation_percent: f64,
}

impl VitalSigns {
    pub fn new_normal() -> Self {
        Self {
            resting_heart_rate_bpm: 70.0,
            systolic_bp_mmhg: 120.0,
            diastolic_bp_mmhg: 80.0,
            respiratory_rate_per_min: 16.0,
            body_temperature_celsius: 37.0,
            oxygen_saturation_percent: 98.0,
        }
    }

    pub fn blood_pressure_category(&self) -> BloodPressureCategory {
        let sys = self.systolic_bp_mmhg;
        let dia = self.diastolic_bp_mmhg;

        if sys < 120.0 && dia < 80.0 {
            BloodPressureCategory::Normal
        } else if sys < 130.0 && dia < 80.0 {
            BloodPressureCategory::Elevated
        } else if sys < 140.0 || dia < 90.0 {
            BloodPressureCategory::Stage1Hypertension
        } else if sys < 180.0 || dia < 120.0 {
            BloodPressureCategory::Stage2Hypertension
        } else {
            BloodPressureCategory::HypertensiveCrisis
        }
    }

    pub fn heart_rate_category(&self, age: f64, sex: BiologicalSex) -> HeartRateCategory {
        let max_hr = 220.0 - age;
        let resting_percentage = (self.resting_heart_rate_bpm / max_hr) * 100.0;

        match self.resting_heart_rate_bpm {
            hr if hr < 40.0 => HeartRateCategory::Bradycardia,
            hr if hr < 60.0 => HeartRateCategory::Athletic,
            hr if hr < 100.0 => HeartRateCategory::Normal,
            _ => HeartRateCategory::Tachycardia,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BloodPressureCategory {
    Normal,
    Elevated,
    Stage1Hypertension,
    Stage2Hypertension,
    HypertensiveCrisis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeartRateCategory {
    Bradycardia,
    Athletic,
    Normal,
    Tachycardia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessMetrics {
    pub vo2_max_ml_kg_min: f64,
    pub one_rep_max_bench_press_kg: f64,
    pub one_rep_max_squat_kg: f64,
    pub one_rep_max_deadlift_kg: f64,
    pub plank_time_seconds: f64,
    pub pushups_max_reps: u32,
    pub pullups_max_reps: u32,
    pub flexibility_sit_and_reach_cm: f64,
}

impl FitnessMetrics {
    pub fn new_sedentary(age: f64, sex: BiologicalSex) -> Self {
        let vo2_max = match sex {
            BiologicalSex::Male => 35.0 - (age * 0.3),
            BiologicalSex::Female => 28.0 - (age * 0.25),
        };

        Self {
            vo2_max_ml_kg_min: vo2_max.max(20.0),
            one_rep_max_bench_press_kg: 40.0,
            one_rep_max_squat_kg: 60.0,
            one_rep_max_deadlift_kg: 80.0,
            plank_time_seconds: 30.0,
            pushups_max_reps: 10,
            pullups_max_reps: 0,
            flexibility_sit_and_reach_cm: 0.0,
        }
    }

    pub fn vo2_max_category(&self, age: f64, sex: BiologicalSex) -> VO2MaxCategory {
        let vo2 = self.vo2_max_ml_kg_min;

        match sex {
            BiologicalSex::Male => {
                if age < 30.0 {
                    match vo2 {
                        v if v < 35.0 => VO2MaxCategory::VeryPoor,
                        v if v < 42.0 => VO2MaxCategory::Poor,
                        v if v < 47.0 => VO2MaxCategory::Fair,
                        v if v < 52.0 => VO2MaxCategory::Good,
                        v if v < 57.0 => VO2MaxCategory::Excellent,
                        _ => VO2MaxCategory::Superior,
                    }
                } else {
                    match vo2 {
                        v if v < 30.0 => VO2MaxCategory::VeryPoor,
                        v if v < 37.0 => VO2MaxCategory::Poor,
                        v if v < 42.0 => VO2MaxCategory::Fair,
                        v if v < 47.0 => VO2MaxCategory::Good,
                        v if v < 52.0 => VO2MaxCategory::Excellent,
                        _ => VO2MaxCategory::Superior,
                    }
                }
            }
            BiologicalSex::Female => {
                if age < 30.0 {
                    match vo2 {
                        v if v < 28.0 => VO2MaxCategory::VeryPoor,
                        v if v < 35.0 => VO2MaxCategory::Poor,
                        v if v < 40.0 => VO2MaxCategory::Fair,
                        v if v < 45.0 => VO2MaxCategory::Good,
                        v if v < 50.0 => VO2MaxCategory::Excellent,
                        _ => VO2MaxCategory::Superior,
                    }
                } else {
                    match vo2 {
                        v if v < 25.0 => VO2MaxCategory::VeryPoor,
                        v if v < 32.0 => VO2MaxCategory::Poor,
                        v if v < 37.0 => VO2MaxCategory::Fair,
                        v if v < 42.0 => VO2MaxCategory::Good,
                        v if v < 47.0 => VO2MaxCategory::Excellent,
                        _ => VO2MaxCategory::Superior,
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VO2MaxCategory {
    VeryPoor,
    Poor,
    Fair,
    Good,
    Excellent,
    Superior,
}

impl BiometricProfile {
    pub fn new(
        height_cm: f64,
        weight_kg: f64,
        age_years: f64,
        sex: BiologicalSex,
    ) -> Self {
        let anthropometric = AnthropometricMeasures::new(height_cm, weight_kg);
        let body_composition = BodyComposition::estimate_from_anthropometrics(
            height_cm,
            weight_kg,
            age_years,
            sex,
        );
        let vital_signs = VitalSigns::new_normal();
        let fitness_metrics = FitnessMetrics::new_sedentary(age_years, sex);

        Self {
            anthropometric,
            body_composition,
            vital_signs,
            fitness_metrics,
            age_years,
            biological_age: age_years,
        }
    }

    pub fn calculate_biological_age(&mut self) {
        let mut age_modifier = 0.0;

        if self.anthropometric.bmi > 30.0 {
            age_modifier += 5.0;
        } else if self.anthropometric.bmi > 25.0 {
            age_modifier += 2.0;
        }

        if self.vital_signs.systolic_bp_mmhg > 140.0 {
            age_modifier += 3.0;
        }

        if self.vital_signs.resting_heart_rate_bpm > 80.0 {
            age_modifier += 2.0;
        }

        if self.fitness_metrics.vo2_max_ml_kg_min < 30.0 {
            age_modifier += 4.0;
        }

        self.biological_age = self.age_years + age_modifier;
    }

    pub fn health_score(&self) -> f64 {
        let mut score: f64 = 100.0;

        match self.anthropometric.bmi_category() {
            BMICategory::Normal => {},
            BMICategory::Overweight => score -= 10.0,
            BMICategory::ObesityClass1 => score -= 20.0,
            BMICategory::ObesityClass2 => score -= 30.0,
            BMICategory::ObesityClass3 => score -= 40.0,
            _ => score -= 15.0,
        }

        match self.vital_signs.blood_pressure_category() {
            BloodPressureCategory::Normal => {},
            BloodPressureCategory::Elevated => score -= 5.0,
            BloodPressureCategory::Stage1Hypertension => score -= 15.0,
            BloodPressureCategory::Stage2Hypertension => score -= 25.0,
            BloodPressureCategory::HypertensiveCrisis => score -= 40.0,
        }

        if self.fitness_metrics.vo2_max_ml_kg_min > 45.0 {
            score += 10.0;
        }

        score.max(0.0).min(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmi_calculation() {
        let mut anthro = AnthropometricMeasures::new(175.0, 70.0);
        anthro.calculate_bmi();
        assert!((anthro.bmi - 22.86).abs() < 0.1);
    }

    #[test]
    fn test_bmi_category() {
        let anthro = AnthropometricMeasures::new(175.0, 70.0);
        assert_eq!(anthro.bmi_category(), BMICategory::Normal);
    }

    #[test]
    fn test_body_composition() {
        let comp = BodyComposition::estimate_from_anthropometrics(
            175.0,
            70.0,
            30.0,
            BiologicalSex::Male,
        );
        assert!(comp.body_fat_percentage > 0.0);
        assert!(comp.lean_body_mass_kg > 0.0);
    }

    #[test]
    fn test_biometric_profile() {
        let profile = BiometricProfile::new(175.0, 70.0, 30.0, BiologicalSex::Male);
        assert_eq!(profile.age_years, 30.0);
        assert!(profile.health_score() > 0.0);
    }

    #[test]
    fn test_biological_age() {
        let mut profile = BiometricProfile::new(175.0, 70.0, 30.0, BiologicalSex::Male);
        profile.calculate_biological_age();
        assert!(profile.biological_age >= profile.age_years);
    }
}
