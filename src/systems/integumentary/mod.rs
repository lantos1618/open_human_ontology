pub mod skin;
pub mod appendages;
pub mod wound_healing;

pub use skin::{Skin, Epidermis, Dermis, Melanin, SkinType};
pub use appendages::{Hair, Nail, SweatGland};
pub use wound_healing::{WoundHealing, WoundType, WoundDepth, HealingPhase, HealingFactors, CellularResponse, GrowthFactors, ScarFormation, ScarType, HealingComplications, WoundTreatment};
