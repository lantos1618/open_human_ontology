use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PainProcessingSystem {
    pub peripheral_nociception: PeripheralNociception,
    pub spinal_processing: SpinalPainProcessing,
    pub ascending_pathways: AscendingPainPathways,
    pub cortical_processing: CorticalPainProcessing,
    pub descending_modulation: DescendingModulation,
    pub pain_memory: PainMemory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeripheralNociception {
    pub a_delta_fibers: NociceptiveFibers,
    pub c_fibers: NociceptiveFibers,
    pub transient_receptor_potential_channels: TRPChannels,
    pub inflammatory_mediators: InflammatoryMediators,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NociceptiveFibers {
    pub fiber_count: u64,
    pub conduction_velocity_ms: f64,
    pub activation_threshold: f64,
    pub sensitization_level: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TRPChannels {
    pub trpv1: f64,
    pub trpa1: f64,
    pub trpm8: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InflammatoryMediators {
    pub prostaglandins: f64,
    pub bradykinin: f64,
    pub substance_p: f64,
    pub cgrp: f64,
    pub ngf: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinalPainProcessing {
    pub dorsal_horn: DorsalHorn,
    pub gate_control: GateControlMechanism,
    pub wide_dynamic_range_neurons: f64,
    pub nociceptive_specific_neurons: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DorsalHorn {
    pub lamina_i_marginal: NeuralLayer,
    pub lamina_ii_substantia_gelatinosa: NeuralLayer,
    pub lamina_iii_iv: NeuralLayer,
    pub lamina_v: NeuralLayer,
    pub central_sensitization: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuralLayer {
    pub neuron_count: u32,
    pub excitability: f64,
    pub glutamate_release: f64,
    pub gaba_release: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateControlMechanism {
    pub a_beta_fiber_activity: f64,
    pub inhibitory_interneuron_activity: f64,
    pub gate_open_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AscendingPainPathways {
    pub spinothalamic_tract: SpinothalamicTract,
    pub spinoreticular_tract: SpinoreticularTract,
    pub spinomesencephalic_tract: SpinomesencephalicTract,
    pub spinohypothalamic_tract: SpinohypothalamicTract,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinothalamicTract {
    pub lateral_spinothalamic: f64,
    pub medial_spinothalamic: f64,
    pub conduction_efficiency: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinoreticularTract {
    pub activity_level: f64,
    pub arousal_contribution: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinomesencephalicTract {
    pub activity_level: f64,
    pub pag_projection: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinohypothalamicTract {
    pub activity_level: f64,
    pub autonomic_response: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorticalPainProcessing {
    pub primary_somatosensory_cortex: CorticalArea,
    pub secondary_somatosensory_cortex: CorticalArea,
    pub anterior_cingulate_cortex: CorticalArea,
    pub insula: CorticalArea,
    pub prefrontal_cortex: CorticalArea,
    pub pain_matrix_integration: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorticalArea {
    pub activation_level: f64,
    pub neuronal_activity: f64,
    pub neurotransmitter_balance: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescendingModulation {
    pub periaqueductal_gray: PAGSystem,
    pub rostral_ventromedial_medulla: RVMSystem,
    pub locus_coeruleus: LocusCoeruleusSystem,
    pub endogenous_opioids: EndogenousOpioidSystem,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PAGSystem {
    pub activity_level: f64,
    pub output_to_rvm: f64,
    pub opioid_receptor_density: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RVMSystem {
    pub on_cells_activity: f64,
    pub off_cells_activity: f64,
    pub neutral_cells_activity: f64,
    pub net_modulation: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocusCoeruleusSystem {
    pub norepinephrine_release: f64,
    pub descending_inhibition: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndogenousOpioidSystem {
    pub beta_endorphin: f64,
    pub enkephalins: f64,
    pub dynorphins: f64,
    pub mu_receptor_availability: f64,
    pub delta_receptor_availability: f64,
    pub kappa_receptor_availability: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PainMemory {
    pub previous_pain_experiences: Vec<PainExperience>,
    pub pain_catastrophizing_score: f64,
    pub pain_anxiety_score: f64,
    pub learned_pain_responses: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PainExperience {
    pub intensity: f64,
    pub duration_minutes: f64,
    pub location: String,
    pub emotional_impact: f64,
}

impl PainProcessingSystem {
    pub fn new_normal() -> Self {
        Self {
            peripheral_nociception: PeripheralNociception::new_normal(),
            spinal_processing: SpinalPainProcessing::new_normal(),
            ascending_pathways: AscendingPainPathways::new_normal(),
            cortical_processing: CorticalPainProcessing::new_normal(),
            descending_modulation: DescendingModulation::new_normal(),
            pain_memory: PainMemory::new_normal(),
        }
    }

    pub fn calculate_pain_intensity(&self, nociceptive_input: f64) -> f64 {
        let peripheral_signal = nociceptive_input *
            self.peripheral_nociception.a_delta_fibers.sensitization_level;

        let spinal_amplification = peripheral_signal *
            (1.0 + self.spinal_processing.dorsal_horn.central_sensitization);

        let gate_effect = spinal_amplification *
            self.spinal_processing.gate_control.gate_open_percentage;

        let descending_inhibition =
            self.descending_modulation.periaqueductal_gray.activity_level +
            self.descending_modulation.rostral_ventromedial_medulla.net_modulation +
            self.descending_modulation.locus_coeruleus.descending_inhibition;

        let final_intensity = gate_effect * (1.0 - descending_inhibition / 3.0);

        let emotional_modulation = 1.0 +
            (self.pain_memory.pain_catastrophizing_score * 0.3) +
            (self.pain_memory.pain_anxiety_score * 0.2);

        (final_intensity * emotional_modulation).max(0.0).min(10.0)
    }

    pub fn assess_chronic_pain_risk(&self) -> ChronicPainRisk {
        let central_sens = self.spinal_processing.dorsal_horn.central_sensitization;
        let descending_dysfunction = 1.0 -
            (self.descending_modulation.periaqueductal_gray.activity_level +
             self.descending_modulation.rostral_ventromedial_medulla.off_cells_activity) / 2.0;
        let psychological_factors =
            (self.pain_memory.pain_catastrophizing_score +
             self.pain_memory.pain_anxiety_score) / 2.0;

        let risk_score = (central_sens + descending_dysfunction + psychological_factors) / 3.0;

        if risk_score > 0.7 {
            ChronicPainRisk::High
        } else if risk_score > 0.4 {
            ChronicPainRisk::Moderate
        } else {
            ChronicPainRisk::Low
        }
    }

    pub fn calculate_pain_tolerance(&self) -> f64 {
        let opioid_contribution =
            (self.descending_modulation.endogenous_opioids.beta_endorphin +
             self.descending_modulation.endogenous_opioids.enkephalins) / 2.0;

        let descending_contribution =
            self.descending_modulation.periaqueductal_gray.activity_level;

        let psychological_contribution = 1.0 -
            (self.pain_memory.pain_catastrophizing_score * 0.5);

        (opioid_contribution + descending_contribution + psychological_contribution) / 3.0
    }
}

impl PeripheralNociception {
    pub fn new_normal() -> Self {
        Self {
            a_delta_fibers: NociceptiveFibers {
                fiber_count: 500_000,
                conduction_velocity_ms: 15.0,
                activation_threshold: 1.0,
                sensitization_level: 1.0,
            },
            c_fibers: NociceptiveFibers {
                fiber_count: 1_000_000,
                conduction_velocity_ms: 1.0,
                activation_threshold: 1.0,
                sensitization_level: 1.0,
            },
            transient_receptor_potential_channels: TRPChannels {
                trpv1: 1.0,
                trpa1: 1.0,
                trpm8: 1.0,
            },
            inflammatory_mediators: InflammatoryMediators {
                prostaglandins: 0.0,
                bradykinin: 0.0,
                substance_p: 0.0,
                cgrp: 0.0,
                ngf: 0.0,
            },
        }
    }
}

impl SpinalPainProcessing {
    pub fn new_normal() -> Self {
        Self {
            dorsal_horn: DorsalHorn {
                lamina_i_marginal: NeuralLayer {
                    neuron_count: 10000,
                    excitability: 1.0,
                    glutamate_release: 1.0,
                    gaba_release: 1.0,
                },
                lamina_ii_substantia_gelatinosa: NeuralLayer {
                    neuron_count: 50000,
                    excitability: 1.0,
                    glutamate_release: 1.0,
                    gaba_release: 1.0,
                },
                lamina_iii_iv: NeuralLayer {
                    neuron_count: 40000,
                    excitability: 1.0,
                    glutamate_release: 1.0,
                    gaba_release: 1.0,
                },
                lamina_v: NeuralLayer {
                    neuron_count: 30000,
                    excitability: 1.0,
                    glutamate_release: 1.0,
                    gaba_release: 1.0,
                },
                central_sensitization: 0.0,
            },
            gate_control: GateControlMechanism {
                a_beta_fiber_activity: 0.5,
                inhibitory_interneuron_activity: 1.0,
                gate_open_percentage: 0.5,
            },
            wide_dynamic_range_neurons: 1.0,
            nociceptive_specific_neurons: 1.0,
        }
    }
}

impl AscendingPainPathways {
    pub fn new_normal() -> Self {
        Self {
            spinothalamic_tract: SpinothalamicTract {
                lateral_spinothalamic: 1.0,
                medial_spinothalamic: 1.0,
                conduction_efficiency: 1.0,
            },
            spinoreticular_tract: SpinoreticularTract {
                activity_level: 1.0,
                arousal_contribution: 1.0,
            },
            spinomesencephalic_tract: SpinomesencephalicTract {
                activity_level: 1.0,
                pag_projection: 1.0,
            },
            spinohypothalamic_tract: SpinohypothalamicTract {
                activity_level: 1.0,
                autonomic_response: 1.0,
            },
        }
    }
}

impl CorticalPainProcessing {
    pub fn new_normal() -> Self {
        Self {
            primary_somatosensory_cortex: CorticalArea {
                activation_level: 1.0,
                neuronal_activity: 1.0,
                neurotransmitter_balance: 1.0,
            },
            secondary_somatosensory_cortex: CorticalArea {
                activation_level: 1.0,
                neuronal_activity: 1.0,
                neurotransmitter_balance: 1.0,
            },
            anterior_cingulate_cortex: CorticalArea {
                activation_level: 1.0,
                neuronal_activity: 1.0,
                neurotransmitter_balance: 1.0,
            },
            insula: CorticalArea {
                activation_level: 1.0,
                neuronal_activity: 1.0,
                neurotransmitter_balance: 1.0,
            },
            prefrontal_cortex: CorticalArea {
                activation_level: 1.0,
                neuronal_activity: 1.0,
                neurotransmitter_balance: 1.0,
            },
            pain_matrix_integration: 1.0,
        }
    }
}

impl DescendingModulation {
    pub fn new_normal() -> Self {
        Self {
            periaqueductal_gray: PAGSystem {
                activity_level: 0.5,
                output_to_rvm: 0.5,
                opioid_receptor_density: 1.0,
            },
            rostral_ventromedial_medulla: RVMSystem {
                on_cells_activity: 0.3,
                off_cells_activity: 0.5,
                neutral_cells_activity: 0.2,
                net_modulation: 0.2,
            },
            locus_coeruleus: LocusCoeruleusSystem {
                norepinephrine_release: 1.0,
                descending_inhibition: 0.3,
            },
            endogenous_opioids: EndogenousOpioidSystem {
                beta_endorphin: 1.0,
                enkephalins: 1.0,
                dynorphins: 1.0,
                mu_receptor_availability: 1.0,
                delta_receptor_availability: 1.0,
                kappa_receptor_availability: 1.0,
            },
        }
    }
}

impl PainMemory {
    pub fn new_normal() -> Self {
        Self {
            previous_pain_experiences: Vec::new(),
            pain_catastrophizing_score: 0.0,
            pain_anxiety_score: 0.0,
            learned_pain_responses: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChronicPainRisk {
    Low,
    Moderate,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_pain_processing() {
        let system = PainProcessingSystem::new_normal();
        assert_eq!(system.peripheral_nociception.a_delta_fibers.fiber_count, 500_000);
    }

    #[test]
    fn test_pain_intensity_calculation() {
        let system = PainProcessingSystem::new_normal();
        let intensity = system.calculate_pain_intensity(5.0);
        assert!((0.0..=10.0).contains(&intensity));
    }

    #[test]
    fn test_chronic_pain_risk() {
        let system = PainProcessingSystem::new_normal();
        let risk = system.assess_chronic_pain_risk();
        assert_eq!(risk, ChronicPainRisk::Low);
    }

    #[test]
    fn test_pain_tolerance() {
        let system = PainProcessingSystem::new_normal();
        let tolerance = system.calculate_pain_tolerance();
        assert!(tolerance > 0.0);
    }
}
