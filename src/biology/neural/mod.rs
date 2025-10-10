//! # Neural Biology Module
//!
//! Models neural structures and signaling.

pub mod neurodegenerative;
pub mod neuron;
pub mod neuroplasticity;

pub use neurodegenerative::{
    AmyloidBeta, DiseaseType, NeurodegenerativeDisease, ProteinMisfolding, TauProtein,
};
pub use neuron::{IonChannel, Neuron, Neurotransmitter, Synapse};
pub use neuroplasticity::{
    LongTermDepression, LongTermPotentiation, Neuroplasticity, SynapticPlasticity, Synaptogenesis,
};
