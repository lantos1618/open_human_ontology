//! # Neural Biology Module
//!
//! Models neural structures and signaling.

pub mod neuron;
pub mod neuroplasticity;

pub use neuron::{IonChannel, Neuron, Neurotransmitter, Synapse};
pub use neuroplasticity::{
    LongTermDepression, LongTermPotentiation, Neuroplasticity, SynapticPlasticity, Synaptogenesis,
};
