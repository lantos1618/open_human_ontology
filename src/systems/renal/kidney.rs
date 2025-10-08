use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kidney {
    pub side: KidneySide,
    pub weight_g: f64,
    pub length_cm: f64,
    pub nephrons: Vec<Nephron>,
    pub blood_flow_ml_per_min: f64,
    pub gfr_ml_per_min: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KidneySide {
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nephron {
    pub nephron_type: NephronType,
    pub glomerulus: Glomerulus,
    pub tubule: RenalTubule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NephronType {
    Cortical,
    Juxtamedullary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glomerulus {
    pub capillary_surface_area_um2: f64,
    pub filtration_coefficient: f64,
    pub hydrostatic_pressure_mmhg: f64,
    pub oncotic_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalTubule {
    pub proximal: TubuleSegment,
    pub loop_of_henle: LoopOfHenle,
    pub distal: TubuleSegment,
    pub collecting_duct: TubuleSegment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TubuleSegment {
    pub length_mm: f64,
    pub reabsorption_rate: f64,
    pub secretion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopOfHenle {
    pub descending_limb: TubuleSegment,
    pub ascending_limb: TubuleSegment,
    pub countercurrent_multiplier: f64,
}

impl Kidney {
    pub fn new_left() -> Self {
        Self {
            side: KidneySide::Left,
            weight_g: 150.0,
            length_cm: 12.0,
            nephrons: (0..1_000_000)
                .map(|i| {
                    if i % 10 == 0 {
                        Nephron::new_juxtamedullary()
                    } else {
                        Nephron::new_cortical()
                    }
                })
                .collect(),
            blood_flow_ml_per_min: 600.0,
            gfr_ml_per_min: 60.0,
        }
    }

    pub fn new_right() -> Self {
        Self {
            side: KidneySide::Right,
            weight_g: 145.0,
            length_cm: 12.0,
            nephrons: (0..1_000_000)
                .map(|i| {
                    if i % 10 == 0 {
                        Nephron::new_juxtamedullary()
                    } else {
                        Nephron::new_cortical()
                    }
                })
                .collect(),
            blood_flow_ml_per_min: 600.0,
            gfr_ml_per_min: 60.0,
        }
    }

    pub fn filtration_fraction(&self) -> f64 {
        self.gfr_ml_per_min / self.blood_flow_ml_per_min
    }

    pub fn renal_plasma_flow(&self) -> f64 {
        self.blood_flow_ml_per_min * 0.55
    }

    pub fn calculate_clearance(&self, plasma_conc: f64, urine_conc: f64, urine_flow: f64) -> f64 {
        (urine_conc * urine_flow) / plasma_conc
    }
}

impl Nephron {
    pub fn new_cortical() -> Self {
        Self {
            nephron_type: NephronType::Cortical,
            glomerulus: Glomerulus::new(),
            tubule: RenalTubule::new_cortical(),
        }
    }

    pub fn new_juxtamedullary() -> Self {
        Self {
            nephron_type: NephronType::Juxtamedullary,
            glomerulus: Glomerulus::new(),
            tubule: RenalTubule::new_juxtamedullary(),
        }
    }

    pub fn filtration_rate(&self) -> f64 {
        self.glomerulus.net_filtration_pressure() * self.glomerulus.filtration_coefficient
    }

    pub fn reabsorption_efficiency(&self) -> f64 {
        let proximal_reabs = self.tubule.proximal.reabsorption_rate;
        let loop_reabs = (self.tubule.loop_of_henle.descending_limb.reabsorption_rate
            + self.tubule.loop_of_henle.ascending_limb.reabsorption_rate)
            / 2.0;
        let distal_reabs = self.tubule.distal.reabsorption_rate;

        (proximal_reabs + loop_reabs + distal_reabs) / 3.0
    }
}

impl Glomerulus {
    pub fn new() -> Self {
        Self {
            capillary_surface_area_um2: 10_000.0,
            filtration_coefficient: 0.5,
            hydrostatic_pressure_mmhg: 55.0,
            oncotic_pressure_mmhg: 30.0,
        }
    }

    pub fn net_filtration_pressure(&self) -> f64 {
        let capsule_pressure = 15.0;
        self.hydrostatic_pressure_mmhg - self.oncotic_pressure_mmhg - capsule_pressure
    }

    pub fn filtration_rate_nl_per_min(&self) -> f64 {
        self.net_filtration_pressure() * self.filtration_coefficient
    }
}

impl Default for Glomerulus {
    fn default() -> Self {
        Self::new()
    }
}

impl RenalTubule {
    pub fn new_cortical() -> Self {
        Self {
            proximal: TubuleSegment::new_proximal(),
            loop_of_henle: LoopOfHenle::new_short(),
            distal: TubuleSegment::new_distal(),
            collecting_duct: TubuleSegment::new_collecting(),
        }
    }

    pub fn new_juxtamedullary() -> Self {
        Self {
            proximal: TubuleSegment::new_proximal(),
            loop_of_henle: LoopOfHenle::new_long(),
            distal: TubuleSegment::new_distal(),
            collecting_duct: TubuleSegment::new_collecting(),
        }
    }

    pub fn total_length_mm(&self) -> f64 {
        self.proximal.length_mm
            + self.loop_of_henle.descending_limb.length_mm
            + self.loop_of_henle.ascending_limb.length_mm
            + self.distal.length_mm
            + self.collecting_duct.length_mm
    }
}

impl TubuleSegment {
    pub fn new_proximal() -> Self {
        Self {
            length_mm: 14.0,
            reabsorption_rate: 0.65,
            secretion_rate: 0.1,
        }
    }

    pub fn new_distal() -> Self {
        Self {
            length_mm: 5.0,
            reabsorption_rate: 0.15,
            secretion_rate: 0.05,
        }
    }

    pub fn new_collecting() -> Self {
        Self {
            length_mm: 20.0,
            reabsorption_rate: 0.10,
            secretion_rate: 0.02,
        }
    }

    pub fn net_transport(&self) -> f64 {
        self.reabsorption_rate - self.secretion_rate
    }
}

impl LoopOfHenle {
    pub fn new_short() -> Self {
        Self {
            descending_limb: TubuleSegment {
                length_mm: 2.0,
                reabsorption_rate: 0.15,
                secretion_rate: 0.0,
            },
            ascending_limb: TubuleSegment {
                length_mm: 2.0,
                reabsorption_rate: 0.25,
                secretion_rate: 0.0,
            },
            countercurrent_multiplier: 200.0,
        }
    }

    pub fn new_long() -> Self {
        Self {
            descending_limb: TubuleSegment {
                length_mm: 10.0,
                reabsorption_rate: 0.20,
                secretion_rate: 0.0,
            },
            ascending_limb: TubuleSegment {
                length_mm: 10.0,
                reabsorption_rate: 0.30,
                secretion_rate: 0.0,
            },
            countercurrent_multiplier: 600.0,
        }
    }

    pub fn concentration_gradient(&self) -> f64 {
        self.countercurrent_multiplier * 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kidney_creation() {
        let left = Kidney::new_left();
        let right = Kidney::new_right();

        assert_eq!(left.side, KidneySide::Left);
        assert_eq!(right.side, KidneySide::Right);
        assert_eq!(left.nephrons.len(), 1_000_000);
    }

    #[test]
    fn test_filtration_fraction() {
        let kidney = Kidney::new_left();
        let ff = kidney.filtration_fraction();

        assert!(ff > 0.0);
        assert!(ff < 0.3);
    }

    #[test]
    fn test_nephron_types() {
        let cortical = Nephron::new_cortical();
        let juxta = Nephron::new_juxtamedullary();

        assert_eq!(cortical.nephron_type, NephronType::Cortical);
        assert_eq!(juxta.nephron_type, NephronType::Juxtamedullary);
    }

    #[test]
    fn test_glomerulus_filtration() {
        let glom = Glomerulus::new();
        let nfp = glom.net_filtration_pressure();

        assert!(nfp > 0.0);
        assert!(nfp < 20.0);
    }

    #[test]
    fn test_tubule_reabsorption() {
        let proximal = TubuleSegment::new_proximal();
        assert!(proximal.reabsorption_rate > proximal.secretion_rate);
    }

    #[test]
    fn test_loop_of_henle() {
        let short_loop = LoopOfHenle::new_short();
        let long_loop = LoopOfHenle::new_long();

        assert!(long_loop.concentration_gradient() > short_loop.concentration_gradient());
    }

    #[test]
    fn test_clearance_calculation() {
        let kidney = Kidney::new_left();
        let clearance = kidney.calculate_clearance(1.0, 100.0, 1.0);
        assert_eq!(clearance, 100.0);
    }
}
