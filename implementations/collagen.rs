use std::collections::VecDeque;

/// Represents a collagen molecule and its hierarchical assembly
pub struct Collagen {
    /// Primary structure
    primary: PrimaryStructure,
    /// Triple helix formation
    helix: TripleHelix,
    /// Fibril organization
    fibril: FibrilStructure,
    /// Mechanical properties
    properties: MechanicalProperties,
}

/// Primary structure of collagen
pub struct PrimaryStructure {
    /// Alpha chains
    chains: Vec<AlphaChain>,
    /// Chain length
    length: usize,
    /// Post-translational modifications
    modifications: Vec<PTM>,
}

/// Individual alpha chain
pub struct AlphaChain {
    /// Amino acid sequence
    sequence: Vec<AminoAcid>,
    /// Chain type (α1 or α2)
    chain_type: ChainType,
}

/// Triple helix structure
pub struct TripleHelix {
    /// Component chains
    chains: [AlphaChain; 3],
    /// Stability
    thermal_stability: f64,
    /// Pitch of the helix
    pitch: f64,
}

/// Fibril organization
pub struct FibrilStructure {
    /// D-period
    d_period: f64,
    /// Diameter
    diameter: f64,
    /// Packing density
    density: f64,
    /// Crosslink sites
    crosslinks: Vec<Crosslink>,
}

/// Mechanical properties
pub struct MechanicalProperties {
    /// Young's modulus (GPa)
    elastic_modulus: f64,
    /// Ultimate strength (MPa)
    tensile_strength: f64,
    /// Strain at failure
    failure_strain: f64,
}

/// Types of amino acids in collagen
#[derive(Clone, Copy)]
pub enum AminoAcid {
    Glycine,
    Proline,
    Hydroxyproline,
    Other(char),
}

/// Types of alpha chains
#[derive(Clone, Copy)]
pub enum ChainType {
    Alpha1,
    Alpha2,
}

/// Post-translational modification
pub struct PTM {
    position: usize,
    modification_type: ModificationType,
}

/// Types of modifications
pub enum ModificationType {
    Hydroxylation,
    Glycosylation,
    Phosphorylation,
}

/// Crosslink structure
pub struct Crosslink {
    position: usize,
    crosslink_type: CrosslinkType,
    maturity: f64,
}

/// Types of crosslinks
pub enum CrosslinkType {
    DHLNL,
    HLNL,
    Pyridinoline,
    Deoxypyridinoline,
}

impl Collagen {
    /// Create a new collagen molecule with default type I structure
    pub fn new() -> Self {
        let alpha1 = AlphaChain {
            sequence: Self::generate_sequence(1),
            chain_type: ChainType::Alpha1,
        };
        
        let alpha2 = AlphaChain {
            sequence: Self::generate_sequence(2),
            chain_type: ChainType::Alpha2,
        };

        Collagen {
            primary: PrimaryStructure {
                chains: vec![alpha1.clone(), alpha1.clone(), alpha2],
                length: 1050,
                modifications: Vec::new(),
            },
            helix: TripleHelix {
                chains: [alpha1.clone(), alpha1, alpha2],
                thermal_stability: 37.0,
                pitch: 8.6,
            },
            fibril: FibrilStructure {
                d_period: 67.0,
                diameter: 50.0,
                density: 0.8,
                crosslinks: Vec::new(),
            },
            properties: MechanicalProperties {
                elastic_modulus: 1.2,
                tensile_strength: 120.0,
                failure_strain: 0.13,
            },
        }
    }

    /// Generate amino acid sequence for different chain types
    fn generate_sequence(chain_number: u8) -> Vec<AminoAcid> {
        let mut sequence = Vec::new();
        let length = 1050;

        for i in 0..length {
            // Every third position must be Glycine
            if i % 3 == 0 {
                sequence.push(AminoAcid::Glycine);
            } else if chain_number == 1 {
                // α1 chain has more Proline
                if i % 3 == 1 {
                    sequence.push(AminoAcid::Proline);
                } else {
                    sequence.push(AminoAcid::Other('X'));
                }
            } else {
                // α2 chain has less Proline
                if i % 6 == 1 {
                    sequence.push(AminoAcid::Proline);
                } else {
                    sequence.push(AminoAcid::Other('X'));
                }
            }
        }

        sequence
    }

