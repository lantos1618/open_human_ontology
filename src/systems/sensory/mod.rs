pub mod hearing;
pub mod mechanoreception;
pub mod nociception;
pub mod proprioception;
pub mod smell;
pub mod taste;
pub mod touch;
pub mod vestibular;
pub mod vision;
pub mod visual_processing;

// Type aliases to resolve conflicts
// Side conflict: hearing vs vision
pub use hearing::Side as HearingSide;
pub use vision::Side as VisionSide;

// OpticNerve conflict: vision vs visual_processing
pub use vision::OpticNerve as VisionOpticNerve;
pub use visual_processing::OpticNerve as VisualProcessingOpticNerve;

// VisualAcuity conflict: vision vs visual_processing
pub use vision::VisualAcuity as VisionVisualAcuity;
pub use visual_processing::VisualAcuity as VisualProcessingAcuity;

// VestibularSystem conflict: hearing vs vestibular
pub use hearing::VestibularSystem as HearingVestibularSystem;
pub use vestibular::VestibularSystem as VestibularVestibularSystem;

// Explicit exports from hearing (excluding conflicting Side and VestibularSystem)
pub use hearing::{AuditorySystem, Ear, OuterEar, MiddleEar, Ossicles, InnerEar, Cochlea, OtolithOrgans, HearingThreshold, HearingLossDegree};

pub use mechanoreception::{Baroreceptors, MechanoreceptorSystem};
pub use nociception::{NociceptorPopulation, PainModulation, PainSystem, PainType};
pub use proprioception::{FallRisk, ProprioceptiveSystem, SpatialOrientationAssessment};
pub use smell::*;
pub use taste::*;
pub use touch::*;
// Explicit exports from vestibular (excluding conflicting VestibularSystem)
pub use vestibular::{VestibularDisorder};

// Explicit exports from vision (excluding conflicting Side, OpticNerve, VisualAcuity)
pub use vision::{VisualSystem, Eye, Cornea, Lens, Retina, Photoreceptors, ConeDistribution, Snellen, ColorVision};

// Explicit exports from visual_processing (excluding conflicting OpticNerve, VisualAcuity)
pub use visual_processing::{VisualPathway, RetinaProcessing, PhotoreceptorLayer, PhotoreceptorPopulation, ConePopulation, BipolarCellLayer, GanglionCellLayer, GanglionCellType, LightAdaptation, LateralGeniculateNucleus, LGNLayer, ReceptiveField, VisualCortex, PrimaryVisualCortex, OrientationColumn, OcularColumn, EyeDominance, SecondaryVisualCortex, VentralStream, ColorProcessing, WavelengthTuning, ObjectRecognition, DorsalStream, MotionProcessing, OpticFlowProcessing, SpatialProcessing, ContrastSensitivity, VisualResponse};
