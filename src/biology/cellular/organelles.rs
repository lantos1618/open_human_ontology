use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Organelle {
    Mitochondrion(Mitochondrion),
    Ribosome(Ribosome),
    EndoplasmicReticulum(EndoplasmicReticulum),
    GolgiApparatus(GolgiApparatus),
    Lysosome(Lysosome),
    Nucleus(Nucleus),
}

impl Organelle {
    pub fn name(&self) -> &str {
        match self {
            Organelle::Mitochondrion(_) => "Mitochondrion",
            Organelle::Ribosome(_) => "Ribosome",
            Organelle::EndoplasmicReticulum(_) => "EndoplasmicReticulum",
            Organelle::GolgiApparatus(_) => "GolgiApparatus",
            Organelle::Lysosome(_) => "Lysosome",
            Organelle::Nucleus(_) => "Nucleus",
        }
    }

    pub fn volume_um3(&self) -> f64 {
        match self {
            Organelle::Mitochondrion(m) => m.volume_um3(),
            Organelle::Ribosome(r) => r.volume_nm3,
            Organelle::EndoplasmicReticulum(er) => er.volume_um3,
            Organelle::GolgiApparatus(g) => g.volume_um3,
            Organelle::Lysosome(l) => l.volume_um3(),
            Organelle::Nucleus(n) => n.volume_um3(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mitochondrion {
    pub length_um: f64,
    pub diameter_um: f64,
    pub membrane_potential_mv: f64,
    pub atp_production_rate: f64,
    pub cristae_density: f64,
}

impl Mitochondrion {
    pub fn new() -> Self {
        Mitochondrion {
            length_um: 2.0,
            diameter_um: 0.5,
            membrane_potential_mv: -180.0,
            atp_production_rate: 1000.0,
            cristae_density: 0.7,
        }
    }

    pub fn volume_um3(&self) -> f64 {
        std::f64::consts::PI * (self.diameter_um / 2.0).powi(2) * self.length_um
    }

    pub fn assess_health(&self) -> f64 {
        let potential_score = (self.membrane_potential_mv.abs() / 180.0).min(1.0);
        let cristae_score = self.cristae_density;
        (potential_score + cristae_score) / 2.0
    }
}

impl Default for Mitochondrion {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ribosome {
    pub subunit_type: RibosomeType,
    pub volume_nm3: f64,
    pub translation_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RibosomeType {
    Free,
    Bound,
}

impl Ribosome {
    pub fn new(subunit_type: RibosomeType) -> Self {
        Ribosome {
            subunit_type,
            volume_nm3: 2500.0,
            translation_rate: 20.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndoplasmicReticulum {
    pub er_type: ERType,
    pub volume_um3: f64,
    pub protein_processing_capacity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ERType {
    Rough,
    Smooth,
}

impl EndoplasmicReticulum {
    pub fn new(er_type: ERType) -> Self {
        let (volume, capacity) = match er_type {
            ERType::Rough => (50.0, 1000.0),
            ERType::Smooth => (30.0, 500.0),
        };

        EndoplasmicReticulum {
            er_type,
            volume_um3: volume,
            protein_processing_capacity: capacity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GolgiApparatus {
    pub volume_um3: f64,
    pub cisternae_count: usize,
    pub vesicle_transport_rate: f64,
}

impl GolgiApparatus {
    pub fn new() -> Self {
        GolgiApparatus {
            volume_um3: 20.0,
            cisternae_count: 6,
            vesicle_transport_rate: 100.0,
        }
    }
}

impl Default for GolgiApparatus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lysosome {
    pub diameter_um: f64,
    pub ph: f64,
    pub enzyme_count: usize,
}

impl Lysosome {
    pub fn new() -> Self {
        Lysosome {
            diameter_um: 0.5,
            ph: 4.5,
            enzyme_count: 50,
        }
    }

    pub fn volume_um3(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * (self.diameter_um / 2.0).powi(3)
    }

    pub fn is_functional(&self) -> bool {
        self.ph >= 4.0 && self.ph <= 5.5
    }
}

impl Default for Lysosome {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nucleus {
    pub diameter_um: f64,
    pub chromatin_condensation: f64,
    pub nucleolus_count: usize,
    pub transcription_rate: f64,
}

impl Nucleus {
    pub fn new(cell_diameter: f64) -> Self {
        Nucleus {
            diameter_um: cell_diameter * 0.3,
            chromatin_condensation: 0.3,
            nucleolus_count: 1,
            transcription_rate: 500.0,
        }
    }

    pub fn volume_um3(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * (self.diameter_um / 2.0).powi(3)
    }

    pub fn nucleus_to_cytoplasm_ratio(&self, cell_volume: f64) -> f64 {
        self.volume_um3() / (cell_volume - self.volume_um3())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mitochondrion_creation() {
        let mito = Mitochondrion::new();
        assert!(mito.volume_um3() > 0.0);
        assert!(mito.assess_health() > 0.0);
    }

    #[test]
    fn test_ribosome_types() {
        let free = Ribosome::new(RibosomeType::Free);
        let bound = Ribosome::new(RibosomeType::Bound);
        assert_eq!(free.subunit_type, RibosomeType::Free);
        assert_eq!(bound.subunit_type, RibosomeType::Bound);
    }

    #[test]
    fn test_lysosome_ph() {
        let lyso = Lysosome::new();
        assert!(lyso.is_functional());
        assert_eq!(lyso.ph, 4.5);
    }

    #[test]
    fn test_nucleus_ratio() {
        let nucleus = Nucleus::new(20.0);
        let ratio = nucleus.nucleus_to_cytoplasm_ratio(4000.0);
        assert!(ratio > 0.0);
    }
}
