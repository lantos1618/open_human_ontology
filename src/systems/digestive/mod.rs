pub mod gi_tract;
pub mod absorption;
pub mod digestion;

pub use gi_tract::{GITract, Stomach, SmallIntestine, LargeIntestine};
pub use absorption::{NutrientAbsorption, Villus, Enterocyte};
pub use digestion::{DigestiveEnzymes, MacronutrientDigestion, GastricSecretion, PancreaticSecretion, BileSecretion};
