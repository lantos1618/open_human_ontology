pub mod appendages;
pub mod skin;
pub mod wound_healing;

pub use appendages::{Hair, Nail, SweatGland};
pub use skin::{Dermis, Epidermis, Melanin, Skin, SkinType};
pub use wound_healing::{
    CellularResponse, GrowthFactors, HealingComplications, HealingFactors, HealingPhase,
    ScarFormation, ScarType, WoundDepth, WoundHealing, WoundTreatment, WoundType,
};
