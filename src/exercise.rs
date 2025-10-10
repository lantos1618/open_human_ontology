use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExercisePhysiology {
    pub cardiovascular_response: CardiovascularResponse,
    pub respiratory_response: RespiratoryResponse,
    pub metabolic_response: MetabolicResponse,
    pub muscular_response: MuscularResponse,
    pub thermoregulatory_response: ThermoregulatoryResponse,
    pub fitness_level: FitnessLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularResponse {
    pub resting_hr_bpm: f64,
    pub max_hr_bpm: f64,
    pub current_hr_bpm: f64,
    pub resting_cardiac_output_l_min: f64,
    pub max_cardiac_output_l_min: f64,
    pub stroke_volume_ml: f64,
    pub blood_pressure_systolic_mmhg: f64,
    pub blood_pressure_diastolic_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryResponse {
    pub resting_rate_bpm: f64,
    pub exercise_rate_bpm: f64,
    pub tidal_volume_ml: f64,
    pub minute_ventilation_l_min: f64,
    pub max_voluntary_ventilation_l_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicResponse {
    pub vo2_ml_kg_min: f64,
    pub vo2_max_ml_kg_min: f64,
    pub vco2_ml_kg_min: f64,
    pub respiratory_exchange_ratio: f64,
    pub energy_expenditure_kcal_min: f64,
    pub lactate_mmol_l: f64,
    pub substrate_utilization: SubstrateUtilization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateUtilization {
    pub carbohydrate_percent: f64,
    pub fat_percent: f64,
    pub protein_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscularResponse {
    pub force_production_n: f64,
    pub power_output_watts: f64,
    pub muscle_glycogen_mmol_kg: f64,
    pub muscle_damage_markers: MuscleDamageMarkers,
    pub fatigue_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleDamageMarkers {
    pub creatine_kinase_u_l: f64,
    pub myoglobin_ng_ml: f64,
    pub lactate_dehydrogenase_u_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermoregulatoryResponse {
    pub core_temp_c: f64,
    pub skin_temp_c: f64,
    pub sweat_rate_l_hr: f64,
    pub heat_production_w: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FitnessLevel {
    Poor,
    BelowAverage,
    Average,
    AboveAverage,
    Excellent,
    Elite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseIntensity {
    Rest,
    VeryLight,
    Light,
    Moderate,
    Vigorous,
    VeryVigorous,
    Maximal,
}

impl ExercisePhysiology {
    pub fn new_sedentary(age_years: f64) -> Self {
        let max_hr = 220.0 - age_years;
        let resting_hr = 75.0;

        Self {
            cardiovascular_response: CardiovascularResponse {
                resting_hr_bpm: resting_hr,
                max_hr_bpm: max_hr,
                current_hr_bpm: resting_hr,
                resting_cardiac_output_l_min: 5.0,
                max_cardiac_output_l_min: 15.0,
                stroke_volume_ml: 70.0,
                blood_pressure_systolic_mmhg: 120.0,
                blood_pressure_diastolic_mmhg: 80.0,
            },
            respiratory_response: RespiratoryResponse {
                resting_rate_bpm: 12.0,
                exercise_rate_bpm: 12.0,
                tidal_volume_ml: 500.0,
                minute_ventilation_l_min: 6.0,
                max_voluntary_ventilation_l_min: 120.0,
            },
            metabolic_response: MetabolicResponse {
                vo2_ml_kg_min: 3.5,
                vo2_max_ml_kg_min: 30.0,
                vco2_ml_kg_min: 2.8,
                respiratory_exchange_ratio: 0.8,
                energy_expenditure_kcal_min: 1.2,
                lactate_mmol_l: 1.0,
                substrate_utilization: SubstrateUtilization {
                    carbohydrate_percent: 40.0,
                    fat_percent: 60.0,
                    protein_percent: 0.0,
                },
            },
            muscular_response: MuscularResponse {
                force_production_n: 0.0,
                power_output_watts: 0.0,
                muscle_glycogen_mmol_kg: 300.0,
                muscle_damage_markers: MuscleDamageMarkers {
                    creatine_kinase_u_l: 100.0,
                    myoglobin_ng_ml: 30.0,
                    lactate_dehydrogenase_u_l: 140.0,
                },
                fatigue_level: 0.0,
            },
            thermoregulatory_response: ThermoregulatoryResponse {
                core_temp_c: 37.0,
                skin_temp_c: 33.0,
                sweat_rate_l_hr: 0.0,
                heat_production_w: 80.0,
            },
            fitness_level: FitnessLevel::Poor,
        }
    }

    pub fn new_trained(age_years: f64) -> Self {
        let max_hr = 220.0 - age_years;
        let resting_hr = 55.0;

        Self {
            cardiovascular_response: CardiovascularResponse {
                resting_hr_bpm: resting_hr,
                max_hr_bpm: max_hr,
                current_hr_bpm: resting_hr,
                resting_cardiac_output_l_min: 5.0,
                max_cardiac_output_l_min: 25.0,
                stroke_volume_ml: 90.0,
                blood_pressure_systolic_mmhg: 115.0,
                blood_pressure_diastolic_mmhg: 75.0,
            },
            respiratory_response: RespiratoryResponse {
                resting_rate_bpm: 12.0,
                exercise_rate_bpm: 12.0,
                tidal_volume_ml: 500.0,
                minute_ventilation_l_min: 6.0,
                max_voluntary_ventilation_l_min: 180.0,
            },
            metabolic_response: MetabolicResponse {
                vo2_ml_kg_min: 3.5,
                vo2_max_ml_kg_min: 55.0,
                vco2_ml_kg_min: 2.8,
                respiratory_exchange_ratio: 0.8,
                energy_expenditure_kcal_min: 1.2,
                lactate_mmol_l: 0.8,
                substrate_utilization: SubstrateUtilization {
                    carbohydrate_percent: 30.0,
                    fat_percent: 70.0,
                    protein_percent: 0.0,
                },
            },
            muscular_response: MuscularResponse {
                force_production_n: 0.0,
                power_output_watts: 0.0,
                muscle_glycogen_mmol_kg: 400.0,
                muscle_damage_markers: MuscleDamageMarkers {
                    creatine_kinase_u_l: 100.0,
                    myoglobin_ng_ml: 30.0,
                    lactate_dehydrogenase_u_l: 140.0,
                },
                fatigue_level: 0.0,
            },
            thermoregulatory_response: ThermoregulatoryResponse {
                core_temp_c: 37.0,
                skin_temp_c: 33.0,
                sweat_rate_l_hr: 0.0,
                heat_production_w: 75.0,
            },
            fitness_level: FitnessLevel::Excellent,
        }
    }

    pub fn simulate_exercise(&mut self, intensity: ExerciseIntensity, duration_min: f64) {
        let hr_reserve =
            self.cardiovascular_response.max_hr_bpm - self.cardiovascular_response.resting_hr_bpm;

        let (hr_percent, vo2_percent) = match intensity {
            ExerciseIntensity::Rest => (0.0, 0.0),
            ExerciseIntensity::VeryLight => (0.3, 0.25),
            ExerciseIntensity::Light => (0.4, 0.35),
            ExerciseIntensity::Moderate => (0.5, 0.50),
            ExerciseIntensity::Vigorous => (0.7, 0.70),
            ExerciseIntensity::VeryVigorous => (0.85, 0.85),
            ExerciseIntensity::Maximal => (1.0, 1.0),
        };

        self.cardiovascular_response.current_hr_bpm =
            self.cardiovascular_response.resting_hr_bpm + hr_reserve * hr_percent;

        self.metabolic_response.vo2_ml_kg_min =
            3.5 + (self.metabolic_response.vo2_max_ml_kg_min - 3.5) * vo2_percent;

        self.metabolic_response.lactate_mmol_l = if vo2_percent < 0.6 {
            1.0 + vo2_percent
        } else {
            1.0 + (vo2_percent - 0.6) * 15.0
        };

        self.respiratory_response.exercise_rate_bpm = 12.0 + 30.0 * vo2_percent;

        self.thermoregulatory_response.core_temp_c = 37.0 + vo2_percent * 2.0;
        self.thermoregulatory_response.sweat_rate_l_hr = vo2_percent * 1.5;

        let glycogen_usage = vo2_percent * duration_min * 2.0;
        self.muscular_response.muscle_glycogen_mmol_kg -= glycogen_usage;

        self.muscular_response.fatigue_level = (vo2_percent * duration_min / 60.0).min(1.0);

        if intensity as usize >= ExerciseIntensity::Vigorous as usize {
            self.muscular_response
                .muscle_damage_markers
                .creatine_kinase_u_l += vo2_percent * duration_min * 10.0;
        }
    }

    pub fn classify_fitness_level(&self) -> FitnessLevel {
        let vo2max = self.metabolic_response.vo2_max_ml_kg_min;

        if vo2max < 30.0 {
            FitnessLevel::Poor
        } else if vo2max < 35.0 {
            FitnessLevel::BelowAverage
        } else if vo2max < 42.0 {
            FitnessLevel::Average
        } else if vo2max < 50.0 {
            FitnessLevel::AboveAverage
        } else if vo2max < 60.0 {
            FitnessLevel::Excellent
        } else {
            FitnessLevel::Elite
        }
    }

    pub fn heart_rate_zones(&self) -> HeartRateZones {
        let max = self.cardiovascular_response.max_hr_bpm;
        let rest = self.cardiovascular_response.resting_hr_bpm;
        let reserve = max - rest;

        HeartRateZones {
            zone1_recovery: (rest, rest + reserve * 0.6),
            zone2_aerobic: (rest + reserve * 0.6, rest + reserve * 0.7),
            zone3_tempo: (rest + reserve * 0.7, rest + reserve * 0.8),
            zone4_threshold: (rest + reserve * 0.8, rest + reserve * 0.9),
            zone5_vo2max: (rest + reserve * 0.9, max),
        }
    }

    pub fn current_intensity_zone(&self) -> usize {
        let zones = self.heart_rate_zones();
        let hr = self.cardiovascular_response.current_hr_bpm;

        if hr < zones.zone1_recovery.1 {
            1
        } else if hr < zones.zone2_aerobic.1 {
            2
        } else if hr < zones.zone3_tempo.1 {
            3
        } else if hr < zones.zone4_threshold.1 {
            4
        } else {
            5
        }
    }

    pub fn estimated_time_to_exhaustion_min(&self) -> f64 {
        let vo2_percent = (self.metabolic_response.vo2_ml_kg_min - 3.5)
            / (self.metabolic_response.vo2_max_ml_kg_min - 3.5);

        if vo2_percent < 0.5 {
            f64::INFINITY
        } else if vo2_percent < 0.7 {
            180.0
        } else if vo2_percent < 0.85 {
            60.0
        } else {
            20.0 * (1.0 - vo2_percent)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateZones {
    pub zone1_recovery: (f64, f64),
    pub zone2_aerobic: (f64, f64),
    pub zone3_tempo: (f64, f64),
    pub zone4_threshold: (f64, f64),
    pub zone5_vo2max: (f64, f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sedentary_physiology() {
        let phys = ExercisePhysiology::new_sedentary(30.0);
        assert_eq!(phys.fitness_level, FitnessLevel::Poor);
        assert!(phys.metabolic_response.vo2_max_ml_kg_min < 35.0);
    }

    #[test]
    fn test_trained_physiology() {
        let phys = ExercisePhysiology::new_trained(30.0);
        assert_eq!(phys.fitness_level, FitnessLevel::Excellent);
        assert!(phys.metabolic_response.vo2_max_ml_kg_min > 50.0);
        assert!(phys.cardiovascular_response.resting_hr_bpm < 60.0);
    }

    #[test]
    fn test_exercise_simulation() {
        let mut phys = ExercisePhysiology::new_sedentary(25.0);
        let initial_hr = phys.cardiovascular_response.current_hr_bpm;

        phys.simulate_exercise(ExerciseIntensity::Moderate, 30.0);

        assert!(phys.cardiovascular_response.current_hr_bpm > initial_hr);
        assert!(phys.metabolic_response.vo2_ml_kg_min > 3.5);
        assert!(phys.thermoregulatory_response.core_temp_c > 37.0);
    }

    #[test]
    fn test_fitness_classification() {
        let mut phys = ExercisePhysiology::new_sedentary(30.0);
        phys.metabolic_response.vo2_max_ml_kg_min = 55.0;
        let level = phys.classify_fitness_level();
        assert_eq!(level, FitnessLevel::Excellent);
    }

    #[test]
    fn test_heart_rate_zones() {
        let phys = ExercisePhysiology::new_sedentary(30.0);
        let zones = phys.heart_rate_zones();

        assert!(zones.zone1_recovery.0 < zones.zone1_recovery.1);
        assert!(zones.zone1_recovery.1 <= zones.zone2_aerobic.0);
        assert!(zones.zone5_vo2max.1 == phys.cardiovascular_response.max_hr_bpm);
    }

    #[test]
    fn test_lactate_accumulation() {
        let mut phys = ExercisePhysiology::new_sedentary(25.0);
        let initial_lactate = phys.metabolic_response.lactate_mmol_l;

        phys.simulate_exercise(ExerciseIntensity::VeryVigorous, 10.0);

        assert!(phys.metabolic_response.lactate_mmol_l > initial_lactate);
        assert!(phys.metabolic_response.lactate_mmol_l > 4.0);
    }

    #[test]
    fn test_glycogen_depletion() {
        let mut phys = ExercisePhysiology::new_sedentary(25.0);
        let initial_glycogen = phys.muscular_response.muscle_glycogen_mmol_kg;

        phys.simulate_exercise(ExerciseIntensity::Vigorous, 60.0);

        assert!(phys.muscular_response.muscle_glycogen_mmol_kg < initial_glycogen);
    }

    #[test]
    fn test_muscle_damage_markers() {
        let mut phys = ExercisePhysiology::new_sedentary(25.0);
        let initial_ck = phys
            .muscular_response
            .muscle_damage_markers
            .creatine_kinase_u_l;

        phys.simulate_exercise(ExerciseIntensity::VeryVigorous, 45.0);

        assert!(
            phys.muscular_response
                .muscle_damage_markers
                .creatine_kinase_u_l
                > initial_ck
        );
    }
}
