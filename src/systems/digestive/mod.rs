pub mod gi_tract;
pub mod absorption;

pub use gi_tract::{GITract, Stomach, SmallIntestine, LargeIntestine};
pub use absorption::{NutrientAbsorption, Villus, Enterocyte};
