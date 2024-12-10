//! # Neuron Module
//! 
//! Models neurons and their electrical/chemical properties.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::biology::{BiologyError, BiologyResult, Molecule};

/// Types of neurons based on morphology
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NeuronType {
    Unipolar,
    Bipolar,
    Multipolar,
    Pseudounipolar,
}

/// Neurotransmitter types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Neurotransmitter {
    Glutamate,
    GABA,
    Acetylcholine,
    Dopamine,
    Serotonin,
    Norepinephrine,
    Substance_P,
    Endorphin,
}

/// Ion channel states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelState {
    Closed,
    Open,
    Inactivated,
}

/// Ion channel types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelType {
    VoltageGatedSodium,
    VoltageGatedPotassium,
    VoltageGatedCalcium,
    LigandGated(Neurotransmitter),
    LeakChannel,
}

/// Ion channel properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IonChannel {
    channel_type: ChannelType,
    state: ChannelState,
    conductance: f64,
    voltage_threshold: Option<f64>,
    inactivation_time: f64,
}

/// Membrane properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Membrane {
    voltage: f64,
    capacitance: f64,
    channels: HashMap<ChannelType, Vec<IonChannel>>,
    ion_concentrations: HashMap<String, f64>,
}

/// Synapse properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Synapse {
    neurotransmitter: Neurotransmitter,
    vesicle_count: u32,
    release_probability: f64,
    receptor_density: f64,
    plasticity_state: f64,
}

/// Represents a neuron
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Neuron {
    neuron_type: NeuronType,
    membrane: Membrane,
    synapses: Vec<Synapse>,
    resting_potential: f64,
    threshold_potential: f64,
    refractory_period: f64,
    is_firing: bool,
}

impl IonChannel {
    /// Create a new ion channel
    pub fn new(channel_type: ChannelType, conductance: f64) -> Self {
        let voltage_threshold = match &channel_type {
            ChannelType::VoltageGatedSodium => Some(-55.0),
            ChannelType::VoltageGatedPotassium => Some(-40.0),
            ChannelType::VoltageGatedCalcium => Some(-50.0),
            _ => None,
        };

        IonChannel {
            channel_type,
            state: ChannelState::Closed,
            conductance,
            voltage_threshold,
            inactivation_time: 1.0,
        }
    }

    /// Update channel state based on voltage
    pub fn update_state(&mut self, voltage: f64) {
        match &self.channel_type {
            ChannelType::VoltageGatedSodium | 
            ChannelType::VoltageGatedPotassium |
            ChannelType::VoltageGatedCalcium => {
                if let Some(threshold) = self.voltage_threshold {
                    if voltage >= threshold {
                        self.state = ChannelState::Open;
                    } else {
                        self.state = ChannelState::Closed;
                    }
                }
            },
            _ => {}
        }
    }

    /// Calculate current through channel
    pub fn calculate_current(&self, voltage: f64, ion_concentration: f64) -> f64 {
        match self.state {
            ChannelState::Open => {
                self.conductance * (voltage - ion_concentration)
            },
            _ => 0.0,
        }
    }
}

impl Membrane {
    /// Create a new membrane
    pub fn new(capacitance: f64) -> Self {
        let mut channels = HashMap::new();
        let mut ion_concentrations = HashMap::new();

        // Initialize with basic channels
        channels.insert(ChannelType::VoltageGatedSodium, 
            vec![IonChannel::new(ChannelType::VoltageGatedSodium, 120.0)]);
        channels.insert(ChannelType::VoltageGatedPotassium,
            vec![IonChannel::new(ChannelType::VoltageGatedPotassium, 36.0)]);
        channels.insert(ChannelType::LeakChannel,
            vec![IonChannel::new(ChannelType::LeakChannel, 0.3)]);

        // Set ion concentrations (mM)
        ion_concentrations.insert("Na+".into(), 140.0);
        ion_concentrations.insert("K+".into(), 5.0);
        ion_concentrations.insert("Cl-".into(), 150.0);

        Membrane {
            voltage: -70.0,
            capacitance,
            channels,
            ion_concentrations,
        }
    }

