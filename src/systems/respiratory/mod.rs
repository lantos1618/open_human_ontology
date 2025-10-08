pub mod lung;
pub mod gas_exchange;
pub mod breathing;

pub use lung::{Lung, LungSide, Lobe, Alveolus};
pub use gas_exchange::{GasExchange, BloodGas, DiffusionParameters};
pub use breathing::{BreathingMechanics, BreathPhase, RespiratoryMuscles, BreathingPattern};
