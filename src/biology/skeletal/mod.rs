pub mod bone;
pub mod joint;
pub mod bone_remodeling;
pub mod biomechanics;

pub use bone::{Bone, BoneType, BoneStructure};
pub use joint::{Joint, JointType, JointMovement};
pub use bone_remodeling::{BoneRemodeling, RemodelingPhase};
pub use biomechanics::{MusculoskeletalBiomechanics, GaitAnalysis, Kinematics, Kinetics};
