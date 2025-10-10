use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WoundHealing {
    pub wound_type: WoundType,
    pub size_cm2: f64,
    pub depth: WoundDepth,
    pub healing_phase: HealingPhase,
    pub days_since_injury: u32,
    pub inflammation_level: f64,
    pub infection_present: bool,
    pub vascularization: f64,
    pub collagen_deposition: f64,
    pub epithelialization_percent: f64,
    pub tensile_strength_percent: f64,
    pub healing_factors: HealingFactors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WoundType {
    Incision,
    Laceration,
    Abrasion,
    Puncture,
    Burn,
    Pressure,
    Surgical,
    Diabetic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WoundDepth {
    Superficial,
    PartialThickness,
    FullThickness,
    Deep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealingPhase {
    Hemostasis,
    Inflammation,
    Proliferation,
    Remodeling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingFactors {
    pub age_years: u32,
    pub nutrition_status: f64,
    pub protein_intake_g_per_day: f64,
    pub vitamin_c_mg_per_day: f64,
    pub zinc_mg_per_day: f64,
    pub blood_glucose_mg_dl: f64,
    pub tissue_perfusion: f64,
    pub oxygenation_mmhg: f64,
    pub smoking: bool,
    pub immunosuppression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularResponse {
    pub neutrophils_per_mm3: f64,
    pub macrophages_per_mm3: f64,
    pub fibroblasts_per_mm3: f64,
    pub keratinocytes_per_mm3: f64,
    pub endothelial_cells_per_mm3: f64,
    pub growth_factors: GrowthFactors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthFactors {
    pub pdgf_ng_ml: f64,
    pub tgf_beta_ng_ml: f64,
    pub vegf_pg_ml: f64,
    pub egf_ng_ml: f64,
    pub fgf_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScarFormation {
    pub scar_type: ScarType,
    pub collagen_alignment: f64,
    pub vascularity: f64,
    pub pigmentation_change: f64,
    pub pruritus_level: f64,
    pub cosmetic_outcome_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScarType {
    Normal,
    Hypertrophic,
    Keloid,
    Atrophic,
    Contracture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealingComplications {
    Infection,
    Dehiscence,
    Necrosis,
    DelayedHealing,
    ChronicWound,
    Hematoma,
    Seroma,
}

impl WoundHealing {
    pub fn new_acute_incision(size_cm2: f64) -> Self {
        Self {
            wound_type: WoundType::Incision,
            size_cm2,
            depth: WoundDepth::FullThickness,
            healing_phase: HealingPhase::Hemostasis,
            days_since_injury: 0,
            inflammation_level: 0.8,
            infection_present: false,
            vascularization: 0.5,
            collagen_deposition: 0.0,
            epithelialization_percent: 0.0,
            tensile_strength_percent: 0.0,
            healing_factors: HealingFactors::new_healthy_adult(),
        }
    }

    pub fn new_chronic_wound(size_cm2: f64) -> Self {
        Self {
            wound_type: WoundType::Pressure,
            size_cm2,
            depth: WoundDepth::Deep,
            healing_phase: HealingPhase::Inflammation,
            days_since_injury: 60,
            inflammation_level: 0.9,
            infection_present: true,
            vascularization: 0.3,
            collagen_deposition: 0.2,
            epithelialization_percent: 10.0,
            tensile_strength_percent: 5.0,
            healing_factors: HealingFactors::new_impaired(),
        }
    }

    pub fn progress_healing(&mut self, days: u32) {
        self.days_since_injury += days;

        match self.healing_phase {
            HealingPhase::Hemostasis if self.days_since_injury > 1 => {
                self.healing_phase = HealingPhase::Inflammation;
                self.inflammation_level = 0.9;
            }
            HealingPhase::Inflammation if self.days_since_injury > 4 => {
                self.healing_phase = HealingPhase::Proliferation;
                self.inflammation_level = 0.5;
                self.vascularization = 0.7;
            }
            HealingPhase::Proliferation if self.days_since_injury > 21 => {
                self.healing_phase = HealingPhase::Remodeling;
                self.inflammation_level = 0.2;
                self.collagen_deposition = 0.8;
            }
            _ => {}
        }

        self.update_healing_progress();
    }

    fn update_healing_progress(&mut self) {
        let healing_rate = self.calculate_healing_rate();

        match self.healing_phase {
            HealingPhase::Proliferation => {
                self.epithelialization_percent += healing_rate * 5.0;
                self.collagen_deposition += healing_rate * 0.03;
                self.vascularization = (self.vascularization + healing_rate * 0.05).min(1.0);
            }
            HealingPhase::Remodeling => {
                self.tensile_strength_percent += healing_rate * 2.0;
                self.collagen_deposition =
                    (self.collagen_deposition + healing_rate * 0.02).min(1.0);
            }
            _ => {}
        }

        self.epithelialization_percent = self.epithelialization_percent.min(100.0);
        self.tensile_strength_percent = self.tensile_strength_percent.min(80.0);
    }

    pub fn calculate_healing_rate(&self) -> f64 {
        let mut rate = 1.0;

        if self.healing_factors.age_years > 60 {
            rate *= 0.7;
        }

        if self.healing_factors.nutrition_status < 0.7 {
            rate *= 0.6;
        }

        if self.healing_factors.blood_glucose_mg_dl > 200.0 {
            rate *= 0.5;
        }

        if self.healing_factors.tissue_perfusion < 0.6 {
            rate *= 0.5;
        }

        if self.healing_factors.smoking {
            rate *= 0.6;
        }

        if self.infection_present {
            rate *= 0.4;
        }

        if self.healing_factors.immunosuppression {
            rate *= 0.5;
        }

        rate
    }

    pub fn is_healing_normally(&self) -> bool {
        let expected_epithelialization = match self.healing_phase {
            HealingPhase::Hemostasis => 0.0,
            HealingPhase::Inflammation => 10.0,
            HealingPhase::Proliferation => 50.0,
            HealingPhase::Remodeling => 100.0,
        };

        self.epithelialization_percent >= expected_epithelialization * 0.7
            && !self.infection_present
            && self.inflammation_level < 0.7
    }

    pub fn assess_infection_risk(&self) -> f64 {
        let mut risk: f64 = 0.0;

        if self.wound_type == WoundType::Diabetic {
            risk += 0.3;
        }

        if self.healing_factors.blood_glucose_mg_dl > 180.0 {
            risk += 0.2;
        }

        if self.healing_factors.tissue_perfusion < 0.5 {
            risk += 0.2;
        }

        if self.depth == WoundDepth::Deep {
            risk += 0.15;
        }

        if self.inflammation_level > 0.8 && self.days_since_injury > 7 {
            risk += 0.15;
        }

        risk.min(1.0)
    }

    pub fn estimated_time_to_heal_days(&self) -> u32 {
        let base_time = match self.wound_type {
            WoundType::Incision | WoundType::Surgical => 14,
            WoundType::Laceration => 21,
            WoundType::Abrasion => 7,
            WoundType::Puncture => 14,
            WoundType::Burn => 30,
            WoundType::Pressure => 45,
            WoundType::Diabetic => 60,
        };

        let size_factor = (self.size_cm2 / 5.0).max(1.0);
        let healing_rate = self.calculate_healing_rate();

        ((base_time as f64) * size_factor / healing_rate) as u32
    }

    pub fn apply_treatment(&mut self, treatment: WoundTreatment) {
        match treatment {
            WoundTreatment::Debridement => {
                self.inflammation_level *= 0.7;
                if self.infection_present {
                    self.infection_present = false;
                }
            }
            WoundTreatment::AntibioticTherapy => {
                if self.infection_present {
                    self.infection_present = false;
                    self.inflammation_level *= 0.8;
                }
            }
            WoundTreatment::GrowthFactors => {
                self.vascularization = (self.vascularization + 0.2).min(1.0);
                self.collagen_deposition = (self.collagen_deposition + 0.1).min(1.0);
            }
            WoundTreatment::NegativePressure => {
                self.vascularization = (self.vascularization + 0.15).min(1.0);
                self.size_cm2 *= 0.9;
            }
            WoundTreatment::HyperbaricOxygen => {
                self.healing_factors.oxygenation_mmhg = 200.0;
                self.vascularization = (self.vascularization + 0.1).min(1.0);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WoundTreatment {
    Debridement,
    AntibioticTherapy,
    GrowthFactors,
    NegativePressure,
    HyperbaricOxygen,
}

impl HealingFactors {
    pub fn new_healthy_adult() -> Self {
        Self {
            age_years: 35,
            nutrition_status: 0.9,
            protein_intake_g_per_day: 70.0,
            vitamin_c_mg_per_day: 90.0,
            zinc_mg_per_day: 11.0,
            blood_glucose_mg_dl: 90.0,
            tissue_perfusion: 0.9,
            oxygenation_mmhg: 95.0,
            smoking: false,
            immunosuppression: false,
        }
    }

    pub fn new_impaired() -> Self {
        Self {
            age_years: 70,
            nutrition_status: 0.5,
            protein_intake_g_per_day: 40.0,
            vitamin_c_mg_per_day: 30.0,
            zinc_mg_per_day: 5.0,
            blood_glucose_mg_dl: 180.0,
            tissue_perfusion: 0.5,
            oxygenation_mmhg: 75.0,
            smoking: true,
            immunosuppression: false,
        }
    }

    pub fn overall_score(&self) -> f64 {
        let mut score = 1.0;

        if self.age_years > 65 {
            score *= 0.8;
        }

        score *= self.nutrition_status;
        score *= self.tissue_perfusion;

        if self.smoking {
            score *= 0.7;
        }

        if self.immunosuppression {
            score *= 0.6;
        }

        score
    }
}

impl CellularResponse {
    pub fn new_inflammation_phase() -> Self {
        Self {
            neutrophils_per_mm3: 5000.0,
            macrophages_per_mm3: 2000.0,
            fibroblasts_per_mm3: 100.0,
            keratinocytes_per_mm3: 50.0,
            endothelial_cells_per_mm3: 200.0,
            growth_factors: GrowthFactors::new_active(),
        }
    }

    pub fn new_proliferation_phase() -> Self {
        Self {
            neutrophils_per_mm3: 1000.0,
            macrophages_per_mm3: 3000.0,
            fibroblasts_per_mm3: 5000.0,
            keratinocytes_per_mm3: 3000.0,
            endothelial_cells_per_mm3: 2000.0,
            growth_factors: GrowthFactors::new_active(),
        }
    }
}

impl GrowthFactors {
    pub fn new_active() -> Self {
        Self {
            pdgf_ng_ml: 15.0,
            tgf_beta_ng_ml: 5.0,
            vegf_pg_ml: 200.0,
            egf_ng_ml: 3.0,
            fgf_ng_ml: 10.0,
        }
    }
}

impl ScarFormation {
    pub fn new_normal() -> Self {
        Self {
            scar_type: ScarType::Normal,
            collagen_alignment: 0.7,
            vascularity: 0.3,
            pigmentation_change: 0.1,
            pruritus_level: 0.0,
            cosmetic_outcome_score: 0.8,
        }
    }

    pub fn new_hypertrophic() -> Self {
        Self {
            scar_type: ScarType::Hypertrophic,
            collagen_alignment: 0.3,
            vascularity: 0.8,
            pigmentation_change: 0.4,
            pruritus_level: 0.7,
            cosmetic_outcome_score: 0.4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acute_wound_creation() {
        let wound = WoundHealing::new_acute_incision(5.0);
        assert_eq!(wound.wound_type, WoundType::Incision);
        assert_eq!(wound.healing_phase, HealingPhase::Hemostasis);
        assert!(!wound.infection_present);
    }

    #[test]
    fn test_chronic_wound_creation() {
        let wound = WoundHealing::new_chronic_wound(10.0);
        assert!(wound.days_since_injury > 30);
        assert!(wound.infection_present);
    }

    #[test]
    fn test_healing_progression() {
        let mut wound = WoundHealing::new_acute_incision(3.0);
        wound.progress_healing(5);
        assert_eq!(wound.healing_phase, HealingPhase::Inflammation);
        wound.progress_healing(17);
        assert_eq!(wound.healing_phase, HealingPhase::Proliferation);
        assert!(wound.epithelialization_percent > 0.0);
    }

    #[test]
    fn test_healing_rate_calculation() {
        let wound = WoundHealing::new_acute_incision(5.0);
        let rate = wound.calculate_healing_rate();
        assert!(rate > 0.0);
        assert!(rate <= 1.0);
    }

    #[test]
    fn test_infection_risk() {
        let wound = WoundHealing::new_chronic_wound(8.0);
        let risk = wound.assess_infection_risk();
        assert!(risk >= 0.3);
    }

    #[test]
    fn test_estimated_healing_time() {
        let wound = WoundHealing::new_acute_incision(2.0);
        let time = wound.estimated_time_to_heal_days();
        assert!(time > 0);
        assert!(time < 60);
    }

    #[test]
    fn test_treatment_application() {
        let mut wound = WoundHealing::new_chronic_wound(5.0);
        wound.apply_treatment(WoundTreatment::AntibioticTherapy);
        assert!(!wound.infection_present);
    }

    #[test]
    fn test_healing_factors() {
        let healthy = HealingFactors::new_healthy_adult();
        let impaired = HealingFactors::new_impaired();
        assert!(healthy.overall_score() > impaired.overall_score());
    }

    #[test]
    fn test_cellular_response() {
        let inflammation = CellularResponse::new_inflammation_phase();
        let proliferation = CellularResponse::new_proliferation_phase();
        assert!(inflammation.neutrophils_per_mm3 > proliferation.neutrophils_per_mm3);
        assert!(proliferation.fibroblasts_per_mm3 > inflammation.fibroblasts_per_mm3);
    }

    #[test]
    fn test_scar_formation() {
        let normal = ScarFormation::new_normal();
        let hypertrophic = ScarFormation::new_hypertrophic();
        assert!(normal.cosmetic_outcome_score > hypertrophic.cosmetic_outcome_score);
    }

    #[test]
    fn test_growth_factors() {
        let gf = GrowthFactors::new_active();
        assert!(gf.vegf_pg_ml > 0.0);
        assert!(gf.pdgf_ng_ml > 0.0);
    }
}
