use crate::biology::cellular::{Cell, CellType};
use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemodelingPhase {
    Activation,
    Resorption,
    Reversal,
    Formation,
    Mineralization,
    Quiescence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneRemodeling {
    pub phase: RemodelingPhase,
    pub osteoclasts: Vec<Cell>,
    pub osteoblasts: Vec<Cell>,
    pub osteocytes: Vec<Cell>,
    pub resorption_depth_um: f64,
    pub formation_rate_um_per_day: f64,
    pub mineralization_lag_time_days: f64,
    pub cycle_duration_days: f64,
}

impl BoneRemodeling {
    pub fn new() -> Self {
        BoneRemodeling {
            phase: RemodelingPhase::Quiescence,
            osteoclasts: Vec::new(),
            osteoblasts: Vec::new(),
            osteocytes: Vec::new(),
            resorption_depth_um: 0.0,
            formation_rate_um_per_day: 1.0,
            mineralization_lag_time_days: 10.0,
            cycle_duration_days: 120.0,
        }
    }

    pub fn activate(&mut self) -> BiologyResult<()> {
        if self.phase != RemodelingPhase::Quiescence {
            return Err(BiologyError::InvalidState(
                "Can only activate from quiescence".to_string(),
            ));
        }

        self.phase = RemodelingPhase::Activation;
        self.recruit_osteoclasts(3)?;
        Ok(())
    }

    pub fn recruit_osteoclasts(&mut self, count: usize) -> BiologyResult<()> {
        for _ in 0..count {
            self.osteoclasts.push(Cell::new(CellType::Osteoclast));
        }
        Ok(())
    }

    pub fn recruit_osteoblasts(&mut self, count: usize) -> BiologyResult<()> {
        for _ in 0..count {
            self.osteoblasts.push(Cell::new(CellType::Osteoblast));
        }
        Ok(())
    }

    pub fn resorb_bone(&mut self, duration_days: f64) -> BiologyResult<f64> {
        if self.phase != RemodelingPhase::Activation && self.phase != RemodelingPhase::Resorption {
            return Err(BiologyError::InvalidState(
                "Not in resorption phase".to_string(),
            ));
        }

        self.phase = RemodelingPhase::Resorption;

        let resorption_rate_um_per_day = 20.0;
        let active_osteoclasts = self.osteoclasts.len();

        self.resorption_depth_um +=
            resorption_rate_um_per_day * duration_days * active_osteoclasts as f64;

        Ok(self.resorption_depth_um)
    }

    pub fn form_bone(&mut self, duration_days: f64) -> BiologyResult<f64> {
        if self.phase != RemodelingPhase::Reversal && self.phase != RemodelingPhase::Formation {
            return Err(BiologyError::InvalidState(
                "Not in formation phase".to_string(),
            ));
        }

        self.phase = RemodelingPhase::Formation;

        if self.osteoblasts.is_empty() {
            self.recruit_osteoblasts(10)?;
        }

        let active_osteoblasts = self.osteoblasts.len();
        let formation = self.formation_rate_um_per_day * duration_days * active_osteoblasts as f64;

        Ok(formation)
    }

    pub fn advance_phase(&mut self) -> BiologyResult<RemodelingPhase> {
        self.phase = match self.phase {
            RemodelingPhase::Quiescence => RemodelingPhase::Activation,
            RemodelingPhase::Activation => {
                if self.osteoclasts.is_empty() {
                    return Err(BiologyError::InvalidState(
                        "Need osteoclasts before resorption".to_string(),
                    ));
                }
                RemodelingPhase::Resorption
            }
            RemodelingPhase::Resorption => RemodelingPhase::Reversal,
            RemodelingPhase::Reversal => RemodelingPhase::Formation,
            RemodelingPhase::Formation => RemodelingPhase::Mineralization,
            RemodelingPhase::Mineralization => RemodelingPhase::Quiescence,
        };

        Ok(self.phase)
    }

    pub fn is_balanced(&self) -> bool {
        let resorption = self.resorption_depth_um;
        let formation_capacity = self.formation_rate_um_per_day
            * (self.cycle_duration_days - self.mineralization_lag_time_days)
            * self.osteoblasts.len() as f64;

        (resorption - formation_capacity).abs() < 10.0
    }

    pub fn coupling_ratio(&self) -> f64 {
        if self.osteoclasts.is_empty() {
            return 0.0;
        }

        self.osteoblasts.len() as f64 / self.osteoclasts.len() as f64
    }
}

impl Default for BoneRemodeling {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remodeling_creation() {
        let remodeling = BoneRemodeling::new();
        assert_eq!(remodeling.phase, RemodelingPhase::Quiescence);
    }

    #[test]
    fn test_activation() {
        let mut remodeling = BoneRemodeling::new();
        remodeling.activate().unwrap();
        assert_eq!(remodeling.phase, RemodelingPhase::Activation);
        assert_eq!(remodeling.osteoclasts.len(), 3);
    }

    #[test]
    fn test_resorption() {
        let mut remodeling = BoneRemodeling::new();
        remodeling.activate().unwrap();
        let depth = remodeling.resorb_bone(1.0).unwrap();
        assert!(depth > 0.0);
    }

    #[test]
    fn test_formation() {
        let mut remodeling = BoneRemodeling::new();
        remodeling.phase = RemodelingPhase::Reversal;
        let formation = remodeling.form_bone(1.0).unwrap();
        assert!(formation > 0.0);
    }

    #[test]
    fn test_phase_advancement() {
        let mut remodeling = BoneRemodeling::new();
        remodeling.advance_phase().unwrap();
        assert_eq!(remodeling.phase, RemodelingPhase::Activation);
    }

    #[test]
    fn test_coupling_ratio() {
        let mut remodeling = BoneRemodeling::new();
        remodeling.recruit_osteoclasts(5).unwrap();
        remodeling.recruit_osteoblasts(10).unwrap();
        assert_eq!(remodeling.coupling_ratio(), 2.0);
    }
}
