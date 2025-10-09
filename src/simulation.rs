use serde::{Deserialize, Serialize};
use crate::human::Human;
use crate::metabolism::ComprehensiveMetabolicNetwork;
use crate::biology::epigenetics::EpigeneticProfile;
use crate::biology::proteomics::ProteomicsProfile;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEngine {
    pub time_step_hours: f64,
    pub current_time_hours: f64,
    pub max_time_hours: f64,
    pub state_history: Vec<SimulationState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub time_hours: f64,
    pub metabolic_state: MetabolicSnapshot,
    pub epigenetic_state: EpigeneticSnapshot,
    pub proteomic_state: ProteomicSnapshot,
    pub physiological_metrics: PhysiologicalMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicSnapshot {
    pub glucose_mg_per_dl: f64,
    pub insulin_uiu_per_ml: f64,
    pub ketones_mmol_per_l: f64,
    pub lactate_mmol_per_l: f64,
    pub fatty_acids_mmol_per_l: f64,
    pub atp_production_rate: f64,
    pub oxygen_consumption_ml_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpigeneticSnapshot {
    pub global_methylation: f64,
    pub histone_acetylation_index: f64,
    pub chromatin_accessibility: f64,
    pub epigenetic_age: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteomicSnapshot {
    pub total_protein_abundance: f64,
    pub phosphorylation_events: usize,
    pub protein_synthesis_rate: f64,
    pub protein_degradation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysiologicalMetrics {
    pub heart_rate_bpm: f64,
    pub blood_pressure_systolic: f64,
    pub blood_pressure_diastolic: f64,
    pub respiratory_rate: f64,
    pub core_temperature_c: f64,
    pub cardiac_output_l_min: f64,
}

impl SimulationEngine {
    pub fn new(time_step_hours: f64, max_time_hours: f64) -> Self {
        Self {
            time_step_hours,
            current_time_hours: 0.0,
            max_time_hours,
            state_history: Vec::new(),
        }
    }

    pub fn run_simulation(&mut self, human: &Human, interventions: Vec<Intervention>) -> SimulationResult {
        let mut current_metabolic = ComprehensiveMetabolicNetwork::new_resting_state();
        let mut current_epigenetic = EpigeneticProfile::new();
        let mut current_proteomic = ProteomicsProfile::new();

        while self.current_time_hours < self.max_time_hours {
            for intervention in &interventions {
                if intervention.should_apply(self.current_time_hours) {
                    self.apply_intervention(
                        intervention,
                        &mut current_metabolic,
                        &mut current_epigenetic,
                        &mut current_proteomic,
                    );
                }
            }

            let state = self.capture_state(
                human,
                &current_metabolic,
                &current_epigenetic,
                &current_proteomic,
            );

            self.state_history.push(state);

            self.update_metabolic_state(&mut current_metabolic, self.time_step_hours);
            self.update_epigenetic_state(&mut current_epigenetic, self.time_step_hours);
            self.update_proteomic_state(&mut current_proteomic, self.time_step_hours);

            self.current_time_hours += self.time_step_hours;
        }

        self.analyze_simulation_results()
    }

    fn capture_state(
        &self,
        human: &Human,
        metabolic: &ComprehensiveMetabolicNetwork,
        epigenetic: &EpigeneticProfile,
        proteomic: &ProteomicsProfile,
    ) -> SimulationState {
        let time_of_day = self.current_time_hours % 24.0;
        let circadian_factor = (time_of_day * std::f64::consts::PI / 12.0).sin();

        SimulationState {
            time_hours: self.current_time_hours,
            metabolic_state: MetabolicSnapshot {
                glucose_mg_per_dl: 90.0 + metabolic.carbohydrate_metabolism.glycolysis_rate / 10.0,
                insulin_uiu_per_ml: 8.0 + metabolic.carbohydrate_metabolism.glycogen_synthesis_rate / 20.0,
                ketones_mmol_per_l: metabolic.lipid_metabolism.ketogenesis_rate / 10.0,
                lactate_mmol_per_l: 1.0 + metabolic.carbohydrate_metabolism.glycolysis_rate / 100.0,
                fatty_acids_mmol_per_l: metabolic.lipid_metabolism.fatty_acid_oxidation_rate / 20.0,
                atp_production_rate: metabolic.energy_coupling.atp_production_rate,
                oxygen_consumption_ml_min: 250.0 + metabolic.energy_coupling.atp_production_rate / 4.0,
            },
            epigenetic_state: EpigeneticSnapshot {
                global_methylation: epigenetic.methylation_patterns.global_methylation_level,
                histone_acetylation_index: epigenetic.histone_modifications.h3k27ac_sites.len() as f64 / 100.0,
                chromatin_accessibility: epigenetic.chromatin_accessibility.open_chromatin_percentage,
                epigenetic_age: epigenetic.age_related_changes.average_epigenetic_age(),
            },
            proteomic_state: ProteomicSnapshot {
                total_protein_abundance: proteomic.protein_expression.len() as f64,
                phosphorylation_events: proteomic.find_phosphorylated_proteins().len(),
                protein_synthesis_rate: 250.0 + circadian_factor * 50.0,
                protein_degradation_rate: 230.0 + circadian_factor * 30.0,
            },
            physiological_metrics: PhysiologicalMetrics {
                heart_rate_bpm: human.systems.cardiovascular.heart.heart_rate_bpm + circadian_factor * 10.0,
                blood_pressure_systolic: 120.0 + circadian_factor * 10.0,
                blood_pressure_diastolic: 80.0 + circadian_factor * 5.0,
                respiratory_rate: human.systems.respiratory.breathing_pattern.rate_bpm + circadian_factor * 2.0,
                core_temperature_c: 37.0 + circadian_factor * 0.3,
                cardiac_output_l_min: human.cardiac_output_l_per_min() + circadian_factor * 0.5,
            },
        }
    }

    fn apply_intervention(
        &self,
        intervention: &Intervention,
        metabolic: &mut ComprehensiveMetabolicNetwork,
        _epigenetic: &mut EpigeneticProfile,
        _proteomic: &mut ProteomicsProfile,
    ) {
        match &intervention.intervention_type {
            InterventionType::Exercise { intensity } => {
                let multiplier = 1.0 + (intensity * 4.0);
                metabolic.carbohydrate_metabolism.glycolysis_rate *= multiplier;
                metabolic.lipid_metabolism.fatty_acid_oxidation_rate *= multiplier * 1.5;
                metabolic.energy_coupling.atp_consumption_rate *= multiplier;
            }
            InterventionType::Fasting { hours } => {
                let factor = (hours / 12.0).min(3.0);
                metabolic.carbohydrate_metabolism.glycolysis_rate *= 0.5;
                metabolic.carbohydrate_metabolism.gluconeogenesis_rate *= (1.0 + factor);
                metabolic.lipid_metabolism.ketogenesis_rate *= 3.0 + factor * 2.0;
            }
            InterventionType::NutrientIntake { nutrient_type, amount } => {
                metabolic.simulate_nutrient_intake(*nutrient_type, *amount);
            }
            InterventionType::Medication { drug_name: _, dose: _ } => {
            }
            InterventionType::Sleep => {
                metabolic.energy_coupling.atp_consumption_rate *= 0.7;
                metabolic.amino_acid_metabolism.protein_synthesis_rate *= 1.3;
            }
        }
    }

    fn update_metabolic_state(&self, metabolic: &mut ComprehensiveMetabolicNetwork, delta_hours: f64) {
        let decay_factor = (-0.1 * delta_hours).exp();

        metabolic.carbohydrate_metabolism.glycolysis_rate *= decay_factor;
        metabolic.lipid_metabolism.fatty_acid_oxidation_rate *= decay_factor;

        metabolic.carbohydrate_metabolism.glycolysis_rate = metabolic.carbohydrate_metabolism.glycolysis_rate.max(50.0);
        metabolic.lipid_metabolism.fatty_acid_oxidation_rate = metabolic.lipid_metabolism.fatty_acid_oxidation_rate.max(30.0);
    }

    fn update_epigenetic_state(&self, _epigenetic: &mut EpigeneticProfile, _delta_hours: f64) {
    }

    fn update_proteomic_state(&self, _proteomic: &mut ProteomicsProfile, _delta_hours: f64) {
    }

    fn analyze_simulation_results(&self) -> SimulationResult {
        if self.state_history.is_empty() {
            return SimulationResult {
                total_duration_hours: 0.0,
                average_glucose: 0.0,
                glucose_variability: 0.0,
                average_heart_rate: 0.0,
                metabolic_stress_score: 0.0,
                epigenetic_drift: 0.0,
                key_events: Vec::new(),
            };
        }

        let glucose_values: Vec<f64> = self.state_history
            .iter()
            .map(|s| s.metabolic_state.glucose_mg_per_dl)
            .collect();

        let average_glucose = glucose_values.iter().sum::<f64>() / glucose_values.len() as f64;

        let glucose_variance = glucose_values
            .iter()
            .map(|g| (g - average_glucose).powi(2))
            .sum::<f64>() / glucose_values.len() as f64;
        let glucose_variability = glucose_variance.sqrt();

        let average_heart_rate = self.state_history
            .iter()
            .map(|s| s.physiological_metrics.heart_rate_bpm)
            .sum::<f64>() / self.state_history.len() as f64;

        let metabolic_stress_score = self.calculate_metabolic_stress();

        let initial_epi_age = self.state_history.first()
            .map(|s| s.epigenetic_state.epigenetic_age)
            .unwrap_or(0.0);
        let final_epi_age = self.state_history.last()
            .map(|s| s.epigenetic_state.epigenetic_age)
            .unwrap_or(0.0);
        let epigenetic_drift = final_epi_age - initial_epi_age;

        let key_events = self.identify_key_events();

        SimulationResult {
            total_duration_hours: self.current_time_hours,
            average_glucose,
            glucose_variability,
            average_heart_rate,
            metabolic_stress_score,
            epigenetic_drift,
            key_events,
        }
    }

    fn calculate_metabolic_stress(&self) -> f64 {
        self.state_history
            .iter()
            .map(|s| {
                let glucose_stress = ((s.metabolic_state.glucose_mg_per_dl - 90.0).abs() / 90.0).min(1.0);
                let ketone_stress = (s.metabolic_state.ketones_mmol_per_l / 5.0).min(1.0);
                let lactate_stress = ((s.metabolic_state.lactate_mmol_per_l - 1.0) / 10.0).min(1.0);

                (glucose_stress + ketone_stress + lactate_stress) / 3.0
            })
            .sum::<f64>() / self.state_history.len() as f64
    }

    fn identify_key_events(&self) -> Vec<KeyEvent> {
        let mut events = Vec::new();

        for (i, state) in self.state_history.iter().enumerate() {
            if state.metabolic_state.glucose_mg_per_dl > 140.0 {
                events.push(KeyEvent {
                    time_hours: state.time_hours,
                    event_type: EventType::Hyperglycemia,
                    severity: ((state.metabolic_state.glucose_mg_per_dl - 140.0) / 100.0).min(1.0),
                });
            }

            if state.metabolic_state.glucose_mg_per_dl < 70.0 {
                events.push(KeyEvent {
                    time_hours: state.time_hours,
                    event_type: EventType::Hypoglycemia,
                    severity: ((70.0 - state.metabolic_state.glucose_mg_per_dl) / 50.0).min(1.0),
                });
            }

            if state.metabolic_state.ketones_mmol_per_l > 3.0 {
                events.push(KeyEvent {
                    time_hours: state.time_hours,
                    event_type: EventType::Ketosis,
                    severity: (state.metabolic_state.ketones_mmol_per_l / 10.0).min(1.0),
                });
            }

            if i > 0 {
                let prev_state = &self.state_history[i - 1];
                let hr_change = (state.physiological_metrics.heart_rate_bpm - prev_state.physiological_metrics.heart_rate_bpm).abs();
                if hr_change > 30.0 {
                    events.push(KeyEvent {
                        time_hours: state.time_hours,
                        event_type: EventType::HeartRateSpike,
                        severity: (hr_change / 100.0).min(1.0),
                    });
                }
            }
        }

        events
    }

    pub fn export_time_series(&self) -> HashMap<String, Vec<(f64, f64)>> {
        let mut series: HashMap<String, Vec<(f64, f64)>> = HashMap::new();

        for state in &self.state_history {
            series.entry("glucose".to_string())
                .or_insert_with(Vec::new)
                .push((state.time_hours, state.metabolic_state.glucose_mg_per_dl));

            series.entry("heart_rate".to_string())
                .or_insert_with(Vec::new)
                .push((state.time_hours, state.physiological_metrics.heart_rate_bpm));

            series.entry("ketones".to_string())
                .or_insert_with(Vec::new)
                .push((state.time_hours, state.metabolic_state.ketones_mmol_per_l));

            series.entry("atp_production".to_string())
                .or_insert_with(Vec::new)
                .push((state.time_hours, state.metabolic_state.atp_production_rate));
        }

        series
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub start_time_hours: f64,
    pub duration_hours: f64,
    pub intervention_type: InterventionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterventionType {
    Exercise { intensity: f64 },
    Fasting { hours: f64 },
    NutrientIntake { nutrient_type: crate::metabolism::comprehensive_pathways::NutrientType, amount: f64 },
    Medication { drug_name: String, dose: f64 },
    Sleep,
}

impl Intervention {
    pub fn should_apply(&self, current_time: f64) -> bool {
        current_time >= self.start_time_hours
            && current_time < (self.start_time_hours + self.duration_hours)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub total_duration_hours: f64,
    pub average_glucose: f64,
    pub glucose_variability: f64,
    pub average_heart_rate: f64,
    pub metabolic_stress_score: f64,
    pub epigenetic_drift: f64,
    pub key_events: Vec<KeyEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    pub time_hours: f64,
    pub event_type: EventType,
    pub severity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Hyperglycemia,
    Hypoglycemia,
    Ketosis,
    HeartRateSpike,
    MetabolicCrisis,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human::Human;

    #[test]
    fn test_simulation_engine_creation() {
        let engine = SimulationEngine::new(1.0, 24.0);
        assert_eq!(engine.time_step_hours, 1.0);
        assert_eq!(engine.max_time_hours, 24.0);
    }

    #[test]
    fn test_intervention_timing() {
        let intervention = Intervention {
            start_time_hours: 5.0,
            duration_hours: 2.0,
            intervention_type: InterventionType::Exercise { intensity: 0.7 },
        };

        assert!(!intervention.should_apply(4.0));
        assert!(intervention.should_apply(5.5));
        assert!(!intervention.should_apply(8.0));
    }

    #[test]
    fn test_run_simulation() {
        let mut engine = SimulationEngine::new(1.0, 24.0);
        let human = Human::new_adult_male("sim_test".to_string(), 30.0, 175.0, 75.0);

        let interventions = vec![
            Intervention {
                start_time_hours: 8.0,
                duration_hours: 1.0,
                intervention_type: InterventionType::Exercise { intensity: 0.6 },
            },
        ];

        let result = engine.run_simulation(&human, interventions);

        assert!(result.total_duration_hours > 0.0);
        assert!(result.average_glucose > 0.0);
        assert!(engine.state_history.len() > 0);
    }

    #[test]
    fn test_export_time_series() {
        let mut engine = SimulationEngine::new(1.0, 12.0);
        let human = Human::new_adult_male("ts_test".to_string(), 30.0, 175.0, 75.0);

        let _result = engine.run_simulation(&human, vec![]);
        let series = engine.export_time_series();

        assert!(series.contains_key("glucose"));
        assert!(series.contains_key("heart_rate"));
        assert!(series["glucose"].len() > 0);
    }

    #[test]
    fn test_fasting_simulation() {
        let mut engine = SimulationEngine::new(1.0, 24.0);
        let human = Human::new_adult_male("fast_test".to_string(), 30.0, 175.0, 75.0);

        let interventions = vec![
            Intervention {
                start_time_hours: 0.0,
                duration_hours: 16.0,
                intervention_type: InterventionType::Fasting { hours: 16.0 },
            },
        ];

        let result = engine.run_simulation(&human, interventions);

        assert!(result.key_events.iter().any(|e| matches!(e.event_type, EventType::Ketosis)));
    }
}
