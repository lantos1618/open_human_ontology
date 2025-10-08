pub mod muscle;
pub mod contraction;
pub mod metabolism;

pub use muscle::{Muscle, MuscleType, FiberComposition, MuscleFiber, FiberType, Sarcomere};
pub use contraction::{ContractionMechanism, ContractionType, Crossbridge, ExcitationContraction};
pub use metabolism::{MuscleEnergyMetabolism, EnergySystem, MitochondrialFunction};
