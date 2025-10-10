use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellArrangement {
    Simple,
    Stratified,
    Pseudostratified,
    Columnar,
    Cuboidal,
    Squamous,
    Random,
    Organized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueOrganization {
    pub arrangement: CellArrangement,
    pub layers: usize,
    pub cell_spacing_um: f64,
    pub orientation: Vector3<f64>,
    pub polarity: f64,
}

impl TissueOrganization {
    pub fn new(arrangement: CellArrangement) -> Self {
        let (layers, spacing) = match arrangement {
            CellArrangement::Simple => (1, 10.0),
            CellArrangement::Stratified => (5, 8.0),
            CellArrangement::Pseudostratified => (1, 7.0),
            CellArrangement::Columnar => (1, 5.0),
            CellArrangement::Cuboidal => (1, 10.0),
            CellArrangement::Squamous => (1, 15.0),
            CellArrangement::Random => (1, 20.0),
            CellArrangement::Organized => (1, 10.0),
        };

        TissueOrganization {
            arrangement,
            layers,
            cell_spacing_um: spacing,
            orientation: Vector3::new(1.0, 0.0, 0.0),
            polarity: 0.0,
        }
    }

    pub fn epithelial_simple_squamous() -> Self {
        TissueOrganization {
            arrangement: CellArrangement::Simple,
            layers: 1,
            cell_spacing_um: 15.0,
            orientation: Vector3::new(0.0, 0.0, 1.0),
            polarity: 1.0,
        }
    }

    pub fn epithelial_stratified_squamous() -> Self {
        TissueOrganization {
            arrangement: CellArrangement::Stratified,
            layers: 10,
            cell_spacing_um: 12.0,
            orientation: Vector3::new(0.0, 0.0, 1.0),
            polarity: 0.8,
        }
    }

    pub fn epithelial_simple_columnar() -> Self {
        TissueOrganization {
            arrangement: CellArrangement::Columnar,
            layers: 1,
            cell_spacing_um: 5.0,
            orientation: Vector3::new(0.0, 0.0, 1.0),
            polarity: 1.0,
        }
    }

    pub fn muscle_striated() -> Self {
        TissueOrganization {
            arrangement: CellArrangement::Organized,
            layers: 100,
            cell_spacing_um: 2.0,
            orientation: Vector3::new(1.0, 0.0, 0.0),
            polarity: 0.9,
        }
    }

    pub fn connective_loose() -> Self {
        TissueOrganization {
            arrangement: CellArrangement::Random,
            layers: 1,
            cell_spacing_um: 50.0,
            orientation: Vector3::new(0.0, 0.0, 0.0),
            polarity: 0.0,
        }
    }

    pub fn total_thickness_um(&self) -> f64 {
        self.layers as f64 * self.cell_spacing_um
    }

    pub fn is_polarized(&self) -> bool {
        self.polarity > 0.5
    }

    pub fn has_directionality(&self) -> bool {
        self.orientation.norm() > 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_creation() {
        let org = TissueOrganization::new(CellArrangement::Simple);
        assert_eq!(org.arrangement, CellArrangement::Simple);
        assert_eq!(org.layers, 1);
    }

    #[test]
    fn test_epithelial_organizations() {
        let simple = TissueOrganization::epithelial_simple_squamous();
        assert_eq!(simple.layers, 1);
        assert!(simple.is_polarized());

        let stratified = TissueOrganization::epithelial_stratified_squamous();
        assert!(stratified.layers > 1);
    }

    #[test]
    fn test_muscle_organization() {
        let muscle = TissueOrganization::muscle_striated();
        assert!(muscle.is_polarized());
        assert!(muscle.has_directionality());
    }

    #[test]
    fn test_connective_organization() {
        let connective = TissueOrganization::connective_loose();
        assert!(!connective.is_polarized());
        assert_eq!(connective.arrangement, CellArrangement::Random);
    }

    #[test]
    fn test_thickness_calculation() {
        let org = TissueOrganization::new(CellArrangement::Stratified);
        let thickness = org.total_thickness_um();
        assert!(thickness > 0.0);
    }
}
