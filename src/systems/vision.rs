use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eye {
    pub cornea: Cornea,
    pub lens: Lens,
    pub retina: Retina,
    pub optic_nerve: OpticNerve,
    pub iris: Iris,
    pub pupil_diameter_mm: f64,
    pub intraocular_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cornea {
    pub curvature_radius_mm: f64,
    pub thickness_um: f64,
    pub refractive_index: f64,
    pub clarity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lens {
    pub thickness_mm: f64,
    pub refractive_power_diopters: f64,
    pub accommodation_amplitude: f64,
    pub transparency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retina {
    pub photoreceptors: Vec<Photoreceptor>,
    pub retinal_pigment_epithelium_health: f64,
    pub macula_density: f64,
    pub thickness_um: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Photoreceptor {
    Rod {
        position: (f64, f64),
        sensitivity: f64,
        response_time_ms: f64,
    },
    Cone {
        position: (f64, f64),
        cone_type: ConeType,
        sensitivity: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConeType {
    S,
    M,
    L,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticNerve {
    pub axon_count: u32,
    pub myelination_quality: f64,
    pub cup_to_disc_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iris {
    pub color: IrisColor,
    pub muscle_tone: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrisColor {
    Brown,
    Blue,
    Green,
    Hazel,
    Gray,
    Amber,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualPathway {
    pub optic_chiasm: OpticChiasm,
    pub lateral_geniculate_nucleus: LateralGeniculateNucleus,
    pub visual_cortex: VisualCortex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticChiasm {
    pub crossing_completeness: f64,
    pub signal_integrity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateralGeniculateNucleus {
    pub layer_count: u8,
    pub neuron_density: f64,
    pub signal_processing_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualCortex {
    pub v1_area: VisualCortexArea,
    pub v2_area: VisualCortexArea,
    pub v3_area: VisualCortexArea,
    pub v4_area: VisualCortexArea,
    pub v5_area: VisualCortexArea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualCortexArea {
    pub neuron_count: u64,
    pub activation_level: f64,
    pub specialization: CorticalSpecialization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorticalSpecialization {
    EdgeDetection,
    ColorProcessing,
    MotionDetection,
    ObjectRecognition,
    SpatialProcessing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualAcuity {
    pub snellen_fraction: (u32, u32),
    pub logmar: f64,
    pub contrast_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorVision {
    pub cone_response: ConeResponse,
    pub color_discrimination_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConeResponse {
    pub s_cone_response: f64,
    pub m_cone_response: f64,
    pub l_cone_response: f64,
}

impl Eye {
    pub fn new() -> Self {
        Self {
            cornea: Cornea {
                curvature_radius_mm: 7.8,
                thickness_um: 550.0,
                refractive_index: 1.376,
                clarity: 1.0,
            },
            lens: Lens {
                thickness_mm: 4.0,
                refractive_power_diopters: 20.0,
                accommodation_amplitude: 8.0,
                transparency: 1.0,
            },
            retina: Retina {
                photoreceptors: Vec::new(),
                retinal_pigment_epithelium_health: 1.0,
                macula_density: 1.0,
                thickness_um: 250.0,
            },
            optic_nerve: OpticNerve {
                axon_count: 1_200_000,
                myelination_quality: 1.0,
                cup_to_disc_ratio: 0.3,
            },
            iris: Iris {
                color: IrisColor::Brown,
                muscle_tone: 1.0,
            },
            pupil_diameter_mm: 4.0,
            intraocular_pressure_mmhg: 15.0,
        }
    }

    pub fn adjust_pupil_size(&mut self, luminance: f64) -> BiologyResult<()> {
        if luminance < 0.0 {
            return Err(BiologyError::InvalidValue(
                "Luminance cannot be negative".to_string(),
            ));
        }

        self.pupil_diameter_mm =
            2.0 + 6.0 * (-0.4 * luminance.log10()).exp() / (1.0 + (-0.4 * luminance.log10()).exp());
        Ok(())
    }

    pub fn accommodate_for_distance(&mut self, distance_m: f64) -> BiologyResult<()> {
        if distance_m <= 0.0 {
            return Err(BiologyError::InvalidValue(
                "Distance must be positive".to_string(),
            ));
        }

        let required_power = 1.0 / distance_m;
        let max_accommodation = self.lens.accommodation_amplitude;

        if required_power > max_accommodation {
            return Err(BiologyError::InvalidValue(
                "Object too close to focus".to_string(),
            ));
        }

        self.lens.refractive_power_diopters = 20.0 + required_power;
        Ok(())
    }

    pub fn calculate_visual_acuity(&self) -> VisualAcuity {
        let base_acuity = self.cornea.clarity * self.lens.transparency * self.retina.macula_density;

        VisualAcuity {
            snellen_fraction: if base_acuity > 0.9 {
                (20, 20)
            } else if base_acuity > 0.7 {
                (20, 30)
            } else if base_acuity > 0.5 {
                (20, 40)
            } else {
                (20, 60)
            },
            logmar: -base_acuity.log10(),
            contrast_sensitivity: base_acuity * 1.5,
        }
    }

    pub fn detect_glaucoma_risk(&self) -> f64 {
        let pressure_risk = if self.intraocular_pressure_mmhg > 21.0 {
            (self.intraocular_pressure_mmhg - 21.0) / 10.0
        } else {
            0.0
        };

        let cup_disc_risk = if self.optic_nerve.cup_to_disc_ratio > 0.5 {
            (self.optic_nerve.cup_to_disc_ratio - 0.5) * 2.0
        } else {
            0.0
        };

        (pressure_risk + cup_disc_risk).min(1.0)
    }

    pub fn detect_cataract_severity(&self) -> f64 {
        1.0 - self.lens.transparency
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualPathway {
    pub fn new() -> Self {
        Self {
            optic_chiasm: OpticChiasm {
                crossing_completeness: 1.0,
                signal_integrity: 1.0,
            },
            lateral_geniculate_nucleus: LateralGeniculateNucleus {
                layer_count: 6,
                neuron_density: 1.0,
                signal_processing_efficiency: 1.0,
            },
            visual_cortex: VisualCortex {
                v1_area: VisualCortexArea {
                    neuron_count: 140_000_000,
                    activation_level: 0.5,
                    specialization: CorticalSpecialization::EdgeDetection,
                },
                v2_area: VisualCortexArea {
                    neuron_count: 100_000_000,
                    activation_level: 0.5,
                    specialization: CorticalSpecialization::ColorProcessing,
                },
                v3_area: VisualCortexArea {
                    neuron_count: 50_000_000,
                    activation_level: 0.5,
                    specialization: CorticalSpecialization::MotionDetection,
                },
                v4_area: VisualCortexArea {
                    neuron_count: 60_000_000,
                    activation_level: 0.5,
                    specialization: CorticalSpecialization::ColorProcessing,
                },
                v5_area: VisualCortexArea {
                    neuron_count: 40_000_000,
                    activation_level: 0.5,
                    specialization: CorticalSpecialization::MotionDetection,
                },
            },
        }
    }

    pub fn process_visual_signal(&mut self, signal_strength: f64) -> BiologyResult<f64> {
        if !(0.0..=1.0).contains(&signal_strength) {
            return Err(BiologyError::InvalidValue(
                "Signal strength must be between 0 and 1".to_string(),
            ));
        }

        let chiasm_output = signal_strength
            * self.optic_chiasm.signal_integrity
            * self.optic_chiasm.crossing_completeness;
        let lgn_output =
            chiasm_output * self.lateral_geniculate_nucleus.signal_processing_efficiency;

        let cortical_processing = (self.visual_cortex.v1_area.activation_level
            + self.visual_cortex.v2_area.activation_level
            + self.visual_cortex.v3_area.activation_level
            + self.visual_cortex.v4_area.activation_level
            + self.visual_cortex.v5_area.activation_level)
            / 5.0;

        Ok(lgn_output * cortical_processing)
    }
}

impl Default for VisualPathway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eye_creation() {
        let eye = Eye::new();
        assert_eq!(eye.optic_nerve.axon_count, 1_200_000);
        assert_eq!(eye.intraocular_pressure_mmhg, 15.0);
    }

    #[test]
    fn test_pupil_adjustment() {
        let mut eye = Eye::new();
        eye.adjust_pupil_size(100.0).unwrap();
        assert!(eye.pupil_diameter_mm < 4.0);
    }

    #[test]
    fn test_accommodation() {
        let mut eye = Eye::new();
        eye.accommodate_for_distance(0.5).unwrap();
        assert!(eye.lens.refractive_power_diopters > 20.0);
    }

    #[test]
    fn test_glaucoma_detection() {
        let mut eye = Eye::new();
        eye.intraocular_pressure_mmhg = 25.0;
        let risk = eye.detect_glaucoma_risk();
        assert!(risk > 0.0);
    }

    #[test]
    fn test_visual_pathway() {
        let mut pathway = VisualPathway::new();
        let output = pathway.process_visual_signal(1.0).unwrap();
        assert!(output > 0.0 && output <= 1.0);
    }
}
