use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TactileSensorySystem {
    pub mechanoreceptors: Mechanoreceptors,
    pub thermoreceptors: Thermoreceptors,
    pub nociceptors: Nociceptors,
    pub proprioceptors: Proprioceptors,
    pub sensory_pathways: SomatosensoryPathways,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mechanoreceptors {
    pub meissners_corpuscles: MeissnersCorpuscles,
    pub merkels_discs: MerkelsDiscs,
    pub pacinian_corpuscles: PacinianCorpuscles,
    pub ruffini_endings: RuffiniEndings,
    pub hair_follicle_receptors: HairFollicleReceptors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeissnersCorpuscles {
    pub density_per_cm2: f64,
    pub adaptation_rate: AdaptationRate,
    pub receptive_field_size_mm2: f64,
    pub sensitivity_threshold_mg: f64,
    pub frequency_response_hz: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkelsDiscs {
    pub density_per_cm2: f64,
    pub adaptation_rate: AdaptationRate,
    pub receptive_field_size_mm2: f64,
    pub spatial_resolution_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacinianCorpuscles {
    pub density_per_cm2: f64,
    pub adaptation_rate: AdaptationRate,
    pub sensitivity_threshold_um: f64,
    pub frequency_response_hz: (f64, f64),
    pub depth_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuffiniEndings {
    pub density_per_cm2: f64,
    pub adaptation_rate: AdaptationRate,
    pub stretch_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairFollicleReceptors {
    pub density_per_cm2: f64,
    pub adaptation_rate: AdaptationRate,
    pub directional_sensitivity: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdaptationRate {
    RapidlyAdapting,
    SlowlyAdapting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thermoreceptors {
    pub cold_receptors: ColdReceptors,
    pub warm_receptors: WarmReceptors,
    pub temperature_discrimination_threshold_c: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColdReceptors {
    pub density_per_cm2: f64,
    pub activation_range_c: (f64, f64),
    pub peak_sensitivity_c: f64,
    pub trpm8_channels: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmReceptors {
    pub density_per_cm2: f64,
    pub activation_range_c: (f64, f64),
    pub peak_sensitivity_c: f64,
    pub trpv_channels: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nociceptors {
    pub mechanical_nociceptors: MechanicalNociceptors,
    pub thermal_nociceptors: ThermalNociceptors,
    pub polymodal_nociceptors: PolymodalNociceptors,
    pub pain_threshold: PainThreshold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanicalNociceptors {
    pub a_delta_fibers: ADeltaFibers,
    pub c_fibers: CFibers,
    pub threshold_force_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalNociceptors {
    pub heat_threshold_c: f64,
    pub cold_threshold_c: f64,
    pub trpv1_channels: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymodalNociceptors {
    pub density_per_cm2: f64,
    pub activation_threshold: f64,
    pub sensitization_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ADeltaFibers {
    pub diameter_um: f64,
    pub conduction_velocity_m_per_s: f64,
    pub myelination: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CFibers {
    pub diameter_um: f64,
    pub conduction_velocity_m_per_s: f64,
    pub myelination: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainThreshold {
    pub acute_pain_threshold: f64,
    pub chronic_sensitization: f64,
    pub endogenous_modulation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proprioceptors {
    pub muscle_spindles: MuscleSpindles,
    pub golgi_tendon_organs: GolgiTendonOrgans,
    pub joint_receptors: JointReceptors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleSpindles {
    pub intrafusal_fibers: IntrafusalFibers,
    pub sensitivity: f64,
    pub stretch_threshold_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrafusalFibers {
    pub nuclear_bag_fibers: f64,
    pub nuclear_chain_fibers: f64,
    pub gamma_motor_innervation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GolgiTendonOrgans {
    pub density_per_muscle: f64,
    pub tension_threshold_n: f64,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointReceptors {
    pub ruffini_endings: f64,
    pub pacinian_corpuscles: f64,
    pub position_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomatosensoryPathways {
    pub dorsal_column_medial_lemniscus: DorsalColumnMedialLemniscusPathway,
    pub spinothalamic_tract: SpinothalamiTract,
    pub spinocerebellar_tract: SpinocerebellarTract,
    pub cortical_processing: SomatosensoryCortex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DorsalColumnMedialLemniscusPathway {
    pub first_order_neurons: f64,
    pub second_order_neurons: f64,
    pub third_order_neurons: f64,
    pub transmission_speed_m_per_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinothalamiTract {
    pub lateral_spinothalamic: LateralSpinothalamiTract,
    pub anterior_spinothalamic: AnteriorSpinothalamiTract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateralSpinothalamiTract {
    pub pain_transmission: f64,
    pub temperature_transmission: f64,
    pub decussation_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnteriorSpinothalamiTract {
    pub crude_touch_transmission: f64,
    pub pressure_transmission: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinocerebellarTract {
    pub proprioceptive_input: f64,
    pub unconscious_processing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomatosensoryCortex {
    pub primary_s1: PrimarySomatosensoryCortex,
    pub secondary_s2: SecondarySomatosensoryCortex,
    pub homunculus_representation: HomonculusMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimarySomatosensoryCortex {
    pub location: String,
    pub receptive_fields: f64,
    pub topographic_organization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondarySomatosensoryCortex {
    pub bilateral_representation: bool,
    pub integration_function: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomonculusMap {
    pub hand_representation_mm2: f64,
    pub face_representation_mm2: f64,
    pub trunk_representation_mm2: f64,
}

impl TactileSensorySystem {
    pub fn new_healthy() -> Self {
        Self {
            mechanoreceptors: Mechanoreceptors {
                meissners_corpuscles: MeissnersCorpuscles {
                    density_per_cm2: 140.0,
                    adaptation_rate: AdaptationRate::RapidlyAdapting,
                    receptive_field_size_mm2: 13.0,
                    sensitivity_threshold_mg: 2.0,
                    frequency_response_hz: (10.0, 50.0),
                },
                merkels_discs: MerkelsDiscs {
                    density_per_cm2: 70.0,
                    adaptation_rate: AdaptationRate::SlowlyAdapting,
                    receptive_field_size_mm2: 2.0,
                    spatial_resolution_mm: 0.5,
                },
                pacinian_corpuscles: PacinianCorpuscles {
                    density_per_cm2: 20.0,
                    adaptation_rate: AdaptationRate::RapidlyAdapting,
                    sensitivity_threshold_um: 0.01,
                    frequency_response_hz: (100.0, 300.0),
                    depth_mm: 2.0,
                },
                ruffini_endings: RuffiniEndings {
                    density_per_cm2: 10.0,
                    adaptation_rate: AdaptationRate::SlowlyAdapting,
                    stretch_sensitivity: 1.0,
                },
                hair_follicle_receptors: HairFollicleReceptors {
                    density_per_cm2: 50.0,
                    adaptation_rate: AdaptationRate::RapidlyAdapting,
                    directional_sensitivity: true,
                },
            },
            thermoreceptors: Thermoreceptors {
                cold_receptors: ColdReceptors {
                    density_per_cm2: 15.0,
                    activation_range_c: (10.0, 35.0),
                    peak_sensitivity_c: 25.0,
                    trpm8_channels: 1.0,
                },
                warm_receptors: WarmReceptors {
                    density_per_cm2: 3.0,
                    activation_range_c: (30.0, 45.0),
                    peak_sensitivity_c: 40.0,
                    trpv_channels: 1.0,
                },
                temperature_discrimination_threshold_c: 0.02,
            },
            nociceptors: Nociceptors {
                mechanical_nociceptors: MechanicalNociceptors {
                    a_delta_fibers: ADeltaFibers {
                        diameter_um: 2.5,
                        conduction_velocity_m_per_s: 12.0,
                        myelination: true,
                    },
                    c_fibers: CFibers {
                        diameter_um: 1.0,
                        conduction_velocity_m_per_s: 1.0,
                        myelination: false,
                    },
                    threshold_force_g: 50.0,
                },
                thermal_nociceptors: ThermalNociceptors {
                    heat_threshold_c: 45.0,
                    cold_threshold_c: 15.0,
                    trpv1_channels: 1.0,
                },
                polymodal_nociceptors: PolymodalNociceptors {
                    density_per_cm2: 200.0,
                    activation_threshold: 1.0,
                    sensitization_factor: 1.0,
                },
                pain_threshold: PainThreshold {
                    acute_pain_threshold: 1.0,
                    chronic_sensitization: 0.0,
                    endogenous_modulation: 1.0,
                },
            },
            proprioceptors: Proprioceptors {
                muscle_spindles: MuscleSpindles {
                    intrafusal_fibers: IntrafusalFibers {
                        nuclear_bag_fibers: 2.0,
                        nuclear_chain_fibers: 5.0,
                        gamma_motor_innervation: 1.0,
                    },
                    sensitivity: 1.0,
                    stretch_threshold_percent: 1.0,
                },
                golgi_tendon_organs: GolgiTendonOrgans {
                    density_per_muscle: 10.0,
                    tension_threshold_n: 0.1,
                    sensitivity: 1.0,
                },
                joint_receptors: JointReceptors {
                    ruffini_endings: 1.0,
                    pacinian_corpuscles: 1.0,
                    position_sensitivity: 1.0,
                },
            },
            sensory_pathways: SomatosensoryPathways {
                dorsal_column_medial_lemniscus: DorsalColumnMedialLemniscusPathway {
                    first_order_neurons: 1.0,
                    second_order_neurons: 1.0,
                    third_order_neurons: 1.0,
                    transmission_speed_m_per_s: 80.0,
                },
                spinothalamic_tract: SpinothalamiTract {
                    lateral_spinothalamic: LateralSpinothalamiTract {
                        pain_transmission: 1.0,
                        temperature_transmission: 1.0,
                        decussation_level: "Spinal cord".to_string(),
                    },
                    anterior_spinothalamic: AnteriorSpinothalamiTract {
                        crude_touch_transmission: 1.0,
                        pressure_transmission: 1.0,
                    },
                },
                spinocerebellar_tract: SpinocerebellarTract {
                    proprioceptive_input: 1.0,
                    unconscious_processing: true,
                },
                cortical_processing: SomatosensoryCortex {
                    primary_s1: PrimarySomatosensoryCortex {
                        location: "Postcentral gyrus".to_string(),
                        receptive_fields: 1.0,
                        topographic_organization: true,
                    },
                    secondary_s2: SecondarySomatosensoryCortex {
                        bilateral_representation: true,
                        integration_function: 1.0,
                    },
                    homunculus_representation: HomonculusMap {
                        hand_representation_mm2: 500.0,
                        face_representation_mm2: 400.0,
                        trunk_representation_mm2: 100.0,
                    },
                },
            },
        }
    }

    pub fn calculate_tactile_acuity(&self) -> f64 {
        self.mechanoreceptors.merkels_discs.spatial_resolution_mm
    }

    pub fn estimate_vibration_detection_threshold(&self) -> f64 {
        self.mechanoreceptors.pacinian_corpuscles.sensitivity_threshold_um
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tactile_system_creation() {
        let system = TactileSensorySystem::new_healthy();
        assert!(system.mechanoreceptors.meissners_corpuscles.density_per_cm2 > 0.0);
    }

    #[test]
    fn test_tactile_acuity() {
        let system = TactileSensorySystem::new_healthy();
        let acuity = system.calculate_tactile_acuity();
        assert_eq!(acuity, 0.5);
    }

    #[test]
    fn test_pain_pathways() {
        let system = TactileSensorySystem::new_healthy();
        assert!(system.nociceptors.mechanical_nociceptors.a_delta_fibers.conduction_velocity_m_per_s > system.nociceptors.mechanical_nociceptors.c_fibers.conduction_velocity_m_per_s);
    }
}
