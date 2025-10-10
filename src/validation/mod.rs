pub mod model_validation;
pub mod ground_truth;
pub mod metrics;

pub use model_validation::{ValidationFramework, ValidationResult, ValidationMetrics};
pub use ground_truth::{GroundTruthData, GroundTruthDatabase, ClinicalReference, EvidenceLevel};
pub use metrics::{ModelAccuracy, PredictionError, CorrelationMetrics};
