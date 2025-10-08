use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditorySystem {
    pub ears: [Ear; 2],
    pub hearing_threshold: HearingThreshold,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ear {
    pub side: Side,
    pub outer_ear: OuterEar,
    pub middle_ear: MiddleEar,
    pub inner_ear: InnerEar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OuterEar {
    pub canal_patency: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiddleEar {
    pub ossicles: Ossicles,
    pub tympanic_membrane_integrity: f64,
    pub eustachian_tube_function: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ossicles {
    pub malleus_mobility: f64,
    pub incus_mobility: f64,
    pub stapes_mobility: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerEar {
    pub cochlea: Cochlea,
    pub vestibular_system: VestibularSystem,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cochlea {
    pub hair_cells_outer: u32,
    pub hair_cells_inner: u32,
    pub spiral_ganglion_neurons: u32,
    pub function: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VestibularSystem {
    pub semicircular_canals: [f64; 3],
    pub otolith_organs: OtolithOrgans,
    pub balance_function: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtolithOrgans {
    pub utricle_function: f64,
    pub saccule_function: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HearingThreshold {
    pub frequencies_hz: Vec<f64>,
    pub left_ear_db: Vec<f64>,
    pub right_ear_db: Vec<f64>,
}

impl HearingThreshold {
    pub fn new() -> Self {
        let frequencies = vec![250.0, 500.0, 1000.0, 2000.0, 4000.0, 8000.0];
        let normal = vec![10.0, 10.0, 10.0, 10.0, 10.0, 15.0];

        HearingThreshold {
            frequencies_hz: frequencies,
            left_ear_db: normal.clone(),
            right_ear_db: normal,
        }
    }

    pub fn average_threshold(&self, ear_db: &[f64]) -> f64 {
        if ear_db.is_empty() {
            return 0.0;
        }
        let sum: f64 = ear_db.iter().take(4).sum();
        sum / 4.0
    }

    pub fn left_ear_average(&self) -> f64 {
        self.average_threshold(&self.left_ear_db)
    }

    pub fn right_ear_average(&self) -> f64 {
        self.average_threshold(&self.right_ear_db)
    }

    pub fn hearing_loss_degree(threshold_db: f64) -> HearingLossDegree {
        match threshold_db as u32 {
            0..=25 => HearingLossDegree::Normal,
            26..=40 => HearingLossDegree::Mild,
            41..=55 => HearingLossDegree::Moderate,
            56..=70 => HearingLossDegree::ModeratelySevere,
            71..=90 => HearingLossDegree::Severe,
            _ => HearingLossDegree::Profound,
        }
    }

    pub fn left_hearing_loss(&self) -> HearingLossDegree {
        Self::hearing_loss_degree(self.left_ear_average())
    }

    pub fn right_hearing_loss(&self) -> HearingLossDegree {
        Self::hearing_loss_degree(self.right_ear_average())
    }
}

impl Default for HearingThreshold {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HearingLossDegree {
    Normal,
    Mild,
    Moderate,
    ModeratelySevere,
    Severe,
    Profound,
}

impl Ear {
    pub fn new(side: Side) -> Self {
        Ear {
            side,
            outer_ear: OuterEar {
                canal_patency: 1.0,
            },
            middle_ear: MiddleEar {
                ossicles: Ossicles {
                    malleus_mobility: 1.0,
                    incus_mobility: 1.0,
                    stapes_mobility: 1.0,
                },
                tympanic_membrane_integrity: 1.0,
                eustachian_tube_function: 1.0,
            },
            inner_ear: InnerEar {
                cochlea: Cochlea {
                    hair_cells_outer: 12000,
                    hair_cells_inner: 3500,
                    spiral_ganglion_neurons: 30000,
                    function: 1.0,
                },
                vestibular_system: VestibularSystem {
                    semicircular_canals: [1.0, 1.0, 1.0],
                    otolith_organs: OtolithOrgans {
                        utricle_function: 1.0,
                        saccule_function: 1.0,
                    },
                    balance_function: 1.0,
                },
            },
        }
    }

    pub fn conductive_loss_present(&self) -> bool {
        self.middle_ear.tympanic_membrane_integrity < 0.8 ||
        self.outer_ear.canal_patency < 0.8
    }

    pub fn sensorineural_loss_present(&self) -> bool {
        self.inner_ear.cochlea.function < 0.8
    }
}

impl AuditorySystem {
    pub fn new() -> Self {
        AuditorySystem {
            ears: [Ear::new(Side::Left), Ear::new(Side::Right)],
            hearing_threshold: HearingThreshold::new(),
        }
    }

    pub fn has_hearing_loss(&self) -> bool {
        !matches!(self.hearing_threshold.left_hearing_loss(), HearingLossDegree::Normal) ||
        !matches!(self.hearing_threshold.right_hearing_loss(), HearingLossDegree::Normal)
    }
}

impl Default for AuditorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hearing_threshold() {
        let threshold = HearingThreshold::new();
        assert_eq!(threshold.left_ear_average(), 10.0);
        assert_eq!(threshold.right_ear_average(), 10.0);
        assert_eq!(threshold.left_hearing_loss(), HearingLossDegree::Normal);
    }

    #[test]
    fn test_hearing_loss_degrees() {
        assert_eq!(
            HearingThreshold::hearing_loss_degree(15.0),
            HearingLossDegree::Normal
        );
        assert_eq!(
            HearingThreshold::hearing_loss_degree(35.0),
            HearingLossDegree::Mild
        );
        assert_eq!(
            HearingThreshold::hearing_loss_degree(65.0),
            HearingLossDegree::ModeratelySevere
        );
    }

    #[test]
    fn test_ear() {
        let ear = Ear::new(Side::Left);
        assert!(!ear.conductive_loss_present());
        assert!(!ear.sensorineural_loss_present());
    }

    #[test]
    fn test_auditory_system() {
        let auditory = AuditorySystem::new();
        assert!(!auditory.has_hearing_loss());
    }
}
