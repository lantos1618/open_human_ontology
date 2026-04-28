pub mod cardiovascular;
pub mod nervous;
pub mod renal;
pub mod respiratory;

pub use cardiovascular::{Blood, BloodVessel, Heart};
pub use nervous::{CentralNervousSystem, PeripheralNervousSystem};
pub use renal::{Filtration, Kidney};
pub use respiratory::{BreathingPattern, GasExchange, Lung};
