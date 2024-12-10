//! # Molecular Biology Module
//! 
//! Defines molecular structures and their interactions in biological systems.

use serde::{Deserialize, Serialize};
use crate::biology::{BiologyError, BiologyResult, Molecule, Modification, Modifiable};

pub mod bone_matrix;
pub mod hydroxyapatite;
pub mod collagen;
pub mod crosslinks;
pub mod enzymes;

/// Represents a molecular structure with 3D coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MolecularStructure {
    /// Name of the structure
    pub name: String,
    /// Type of molecule
    pub molecule_type: Molecule,
    /// 3D coordinates of atoms
    pub coordinates: Vec<Coordinate3D>,
    /// Bonds between atoms
    pub bonds: Vec<Bond>,
}

/// 3D coordinate in nanometers
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Coordinate3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Chemical bond between atoms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bond {
    /// Index of first atom
    pub atom1: usize,
    /// Index of second atom
    pub atom2: usize,
    /// Type of bond
    pub bond_type: BondType,
    /// Bond strength in kJ/mol
    pub strength: f64,
}

/// Types of chemical bonds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BondType {
    Single,
    Double,
    Triple,
    Ionic,
    Hydrogen,
    VanDerWaals,
    Metallic,
    Coordinate,
}

impl MolecularStructure {
    /// Create a new molecular structure
    pub fn new(name: String, molecule_type: Molecule) -> Self {
        MolecularStructure {
            name,
            molecule_type,
            coordinates: Vec::new(),
            bonds: Vec::new(),
        }
    }

    /// Add an atom at specified coordinates
    pub fn add_atom(&mut self, coord: Coordinate3D) -> usize {
        self.coordinates.push(coord);
        self.coordinates.len() - 1
    }

    /// Add a bond between atoms
    pub fn add_bond(&mut self, atom1: usize, atom2: usize, bond_type: BondType, strength: f64) -> BiologyResult<()> {
        if atom1 >= self.coordinates.len() || atom2 >= self.coordinates.len() {
            return Err(BiologyError::InvalidInteraction("Atom index out of bounds".into()));
        }

        self.bonds.push(Bond {
            atom1,
            atom2,
            bond_type,
            strength,
        });

        Ok(())
    }

    /// Calculate center of mass
    pub fn center_of_mass(&self) -> Coordinate3D {
        let n = self.coordinates.len() as f64;
        let sum = self.coordinates.iter().fold(Coordinate3D { x: 0.0, y: 0.0, z: 0.0 }, |acc, coord| {
            Coordinate3D {
                x: acc.x + coord.x,
                y: acc.y + coord.y,
                z: acc.z + coord.z,
            }
        });

        Coordinate3D {
            x: sum.x / n,
            y: sum.y / n,
            z: sum.z / n,
        }
    }

    /// Calculate radius of gyration
    pub fn radius_of_gyration(&self) -> f64 {
        let center = self.center_of_mass();
        let n = self.coordinates.len() as f64;
        
        let sum_sq_dist = self.coordinates.iter().map(|coord| {
            let dx = coord.x - center.x;
            let dy = coord.y - center.y;
            let dz = coord.z - center.z;
            dx * dx + dy * dy + dz * dz
        }).sum::<f64>();

        (sum_sq_dist / n).sqrt()
    }
}

impl Modifiable for MolecularStructure {
    fn add_modification(&mut self, modification: Modification) -> BiologyResult<()> {
        match &mut self.molecule_type {
            Molecule::Protein { modifications, .. } => {
                modifications.push(modification);
                Ok(())
            },
            _ => Err(BiologyError::InvalidModification(
                "Only proteins can be modified".into()
            )),
        }
    }

    fn remove_modification(&mut self, modification: &Modification) -> BiologyResult<()> {
        match &mut self.molecule_type {
            Molecule::Protein { modifications, .. } => {
                if let Some(pos) = modifications.iter().position(|m| m == modification) {
                    modifications.remove(pos);
                    Ok(())
                } else {
                    Err(BiologyError::InvalidModification(
                        "Modification not found".into()
                    ))
                }
            },
            _ => Err(BiologyError::InvalidModification(
                "Only proteins can be modified".into()
            )),
        }
    }

    fn get_modifications(&self) -> &[Modification] {
        match &self.molecule_type {
            Molecule::Protein { modifications, .. } => modifications,
            _ => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molecular_structure() {
        let mut structure = MolecularStructure::new(
            "Test".into(),
            Molecule::Protein {
                name: "Test".into(),
                sequence: vec![],
                modifications: vec![],
            },
        );

        let atom1 = structure.add_atom(Coordinate3D { x: 0.0, y: 0.0, z: 0.0 });
        let atom2 = structure.add_atom(Coordinate3D { x: 1.0, y: 0.0, z: 0.0 });

        assert!(structure.add_bond(atom1, atom2, BondType::Single, 1.0).is_ok());
    }

    #[test]
    fn test_center_of_mass() {
        let mut structure = MolecularStructure::new(
            "Test".into(),
            Molecule::Protein {
                name: "Test".into(),
                sequence: vec![],
                modifications: vec![],
            },
        );

        structure.add_atom(Coordinate3D { x: -1.0, y: 0.0, z: 0.0 });
        structure.add_atom(Coordinate3D { x: 1.0, y: 0.0, z: 0.0 });

        let com = structure.center_of_mass();
        assert_eq!(com.x, 0.0);
        assert_eq!(com.y, 0.0);
        assert_eq!(com.z, 0.0);
    }
} 