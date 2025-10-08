use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleEnergyMetabolism {
    pub atp_concentration_mm: f64,
    pub creatine_phosphate_mm: f64,
    pub glycogen_g_per_100g: f64,
    pub lactate_mm: f64,
    pub oxygen_consumption_ml_min: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergySystem {
    PhosphateSystem,
    GlycolyticSystem,
    OxidativeSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialFunction {
    pub mitochondrial_density: f64,
    pub oxygen_consumption_rate: f64,
    pub atp_production_rate: f64,
    pub respiratory_quotient: f64,
}

impl MuscleEnergyMetabolism {
    pub fn new_resting() -> Self {
        Self {
            atp_concentration_mm: 5.0,
            creatine_phosphate_mm: 25.0,
            glycogen_g_per_100g: 1.5,
            lactate_mm: 1.0,
            oxygen_consumption_ml_min: 3.5,
        }
    }

    pub fn new_exercising() -> Self {
        Self {
            atp_concentration_mm: 3.0,
            creatine_phosphate_mm: 10.0,
            glycogen_g_per_100g: 0.8,
            lactate_mm: 10.0,
            oxygen_consumption_ml_min: 50.0,
        }
    }

    pub fn atp_turnover_rate(&self) -> f64 {
        self.oxygen_consumption_ml_min * 6.3
    }

    pub fn energy_deficit(&self) -> bool {
        self.atp_concentration_mm < 2.0 || self.creatine_phosphate_mm < 5.0
    }

    pub fn glycogen_depleted(&self) -> bool {
        self.glycogen_g_per_100g < 0.3
    }

    pub fn lactate_threshold_exceeded(&self) -> bool {
        self.lactate_mm > 4.0
    }

    pub fn dominant_energy_system(&self) -> EnergySystem {
        if self.lactate_mm > 8.0 {
            EnergySystem::GlycolyticSystem
        } else if self.oxygen_consumption_ml_min > 30.0 {
            EnergySystem::OxidativeSystem
        } else {
            EnergySystem::PhosphateSystem
        }
    }

    pub fn estimated_fatigue_level(&self) -> f64 {
        let atp_factor = (5.0 - self.atp_concentration_mm) / 5.0;
        let pcr_factor = (25.0 - self.creatine_phosphate_mm) / 25.0;
        let glycogen_factor = (1.5 - self.glycogen_g_per_100g) / 1.5;
        let lactate_factor = (self.lactate_mm - 1.0) / 15.0;

        ((atp_factor + pcr_factor + glycogen_factor + lactate_factor) / 4.0).clamp(0.0, 1.0)
    }
}

impl MitochondrialFunction {
    pub fn new_type_i_fiber() -> Self {
        Self {
            mitochondrial_density: 0.15,
            oxygen_consumption_rate: 5.0,
            atp_production_rate: 35.0,
            respiratory_quotient: 0.7,
        }
    }

    pub fn new_type_ii_fiber() -> Self {
        Self {
            mitochondrial_density: 0.05,
            oxygen_consumption_rate: 2.0,
            atp_production_rate: 15.0,
            respiratory_quotient: 1.0,
        }
    }

    pub fn oxidative_capacity(&self) -> f64 {
        self.mitochondrial_density * self.oxygen_consumption_rate
    }

    pub fn primarily_fat_oxidation(&self) -> bool {
        self.respiratory_quotient < 0.85
    }

    pub fn primarily_carbohydrate_oxidation(&self) -> bool {
        self.respiratory_quotient > 0.95
    }

    pub fn atp_efficiency(&self) -> f64 {
        self.atp_production_rate / self.oxygen_consumption_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resting_metabolism() {
        let metabolism = MuscleEnergyMetabolism::new_resting();
        assert!(!metabolism.energy_deficit());
        assert!(!metabolism.glycogen_depleted());
    }

    #[test]
    fn test_exercising_metabolism() {
        let metabolism = MuscleEnergyMetabolism::new_exercising();
        assert!(metabolism.lactate_threshold_exceeded());
        assert_eq!(metabolism.dominant_energy_system(), EnergySystem::GlycolyticSystem);
    }

    #[test]
    fn test_fatigue_estimation() {
        let resting = MuscleEnergyMetabolism::new_resting();
        let exercising = MuscleEnergyMetabolism::new_exercising();

        assert!(exercising.estimated_fatigue_level() > resting.estimated_fatigue_level());
    }

    #[test]
    fn test_mitochondrial_function() {
        let type_i = MitochondrialFunction::new_type_i_fiber();
        let type_ii = MitochondrialFunction::new_type_ii_fiber();

        assert!(type_i.oxidative_capacity() > type_ii.oxidative_capacity());
        assert!(type_i.primarily_fat_oxidation());
        assert!(type_ii.primarily_carbohydrate_oxidation());
    }

    #[test]
    fn test_atp_efficiency() {
        let mito = MitochondrialFunction::new_type_i_fiber();
        let efficiency = mito.atp_efficiency();
        assert!(efficiency > 0.0);
    }
}
