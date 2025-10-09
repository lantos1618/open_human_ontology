use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProprioceptiveSystem {
    pub muscle_spindles: MuscleSpindlePopulation,
    pub golgi_tendon_organs: GolgiTendonOrganPopulation,
    pub joint_receptors: JointReceptorPopulation,
    pub vestibular_system: VestibularSystem,
    pub body_awareness: BodyAwareness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleSpindlePopulation {
    pub total_count: u32,
    pub intrafusal_fibers: IntrafusalFibers,
    pub sensitivity: f64,
    pub adaptation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrafusalFibers {
    pub nuclear_bag_fibers: u32,
    pub nuclear_chain_fibers: u32,
    pub gamma_motor_innervation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GolgiTendonOrganPopulation {
    pub total_count: u32,
    pub tension_threshold_n: f64,
    pub dynamic_range: f64,
    pub response_latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointReceptorPopulation {
    pub ruffini_endings: u32,
    pub pacinian_corpuscles: u32,
    pub joint_position_accuracy_degrees: f64,
    pub movement_detection_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestibularSystem {
    pub semicircular_canals: SemicircularCanals,
    pub otolith_organs: OtolithOrgans,
    pub vestibular_nuclei_activity: f64,
    pub vestibulo_ocular_reflex_gain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemicircularCanals {
    pub anterior: Canal,
    pub posterior: Canal,
    pub horizontal: Canal,
    pub endolymph_viscosity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canal {
    pub hair_cells: u32,
    pub cupula_displacement_sensitivity: f64,
    pub angular_velocity_threshold_deg_per_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtolithOrgans {
    pub utricle: OtolithOrgan,
    pub saccule: OtolithOrgan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtolithOrgan {
    pub hair_cells: u32,
    pub otoconia_mass_mg: f64,
    pub linear_acceleration_threshold_m_per_s2: f64,
    pub gravity_sensing_accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyAwareness {
    pub spatial_position_accuracy: f64,
    pub movement_coordination_score: f64,
    pub balance_stability_index: f64,
    pub kinesthetic_acuity: f64,
}

impl ProprioceptiveSystem {
    pub fn new_healthy_adult() -> Self {
        Self {
            muscle_spindles: MuscleSpindlePopulation {
                total_count: 250_000,
                intrafusal_fibers: IntrafusalFibers {
                    nuclear_bag_fibers: 100_000,
                    nuclear_chain_fibers: 150_000,
                    gamma_motor_innervation: 0.8,
                },
                sensitivity: 0.85,
                adaptation_rate: 0.7,
            },
            golgi_tendon_organs: GolgiTendonOrganPopulation {
                total_count: 150_000,
                tension_threshold_n: 0.5,
                dynamic_range: 100.0,
                response_latency_ms: 15.0,
            },
            joint_receptors: JointReceptorPopulation {
                ruffini_endings: 50_000,
                pacinian_corpuscles: 30_000,
                joint_position_accuracy_degrees: 2.0,
                movement_detection_threshold: 0.2,
            },
            vestibular_system: VestibularSystem {
                semicircular_canals: SemicircularCanals {
                    anterior: Canal {
                        hair_cells: 7000,
                        cupula_displacement_sensitivity: 0.95,
                        angular_velocity_threshold_deg_per_s: 2.0,
                    },
                    posterior: Canal {
                        hair_cells: 7000,
                        cupula_displacement_sensitivity: 0.95,
                        angular_velocity_threshold_deg_per_s: 2.0,
                    },
                    horizontal: Canal {
                        hair_cells: 7000,
                        cupula_displacement_sensitivity: 0.95,
                        angular_velocity_threshold_deg_per_s: 2.0,
                    },
                    endolymph_viscosity: 1.3,
                },
                otolith_organs: OtolithOrgans {
                    utricle: OtolithOrgan {
                        hair_cells: 30_000,
                        otoconia_mass_mg: 0.5,
                        linear_acceleration_threshold_m_per_s2: 0.01,
                        gravity_sensing_accuracy: 0.95,
                    },
                    saccule: OtolithOrgan {
                        hair_cells: 16_000,
                        otoconia_mass_mg: 0.3,
                        linear_acceleration_threshold_m_per_s2: 0.01,
                        gravity_sensing_accuracy: 0.93,
                    },
                },
                vestibular_nuclei_activity: 0.8,
                vestibulo_ocular_reflex_gain: 0.95,
            },
            body_awareness: BodyAwareness {
                spatial_position_accuracy: 0.9,
                movement_coordination_score: 0.85,
                balance_stability_index: 0.88,
                kinesthetic_acuity: 0.87,
            },
        }
    }

    pub fn calculate_balance_score(&self) -> f64 {
        let vestibular_contribution = 0.4 * self.vestibular_system.vestibulo_ocular_reflex_gain;
        let proprioceptive_contribution = 0.35 * self.body_awareness.balance_stability_index;
        let joint_contribution = 0.25 * (1.0 - self.joint_receptors.joint_position_accuracy_degrees / 10.0);

        vestibular_contribution + proprioceptive_contribution + joint_contribution
    }

    pub fn assess_fall_risk(&self) -> FallRisk {
        let balance_score = self.calculate_balance_score();
        let coordination = self.body_awareness.movement_coordination_score;

        let risk_score = 1.0 - ((balance_score + coordination) / 2.0);

        if risk_score < 0.2 {
            FallRisk::Low
        } else if risk_score < 0.4 {
            FallRisk::Moderate
        } else if risk_score < 0.6 {
            FallRisk::High
        } else {
            FallRisk::VeryHigh
        }
    }

    pub fn assess_spatial_orientation(&self) -> SpatialOrientationAssessment {
        SpatialOrientationAssessment {
            vertical_perception_accuracy: self.vestibular_system.otolith_organs.utricle.gravity_sensing_accuracy,
            horizontal_plane_stability: self.vestibular_system.semicircular_canals.horizontal.cupula_displacement_sensitivity,
            three_dimensional_awareness: self.body_awareness.spatial_position_accuracy,
            movement_detection_sensitivity: self.joint_receptors.movement_detection_threshold,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallRisk {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialOrientationAssessment {
    pub vertical_perception_accuracy: f64,
    pub horizontal_plane_stability: f64,
    pub three_dimensional_awareness: f64,
    pub movement_detection_sensitivity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_proprioceptive_system() {
        let system = ProprioceptiveSystem::new_healthy_adult();
        assert_eq!(system.muscle_spindles.total_count, 250_000);
        assert!(system.vestibular_system.vestibulo_ocular_reflex_gain > 0.9);
    }

    #[test]
    fn test_balance_score() {
        let system = ProprioceptiveSystem::new_healthy_adult();
        let balance = system.calculate_balance_score();
        assert!(balance > 0.7);
        assert!(balance <= 1.0);
    }

    #[test]
    fn test_fall_risk_assessment() {
        let system = ProprioceptiveSystem::new_healthy_adult();
        let risk = system.assess_fall_risk();
        assert_eq!(risk, FallRisk::Low);
    }

    #[test]
    fn test_spatial_orientation() {
        let system = ProprioceptiveSystem::new_healthy_adult();
        let orientation = system.assess_spatial_orientation();
        assert!(orientation.vertical_perception_accuracy > 0.9);
        assert!(orientation.three_dimensional_awareness > 0.8);
    }
}
