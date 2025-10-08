use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hair {
    pub hair_type: HairType,
    pub follicle: HairFollicle,
    pub shaft: HairShaft,
    pub growth_rate_mm_per_day: f64,
    pub pigmentation: HairPigmentation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairType {
    Terminal,
    Vellus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairFollicle {
    pub depth_mm: f64,
    pub growth_phase: GrowthPhase,
    pub sebaceous_gland: SebaceousGland,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrowthPhase {
    Anagen,
    Catagen,
    Telogen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SebaceousGland {
    pub sebum_production_mg_per_day: f64,
    pub size_mm3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairShaft {
    pub diameter_um: f64,
    pub medulla_present: bool,
    pub cortex_thickness_um: f64,
    pub cuticle_layers: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairPigmentation {
    pub eumelanin_concentration: f64,
    pub pheomelanin_concentration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nail {
    pub nail_plate: NailPlate,
    pub nail_bed: NailBed,
    pub nail_matrix: NailMatrix,
    pub growth_rate_mm_per_month: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NailPlate {
    pub thickness_mm: f64,
    pub length_mm: f64,
    pub width_mm: f64,
    pub keratin_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NailBed {
    pub blood_supply_level: f64,
    pub nerve_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NailMatrix {
    pub cell_division_rate_per_day: f64,
    pub keratinocyte_production: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SweatGland {
    pub gland_type: SweatGlandType,
    pub secretion_rate_ml_per_hour: f64,
    pub duct_length_mm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SweatGlandType {
    Eccrine,
    Apocrine,
}

impl Hair {
    pub fn new_terminal() -> Self {
        Self {
            hair_type: HairType::Terminal,
            follicle: HairFollicle::new_terminal(),
            shaft: HairShaft::new_terminal(),
            growth_rate_mm_per_day: 0.35,
            pigmentation: HairPigmentation::new_brown(),
        }
    }

    pub fn new_vellus() -> Self {
        Self {
            hair_type: HairType::Vellus,
            follicle: HairFollicle::new_vellus(),
            shaft: HairShaft::new_vellus(),
            growth_rate_mm_per_day: 0.1,
            pigmentation: HairPigmentation::new_light(),
        }
    }

    pub fn is_actively_growing(&self) -> bool {
        matches!(self.follicle.growth_phase, GrowthPhase::Anagen)
    }

    pub fn annual_growth_cm(&self) -> f64 {
        if self.is_actively_growing() {
            self.growth_rate_mm_per_day * 365.0 / 10.0
        } else {
            0.0
        }
    }
}

impl HairFollicle {
    pub fn new_terminal() -> Self {
        Self {
            depth_mm: 4.0,
            growth_phase: GrowthPhase::Anagen,
            sebaceous_gland: SebaceousGland::new(),
        }
    }

    pub fn new_vellus() -> Self {
        Self {
            depth_mm: 1.0,
            growth_phase: GrowthPhase::Anagen,
            sebaceous_gland: SebaceousGland::new_small(),
        }
    }

    pub fn phase_duration_months(&self) -> f64 {
        match self.growth_phase {
            GrowthPhase::Anagen => 36.0,
            GrowthPhase::Catagen => 0.5,
            GrowthPhase::Telogen => 3.0,
        }
    }
}

impl SebaceousGland {
    pub fn new() -> Self {
        Self {
            sebum_production_mg_per_day: 1.0,
            size_mm3: 0.5,
        }
    }

    pub fn new_small() -> Self {
        Self {
            sebum_production_mg_per_day: 0.1,
            size_mm3: 0.1,
        }
    }
}

impl Default for SebaceousGland {
    fn default() -> Self {
        Self::new()
    }
}

impl HairShaft {
    pub fn new_terminal() -> Self {
        Self {
            diameter_um: 80.0,
            medulla_present: true,
            cortex_thickness_um: 30.0,
            cuticle_layers: 8,
        }
    }

    pub fn new_vellus() -> Self {
        Self {
            diameter_um: 30.0,
            medulla_present: false,
            cortex_thickness_um: 12.0,
            cuticle_layers: 4,
        }
    }

    pub fn cross_sectional_area_um2(&self) -> f64 {
        std::f64::consts::PI * (self.diameter_um / 2.0).powi(2)
    }
}

impl HairPigmentation {
    pub fn new_brown() -> Self {
        Self {
            eumelanin_concentration: 0.8,
            pheomelanin_concentration: 0.2,
        }
    }

    pub fn new_blonde() -> Self {
        Self {
            eumelanin_concentration: 0.2,
            pheomelanin_concentration: 0.6,
        }
    }

    pub fn new_red() -> Self {
        Self {
            eumelanin_concentration: 0.1,
            pheomelanin_concentration: 0.9,
        }
    }

    pub fn new_light() -> Self {
        Self {
            eumelanin_concentration: 0.1,
            pheomelanin_concentration: 0.1,
        }
    }

    pub fn perceived_color(&self) -> String {
        if self.pheomelanin_concentration > 0.7 {
            "Red".to_string()
        } else if self.eumelanin_concentration > 0.6 {
            "Brown/Black".to_string()
        } else if self.pheomelanin_concentration > self.eumelanin_concentration {
            "Blonde".to_string()
        } else {
            "Light Brown".to_string()
        }
    }
}

impl Nail {
    pub fn new_finger() -> Self {
        Self {
            nail_plate: NailPlate::new(12.0, 10.0),
            nail_bed: NailBed::new(),
            nail_matrix: NailMatrix::new(),
            growth_rate_mm_per_month: 3.5,
        }
    }

    pub fn new_toe() -> Self {
        Self {
            nail_plate: NailPlate::new(15.0, 8.0),
            nail_bed: NailBed::new(),
            nail_matrix: NailMatrix::new(),
            growth_rate_mm_per_month: 1.6,
        }
    }

    pub fn time_to_regrow_days(&self) -> f64 {
        (self.nail_plate.length_mm / self.growth_rate_mm_per_month) * 30.0
    }
}

impl NailPlate {
    pub fn new(length_mm: f64, width_mm: f64) -> Self {
        Self {
            thickness_mm: 0.5,
            length_mm,
            width_mm,
            keratin_density: 1.2,
        }
    }

    pub fn volume_mm3(&self) -> f64 {
        self.thickness_mm * self.length_mm * self.width_mm
    }
}

impl NailBed {
    pub fn new() -> Self {
        Self {
            blood_supply_level: 0.8,
            nerve_density: 0.7,
        }
    }
}

impl Default for NailBed {
    fn default() -> Self {
        Self::new()
    }
}

impl NailMatrix {
    pub fn new() -> Self {
        Self {
            cell_division_rate_per_day: 1000.0,
            keratinocyte_production: 500.0,
        }
    }
}

impl Default for NailMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl SweatGland {
    pub fn new_eccrine() -> Self {
        Self {
            gland_type: SweatGlandType::Eccrine,
            secretion_rate_ml_per_hour: 0.1,
            duct_length_mm: 5.0,
        }
    }

    pub fn new_apocrine() -> Self {
        Self {
            gland_type: SweatGlandType::Apocrine,
            secretion_rate_ml_per_hour: 0.01,
            duct_length_mm: 3.0,
        }
    }

    pub fn daily_secretion_ml(&self) -> f64 {
        self.secretion_rate_ml_per_hour * 24.0
    }

    pub fn thermoregulation_contribution(&self) -> f64 {
        match self.gland_type {
            SweatGlandType::Eccrine => self.secretion_rate_ml_per_hour * 10.0,
            SweatGlandType::Apocrine => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hair_terminal() {
        let hair = Hair::new_terminal();
        assert_eq!(hair.hair_type, HairType::Terminal);
        assert!(hair.is_actively_growing());
        assert!(hair.annual_growth_cm() > 10.0);
    }

    #[test]
    fn test_hair_vellus() {
        let hair = Hair::new_vellus();
        assert_eq!(hair.hair_type, HairType::Vellus);
        assert!(hair.shaft.diameter_um < 40.0);
    }

    #[test]
    fn test_growth_phases() {
        let follicle = HairFollicle::new_terminal();
        let anagen_duration = follicle.phase_duration_months();
        assert!(anagen_duration > 24.0);
    }

    #[test]
    fn test_hair_pigmentation() {
        let brown = HairPigmentation::new_brown();
        let red = HairPigmentation::new_red();

        assert!(brown.eumelanin_concentration > red.eumelanin_concentration);
        assert!(red.pheomelanin_concentration > brown.pheomelanin_concentration);
    }

    #[test]
    fn test_nail_growth() {
        let finger = Nail::new_finger();
        let toe = Nail::new_toe();

        assert!(finger.growth_rate_mm_per_month > toe.growth_rate_mm_per_month);
        assert!(finger.time_to_regrow_days() < toe.time_to_regrow_days());
    }

    #[test]
    fn test_sweat_glands() {
        let eccrine = SweatGland::new_eccrine();
        let apocrine = SweatGland::new_apocrine();

        assert!(eccrine.secretion_rate_ml_per_hour > apocrine.secretion_rate_ml_per_hour);
        assert!(eccrine.thermoregulation_contribution() > 0.0);
        assert_eq!(apocrine.thermoregulation_contribution(), 0.0);
    }
}
