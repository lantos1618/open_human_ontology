pub mod gi_tract;
pub mod absorption;
pub mod digestion;
pub mod liver;
pub mod pancreas;

pub use gi_tract::{GITract, Stomach, SmallIntestine, LargeIntestine};
pub use absorption::{NutrientAbsorption, Villus, Enterocyte};
pub use digestion::{DigestiveEnzymes, MacronutrientDigestion, GastricSecretion, PancreaticSecretion, BileSecretion};
pub use liver::{Liver, DetoxificationCapacity, HepaticMetabolism, BileProduction, LiverFunctionAssessment, HepatocyteZone, ProteinSynthesis};
pub use pancreas::Pancreas;
