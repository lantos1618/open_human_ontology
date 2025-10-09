use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualPathway {
    pub retina: RetinaProcessing,
    pub optic_nerve: OpticNerve,
    pub lateral_geniculate: LateralGeniculateNucleus,
    pub visual_cortex: VisualCortex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetinaProcessing {
    pub photoreceptors: PhotoreceptorLayer,
    pub bipolar_cells: BipolarCellLayer,
    pub ganglion_cells: GanglionCellLayer,
    pub light_adaptation_state: LightAdaptation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoreceptorLayer {
    pub rods: PhotoreceptorPopulation,
    pub cones: ConePopulation,
    pub rhodopsin_level: f64,
    pub dark_current_pa: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoreceptorPopulation {
    pub count: usize,
    pub sensitivity_quanta: f64,
    pub response_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConePopulation {
    pub s_cones: PhotoreceptorPopulation,
    pub m_cones: PhotoreceptorPopulation,
    pub l_cones: PhotoreceptorPopulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BipolarCellLayer {
    pub on_center_cells: usize,
    pub off_center_cells: usize,
    pub receptive_field_size_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GanglionCellLayer {
    pub magnocellular: GanglionCellType,
    pub parvocellular: GanglionCellType,
    pub koniocellular: GanglionCellType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GanglionCellType {
    pub count: usize,
    pub receptive_field_size_deg: f64,
    pub conduction_velocity_m_per_s: f64,
    pub temporal_resolution_hz: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightAdaptation {
    DarkAdapted,
    Mesopic,
    LightAdapted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticNerve {
    pub fiber_count: usize,
    pub myelination: f64,
    pub conduction_latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateralGeniculateNucleus {
    pub magnocellular_layers: Vec<LGNLayer>,
    pub parvocellular_layers: Vec<LGNLayer>,
    pub koniocellular_layers: Vec<LGNLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LGNLayer {
    pub neuron_count: usize,
    pub receptive_field_properties: ReceptiveField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptiveField {
    pub center_size_deg: f64,
    pub surround_size_deg: f64,
    pub center_surround_antagonism: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualCortex {
    pub v1: PrimaryVisualCortex,
    pub v2: SecondaryVisualCortex,
    pub ventral_stream: VentralStream,
    pub dorsal_stream: DorsalStream,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryVisualCortex {
    pub simple_cells: usize,
    pub complex_cells: usize,
    pub hypercomplex_cells: usize,
    pub orientation_columns: Vec<OrientationColumn>,
    pub ocular_dominance_columns: Vec<OcularColumn>,
    pub retinotopic_map: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrientationColumn {
    pub preferred_orientation_deg: f64,
    pub tuning_width_deg: f64,
    pub response_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcularColumn {
    pub eye_dominance: EyeDominance,
    pub binocularity_index: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EyeDominance {
    LeftEye,
    RightEye,
    Binocular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryVisualCortex {
    pub thickness_mm: f64,
    pub neuron_density_per_mm3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentralStream {
    pub v4_color: ColorProcessing,
    pub inferior_temporal: ObjectRecognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorProcessing {
    pub color_opponent_cells: usize,
    pub wavelength_sensitivity: Vec<WavelengthTuning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavelengthTuning {
    pub peak_wavelength_nm: f64,
    pub bandwidth_nm: f64,
    pub response_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRecognition {
    pub face_selective_neurons: usize,
    pub object_selective_neurons: usize,
    pub view_invariance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DorsalStream {
    pub mt_motion: MotionProcessing,
    pub mst_optic_flow: OpticFlowProcessing,
    pub posterior_parietal: SpatialProcessing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionProcessing {
    pub direction_selective_cells: usize,
    pub speed_tuning_deg_per_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpticFlowProcessing {
    pub expansion_cells: usize,
    pub rotation_cells: usize,
    pub translation_cells: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialProcessing {
    pub reach_neurons: usize,
    pub grasp_neurons: usize,
    pub saccade_neurons: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualAcuity {
    pub snellen_fraction: (u32, u32),
    pub logmar: f64,
    pub contrast_sensitivity: ContrastSensitivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastSensitivity {
    pub spatial_frequencies: Vec<f64>,
    pub sensitivity_values: Vec<f64>,
    pub peak_frequency_cpd: f64,
}

impl VisualPathway {
    pub fn new() -> Self {
        Self {
            retina: RetinaProcessing::default(),
            optic_nerve: OpticNerve::default(),
            lateral_geniculate: LateralGeniculateNucleus::default(),
            visual_cortex: VisualCortex::default(),
        }
    }

    pub fn process_visual_input(&self, luminance_cd_per_m2: f64) -> VisualResponse {
        let adaptation = if luminance_cd_per_m2 < 0.01 {
            LightAdaptation::DarkAdapted
        } else if luminance_cd_per_m2 < 10.0 {
            LightAdaptation::Mesopic
        } else {
            LightAdaptation::LightAdapted
        };

        VisualResponse {
            adaptation,
            photoreceptor_response: self.retina.photoreceptors.rods.sensitivity_quanta * luminance_cd_per_m2,
            ganglion_firing_rate_hz: 50.0,
            cortical_response: 1.0,
        }
    }
}

impl Default for VisualPathway {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualResponse {
    pub adaptation: LightAdaptation,
    pub photoreceptor_response: f64,
    pub ganglion_firing_rate_hz: f64,
    pub cortical_response: f64,
}

impl RetinaProcessing {
    pub fn default() -> Self {
        Self {
            photoreceptors: PhotoreceptorLayer::default(),
            bipolar_cells: BipolarCellLayer::default(),
            ganglion_cells: GanglionCellLayer::default(),
            light_adaptation_state: LightAdaptation::Mesopic,
        }
    }
}

impl PhotoreceptorLayer {
    pub fn default() -> Self {
        Self {
            rods: PhotoreceptorPopulation {
                count: 120_000_000,
                sensitivity_quanta: 1.0,
                response_time_ms: 200.0,
            },
            cones: ConePopulation::default(),
            rhodopsin_level: 1.0,
            dark_current_pa: 20.0,
        }
    }
}

impl ConePopulation {
    pub fn default() -> Self {
        Self {
            s_cones: PhotoreceptorPopulation {
                count: 500_000,
                sensitivity_quanta: 0.1,
                response_time_ms: 50.0,
            },
            m_cones: PhotoreceptorPopulation {
                count: 2_500_000,
                sensitivity_quanta: 0.15,
                response_time_ms: 50.0,
            },
            l_cones: PhotoreceptorPopulation {
                count: 3_000_000,
                sensitivity_quanta: 0.12,
                response_time_ms: 50.0,
            },
        }
    }
}

impl BipolarCellLayer {
    pub fn default() -> Self {
        Self {
            on_center_cells: 600_000,
            off_center_cells: 600_000,
            receptive_field_size_deg: 0.5,
        }
    }
}

impl GanglionCellLayer {
    pub fn default() -> Self {
        Self {
            magnocellular: GanglionCellType {
                count: 400_000,
                receptive_field_size_deg: 2.0,
                conduction_velocity_m_per_s: 20.0,
                temporal_resolution_hz: 80.0,
            },
            parvocellular: GanglionCellType {
                count: 800_000,
                receptive_field_size_deg: 0.3,
                conduction_velocity_m_per_s: 10.0,
                temporal_resolution_hz: 20.0,
            },
            koniocellular: GanglionCellType {
                count: 50_000,
                receptive_field_size_deg: 1.0,
                conduction_velocity_m_per_s: 15.0,
                temporal_resolution_hz: 40.0,
            },
        }
    }
}

impl OpticNerve {
    pub fn default() -> Self {
        Self {
            fiber_count: 1_200_000,
            myelination: 0.9,
            conduction_latency_ms: 10.0,
        }
    }
}

impl LateralGeniculateNucleus {
    pub fn default() -> Self {
        Self {
            magnocellular_layers: vec![LGNLayer::default(); 2],
            parvocellular_layers: vec![LGNLayer::default(); 4],
            koniocellular_layers: vec![LGNLayer::default(); 6],
        }
    }
}

impl LGNLayer {
    pub fn default() -> Self {
        Self {
            neuron_count: 100_000,
            receptive_field_properties: ReceptiveField::default(),
        }
    }
}

impl ReceptiveField {
    pub fn default() -> Self {
        Self {
            center_size_deg: 0.5,
            surround_size_deg: 2.0,
            center_surround_antagonism: 0.8,
        }
    }
}

impl VisualCortex {
    pub fn default() -> Self {
        Self {
            v1: PrimaryVisualCortex::default(),
            v2: SecondaryVisualCortex::default(),
            ventral_stream: VentralStream::default(),
            dorsal_stream: DorsalStream::default(),
        }
    }
}

impl PrimaryVisualCortex {
    pub fn default() -> Self {
        Self {
            simple_cells: 50_000_000,
            complex_cells: 100_000_000,
            hypercomplex_cells: 20_000_000,
            orientation_columns: Vec::new(),
            ocular_dominance_columns: Vec::new(),
            retinotopic_map: true,
        }
    }
}

impl SecondaryVisualCortex {
    pub fn default() -> Self {
        Self {
            thickness_mm: 2.0,
            neuron_density_per_mm3: 100_000.0,
        }
    }
}

impl VentralStream {
    pub fn default() -> Self {
        Self {
            v4_color: ColorProcessing::default(),
            inferior_temporal: ObjectRecognition::default(),
        }
    }
}

impl ColorProcessing {
    pub fn default() -> Self {
        Self {
            color_opponent_cells: 1_000_000,
            wavelength_sensitivity: Vec::new(),
        }
    }
}

impl ObjectRecognition {
    pub fn default() -> Self {
        Self {
            face_selective_neurons: 100_000,
            object_selective_neurons: 500_000,
            view_invariance: 0.7,
        }
    }
}

impl DorsalStream {
    pub fn default() -> Self {
        Self {
            mt_motion: MotionProcessing::default(),
            mst_optic_flow: OpticFlowProcessing::default(),
            posterior_parietal: SpatialProcessing::default(),
        }
    }
}

impl MotionProcessing {
    pub fn default() -> Self {
        Self {
            direction_selective_cells: 1_000_000,
            speed_tuning_deg_per_s: 10.0,
        }
    }
}

impl OpticFlowProcessing {
    pub fn default() -> Self {
        Self {
            expansion_cells: 50_000,
            rotation_cells: 50_000,
            translation_cells: 50_000,
        }
    }
}

impl SpatialProcessing {
    pub fn default() -> Self {
        Self {
            reach_neurons: 100_000,
            grasp_neurons: 100_000,
            saccade_neurons: 100_000,
        }
    }
}

impl VisualAcuity {
    pub fn normal_vision() -> Self {
        Self {
            snellen_fraction: (20, 20),
            logmar: 0.0,
            contrast_sensitivity: ContrastSensitivity::default(),
        }
    }

    pub fn is_normal(&self) -> bool {
        self.logmar <= 0.1
    }

    pub fn is_legally_blind(&self) -> bool {
        self.logmar >= 1.0
    }
}

impl ContrastSensitivity {
    pub fn default() -> Self {
        Self {
            spatial_frequencies: vec![1.0, 2.0, 4.0, 8.0, 16.0],
            sensitivity_values: vec![100.0, 200.0, 150.0, 80.0, 30.0],
            peak_frequency_cpd: 4.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visual_pathway() {
        let pathway = VisualPathway::new();
        assert_eq!(pathway.retina.photoreceptors.rods.count, 120_000_000);
    }

    #[test]
    fn test_visual_processing() {
        let pathway = VisualPathway::new();

        let dark_response = pathway.process_visual_input(0.001);
        assert_eq!(dark_response.adaptation, LightAdaptation::DarkAdapted);

        let bright_response = pathway.process_visual_input(100.0);
        assert_eq!(bright_response.adaptation, LightAdaptation::LightAdapted);
    }

    #[test]
    fn test_visual_acuity() {
        let acuity = VisualAcuity::normal_vision();
        assert!(acuity.is_normal());
        assert!(!acuity.is_legally_blind());
    }
}
