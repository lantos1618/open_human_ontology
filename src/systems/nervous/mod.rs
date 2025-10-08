pub mod central;
pub mod peripheral;

pub use central::{CentralNervousSystem, Brain, SpinalCord};
pub use peripheral::{PeripheralNervousSystem, AutonomicNervousSystem, Sympathetic, Parasympathetic};
