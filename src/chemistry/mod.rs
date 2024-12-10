//! # Chemistry Module
//! 
//! Defines chemical reactions and properties in biological systems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::biology::{BiologyError, BiologyResult, Molecule, Concentration};

/// Represents a chemical reaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    /// Name of the reaction
    pub name: String,
    /// Reactants with stoichiometric coefficients
    pub reactants: HashMap<Molecule, f64>,
    /// Products with stoichiometric coefficients
    pub products: HashMap<Molecule, f64>,
    /// Rate constant
    pub rate_constant: f64,
    /// Activation energy in kJ/mol
    pub activation_energy: f64,
    /// Temperature dependence
    pub temperature_dependence: TemperatureDependence,
}

/// Temperature dependence of reaction rates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemperatureDependence {
    /// Arrhenius equation
    Arrhenius {
        /// Pre-exponential factor
        a: f64,
        /// Activation energy in kJ/mol
        ea: f64,
    },
    /// Eyring equation
    Eyring {
        /// Enthalpy of activation in kJ/mol
        delta_h: f64,
        /// Entropy of activation in J/(mol·K)
        delta_s: f64,
    },
}

/// Chemical equilibrium properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equilibrium {
    /// Forward reaction
    pub forward: Reaction,
    /// Reverse reaction
    pub reverse: Reaction,
    /// Equilibrium constant
    pub k_eq: f64,
}

/// pH effects on reactions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct pHEffect {
    /// Optimal pH
    pub optimal_ph: f64,
    /// pH range for activity
    pub ph_range: (f64, f64),
    /// pH sensitivity factor
    pub sensitivity: f64,
}

impl Reaction {
    /// Create a new reaction
    pub fn new(
        name: String,
        reactants: HashMap<Molecule, f64>,
        products: HashMap<Molecule, f64>,
        rate_constant: f64,
        activation_energy: f64,
    ) -> Self {
        Reaction {
            name,
            reactants,
            products,
            rate_constant,
            activation_energy,
            temperature_dependence: TemperatureDependence::Arrhenius {
                a: 1.0,
                ea: activation_energy,
            },
        }
    }

    /// Calculate reaction rate
    pub fn calculate_rate(&self, concentrations: &HashMap<Molecule, Concentration>) -> BiologyResult<f64> {
        let mut rate = self.rate_constant;

        // Calculate rate based on reactant concentrations
        for (reactant, stoich) in &self.reactants {
            if let Some(conc) = concentrations.get(reactant) {
                rate *= conc.value.powf(*stoich);
            } else {
                return Err(BiologyError::InvalidConcentration(
                    format!("Missing concentration for reactant: {:?}", reactant)
                ));
            }
        }

        Ok(rate)
    }

    /// Calculate temperature effect on rate constant
    pub fn temperature_effect(&self, temperature: f64) -> f64 {
        const R: f64 = 8.314; // Gas constant in J/(mol·K)

        match self.temperature_dependence {
            TemperatureDependence::Arrhenius { a, ea } => {
                a * (-ea * 1000.0 / (R * temperature)).exp()
            },
            TemperatureDependence::Eyring { delta_h, delta_s } => {
                let kb = 1.380649e-23; // Boltzmann constant
                let h = 6.62607015e-34; // Planck constant
                
                (kb * temperature / h) * 
                (-delta_h * 1000.0 / (R * temperature)).exp() *
                (delta_s / R).exp()
            },
        }
    }
}

impl Equilibrium {
    /// Create a new equilibrium
    pub fn new(forward: Reaction, reverse: Reaction, k_eq: f64) -> Self {
        Equilibrium {
            forward,
            reverse,
            k_eq,
        }
    }

    /// Calculate equilibrium concentrations
    pub fn calculate_equilibrium(
        &self,
        initial_concentrations: &HashMap<Molecule, Concentration>,
    ) -> BiologyResult<HashMap<Molecule, Concentration>> {
        let mut equilibrium_concentrations = initial_concentrations.clone();
        
        // Simple iteration to approach equilibrium
        for _ in 0..100 {
            let forward_rate = self.forward.calculate_rate(&equilibrium_concentrations)?;
            let reverse_rate = self.reverse.calculate_rate(&equilibrium_concentrations)?;
            
            if (forward_rate / reverse_rate - self.k_eq).abs() < 1e-6 {
                break;
            }
            
            // Adjust concentrations
            let delta = (forward_rate - reverse_rate) * 0.1;
            for (reactant, stoich) in &self.forward.reactants {
                if let Some(conc) = equilibrium_concentrations.get_mut(reactant) {
                    conc.value -= delta * stoich;
                }
            }
            for (product, stoich) in &self.forward.products {
                if let Some(conc) = equilibrium_concentrations.get_mut(product) {
                    conc.value += delta * stoich;
                }
            }
        }
        
        Ok(equilibrium_concentrations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biology::ConcentrationUnit;

    #[test]
    fn test_reaction_rate() {
        let mut reactants = HashMap::new();
        let mut products = HashMap::new();
        let mut concentrations = HashMap::new();

        let reactant = Molecule::Protein {
            name: "Test".into(),
            sequence: vec![],
            modifications: vec![],
        };

        reactants.insert(reactant.clone(), 1.0);
        concentrations.insert(reactant, Concentration {
            value: 2.0,
            unit: ConcentrationUnit::Molar,
        });

        let reaction = Reaction::new(
            "Test".into(),
            reactants,
            products,
            1.0,
            50.0,
        );

        let rate = reaction.calculate_rate(&concentrations).unwrap();
        assert_eq!(rate, 2.0);
    }

    #[test]
    fn test_temperature_effect() {
        let reaction = Reaction {
            name: "Test".into(),
            reactants: HashMap::new(),
            products: HashMap::new(),
            rate_constant: 1.0,
            activation_energy: 50.0,
            temperature_dependence: TemperatureDependence::Arrhenius {
                a: 1.0,
                ea: 50.0,
            },
        };

        let rate_298 = reaction.temperature_effect(298.0);
        let rate_308 = reaction.temperature_effect(308.0);
        assert!(rate_308 > rate_298); // Rate increases with temperature
    }
} 