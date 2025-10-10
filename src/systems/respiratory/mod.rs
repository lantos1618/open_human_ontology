pub mod lung;
pub mod gas_exchange;
pub mod breathing;
pub mod ventilation;
pub mod pulmonary_function;
pub mod oxygen_transport;
pub mod respiratory_mechanics;

pub use lung::{Lung, LungSide, Lobe, Alveolus};
pub use gas_exchange::{GasExchange, BloodGas, DiffusionParameters};
pub use breathing::{BreathingMechanics, BreathPhase, RespiratoryMuscles, BreathingPattern};
pub use ventilation::{VentilationMechanics, LungVolumes, LungCapacities, VentilationPerfusionRatio, AlveolarVentilation};
pub use pulmonary_function::{PulmonaryFunctionTest, Spirometry, ArterialBloodGas, OxygenationMetrics};
pub use oxygen_transport::{Hemoglobin, HemoglobinVariant, OxygenTransport, OxygenContent, TissueOxygenation};
pub use respiratory_mechanics::{RespiratoryMechanics, PressureVolumeCurve, VentilationPerfusionMatching, SurfactantSystem};
