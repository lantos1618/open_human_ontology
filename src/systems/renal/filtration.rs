use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filtration {
    pub gfr_ml_per_min: f64,
    pub filtration_pressure_mmhg: f64,
    pub permeability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrineFormation {
    pub filtrate_volume_l_per_day: f64,
    pub reabsorption_volume_l_per_day: f64,
    pub secretion_volume_l_per_day: f64,
    pub final_urine_volume_l_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Electrolytes {
    pub sodium_meq_l: f64,
    pub potassium_meq_l: f64,
    pub chloride_meq_l: f64,
    pub bicarbonate_meq_l: f64,
    pub calcium_mg_dl: f64,
    pub phosphate_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reabsorption {
    pub glucose_reabsorption_percent: f64,
    pub amino_acid_reabsorption_percent: f64,
    pub water_reabsorption_percent: f64,
    pub sodium_reabsorption_percent: f64,
}

impl Filtration {
    pub fn new_normal() -> Self {
        Self {
            gfr_ml_per_min: 125.0,
            filtration_pressure_mmhg: 10.0,
            permeability: 1.0,
        }
    }

    pub fn estimated_gfr_ml_per_min_per_1_73m2(&self, age: u32, is_female: bool) -> f64 {
        let serum_creatinine: f64 = 1.0;
        let mut gfr = 175.0 * serum_creatinine.powf(-1.154) * (age as f64).powf(-0.203);

        if is_female {
            gfr *= 0.742;
        }

        gfr
    }

    pub fn daily_filtrate_volume_liters(&self) -> f64 {
        self.gfr_ml_per_min * 60.0 * 24.0 / 1000.0
    }

    pub fn is_impaired(&self) -> bool {
        self.gfr_ml_per_min < 60.0
    }
}

impl UrineFormation {
    pub fn new_normal() -> Self {
        Self {
            filtrate_volume_l_per_day: 180.0,
            reabsorption_volume_l_per_day: 178.5,
            secretion_volume_l_per_day: 0.0,
            final_urine_volume_l_per_day: 1.5,
        }
    }

    pub fn reabsorption_efficiency(&self) -> f64 {
        (self.reabsorption_volume_l_per_day / self.filtrate_volume_l_per_day) * 100.0
    }

    pub fn concentration_factor(&self) -> f64 {
        self.filtrate_volume_l_per_day / self.final_urine_volume_l_per_day
    }

    pub fn verify_balance(&self) -> bool {
        let calculated_urine =
            self.filtrate_volume_l_per_day - self.reabsorption_volume_l_per_day + self.secretion_volume_l_per_day;
        (calculated_urine - self.final_urine_volume_l_per_day).abs() < 0.1
    }
}

impl Electrolytes {
    pub fn new_plasma() -> Self {
        Self {
            sodium_meq_l: 140.0,
            potassium_meq_l: 4.0,
            chloride_meq_l: 105.0,
            bicarbonate_meq_l: 24.0,
            calcium_mg_dl: 10.0,
            phosphate_mg_dl: 3.5,
        }
    }

    pub fn new_urine() -> Self {
        Self {
            sodium_meq_l: 50.0,
            potassium_meq_l: 25.0,
            chloride_meq_l: 50.0,
            bicarbonate_meq_l: 0.0,
            calcium_mg_dl: 15.0,
            phosphate_mg_dl: 40.0,
        }
    }

    pub fn anion_gap(&self) -> f64 {
        self.sodium_meq_l - (self.chloride_meq_l + self.bicarbonate_meq_l)
    }

    pub fn osmolality(&self) -> f64 {
        2.0 * self.sodium_meq_l + (100.0 / 18.0) + (20.0 / 2.8)
    }
}

impl Reabsorption {
    pub fn new_normal() -> Self {
        Self {
            glucose_reabsorption_percent: 100.0,
            amino_acid_reabsorption_percent: 100.0,
            water_reabsorption_percent: 99.0,
            sodium_reabsorption_percent: 99.5,
        }
    }

    pub fn calculate_excreted_amount(&self, filtered_amount: f64, substance: &str) -> f64 {
        let reabsorption_rate = match substance {
            "glucose" => self.glucose_reabsorption_percent,
            "amino_acid" => self.amino_acid_reabsorption_percent,
            "water" => self.water_reabsorption_percent,
            "sodium" => self.sodium_reabsorption_percent,
            _ => 0.0,
        };

        filtered_amount * (1.0 - reabsorption_rate / 100.0)
    }

    pub fn is_glucosuria(&self) -> bool {
        self.glucose_reabsorption_percent < 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filtration() {
        let filtration = Filtration::new_normal();
        assert_eq!(filtration.gfr_ml_per_min, 125.0);
        assert!(!filtration.is_impaired());

        let daily_volume = filtration.daily_filtrate_volume_liters();
        assert!(daily_volume > 170.0);
    }

    #[test]
    fn test_estimated_gfr() {
        let filtration = Filtration::new_normal();
        let gfr_male = filtration.estimated_gfr_ml_per_min_per_1_73m2(40, false);
        let gfr_female = filtration.estimated_gfr_ml_per_min_per_1_73m2(40, true);

        assert!(gfr_male > gfr_female);
    }

    #[test]
    fn test_urine_formation() {
        let urine = UrineFormation::new_normal();
        assert!(urine.verify_balance());

        let efficiency = urine.reabsorption_efficiency();
        assert!(efficiency > 99.0);
    }

    #[test]
    fn test_concentration_factor() {
        let urine = UrineFormation::new_normal();
        let factor = urine.concentration_factor();
        assert!(factor > 100.0);
    }

    #[test]
    fn test_electrolytes() {
        let plasma = Electrolytes::new_plasma();
        let urine = Electrolytes::new_urine();

        let anion_gap = plasma.anion_gap();
        assert!(anion_gap > 8.0 && anion_gap < 16.0);

        assert!(plasma.sodium_meq_l > urine.sodium_meq_l);
        assert!(urine.potassium_meq_l > plasma.potassium_meq_l);
    }

    #[test]
    fn test_osmolality() {
        let plasma = Electrolytes::new_plasma();
        let osm = plasma.osmolality();
        assert!(osm > 280.0 && osm < 295.0);
    }

    #[test]
    fn test_reabsorption() {
        let reabs = Reabsorption::new_normal();
        assert!(!reabs.is_glucosuria());

        let excreted_glucose = reabs.calculate_excreted_amount(180.0, "glucose");
        assert_eq!(excreted_glucose, 0.0);

        let excreted_water = reabs.calculate_excreted_amount(180.0, "water");
        assert!(excreted_water < 2.0);
    }
}
