use serde::{Deserialize, Serialize};
use crate::biology::{BiologyError, BiologyResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ear {
    pub outer_ear: OuterEar,
    pub middle_ear: MiddleEar,
    pub inner_ear: InnerEar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OuterEar {
    pub pinna: Pinna,
    pub ear_canal: EarCanal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pinna {
    pub surface_area_cm2: f64,
    pub shape_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarCanal {
    pub length_mm: f64,
    pub diameter_mm: f64,
    pub wax_buildup: f64,
    pub resonant_frequency_hz: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddleEar {
    pub tympanic_membrane: TympanicMembrane,
    pub ossicles: Ossicles,
    pub eustachian_tube: EustachianTube,
    pub middle_ear_pressure_pa: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TympanicMembrane {
    pub thickness_um: f64,
    pub surface_area_mm2: f64,
    pub compliance: f64,
    pub integrity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ossicles {
    pub malleus: Ossicle,
    pub incus: Ossicle,
    pub stapes: Ossicle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ossicle {
    pub mass_mg: f64,
    pub mobility: f64,
    pub articulation_quality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EustachianTube {
    pub patency: f64,
    pub length_mm: f64,
    pub opening_pressure_pa: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerEar {
    pub cochlea: Cochlea,
    pub vestibular_system: VestibularSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cochlea {
    pub basilar_membrane: BasilarMembrane,
    pub organ_of_corti: OrganOfCorti,
    pub cochlear_fluid: CochlearFluid,
    pub spiral_ganglion: SpiralGanglion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasilarMembrane {
    pub length_mm: f64,
    pub width_at_base_mm: f64,
    pub width_at_apex_mm: f64,
    pub stiffness_gradient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganOfCorti {
    pub inner_hair_cells: Vec<HairCell>,
    pub outer_hair_cells: Vec<HairCell>,
    pub supporting_cells_health: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairCell {
    pub position_mm: f64,
    pub stereocilia_count: u32,
    pub sensitivity: f64,
    pub frequency_tuning_hz: f64,
    pub health: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CochlearFluid {
    pub endolymph_k_concentration_mm: f64,
    pub perilymph_na_concentration_mm: f64,
    pub endocochlear_potential_mv: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiralGanglion {
    pub neuron_count: u32,
    pub myelination_quality: f64,
    pub firing_synchrony: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestibularSystem {
    pub semicircular_canals: SemicircularCanals,
    pub otolith_organs: OtolithOrgans,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemicircularCanals {
    pub anterior_canal: SemicircularCanal,
    pub posterior_canal: SemicircularCanal,
    pub horizontal_canal: SemicircularCanal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemicircularCanal {
    pub radius_mm: f64,
    pub hair_cell_count: u32,
    pub cupula_deflection_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtolithOrgans {
    pub utricle: OtolithOrgan,
    pub saccule: OtolithOrgan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtolithOrgan {
    pub otolith_mass_mg: f64,
    pub hair_cell_count: u32,
    pub linear_acceleration_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditoryPathway {
    pub cochlear_nucleus: CochlearNucleus,
    pub superior_olivary_complex: SuperiorOlivaryComplex,
    pub inferior_colliculus: InferiorColliculus,
    pub medial_geniculate_nucleus: MedialGeniculateNucleus,
    pub auditory_cortex: AuditoryCortex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CochlearNucleus {
    pub neuron_count: u64,
    pub signal_processing_fidelity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperiorOlivaryComplex {
    pub neuron_count: u64,
    pub interaural_time_difference_sensitivity_us: f64,
    pub interaural_level_difference_sensitivity_db: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferiorColliculus {
    pub neuron_count: u64,
    pub frequency_map_precision: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedialGeniculateNucleus {
    pub neuron_count: u64,
    pub relay_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditoryCortex {
    pub primary_auditory_cortex: CortexArea,
    pub secondary_auditory_cortex: CortexArea,
    pub tonotopic_map_quality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortexArea {
    pub neuron_count: u64,
    pub activation_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HearingAcuity {
    pub pure_tone_thresholds_db: Vec<(f64, f64)>,
    pub speech_reception_threshold_db: f64,
    pub word_recognition_score: f64,
}

impl Ear {
    pub fn new() -> Self {
        Self {
            outer_ear: OuterEar {
                pinna: Pinna {
                    surface_area_cm2: 6.0,
                    shape_efficiency: 0.9,
                },
                ear_canal: EarCanal {
                    length_mm: 26.0,
                    diameter_mm: 7.0,
                    wax_buildup: 0.1,
                    resonant_frequency_hz: 3000.0,
                },
            },
            middle_ear: MiddleEar {
                tympanic_membrane: TympanicMembrane {
                    thickness_um: 74.0,
                    surface_area_mm2: 85.0,
                    compliance: 1.0,
                    integrity: 1.0,
                },
                ossicles: Ossicles {
                    malleus: Ossicle {
                        mass_mg: 23.0,
                        mobility: 1.0,
                        articulation_quality: 1.0,
                    },
                    incus: Ossicle {
                        mass_mg: 30.0,
                        mobility: 1.0,
                        articulation_quality: 1.0,
                    },
                    stapes: Ossicle {
                        mass_mg: 3.0,
                        mobility: 1.0,
                        articulation_quality: 1.0,
                    },
                },
                eustachian_tube: EustachianTube {
                    patency: 1.0,
                    length_mm: 35.0,
                    opening_pressure_pa: 400.0,
                },
                middle_ear_pressure_pa: 101325.0,
            },
            inner_ear: InnerEar {
                cochlea: Cochlea {
                    basilar_membrane: BasilarMembrane {
                        length_mm: 35.0,
                        width_at_base_mm: 0.04,
                        width_at_apex_mm: 0.5,
                        stiffness_gradient: 1.0,
                    },
                    organ_of_corti: OrganOfCorti {
                        inner_hair_cells: Vec::new(),
                        outer_hair_cells: Vec::new(),
                        supporting_cells_health: 1.0,
                    },
                    cochlear_fluid: CochlearFluid {
                        endolymph_k_concentration_mm: 150.0,
                        perilymph_na_concentration_mm: 140.0,
                        endocochlear_potential_mv: 80.0,
                    },
                    spiral_ganglion: SpiralGanglion {
                        neuron_count: 30000,
                        myelination_quality: 1.0,
                        firing_synchrony: 1.0,
                    },
                },
                vestibular_system: VestibularSystem {
                    semicircular_canals: SemicircularCanals {
                        anterior_canal: SemicircularCanal {
                            radius_mm: 3.2,
                            hair_cell_count: 7000,
                            cupula_deflection_sensitivity: 1.0,
                        },
                        posterior_canal: SemicircularCanal {
                            radius_mm: 3.2,
                            hair_cell_count: 7000,
                            cupula_deflection_sensitivity: 1.0,
                        },
                        horizontal_canal: SemicircularCanal {
                            radius_mm: 3.2,
                            hair_cell_count: 7000,
                            cupula_deflection_sensitivity: 1.0,
                        },
                    },
                    otolith_organs: OtolithOrgans {
                        utricle: OtolithOrgan {
                            otolith_mass_mg: 2.0,
                            hair_cell_count: 30000,
                            linear_acceleration_sensitivity: 1.0,
                        },
                        saccule: OtolithOrgan {
                            otolith_mass_mg: 1.5,
                            hair_cell_count: 16000,
                            linear_acceleration_sensitivity: 1.0,
                        },
                    },
                },
            },
        }
    }

    pub fn calculate_sound_amplification(&self) -> f64 {
        let tympanic_area = self.middle_ear.tympanic_membrane.surface_area_mm2;
        let stapes_footplate_area = 3.2;
        let area_ratio = tympanic_area / stapes_footplate_area;

        let lever_ratio = 1.3;

        let amplification_db = 20.0 * (area_ratio * lever_ratio).log10();
        amplification_db * self.middle_ear.tympanic_membrane.integrity
    }

    pub fn detect_frequency(&self, frequency_hz: f64) -> BiologyResult<f64> {
        if !(20.0..=20000.0).contains(&frequency_hz) {
            return Ok(0.0);
        }

        let position_mm = 35.0 * (1.0 - (frequency_hz.log10() - 1.3) / 2.7);

        let closest_cell_sensitivity = self
            .inner_ear
            .cochlea
            .organ_of_corti
            .inner_hair_cells
            .iter()
            .filter(|cell| (cell.position_mm - position_mm).abs() < 1.0)
            .map(|cell| cell.sensitivity * cell.health)
            .fold(0.0f64, |max, val| max.max(val));

        Ok(closest_cell_sensitivity)
    }

    pub fn calculate_hearing_loss(&self) -> f64 {
        let healthy_cells = self
            .inner_ear
            .cochlea
            .organ_of_corti
            .inner_hair_cells
            .iter()
            .filter(|cell| cell.health > 0.5)
            .count();

        let total_cells = self.inner_ear.cochlea.organ_of_corti.inner_hair_cells.len();

        if total_cells == 0 {
            return 0.0;
        }

        let cell_health_ratio = healthy_cells as f64 / total_cells as f64;
        ((1.0 - cell_health_ratio) * 100.0).min(100.0)
    }

    pub fn detect_tinnitus_risk(&self) -> f64 {
        let damaged_cells = self
            .inner_ear
            .cochlea
            .organ_of_corti
            .inner_hair_cells
            .iter()
            .filter(|cell| cell.health < 0.7)
            .count();

        let total_cells = self.inner_ear.cochlea.organ_of_corti.inner_hair_cells.len();

        if total_cells == 0 {
            return 0.0;
        }

        (damaged_cells as f64 / total_cells as f64).min(1.0)
    }
}

impl Default for Ear {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditoryPathway {
    pub fn new() -> Self {
        Self {
            cochlear_nucleus: CochlearNucleus {
                neuron_count: 88000,
                signal_processing_fidelity: 1.0,
            },
            superior_olivary_complex: SuperiorOlivaryComplex {
                neuron_count: 34000,
                interaural_time_difference_sensitivity_us: 10.0,
                interaural_level_difference_sensitivity_db: 1.0,
            },
            inferior_colliculus: InferiorColliculus {
                neuron_count: 392000,
                frequency_map_precision: 1.0,
            },
            medial_geniculate_nucleus: MedialGeniculateNucleus {
                neuron_count: 500000,
                relay_efficiency: 1.0,
            },
            auditory_cortex: AuditoryCortex {
                primary_auditory_cortex: CortexArea {
                    neuron_count: 100_000_000,
                    activation_level: 0.5,
                },
                secondary_auditory_cortex: CortexArea {
                    neuron_count: 80_000_000,
                    activation_level: 0.5,
                },
                tonotopic_map_quality: 1.0,
            },
        }
    }

    pub fn process_auditory_signal(&self, signal_strength: f64) -> BiologyResult<f64> {
        if !(0.0..=1.0).contains(&signal_strength) {
            return Err(BiologyError::InvalidValue(
                "Signal strength must be between 0 and 1".to_string(),
            ));
        }

        let cn_output = signal_strength * self.cochlear_nucleus.signal_processing_fidelity;
        let soc_output = cn_output * 0.95;
        let ic_output = soc_output * self.inferior_colliculus.frequency_map_precision;
        let mgn_output = ic_output * self.medial_geniculate_nucleus.relay_efficiency;

        let cortical_processing = (self.auditory_cortex.primary_auditory_cortex.activation_level
            + self.auditory_cortex.secondary_auditory_cortex.activation_level)
            / 2.0;

        Ok(mgn_output * cortical_processing * self.auditory_cortex.tonotopic_map_quality)
    }

    pub fn localize_sound(
        &self,
        interaural_time_difference_us: f64,
        interaural_level_difference_db: f64,
    ) -> BiologyResult<f64> {
        let time_cue = (interaural_time_difference_us
            / self
                .superior_olivary_complex
                .interaural_time_difference_sensitivity_us)
            .abs()
            .min(1.0);

        let level_cue = (interaural_level_difference_db
            / self
                .superior_olivary_complex
                .interaural_level_difference_sensitivity_db)
            .abs()
            .min(1.0);

        Ok((time_cue + level_cue) / 2.0)
    }
}

impl Default for AuditoryPathway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ear_creation() {
        let ear = Ear::new();
        assert_eq!(ear.inner_ear.cochlea.spiral_ganglion.neuron_count, 30000);
    }

    #[test]
    fn test_sound_amplification() {
        let ear = Ear::new();
        let amplification = ear.calculate_sound_amplification();
        assert!(amplification > 0.0);
    }

    #[test]
    fn test_frequency_detection() {
        let mut ear = Ear::new();
        ear.inner_ear.cochlea.organ_of_corti.inner_hair_cells = vec![
            HairCell {
                position_mm: 10.0,
                stereocilia_count: 50,
                sensitivity: 1.0,
                frequency_tuning_hz: 1000.0,
                health: 1.0,
            },
        ];

        let detection = ear.detect_frequency(1000.0).unwrap();
        assert!(detection >= 0.0);
    }

    #[test]
    fn test_auditory_pathway() {
        let pathway = AuditoryPathway::new();
        let output = pathway.process_auditory_signal(1.0).unwrap();
        assert!(output > 0.0 && output <= 1.0);
    }

    #[test]
    fn test_sound_localization() {
        let pathway = AuditoryPathway::new();
        let localization = pathway.localize_sound(100.0, 5.0).unwrap();
        assert!((0.0..=1.0).contains(&localization));
    }
}
