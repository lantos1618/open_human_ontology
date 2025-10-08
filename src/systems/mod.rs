pub mod cardiovascular;
pub mod respiratory;
pub mod muscular;
pub mod digestive;
pub mod endocrine;
pub mod nervous;
pub mod renal;
pub mod integumentary;
pub mod immune;

pub use cardiovascular::{Heart, BloodVessel, Blood};
pub use respiratory::{Lung, GasExchange, BreathingPattern};
pub use muscular::{Muscle, ContractionMechanism};
pub use digestive::{GITract, NutrientAbsorption};
pub use endocrine::{Hormone, EndocrineLandscape};
pub use nervous::{CentralNervousSystem, PeripheralNervousSystem};
pub use renal::{Kidney, Filtration};
pub use integumentary::{Skin, Hair, Nail};
pub use immune::{LymphaticSystem, WhiteBloodCell};
