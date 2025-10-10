pub mod absorption;
pub mod digestion;
pub mod gi_tract;
pub mod gut_brain_axis;
pub mod liver;
pub mod motility;
pub mod pancreas;

pub use absorption::{Enterocyte, NutrientAbsorption, Villus};
pub use digestion::{
    BileSecretion, DigestiveEnzymes, GastricSecretion, MacronutrientDigestion, PancreaticSecretion,
};
pub use gi_tract::{GITract, LargeIntestine, SmallIntestine, Stomach};
pub use gut_brain_axis::{
    GutBrainAxis, GutBrainCommunicationPathway, GutBrainDisorder, GutBrainDisorderType,
    HPAAxisActivity, ImmuneSignaling, NeurotransmitterProduction, StressResponse,
};
pub use liver::{
    BileProduction, DetoxificationCapacity, HepaticMetabolism, HepatocyteZone, Liver,
    LiverFunctionAssessment, ProteinSynthesis,
};
pub use motility::{
    BristolStoolScale, ColonTransit, EntericNervousSystem, EntericNeurotransmitter, EntericReflex,
    GIRegion, GastricEmptying, GutMotility, MealComposition, MotilityPattern, SmoothMuscleLayers,
};
pub use pancreas::Pancreas;