    /// Add post-translational modification
    pub fn add_modification(&mut self, position: usize, mod_type: ModificationType) {
        self.primary.modifications.push(PTM {
            position,
            modification_type: mod_type,
        });
        self.update_stability();
    }

    /// Add crosslink to the structure
    pub fn add_crosslink(&mut self, position: usize, crosslink_type: CrosslinkType) {
        self.fibril.crosslinks.push(Crosslink {
            position,
            crosslink_type,
            maturity: 0.0,
        });
        self.update_properties();
    }

    /// Calculate stability based on sequence and modifications
    pub fn calculate_stability(&self) -> f64 {
        let sequence_stability = self.sequence_stability();
        let modification_effect = self.modification_effect();
        let crosslink_effect = self.crosslink_effect();

        sequence_stability * modification_effect * crosslink_effect
    }

    /// Calculate stability contribution from sequence
    fn sequence_stability(&self) -> f64 {
        let mut stability = 0.0;
        
        for chain in &self.primary.chains {
            let mut gly_count = 0;
            let mut pro_count = 0;
            
            for aa in &chain.sequence {
                match aa {
                    AminoAcid::Glycine => gly_count += 1,
                    AminoAcid::Proline | AminoAcid::Hydroxyproline => pro_count += 1,
                    _ => (),
                }
            }
            
            // Stability increases with Gly-Pro-X triplets
            stability += (gly_count as f64 / chain.sequence.len() as f64) *
                        (pro_count as f64 / chain.sequence.len() as f64);
        }
        
        stability / self.primary.chains.len() as f64
    }

    /// Calculate effect of modifications on stability
    fn modification_effect(&self) -> f64 {
        let hydroxylation_count = self.primary.modifications.iter()
            .filter(|m| matches!(m.modification_type, ModificationType::Hydroxylation))
            .count();
            
        // Hydroxylation increases stability
        1.0 + (hydroxylation_count as f64 * 0.1)
    }

    /// Calculate effect of crosslinks on stability
    fn crosslink_effect(&self) -> f64 {
        let mature_crosslinks = self.fibril.crosslinks.iter()
            .filter(|c| c.maturity > 0.8)
            .count();
            
        // Mature crosslinks increase stability
        1.0 + (mature_crosslinks as f64 * 0.2)
    }

    /// Update thermal stability based on modifications
    fn update_stability(&mut self) {
        self.helix.thermal_stability = 37.0 * self.calculate_stability();
    }

    /// Update mechanical properties based on structure
    fn update_properties(&mut self) {
        let base_modulus = 1.2;
        let base_strength = 120.0;
        
        // Crosslinks increase mechanical properties
        let crosslink_factor = 1.0 + (self.fibril.crosslinks.len() as f64 * 0.1);
        
        self.properties.elastic_modulus = base_modulus * crosslink_factor;
        self.properties.tensile_strength = base_strength * crosslink_factor;
    }

    /// Simulate fibril assembly
    pub fn assemble_fibril(&mut self, temperature: f64, ph: f64) -> bool {
        // Check if conditions are suitable for assembly
        if temperature > self.helix.thermal_stability || ph < 6.0 || ph > 8.5 {
            return false;
        }

        // Simulate assembly process
        self.fibril.diameter = 50.0 * self.calculate_stability();
        self.fibril.density = 0.8 * (1.0 - (temperature - 37.0).abs() / 37.0);
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collagen_creation() {
        let collagen = Collagen::new();
        assert_eq!(collagen.primary.chains.len(), 3);
        assert_eq!(collagen.primary.length, 1050);
    }

    #[test]
    fn test_modification() {
        let mut collagen = Collagen::new();
        let initial_stability = collagen.calculate_stability();
        collagen.add_modification(10, ModificationType::Hydroxylation);
        assert!(collagen.calculate_stability() > initial_stability);
    }

    #[test]
    fn test_fibril_assembly() {
        let mut collagen = Collagen::new();
        assert!(collagen.assemble_fibril(37.0, 7.4));
        assert!(!collagen.assemble_fibril(50.0, 7.4));
    }
} 