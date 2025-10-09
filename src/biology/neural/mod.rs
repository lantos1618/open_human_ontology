//! # Neural Biology Module
//!
//! Models neural structures and signaling.

pub mod neuron;
pub mod neuroplasticity;
pub mod neurodegenerative;

pub use neuron::{Neuron, Neurotransmitter, IonChannel, Synapse};
pub use neuroplasticity::{Neuroplasticity, SynapticPlasticity, LongTermPotentiation, LongTermDepression, Synaptogenesis};
pub use neurodegenerative::{NeurodegenerativeDisease, DiseaseType, ProteinMisfolding, AmyloidBeta, TauProtein};
