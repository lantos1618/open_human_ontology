use crate::biology::{BiologyError, BiologyResult};
use crate::biology::cellular::Cell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TissueType {
    Epithelial,
    Connective,
    Muscle,
    Nervous,
    Bone,
    Cartilage,
    Blood,
    Adipose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TissueState {
    Healthy,
    Inflamed,
    Damaged,
    Regenerating,
    Fibrotic,
    Necrotic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tissue {
    pub tissue_type: TissueType,
    pub state: TissueState,
    pub cells: Vec<Cell>,
    pub vascularization: f64,
    pub innervation_density: f64,
    pub thickness_um: f64,
}

impl Tissue {
    pub fn new(tissue_type: TissueType) -> Self {
        let (vascularization, innervation, thickness) = match tissue_type {
            TissueType::Epithelial => (0.3, 0.2, 50.0),
            TissueType::Connective => (0.5, 0.4, 1000.0),
            TissueType::Muscle => (0.8, 0.7, 5000.0),
            TissueType::Nervous => (0.9, 1.0, 2000.0),
            TissueType::Bone => (0.4, 0.3, 10000.0),
            TissueType::Cartilage => (0.0, 0.0, 2000.0),
            TissueType::Blood => (1.0, 0.0, 0.0),
            TissueType::Adipose => (0.6, 0.2, 5000.0),
        };

        Tissue {
            tissue_type,
            state: TissueState::Healthy,
            cells: Vec::new(),
            vascularization,
            innervation_density: innervation,
            thickness_um: thickness,
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn cell_density_per_mm3(&self) -> f64 {
        if self.thickness_um == 0.0 {
            return 0.0;
        }
        let volume_mm3 = (self.thickness_um / 1000.0).powi(3);
        self.cells.len() as f64 / volume_mm3
    }

    pub fn assess_health(&self) -> f64 {
        if self.cells.is_empty() {
            return 0.0;
        }

        let healthy_cells = self
            .cells
            .iter()
            .filter(|c| c.is_healthy())
            .count();

        healthy_cells as f64 / self.cells.len() as f64
    }

    pub fn is_avascular(&self) -> bool {
        self.vascularization < 0.1
    }

    pub fn oxygen_diffusion_limit_um(&self) -> f64 {
        if self.is_avascular() {
            100.0
        } else {
            200.0
        }
    }

    pub fn regenerate(&mut self) -> BiologyResult<()> {
        match self.state {
            TissueState::Damaged => {
                self.state = TissueState::Regenerating;
                Ok(())
            }
            TissueState::Necrotic => {
                Err(BiologyError::InvalidState(
                    "Cannot regenerate necrotic tissue".to_string()
                ))
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biology::cellular::CellType;

    #[test]
    fn test_tissue_creation() {
        let tissue = Tissue::new(TissueType::Muscle);
        assert_eq!(tissue.tissue_type, TissueType::Muscle);
        assert_eq!(tissue.state, TissueState::Healthy);
    }

    #[test]
    fn test_add_cells() {
        let mut tissue = Tissue::new(TissueType::Muscle);
        tissue.add_cell(Cell::new(CellType::Muscle));
        assert_eq!(tissue.cell_count(), 1);
    }

    #[test]
    fn test_assess_health() {
        let mut tissue = Tissue::new(TissueType::Muscle);
        tissue.add_cell(Cell::new(CellType::Muscle));
        let health = tissue.assess_health();
        assert!(health > 0.0);
    }

    #[test]
    fn test_avascular_tissue() {
        let cartilage = Tissue::new(TissueType::Cartilage);
        assert!(cartilage.is_avascular());
    }

    #[test]
    fn test_regeneration() {
        let mut tissue = Tissue::new(TissueType::Muscle);
        tissue.state = TissueState::Damaged;
        tissue.regenerate().unwrap();
        assert_eq!(tissue.state, TissueState::Regenerating);
    }
}
