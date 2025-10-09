use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MechanoreceptorSystem {
    pub cutaneous_receptors: CutaneousReceptors,
    pub deep_tissue_receptors: DeepTissueReceptors,
    pub visceral_mechanoreceptors: VisceralMechanoreceptors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CutaneousReceptors {
    pub meissners_corpuscles: ReceptorPopulation,
    pub merkels_discs: ReceptorPopulation,
    pub pacinian_corpuscles: ReceptorPopulation,
    pub ruffini_endings: ReceptorPopulation,
    pub free_nerve_endings: ReceptorPopulation,
    pub hair_follicle_receptors: ReceptorPopulation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeepTissueReceptors {
    pub muscle_spindles: MuscleSpindles,
    pub golgi_tendon_organs: GolgiTendonOrgans,
    pub joint_receptors: JointReceptors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisceralMechanoreceptors {
    pub baroreceptors: Baroreceptors,
    pub stretch_receptors: StretchReceptors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceptorPopulation {
    pub count: u64,
    pub density_per_cm2: f64,
    pub receptive_field_size_mm2: f64,
    pub adaptation_rate: AdaptationRate,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdaptationRate {
    Slow,
    Rapid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuscleSpindles {
    pub total_count: u32,
    pub intrafusal_fibers: IntrafusalFibers,
    pub gamma_motor_neurons: u32,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrafusalFibers {
    pub nuclear_bag_fibers: u32,
    pub nuclear_chain_fibers: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GolgiTendonOrgans {
    pub count: u32,
    pub tension_threshold_n: f64,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JointReceptors {
    pub ruffini_endings: u32,
    pub pacinian_corpuscles: u32,
    pub golgi_like_endings: u32,
    pub free_nerve_endings: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Baroreceptors {
    pub carotid_sinus: BaroreceptorCluster,
    pub aortic_arch: BaroreceptorCluster,
    pub cardiopulmonary: BaroreceptorCluster,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaroreceptorCluster {
    pub receptor_count: u32,
    pub baseline_pressure_mmhg: f64,
    pub sensitivity: f64,
    pub firing_rate_hz: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StretchReceptors {
    pub lung_stretch_receptors: u32,
    pub bladder_stretch_receptors: u32,
    pub gastric_stretch_receptors: u32,
    pub intestinal_stretch_receptors: u32,
}

impl MechanoreceptorSystem {
    pub fn new_normal() -> Self {
        Self {
            cutaneous_receptors: CutaneousReceptors::new_normal(),
            deep_tissue_receptors: DeepTissueReceptors::new_normal(),
            visceral_mechanoreceptors: VisceralMechanoreceptors::new_normal(),
        }
    }

    pub fn calculate_tactile_acuity(&self) -> f64 {
        let meissner_contribution = self.cutaneous_receptors.meissners_corpuscles.sensitivity * 0.4;
        let merkel_contribution = self.cutaneous_receptors.merkels_discs.sensitivity * 0.4;
        let pacinian_contribution = self.cutaneous_receptors.pacinian_corpuscles.sensitivity * 0.2;

        meissner_contribution + merkel_contribution + pacinian_contribution
    }

    pub fn calculate_proprioceptive_accuracy(&self) -> f64 {
        let spindle_contrib = self.deep_tissue_receptors.muscle_spindles.sensitivity * 0.5;
        let gto_contrib = self.deep_tissue_receptors.golgi_tendon_organs.sensitivity * 0.3;
        let joint_contrib = 0.2;

        spindle_contrib + gto_contrib + joint_contrib
    }
}

impl CutaneousReceptors {
    pub fn new_normal() -> Self {
        Self {
            meissners_corpuscles: ReceptorPopulation {
                count: 150_000,
                density_per_cm2: 70.0,
                receptive_field_size_mm2: 4.0,
                adaptation_rate: AdaptationRate::Rapid,
                sensitivity: 1.0,
            },
            merkels_discs: ReceptorPopulation {
                count: 120_000,
                density_per_cm2: 100.0,
                receptive_field_size_mm2: 2.0,
                adaptation_rate: AdaptationRate::Slow,
                sensitivity: 1.0,
            },
            pacinian_corpuscles: ReceptorPopulation {
                count: 350_000,
                density_per_cm2: 20.0,
                receptive_field_size_mm2: 100.0,
                adaptation_rate: AdaptationRate::Rapid,
                sensitivity: 1.0,
            },
            ruffini_endings: ReceptorPopulation {
                count: 200_000,
                density_per_cm2: 10.0,
                receptive_field_size_mm2: 50.0,
                adaptation_rate: AdaptationRate::Slow,
                sensitivity: 1.0,
            },
            free_nerve_endings: ReceptorPopulation {
                count: 3_000_000,
                density_per_cm2: 200.0,
                receptive_field_size_mm2: 1.0,
                adaptation_rate: AdaptationRate::Slow,
                sensitivity: 1.0,
            },
            hair_follicle_receptors: ReceptorPopulation {
                count: 5_000_000,
                density_per_cm2: 50.0,
                receptive_field_size_mm2: 10.0,
                adaptation_rate: AdaptationRate::Rapid,
                sensitivity: 1.0,
            },
        }
    }
}

impl DeepTissueReceptors {
    pub fn new_normal() -> Self {
        Self {
            muscle_spindles: MuscleSpindles::new_normal(),
            golgi_tendon_organs: GolgiTendonOrgans::new_normal(),
            joint_receptors: JointReceptors::new_normal(),
        }
    }
}

impl MuscleSpindles {
    pub fn new_normal() -> Self {
        Self {
            total_count: 40000,
            intrafusal_fibers: IntrafusalFibers {
                nuclear_bag_fibers: 80000,
                nuclear_chain_fibers: 120000,
            },
            gamma_motor_neurons: 30000,
            sensitivity: 1.0,
        }
    }
}

impl GolgiTendonOrgans {
    pub fn new_normal() -> Self {
        Self {
            count: 100000,
            tension_threshold_n: 0.1,
            sensitivity: 1.0,
        }
    }
}

impl JointReceptors {
    pub fn new_normal() -> Self {
        Self {
            ruffini_endings: 50000,
            pacinian_corpuscles: 30000,
            golgi_like_endings: 20000,
            free_nerve_endings: 100000,
        }
    }
}

impl VisceralMechanoreceptors {
    pub fn new_normal() -> Self {
        Self {
            baroreceptors: Baroreceptors::new_normal(),
            stretch_receptors: StretchReceptors::new_normal(),
        }
    }
}

impl Baroreceptors {
    pub fn new_normal() -> Self {
        Self {
            carotid_sinus: BaroreceptorCluster {
                receptor_count: 50,
                baseline_pressure_mmhg: 90.0,
                sensitivity: 1.0,
                firing_rate_hz: 40.0,
            },
            aortic_arch: BaroreceptorCluster {
                receptor_count: 80,
                baseline_pressure_mmhg: 90.0,
                sensitivity: 1.0,
                firing_rate_hz: 40.0,
            },
            cardiopulmonary: BaroreceptorCluster {
                receptor_count: 100,
                baseline_pressure_mmhg: 8.0,
                sensitivity: 1.0,
                firing_rate_hz: 20.0,
            },
        }
    }
}

impl StretchReceptors {
    pub fn new_normal() -> Self {
        Self {
            lung_stretch_receptors: 1000,
            bladder_stretch_receptors: 500,
            gastric_stretch_receptors: 2000,
            intestinal_stretch_receptors: 5000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_mechanoreceptor_system() {
        let system = MechanoreceptorSystem::new_normal();
        assert_eq!(system.cutaneous_receptors.meissners_corpuscles.count, 150_000);
    }

    #[test]
    fn test_tactile_acuity() {
        let system = MechanoreceptorSystem::new_normal();
        let acuity = system.calculate_tactile_acuity();
        assert!((acuity - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_proprioceptive_accuracy() {
        let system = MechanoreceptorSystem::new_normal();
        let accuracy = system.calculate_proprioceptive_accuracy();
        assert!(accuracy > 0.8);
    }
}