    /// Update membrane voltage
    pub fn update_voltage(&mut self, current: f64, dt: f64) {
        let dv = (current / self.capacitance) * dt;
        self.voltage += dv;

        // Update channel states
        for channels in self.channels.values_mut() {
            for channel in channels {
                channel.update_state(self.voltage);
            }
        }
    }

    /// Calculate total membrane current
    pub fn calculate_total_current(&self) -> f64 {
        let mut total_current = 0.0;
        
        for channels in self.channels.values() {
            for channel in channels {
                let ion_conc = match &channel.channel_type {
                    ChannelType::VoltageGatedSodium => *self.ion_concentrations.get("Na+").unwrap_or(&140.0),
                    ChannelType::VoltageGatedPotassium => *self.ion_concentrations.get("K+").unwrap_or(&5.0),
                    _ => 0.0,
                };
                total_current += channel.calculate_current(self.voltage, ion_conc);
            }
        }
        
        total_current
    }
}

impl Synapse {
    /// Create a new synapse
    pub fn new(neurotransmitter: Neurotransmitter) -> Self {
        Synapse {
            neurotransmitter,
            vesicle_count: 100,
            release_probability: 0.3,
            receptor_density: 1.0,
            plasticity_state: 1.0,
        }
    }

    /// Release neurotransmitter
    pub fn release_neurotransmitter(&mut self) -> u32 {
        let release_amount = (self.vesicle_count as f64 * self.release_probability) as u32;
        self.vesicle_count -= release_amount;
        release_amount
    }

    /// Update synaptic plasticity
    pub fn update_plasticity(&mut self, activity: f64) {
        // Simple plasticity rule
        if activity > 0.5 {
            self.plasticity_state *= 1.1; // LTP
        } else {
            self.plasticity_state *= 0.9; // LTD
        }
        
        // Bound plasticity
        self.plasticity_state = self.plasticity_state.clamp(0.1, 5.0);
    }
}

impl Neuron {
    /// Create a new neuron
    pub fn new(neuron_type: NeuronType) -> Self {
        Neuron {
            neuron_type,
            membrane: Membrane::new(1.0),
            synapses: vec![],
            resting_potential: -70.0,
            threshold_potential: -55.0,
            refractory_period: 2.0,
            is_firing: false,
        }
    }

    /// Update neuron state
    pub fn update(&mut self, dt: f64) -> BiologyResult<()> {
        // Update membrane voltage
        let current = self.membrane.calculate_total_current();
        self.membrane.update_voltage(current, dt);

        // Check for action potential
        if self.membrane.voltage >= self.threshold_potential && !self.is_firing {
            self.fire_action_potential();
        }

        Ok(())
    }

    /// Fire an action potential
    fn fire_action_potential(&mut self) {
        self.is_firing = true;
        
        // Release neurotransmitters from all synapses
        for synapse in &mut self.synapses {
            synapse.release_neurotransmitter();
        }

        // Reset after refractory period
        self.membrane.voltage = self.resting_potential;
        self.is_firing = false;
    }

    /// Add a synapse
    pub fn add_synapse(&mut self, neurotransmitter: Neurotransmitter) {
        self.synapses.push(Synapse::new(neurotransmitter));
    }

    /// Get membrane voltage
    pub fn get_voltage(&self) -> f64 {
        self.membrane.voltage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_creation() {
        let neuron = Neuron::new(NeuronType::Multipolar);
        assert_eq!(neuron.get_voltage(), -70.0);
    }

    #[test]
    fn test_action_potential() {
        let mut neuron = Neuron::new(NeuronType::Multipolar);
        
        // Simulate strong input
        neuron.membrane.voltage = -54.0; // Above threshold
        neuron.update(0.1).unwrap();
        
        assert!(neuron.membrane.voltage <= neuron.resting_potential);
    }

    #[test]
    fn test_synapse() {
        let mut synapse = Synapse::new(Neurotransmitter::Glutamate);
        let released = synapse.release_neurotransmitter();
        assert!(released > 0);
        assert!(synapse.vesicle_count < 100);
    }
} 