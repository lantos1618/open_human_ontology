use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidBalance {
    pub total_body_water_l: f64,
    pub intracellular_fluid_l: f64,
    pub extracellular_fluid_l: f64,
    pub plasma_volume_l: f64,
    pub interstitial_fluid_l: f64,
    pub daily_intake_ml: f64,
    pub daily_output_ml: f64,
    pub insensible_losses_ml: f64,
    pub balance_status: BalanceStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceStatus {
    Balanced,
    PositiveBalance,
    NegativeBalance,
    Overload,
    Depletion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidCompartment {
    pub volume_l: f64,
    pub osmolality_mosm_kg: f64,
    pub sodium_meq_l: f64,
    pub potassium_meq_l: f64,
    pub protein_g_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidIntake {
    pub oral_fluids_ml: f64,
    pub food_water_ml: f64,
    pub metabolic_water_ml: f64,
    pub iv_fluids_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidOutput {
    pub urine_ml: f64,
    pub feces_ml: f64,
    pub respiratory_losses_ml: f64,
    pub skin_losses_ml: f64,
    pub sweat_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalFluidRegulation {
    pub glomerular_filtration_rate_ml_min: f64,
    pub urine_output_ml_hour: f64,
    pub urine_specific_gravity: f64,
    pub urine_osmolality_mosm_kg: f64,
    pub fractional_sodium_excretion_percent: f64,
    pub free_water_clearance_ml_min: f64,
    pub adh_level_pg_ml: f64,
    pub aldosterone_ng_dl: f64,
    pub anp_pg_ml: f64,
    pub renin_activity_ng_ml_hr: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdemaAssessment {
    pub present: bool,
    pub location: Vec<EdemaLocation>,
    pub severity: EdemaSeverity,
    pub pitting: bool,
    pub cause: Option<EdemaCause>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdemaLocation {
    Peripheral,
    Pulmonary,
    Cerebral,
    Periorbital,
    Ascites,
    Anasarca,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdemaSeverity {
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdemaCause {
    HeartFailure,
    RenalFailure,
    LiverFailure,
    Hypoalbuminemia,
    VenousInsufficiency,
    Lymphatic,
    Inflammatory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DehydrationAssessment {
    pub severity: DehydrationSeverity,
    pub type_of_dehydration: DehydrationType,
    pub clinical_signs: Vec<String>,
    pub fluid_deficit_ml: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DehydrationSeverity {
    None,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DehydrationType {
    Isotonic,
    Hypertonic,
    Hypotonic,
}

impl FluidBalance {
    pub fn new_normal_adult(body_weight_kg: f64) -> Self {
        let tbw = body_weight_kg * 0.6;
        let icf = tbw * 0.67;
        let ecf = tbw * 0.33;
        let plasma = ecf * 0.25;
        let interstitial = ecf * 0.75;

        Self {
            total_body_water_l: tbw,
            intracellular_fluid_l: icf,
            extracellular_fluid_l: ecf,
            plasma_volume_l: plasma,
            interstitial_fluid_l: interstitial,
            daily_intake_ml: 2500.0,
            daily_output_ml: 2500.0,
            insensible_losses_ml: 900.0,
            balance_status: BalanceStatus::Balanced,
        }
    }

    pub fn calculate_balance(&self) -> f64 {
        self.daily_intake_ml - self.daily_output_ml
    }

    pub fn update_balance_status(&mut self) {
        let balance = self.calculate_balance();

        self.balance_status = match balance {
            b if b.abs() < 200.0 => BalanceStatus::Balanced,
            b if b > 1000.0 => BalanceStatus::Overload,
            b if b > 200.0 => BalanceStatus::PositiveBalance,
            b if b < -1000.0 => BalanceStatus::Depletion,
            _ => BalanceStatus::NegativeBalance,
        };
    }

    pub fn assess_hydration_status(&self, plasma_osmolality: f64) -> DehydrationSeverity {
        if plasma_osmolality < 285.0 {
            DehydrationSeverity::None
        } else if plasma_osmolality < 295.0 {
            DehydrationSeverity::Mild
        } else if plasma_osmolality < 310.0 {
            DehydrationSeverity::Moderate
        } else {
            DehydrationSeverity::Severe
        }
    }

    pub fn calculate_plasma_osmolality(
        &self,
        sodium_meq_l: f64,
        glucose_mg_dl: f64,
        bun_mg_dl: f64,
    ) -> f64 {
        2.0 * sodium_meq_l + glucose_mg_dl / 18.0 + bun_mg_dl / 2.8
    }

    pub fn water_deficit_ml(&self, current_sodium_meq_l: f64, target_sodium_meq_l: f64) -> f64 {
        let tbw = self.total_body_water_l * 1000.0;
        tbw * ((current_sodium_meq_l / target_sodium_meq_l) - 1.0)
    }
}

impl FluidIntake {
    pub fn new_normal() -> Self {
        Self {
            oral_fluids_ml: 1500.0,
            food_water_ml: 800.0,
            metabolic_water_ml: 200.0,
            iv_fluids_ml: 0.0,
        }
    }

    pub fn total_intake_ml(&self) -> f64 {
        self.oral_fluids_ml + self.food_water_ml + self.metabolic_water_ml + self.iv_fluids_ml
    }
}

impl FluidOutput {
    pub fn new_normal() -> Self {
        Self {
            urine_ml: 1500.0,
            feces_ml: 200.0,
            respiratory_losses_ml: 400.0,
            skin_losses_ml: 400.0,
            sweat_ml: 0.0,
        }
    }

    pub fn total_output_ml(&self) -> f64 {
        self.urine_ml
            + self.feces_ml
            + self.respiratory_losses_ml
            + self.skin_losses_ml
            + self.sweat_ml
    }

    pub fn insensible_losses_ml(&self) -> f64 {
        self.respiratory_losses_ml + self.skin_losses_ml
    }
}

impl RenalFluidRegulation {
    pub fn new_normal() -> Self {
        Self {
            glomerular_filtration_rate_ml_min: 100.0,
            urine_output_ml_hour: 60.0,
            urine_specific_gravity: 1.015,
            urine_osmolality_mosm_kg: 500.0,
            fractional_sodium_excretion_percent: 1.0,
            free_water_clearance_ml_min: 0.0,
            adh_level_pg_ml: 2.5,
            aldosterone_ng_dl: 10.0,
            anp_pg_ml: 50.0,
            renin_activity_ng_ml_hr: 1.5,
        }
    }

    pub fn is_oliguric(&self) -> bool {
        self.urine_output_ml_hour < 25.0
    }

    pub fn is_anuric(&self) -> bool {
        self.urine_output_ml_hour < 5.0
    }

    pub fn is_polyuric(&self) -> bool {
        self.urine_output_ml_hour > 200.0
    }

    pub fn calculate_urine_flow_rate(&self) -> f64 {
        self.urine_output_ml_hour / 60.0
    }

    pub fn assess_concentration_ability(&self) -> &str {
        if self.urine_osmolality_mosm_kg > 800.0 {
            "Excellent"
        } else if self.urine_osmolality_mosm_kg > 500.0 {
            "Normal"
        } else if self.urine_osmolality_mosm_kg > 300.0 {
            "Impaired"
        } else {
            "Poor"
        }
    }

    pub fn activate_water_conservation(&mut self) {
        self.adh_level_pg_ml = 10.0;
        self.urine_osmolality_mosm_kg = 1200.0;
        self.urine_specific_gravity = 1.030;
        self.urine_output_ml_hour = 20.0;
        self.free_water_clearance_ml_min = -2.0;
    }

    pub fn activate_water_excretion(&mut self) {
        self.adh_level_pg_ml = 0.5;
        self.urine_osmolality_mosm_kg = 100.0;
        self.urine_specific_gravity = 1.003;
        self.urine_output_ml_hour = 200.0;
        self.free_water_clearance_ml_min = 5.0;
    }

    pub fn assess_renal_function(&self) -> &str {
        match self.glomerular_filtration_rate_ml_min {
            gfr if gfr >= 90.0 => "Normal",
            gfr if gfr >= 60.0 => "Mild reduction",
            gfr if gfr >= 30.0 => "Moderate reduction",
            gfr if gfr >= 15.0 => "Severe reduction",
            _ => "Kidney failure",
        }
    }
}

impl FluidCompartment {
    pub fn new_intracellular(volume_l: f64) -> Self {
        Self {
            volume_l,
            osmolality_mosm_kg: 285.0,
            sodium_meq_l: 10.0,
            potassium_meq_l: 140.0,
            protein_g_l: 50.0,
        }
    }

    pub fn new_extracellular(volume_l: f64) -> Self {
        Self {
            volume_l,
            osmolality_mosm_kg: 285.0,
            sodium_meq_l: 140.0,
            potassium_meq_l: 4.0,
            protein_g_l: 2.0,
        }
    }

    pub fn new_plasma(volume_l: f64) -> Self {
        Self {
            volume_l,
            osmolality_mosm_kg: 285.0,
            sodium_meq_l: 140.0,
            potassium_meq_l: 4.0,
            protein_g_l: 70.0,
        }
    }

    pub fn calculate_osmolality(&self) -> f64 {
        2.0 * self.sodium_meq_l + 2.0 * self.potassium_meq_l
    }
}

impl EdemaAssessment {
    pub fn none() -> Self {
        Self {
            present: false,
            location: Vec::new(),
            severity: EdemaSeverity::Mild,
            pitting: false,
            cause: None,
        }
    }

    pub fn peripheral_edema(severity: EdemaSeverity) -> Self {
        Self {
            present: true,
            location: vec![EdemaLocation::Peripheral],
            severity,
            pitting: true,
            cause: Some(EdemaCause::HeartFailure),
        }
    }

    pub fn pulmonary_edema() -> Self {
        Self {
            present: true,
            location: vec![EdemaLocation::Pulmonary],
            severity: EdemaSeverity::Severe,
            pitting: false,
            cause: Some(EdemaCause::HeartFailure),
        }
    }
}

impl DehydrationAssessment {
    pub fn assess(fluid_deficit_ml: f64, sodium_meq_l: f64) -> Self {
        let severity = match fluid_deficit_ml.abs() {
            d if d < 1000.0 => DehydrationSeverity::None,
            d if d < 2000.0 => DehydrationSeverity::Mild,
            d if d < 4000.0 => DehydrationSeverity::Moderate,
            _ => DehydrationSeverity::Severe,
        };

        let dehydration_type = match sodium_meq_l {
            na if na < 135.0 => DehydrationType::Hypotonic,
            na if na > 145.0 => DehydrationType::Hypertonic,
            _ => DehydrationType::Isotonic,
        };

        let mut clinical_signs = Vec::new();
        if severity != DehydrationSeverity::None {
            clinical_signs.push("Thirst".to_string());
            clinical_signs.push("Dry mucous membranes".to_string());
        }
        if severity == DehydrationSeverity::Moderate || severity == DehydrationSeverity::Severe {
            clinical_signs.push("Decreased skin turgor".to_string());
            clinical_signs.push("Tachycardia".to_string());
        }
        if severity == DehydrationSeverity::Severe {
            clinical_signs.push("Hypotension".to_string());
            clinical_signs.push("Altered mental status".to_string());
        }

        Self {
            severity,
            type_of_dehydration: dehydration_type,
            clinical_signs,
            fluid_deficit_ml,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid_balance_creation() {
        let balance = FluidBalance::new_normal_adult(70.0);
        assert!((balance.total_body_water_l - 42.0).abs() < 1.0);
        assert_eq!(balance.balance_status, BalanceStatus::Balanced);
    }

    #[test]
    fn test_balance_calculation() {
        let balance = FluidBalance::new_normal_adult(70.0);
        assert_eq!(balance.calculate_balance(), 0.0);
    }

    #[test]
    fn test_plasma_osmolality() {
        let balance = FluidBalance::new_normal_adult(70.0);
        let osm = balance.calculate_plasma_osmolality(140.0, 100.0, 20.0);
        assert!((osm - 292.7).abs() < 1.0);
    }

    #[test]
    fn test_water_deficit() {
        let balance = FluidBalance::new_normal_adult(70.0);
        let deficit = balance.water_deficit_ml(150.0, 140.0);
        assert!(deficit > 2000.0);
    }

    #[test]
    fn test_fluid_intake() {
        let intake = FluidIntake::new_normal();
        assert_eq!(intake.total_intake_ml(), 2500.0);
    }

    #[test]
    fn test_fluid_output() {
        let output = FluidOutput::new_normal();
        assert_eq!(output.total_output_ml(), 2500.0);
        assert_eq!(output.insensible_losses_ml(), 800.0);
    }

    #[test]
    fn test_renal_regulation() {
        let renal = RenalFluidRegulation::new_normal();
        assert!(!renal.is_oliguric());
        assert!(!renal.is_polyuric());
        assert_eq!(renal.assess_renal_function(), "Normal");
    }

    #[test]
    fn test_oliguria() {
        let mut renal = RenalFluidRegulation::new_normal();
        renal.urine_output_ml_hour = 20.0;
        assert!(renal.is_oliguric());
    }

    #[test]
    fn test_water_conservation() {
        let mut renal = RenalFluidRegulation::new_normal();
        renal.activate_water_conservation();
        assert!(renal.urine_osmolality_mosm_kg > 1000.0);
        assert!(renal.urine_output_ml_hour < 30.0);
    }

    #[test]
    fn test_water_excretion() {
        let mut renal = RenalFluidRegulation::new_normal();
        renal.activate_water_excretion();
        assert!(renal.urine_osmolality_mosm_kg < 200.0);
        assert!(renal.urine_output_ml_hour > 150.0);
    }

    #[test]
    fn test_fluid_compartments() {
        let icf = FluidCompartment::new_intracellular(28.0);
        let ecf = FluidCompartment::new_extracellular(14.0);
        assert!(icf.potassium_meq_l > ecf.potassium_meq_l);
        assert!(ecf.sodium_meq_l > icf.sodium_meq_l);
    }

    #[test]
    fn test_edema_assessment() {
        let edema = EdemaAssessment::peripheral_edema(EdemaSeverity::Moderate);
        assert!(edema.present);
        assert!(edema.pitting);
    }

    #[test]
    fn test_dehydration_assessment() {
        let dehydration = DehydrationAssessment::assess(3000.0, 148.0);
        assert_eq!(dehydration.severity, DehydrationSeverity::Moderate);
        assert_eq!(dehydration.type_of_dehydration, DehydrationType::Hypertonic);
        assert!(!dehydration.clinical_signs.is_empty());
    }

    #[test]
    fn test_concentration_ability() {
        let mut renal = RenalFluidRegulation::new_normal();
        renal.urine_osmolality_mosm_kg = 600.0;
        assert_eq!(renal.assess_concentration_ability(), "Normal");
    }
}
