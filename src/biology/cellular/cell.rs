use crate::biology::{BiologyError, BiologyResult, Compartment};
use crate::biology::cellular::organelles::Organelle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellType {
    Neuron,
    Muscle,
    Epithelial,
    Connective,
    BloodCell,
    Osteoblast,
    Osteoclast,
    Osteocyte,
    Hepatocyte,
    Cardiomyocyte,
    Adipocyte,
    Fibroblast,
    Macrophage,
    Lymphocyte,
    StemCell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellState {
    Healthy,
    Stressed,
    Apoptotic,
    Necrotic,
    Proliferating,
    Differentiated,
    Senescent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellCycle {
    G0,
    G1,
    S,
    G2,
    M,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: CellType,
    pub state: CellState,
    pub cycle: CellCycle,
    pub diameter_um: f64,
    pub membrane_potential_mv: f64,
    pub atp_concentration_mm: f64,
    pub compartment: Compartment,
    pub organelles: Vec<Organelle>,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        let (diameter, atp) = match cell_type {
            CellType::Neuron => (20.0, 5.0),
            CellType::Muscle => (50.0, 8.0),
            CellType::Epithelial => (15.0, 4.0),
            CellType::Connective => (20.0, 4.0),
            CellType::BloodCell => (7.0, 3.0),
            CellType::Osteoblast => (20.0, 5.0),
            CellType::Osteoclast => (50.0, 6.0),
            CellType::Osteocyte => (25.0, 4.0),
            CellType::Hepatocyte => (30.0, 7.0),
            CellType::Cardiomyocyte => (100.0, 10.0),
            CellType::Adipocyte => (100.0, 3.0),
            CellType::Fibroblast => (20.0, 4.0),
            CellType::Macrophage => (25.0, 6.0),
            CellType::Lymphocyte => (10.0, 5.0),
            CellType::StemCell => (15.0, 5.0),
        };

        Cell {
            cell_type,
            state: CellState::Healthy,
            cycle: CellCycle::G0,
            diameter_um: diameter,
            membrane_potential_mv: -70.0,
            atp_concentration_mm: atp,
            compartment: Compartment::Intracellular,
            organelles: Vec::new(),
        }
    }

    pub fn volume_um3(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * (self.diameter_um / 2.0).powi(3)
    }

    pub fn surface_area_um2(&self) -> f64 {
        4.0 * std::f64::consts::PI * (self.diameter_um / 2.0).powi(2)
    }

    pub fn is_healthy(&self) -> bool {
        self.state == CellState::Healthy
            && self.atp_concentration_mm > 1.0
            && self.membrane_potential_mv.abs() > 40.0
    }

    pub fn assess_energy_state(&self) -> BiologyResult<f64> {
        if self.atp_concentration_mm < 0.0 {
            return Err(BiologyError::InvalidValue(
                "ATP concentration cannot be negative".to_string()
            ));
        }

        let normal_atp = match self.cell_type {
            CellType::Cardiomyocyte => 10.0,
            CellType::Neuron => 5.0,
            CellType::Muscle => 8.0,
            CellType::Hepatocyte => 7.0,
            _ => 4.0,
        };

        Ok(self.atp_concentration_mm / normal_atp)
    }

    pub fn add_organelle(&mut self, organelle: Organelle) {
        self.organelles.push(organelle);
    }

    pub fn organelle_count(&self, organelle_type: &str) -> usize {
        self.organelles
            .iter()
            .filter(|o| o.name() == organelle_type)
            .count()
    }

    pub fn undergo_apoptosis(&mut self) -> BiologyResult<()> {
        if self.state == CellState::Necrotic {
            return Err(BiologyError::InvalidState(
                "Cannot undergo apoptosis from necrotic state".to_string()
            ));
        }
        self.state = CellState::Apoptotic;
        self.membrane_potential_mv = 0.0;
        self.atp_concentration_mm = 0.0;
        Ok(())
    }

    pub fn enter_cell_cycle(&mut self, phase: CellCycle) -> BiologyResult<()> {
        if self.state != CellState::Healthy && self.state != CellState::Proliferating {
            return Err(BiologyError::InvalidState(
                "Only healthy cells can enter cell cycle".to_string()
            ));
        }
        self.cycle = phase;
        self.state = CellState::Proliferating;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new(CellType::Neuron);
        assert_eq!(cell.cell_type, CellType::Neuron);
        assert_eq!(cell.state, CellState::Healthy);
        assert!(cell.diameter_um > 0.0);
    }

    #[test]
    fn test_cell_volume() {
        let cell = Cell::new(CellType::Neuron);
        let volume = cell.volume_um3();
        assert!(volume > 0.0);
    }

    #[test]
    fn test_cell_health() {
        let cell = Cell::new(CellType::Neuron);
        assert!(cell.is_healthy());
    }

    #[test]
    fn test_energy_state() {
        let cell = Cell::new(CellType::Neuron);
        let energy = cell.assess_energy_state().unwrap();
        assert!(energy > 0.0);
    }

    #[test]
    fn test_apoptosis() {
        let mut cell = Cell::new(CellType::Neuron);
        cell.undergo_apoptosis().unwrap();
        assert_eq!(cell.state, CellState::Apoptotic);
        assert_eq!(cell.membrane_potential_mv, 0.0);
    }

    #[test]
    fn test_cell_cycle() {
        let mut cell = Cell::new(CellType::Neuron);
        cell.enter_cell_cycle(CellCycle::G1).unwrap();
        assert_eq!(cell.cycle, CellCycle::G1);
        assert_eq!(cell.state, CellState::Proliferating);
    }
}
