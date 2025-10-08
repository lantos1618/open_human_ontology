pub mod muscle;
pub mod contraction;

pub use muscle::{Muscle, MuscleType, FiberComposition, MuscleFiber, FiberType, Sarcomere};
pub use contraction::{ContractionMechanism, ContractionType, Crossbridge, ExcitationContraction};
