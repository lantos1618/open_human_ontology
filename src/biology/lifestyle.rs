use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifestyleProfile {
    pub exercise: ExerciseProfile,
    pub nutrition: NutritionProfile,
    pub sleep: SleepProfile,
    pub stress: StressProfile,
    pub substance_use: SubstanceUse,
    pub environmental_exposures: EnvironmentalExposures,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseProfile {
    pub weekly_aerobic_minutes: f64,
    pub weekly_resistance_sessions: u32,
    pub weekly_flexibility_sessions: u32,
    pub average_intensity: ExerciseIntensity,
    pub vo2_max_ml_kg_min: f64,
    pub step_count_daily_average: u32,
    pub sedentary_hours_per_day: f64,
    pub metabolic_equivalent_hours_per_week: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseIntensity {
    Sedentary,
    Light,
    Moderate,
    Vigorous,
    VeryVigorous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionProfile {
    pub daily_caloric_intake: f64,
    pub macronutrients: Macronutrients,
    pub micronutrient_adequacy_score: f64,
    pub fiber_grams_per_day: f64,
    pub water_liters_per_day: f64,
    pub meal_frequency: u32,
    pub eating_window_hours: f64,
    pub diet_quality_index: f64,
    pub processed_food_percentage: f64,
    pub plant_based_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macronutrients {
    pub protein_grams: f64,
    pub carbohydrate_grams: f64,
    pub fat_grams: f64,
    pub saturated_fat_grams: f64,
    pub omega3_grams: f64,
    pub omega6_grams: f64,
    pub sugar_grams: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepProfile {
    pub average_sleep_hours: f64,
    pub sleep_efficiency_percent: f64,
    pub rem_sleep_percent: f64,
    pub deep_sleep_percent: f64,
    pub light_sleep_percent: f64,
    pub sleep_latency_minutes: f64,
    pub wake_after_sleep_onset_minutes: f64,
    pub sleep_quality_score: f64,
    pub chronotype_alignment: f64,
    pub sleep_debt_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressProfile {
    pub perceived_stress_scale: f64,
    pub chronic_stress_score: f64,
    pub acute_stressors_per_week: u32,
    pub stress_resilience_score: f64,
    pub mindfulness_practice_minutes_per_week: f64,
    pub social_support_score: f64,
    pub work_life_balance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceUse {
    pub alcohol: AlcoholUse,
    pub tobacco: TobaccoUse,
    pub caffeine_mg_per_day: f64,
    pub recreational_drug_use: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholUse {
    pub drinks_per_week: f64,
    pub binge_drinking_episodes_per_month: u32,
    pub years_of_use: f64,
    pub abstinent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TobaccoUse {
    pub current_smoker: bool,
    pub cigarettes_per_day: f64,
    pub pack_years: f64,
    pub years_since_quitting: Option<f64>,
    pub secondhand_exposure_hours_per_week: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalExposures {
    pub air_quality_index_average: f64,
    pub occupational_hazards: Vec<String>,
    pub sunlight_hours_per_week: f64,
    pub uv_exposure_index: f64,
    pub noise_exposure_db_average: f64,
    pub blue_light_hours_per_day: f64,
}

impl LifestyleProfile {
    pub fn new_healthy_baseline() -> Self {
        Self {
            exercise: ExerciseProfile {
                weekly_aerobic_minutes: 150.0,
                weekly_resistance_sessions: 2,
                weekly_flexibility_sessions: 2,
                average_intensity: ExerciseIntensity::Moderate,
                vo2_max_ml_kg_min: 40.0,
                step_count_daily_average: 10000,
                sedentary_hours_per_day: 6.0,
                metabolic_equivalent_hours_per_week: 15.0,
            },
            nutrition: NutritionProfile {
                daily_caloric_intake: 2200.0,
                macronutrients: Macronutrients {
                    protein_grams: 110.0,
                    carbohydrate_grams: 250.0,
                    fat_grams: 75.0,
                    saturated_fat_grams: 20.0,
                    omega3_grams: 2.0,
                    omega6_grams: 10.0,
                    sugar_grams: 40.0,
                },
                micronutrient_adequacy_score: 0.85,
                fiber_grams_per_day: 30.0,
                water_liters_per_day: 2.5,
                meal_frequency: 3,
                eating_window_hours: 12.0,
                diet_quality_index: 0.8,
                processed_food_percentage: 20.0,
                plant_based_percentage: 60.0,
            },
            sleep: SleepProfile {
                average_sleep_hours: 7.5,
                sleep_efficiency_percent: 85.0,
                rem_sleep_percent: 22.0,
                deep_sleep_percent: 18.0,
                light_sleep_percent: 60.0,
                sleep_latency_minutes: 15.0,
                wake_after_sleep_onset_minutes: 20.0,
                sleep_quality_score: 0.85,
                chronotype_alignment: 0.8,
                sleep_debt_hours: 0.0,
            },
            stress: StressProfile {
                perceived_stress_scale: 4.0,
                chronic_stress_score: 0.3,
                acute_stressors_per_week: 2,
                stress_resilience_score: 0.75,
                mindfulness_practice_minutes_per_week: 60.0,
                social_support_score: 0.8,
                work_life_balance_score: 0.75,
            },
            substance_use: SubstanceUse {
                alcohol: AlcoholUse {
                    drinks_per_week: 3.0,
                    binge_drinking_episodes_per_month: 0,
                    years_of_use: 5.0,
                    abstinent: false,
                },
                tobacco: TobaccoUse {
                    current_smoker: false,
                    cigarettes_per_day: 0.0,
                    pack_years: 0.0,
                    years_since_quitting: Some(5.0),
                    secondhand_exposure_hours_per_week: 0.0,
                },
                caffeine_mg_per_day: 200.0,
                recreational_drug_use: vec![],
            },
            environmental_exposures: EnvironmentalExposures {
                air_quality_index_average: 50.0,
                occupational_hazards: vec![],
                sunlight_hours_per_week: 7.0,
                uv_exposure_index: 3.0,
                noise_exposure_db_average: 60.0,
                blue_light_hours_per_day: 4.0,
            },
        }
    }

    pub fn calculate_overall_health_score(&self) -> f64 {
        let exercise_score = self.calculate_exercise_score();
        let nutrition_score = self.calculate_nutrition_score();
        let sleep_score = self.sleep.sleep_quality_score;
        let stress_score = 1.0 - (self.stress.chronic_stress_score);
        let substance_score = self.calculate_substance_use_score();

        (exercise_score * 0.25 +
         nutrition_score * 0.25 +
         sleep_score * 0.2 +
         stress_score * 0.15 +
         substance_score * 0.15).clamp(0.0, 1.0)
    }

    pub fn calculate_exercise_score(&self) -> f64 {
        let mut score = 0.0;

        if self.exercise.weekly_aerobic_minutes >= 150.0 {
            score += 0.4;
        } else {
            score += (self.exercise.weekly_aerobic_minutes / 150.0) * 0.4;
        }

        if self.exercise.weekly_resistance_sessions >= 2 {
            score += 0.3;
        } else {
            score += (self.exercise.weekly_resistance_sessions as f64 / 2.0) * 0.3;
        }

        if self.exercise.sedentary_hours_per_day <= 6.0 {
            score += 0.3;
        } else {
            score += (12.0 - self.exercise.sedentary_hours_per_day) / 6.0 * 0.3;
        }

        score.clamp(0.0, 1.0)
    }

    pub fn calculate_nutrition_score(&self) -> f64 {
        let mut score = self.nutrition.diet_quality_index;

        if self.nutrition.fiber_grams_per_day >= 25.0 {
            score += 0.05;
        }

        if self.nutrition.processed_food_percentage <= 20.0 {
            score += 0.05;
        }

        if self.nutrition.plant_based_percentage >= 50.0 {
            score += 0.05;
        }

        score.clamp(0.0, 1.0)
    }

    pub fn calculate_substance_use_score(&self) -> f64 {
        let mut score = 1.0;

        if self.substance_use.tobacco.current_smoker {
            score -= 0.4;
        }

        if self.substance_use.alcohol.drinks_per_week > 14.0 {
            score -= 0.2;
        } else if self.substance_use.alcohol.drinks_per_week > 7.0 {
            score -= 0.1;
        }

        if self.substance_use.alcohol.binge_drinking_episodes_per_month > 0 {
            score -= 0.15;
        }

        if self.substance_use.caffeine_mg_per_day > 400.0 {
            score -= 0.05;
        }

        score -= self.substance_use.recreational_drug_use.len() as f64 * 0.1;

        score.clamp(0.0, 1.0)
    }

    pub fn assess_cardiovascular_health_impact(&self) -> f64 {
        let exercise_benefit = (self.exercise.vo2_max_ml_kg_min / 50.0).min(1.0) * 0.4;

        let nutrition_benefit = if self.nutrition.macronutrients.omega3_grams >= 2.0 {
            0.2
        } else {
            (self.nutrition.macronutrients.omega3_grams / 2.0) * 0.2
        };

        let smoking_penalty = if self.substance_use.tobacco.current_smoker {
            -0.3
        } else {
            0.0
        };

        let alcohol_impact = if self.substance_use.alcohol.drinks_per_week <= 7.0 {
            0.05
        } else {
            -(self.substance_use.alcohol.drinks_per_week - 7.0) * 0.02
        };

        (exercise_benefit + nutrition_benefit + smoking_penalty + alcohol_impact).clamp(-1.0, 1.0)
    }

    pub fn assess_metabolic_health_impact(&self) -> f64 {
        let exercise_benefit = (self.exercise.metabolic_equivalent_hours_per_week / 20.0).min(1.0) * 0.3;

        let nutrition_benefit = (1.0 - self.nutrition.processed_food_percentage / 100.0) * 0.3;

        let sleep_benefit = if self.sleep.average_sleep_hours >= 7.0 && self.sleep.average_sleep_hours <= 9.0 {
            0.2
        } else {
            0.1
        };

        let sedentary_penalty = if self.exercise.sedentary_hours_per_day > 8.0 {
            -((self.exercise.sedentary_hours_per_day - 8.0) * 0.05)
        } else {
            0.0
        };

        (exercise_benefit + nutrition_benefit + sleep_benefit + sedentary_penalty).clamp(-1.0, 1.0)
    }

    pub fn assess_cognitive_health_impact(&self) -> f64 {
        let sleep_benefit = self.sleep.sleep_quality_score * 0.3;

        let exercise_benefit = if self.exercise.weekly_aerobic_minutes >= 150.0 {
            0.25
        } else {
            (self.exercise.weekly_aerobic_minutes / 150.0) * 0.25
        };

        let stress_impact = (1.0 - self.stress.chronic_stress_score) * 0.2;

        let nutrition_benefit = if self.nutrition.macronutrients.omega3_grams >= 2.0 {
            0.15
        } else {
            (self.nutrition.macronutrients.omega3_grams / 2.0) * 0.15
        };

        let social_benefit = self.stress.social_support_score * 0.1;

        (sleep_benefit + exercise_benefit + stress_impact + nutrition_benefit + social_benefit).clamp(0.0, 1.0)
    }

    pub fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.exercise.weekly_aerobic_minutes < 150.0 {
            recommendations.push(format!(
                "Increase aerobic exercise to at least 150 minutes per week (currently: {:.0} min)",
                self.exercise.weekly_aerobic_minutes
            ));
        }

        if self.exercise.weekly_resistance_sessions < 2 {
            recommendations.push(
                "Add at least 2 resistance training sessions per week".to_string()
            );
        }

        if self.sleep.average_sleep_hours < 7.0 {
            recommendations.push(format!(
                "Increase sleep duration to 7-9 hours (currently: {:.1} hours)",
                self.sleep.average_sleep_hours
            ));
        }

        if self.sleep.sleep_efficiency_percent < 85.0 {
            recommendations.push(
                "Improve sleep hygiene to increase sleep efficiency".to_string()
            );
        }

        if self.nutrition.fiber_grams_per_day < 25.0 {
            recommendations.push(format!(
                "Increase fiber intake to at least 25g per day (currently: {:.0}g)",
                self.nutrition.fiber_grams_per_day
            ));
        }

        if self.nutrition.processed_food_percentage > 30.0 {
            recommendations.push(
                "Reduce processed food consumption to less than 30% of diet".to_string()
            );
        }

        if self.substance_use.tobacco.current_smoker {
            recommendations.push(
                "Quit smoking - the single most important health intervention".to_string()
            );
        }

        if self.substance_use.alcohol.drinks_per_week > 14.0 {
            recommendations.push(
                "Reduce alcohol consumption to recommended limits (≤14 drinks/week)".to_string()
            );
        }

        if self.stress.chronic_stress_score > 0.5 {
            recommendations.push(
                "Implement stress reduction techniques (meditation, therapy, etc.)".to_string()
            );
        }

        if self.exercise.sedentary_hours_per_day > 8.0 {
            recommendations.push(
                "Reduce sedentary time with regular movement breaks".to_string()
            );
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_baseline() {
        let profile = LifestyleProfile::new_healthy_baseline();
        assert_eq!(profile.exercise.weekly_aerobic_minutes, 150.0);
        assert!(!profile.substance_use.tobacco.current_smoker);
    }

    #[test]
    fn test_overall_health_score() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let score = profile.calculate_overall_health_score();
        assert!(score > 0.7);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_exercise_score() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let score = profile.calculate_exercise_score();
        assert!(score > 0.7);
    }

    #[test]
    fn test_nutrition_score() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let score = profile.calculate_nutrition_score();
        assert!(score > 0.7);
    }

    #[test]
    fn test_substance_use_score() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let score = profile.calculate_substance_use_score();
        assert!(score > 0.8);
    }

    #[test]
    fn test_cardiovascular_impact() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let impact = profile.assess_cardiovascular_health_impact();
        assert!(impact > 0.0);
    }

    #[test]
    fn test_metabolic_impact() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let impact = profile.assess_metabolic_health_impact();
        assert!(impact > 0.0);
    }

    #[test]
    fn test_cognitive_impact() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let impact = profile.assess_cognitive_health_impact();
        assert!(impact > 0.5);
    }

    #[test]
    fn test_recommendations_healthy() {
        let profile = LifestyleProfile::new_healthy_baseline();
        let recs = profile.generate_recommendations();
        assert!(recs.len() < 5);
    }

    #[test]
    fn test_smoking_impact() {
        let mut profile = LifestyleProfile::new_healthy_baseline();
        profile.substance_use.tobacco.current_smoker = true;

        let score = profile.calculate_substance_use_score();
        assert!(score < 0.7);

        let recs = profile.generate_recommendations();
        assert!(recs.iter().any(|r| r.contains("smoking")));
    }
}
