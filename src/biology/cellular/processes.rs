use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellularProcess {
    EnergyProduction(EnergyProduction),
    ProteinSynthesis(ProteinSynthesis),
    CellDivision(CellDivision),
    Autophagy(Autophagy),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProduction {
    pub pathway: EnergyPathway,
    pub glucose_consumed_mm: f64,
    pub oxygen_consumed_mm: f64,
    pub atp_produced: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyPathway {
    Glycolysis,
    CitricAcidCycle,
    OxidativePhosphorylation,
    FattyAcidOxidation,
}

impl EnergyProduction {
    pub fn aerobic_respiration(glucose_mm: f64) -> BiologyResult<Self> {
        if glucose_mm < 0.0 {
            return Err(BiologyError::InvalidValue(
                "Glucose concentration cannot be negative".to_string()
            ));
        }

        Ok(EnergyProduction {
            pathway: EnergyPathway::OxidativePhosphorylation,
            glucose_consumed_mm: glucose_mm,
            oxygen_consumed_mm: glucose_mm * 6.0,
            atp_produced: glucose_mm * 30.0,
        })
    }

    pub fn anaerobic_glycolysis(glucose_mm: f64) -> BiologyResult<Self> {
        if glucose_mm < 0.0 {
            return Err(BiologyError::InvalidValue(
                "Glucose concentration cannot be negative".to_string()
            ));
        }

        Ok(EnergyProduction {
            pathway: EnergyPathway::Glycolysis,
            glucose_consumed_mm: glucose_mm,
            oxygen_consumed_mm: 0.0,
            atp_produced: glucose_mm * 2.0,
        })
    }

    pub fn efficiency(&self) -> f64 {
        if self.glucose_consumed_mm == 0.0 {
            return 0.0;
        }
        self.atp_produced / self.glucose_consumed_mm
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinSynthesis {
    pub mrna_template: String,
    pub amino_acid_count: usize,
    pub synthesis_rate_aa_per_sec: f64,
    pub energy_cost_atp: f64,
}

impl ProteinSynthesis {
    pub fn new(mrna_length: usize) -> Self {
        let amino_acids = mrna_length / 3;
        ProteinSynthesis {
            mrna_template: "AUGCGU".repeat(amino_acids / 2),
            amino_acid_count: amino_acids,
            synthesis_rate_aa_per_sec: 20.0,
            energy_cost_atp: (amino_acids * 4) as f64,
        }
    }

    pub fn synthesis_time_seconds(&self) -> f64 {
        self.amino_acid_count as f64 / self.synthesis_rate_aa_per_sec
    }

    pub fn atp_per_second(&self) -> f64 {
        let time = self.synthesis_time_seconds();
        if time == 0.0 {
            return 0.0;
        }
        self.energy_cost_atp / time
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellDivision {
    pub phase: MitosisPhase,
    pub duration_hours: f64,
    pub chromosome_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitosisPhase {
    Prophase,
    Metaphase,
    Anaphase,
    Telophase,
    Cytokinesis,
}

impl CellDivision {
    pub fn new() -> Self {
        CellDivision {
            phase: MitosisPhase::Prophase,
            duration_hours: 1.0,
            chromosome_count: 46,
        }
    }

    pub fn advance_phase(&mut self) -> BiologyResult<()> {
        self.phase = match self.phase {
            MitosisPhase::Prophase => MitosisPhase::Metaphase,
            MitosisPhase::Metaphase => MitosisPhase::Anaphase,
            MitosisPhase::Anaphase => MitosisPhase::Telophase,
            MitosisPhase::Telophase => MitosisPhase::Cytokinesis,
            MitosisPhase::Cytokinesis => {
                return Err(BiologyError::InvalidState(
                    "Division complete".to_string()
                ));
            }
        };
        Ok(())
    }
}

impl Default for CellDivision {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Autophagy {
    pub autophagosome_count: usize,
    pub degradation_rate: f64,
    pub protein_recycled_mg: f64,
}

impl Autophagy {
    pub fn new() -> Self {
        Autophagy {
            autophagosome_count: 0,
            degradation_rate: 10.0,
            protein_recycled_mg: 0.0,
        }
    }

    pub fn initiate(&mut self, stress_level: f64) -> BiologyResult<()> {
        if !(0.0..=1.0).contains(&stress_level) {
            return Err(BiologyError::InvalidValue(
                "Stress level must be between 0 and 1".to_string()
            ));
        }

        self.autophagosome_count = (stress_level * 100.0) as usize;
        self.protein_recycled_mg = stress_level * 0.5;
        Ok(())
    }
}

impl Default for Autophagy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aerobic_respiration() {
        let energy = EnergyProduction::aerobic_respiration(1.0).unwrap();
        assert_eq!(energy.atp_produced, 30.0);
        assert!(energy.efficiency() > 0.0);
    }

    #[test]
    fn test_anaerobic_glycolysis() {
        let energy = EnergyProduction::anaerobic_glycolysis(1.0).unwrap();
        assert_eq!(energy.atp_produced, 2.0);
        assert_eq!(energy.oxygen_consumed_mm, 0.0);
    }

    #[test]
    fn test_protein_synthesis() {
        let synthesis = ProteinSynthesis::new(300);
        assert_eq!(synthesis.amino_acid_count, 100);
        assert!(synthesis.synthesis_time_seconds() > 0.0);
    }

    #[test]
    fn test_cell_division() {
        let mut division = CellDivision::new();
        assert_eq!(division.phase, MitosisPhase::Prophase);
        division.advance_phase().unwrap();
        assert_eq!(division.phase, MitosisPhase::Metaphase);
    }

    #[test]
    fn test_autophagy() {
        let mut autophagy = Autophagy::new();
        autophagy.initiate(0.5).unwrap();
        assert_eq!(autophagy.autophagosome_count, 50);
    }
}
