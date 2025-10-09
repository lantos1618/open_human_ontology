use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Liver {
    pub mass_g: f64,
    pub hepatocytes_count: u64,
    pub bile_production_ml_per_day: f64,
    pub glycogen_storage_g: f64,
    pub protein_synthesis_rate: f64,
    pub detox_capacity: DetoxificationCapacity,
    pub metabolic_function: HepaticMetabolism,
}

impl Liver {
    pub fn new_adult() -> Self {
        Liver {
            mass_g: 1500.0,
            hepatocytes_count: 240_000_000_000,
            bile_production_ml_per_day: 600.0,
            glycogen_storage_g: 100.0,
            protein_synthesis_rate: 1.0,
            detox_capacity: DetoxificationCapacity::new(),
            metabolic_function: HepaticMetabolism::new(),
        }
    }

    pub fn synthesize_albumin(&self) -> f64 {
        12.0 * self.protein_synthesis_rate
    }

    pub fn synthesize_clotting_factors(&self) -> Vec<String> {
        vec![
            "Factor I (Fibrinogen)".to_string(),
            "Factor II (Prothrombin)".to_string(),
            "Factor V".to_string(),
            "Factor VII".to_string(),
            "Factor IX".to_string(),
            "Factor X".to_string(),
            "Factor XI".to_string(),
        ]
    }

    pub fn glucose_regulation(&mut self, blood_glucose_mm: f64) -> BiologyResult<()> {
        if blood_glucose_mm > 7.0 {
            self.glycogen_storage_g += (blood_glucose_mm - 5.0) * 10.0;
            self.metabolic_function.glycogenesis_active = true;
            self.metabolic_function.glycogenolysis_active = false;
        } else if blood_glucose_mm < 4.0 {
            if self.glycogen_storage_g > 10.0 {
                self.glycogen_storage_g -= 10.0;
                self.metabolic_function.glycogenolysis_active = true;
                self.metabolic_function.glycogenesis_active = false;
            } else {
                self.metabolic_function.gluconeogenesis_active = true;
            }
        }
        Ok(())
    }

    pub fn assess_function(&self) -> LiverFunctionAssessment {
        let albumin_level = self.synthesize_albumin();
        let bilirubin_clearance = self.detox_capacity.bilirubin_conjugation_rate;
        let synthetic_function = self.protein_synthesis_rate;

        LiverFunctionAssessment {
            albumin_g_dl: albumin_level / 3.0,
            bilirubin_mg_dl: 1.0 / bilirubin_clearance,
            inr: 1.0 / synthetic_function,
            child_pugh_score: 5,
            meld_score: 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetoxificationCapacity {
    pub phase_i_active: bool,
    pub phase_ii_active: bool,
    pub cyp450_activity: f64,
    pub glutathione_conjugation: f64,
    pub sulfation_capacity: f64,
    pub glucuronidation_capacity: f64,
    pub bilirubin_conjugation_rate: f64,
    pub ammonia_conversion_rate: f64,
}

impl DetoxificationCapacity {
    pub fn new() -> Self {
        DetoxificationCapacity {
            phase_i_active: true,
            phase_ii_active: true,
            cyp450_activity: 1.0,
            glutathione_conjugation: 1.0,
            sulfation_capacity: 1.0,
            glucuronidation_capacity: 1.0,
            bilirubin_conjugation_rate: 1.0,
            ammonia_conversion_rate: 1.0,
        }
    }

    pub fn phase_i_reactions(&self) -> Vec<&str> {
        vec![
            "Oxidation (CYP450)",
            "Reduction",
            "Hydrolysis",
        ]
    }

    pub fn phase_ii_reactions(&self) -> Vec<&str> {
        vec![
            "Glucuronidation",
            "Sulfation",
            "Glutathione conjugation",
            "Acetylation",
            "Methylation",
        ]
    }

    pub fn detoxify_ammonia(&mut self, ammonia_mm: f64) -> BiologyResult<f64> {
        if !self.phase_i_active {
            return Err(BiologyError::InvalidState(
                "Detoxification impaired".to_string()
            ));
        }

        let urea_produced = ammonia_mm * self.ammonia_conversion_rate;
        Ok(urea_produced)
    }

    pub fn conjugate_bilirubin(&mut self, unconjugated_bilirubin_mg: f64) -> f64 {
        unconjugated_bilirubin_mg * self.bilirubin_conjugation_rate
    }
}

impl Default for DetoxificationCapacity {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticMetabolism {
    pub glycogenesis_active: bool,
    pub glycogenolysis_active: bool,
    pub gluconeogenesis_active: bool,
    pub lipogenesis_rate: f64,
    pub beta_oxidation_rate: f64,
    pub ketogenesis_active: bool,
    pub cholesterol_synthesis_mg_per_day: f64,
    pub bile_acid_synthesis_mg_per_day: f64,
}

impl HepaticMetabolism {
    pub fn new() -> Self {
        HepaticMetabolism {
            glycogenesis_active: false,
            glycogenolysis_active: false,
            gluconeogenesis_active: false,
            lipogenesis_rate: 1.0,
            beta_oxidation_rate: 1.0,
            ketogenesis_active: false,
            cholesterol_synthesis_mg_per_day: 800.0,
            bile_acid_synthesis_mg_per_day: 500.0,
        }
    }

    pub fn metabolic_state(&self) -> &str {
        if self.glycogenesis_active {
            "Fed state - storing glucose as glycogen"
        } else if self.glycogenolysis_active {
            "Fasting state - breaking down glycogen"
        } else if self.gluconeogenesis_active {
            "Prolonged fasting - making new glucose"
        } else if self.ketogenesis_active {
            "Ketogenic state - producing ketone bodies"
        } else {
            "Basal metabolic state"
        }
    }

    pub fn switch_to_fasting(&mut self) {
        self.glycogenesis_active = false;
        self.glycogenolysis_active = true;
        self.beta_oxidation_rate = 1.5;
    }

    pub fn switch_to_fed(&mut self) {
        self.glycogenesis_active = true;
        self.glycogenolysis_active = false;
        self.gluconeogenesis_active = false;
        self.ketogenesis_active = false;
        self.lipogenesis_rate = 1.5;
    }

    pub fn switch_to_ketogenic(&mut self) {
        self.glycogenolysis_active = false;
        self.gluconeogenesis_active = true;
        self.ketogenesis_active = true;
        self.beta_oxidation_rate = 2.0;
    }
}

impl Default for HepaticMetabolism {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BileProduction {
    pub bile_salts_mm: f64,
    pub cholesterol_mm: f64,
    pub bilirubin_mm: f64,
    pub phospholipids_mm: f64,
    pub production_rate_ml_per_hour: f64,
    pub enterohepatic_circulation_active: bool,
}

impl BileProduction {
    pub fn new() -> Self {
        BileProduction {
            bile_salts_mm: 30.0,
            cholesterol_mm: 5.0,
            bilirubin_mm: 0.5,
            phospholipids_mm: 8.0,
            production_rate_ml_per_hour: 25.0,
            enterohepatic_circulation_active: true,
        }
    }

    pub fn primary_bile_acids(&self) -> Vec<&str> {
        vec!["Cholic acid", "Chenodeoxycholic acid"]
    }

    pub fn secondary_bile_acids(&self) -> Vec<&str> {
        vec!["Deoxycholic acid", "Lithocholic acid"]
    }

    pub fn daily_production(&self) -> f64 {
        self.production_rate_ml_per_hour * 24.0
    }

    pub fn recirculation_efficiency(&self) -> f64 {
        if self.enterohepatic_circulation_active {
            0.95
        } else {
            0.0
        }
    }
}

impl Default for BileProduction {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiverFunctionAssessment {
    pub albumin_g_dl: f64,
    pub bilirubin_mg_dl: f64,
    pub inr: f64,
    pub child_pugh_score: u32,
    pub meld_score: u32,
}

impl LiverFunctionAssessment {
    pub fn is_cirrhotic(&self) -> bool {
        self.child_pugh_score >= 7
    }

    pub fn severity(&self) -> &str {
        if self.child_pugh_score <= 6 {
            "Child-Pugh A: Well-compensated"
        } else if self.child_pugh_score <= 9 {
            "Child-Pugh B: Significant functional compromise"
        } else {
            "Child-Pugh C: Decompensated cirrhosis"
        }
    }

    pub fn transplant_priority(&self) -> &str {
        if self.meld_score < 10 {
            "Low priority"
        } else if self.meld_score < 20 {
            "Moderate priority"
        } else if self.meld_score < 30 {
            "High priority"
        } else {
            "Urgent - highest priority"
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HepatocyteZone {
    Zone1Periportal,
    Zone2Intermediate,
    Zone3Pericentral,
}

impl HepatocyteZone {
    pub fn oxygen_tension(&self) -> &str {
        match self {
            HepatocyteZone::Zone1Periportal => "High O2 (~60-65 mmHg)",
            HepatocyteZone::Zone2Intermediate => "Moderate O2 (~35-50 mmHg)",
            HepatocyteZone::Zone3Pericentral => "Low O2 (~25-35 mmHg)",
        }
    }

    pub fn primary_functions(&self) -> Vec<&str> {
        match self {
            HepatocyteZone::Zone1Periportal => vec![
                "Oxidative metabolism",
                "Gluconeogenesis",
                "Beta-oxidation",
                "Cholesterol synthesis",
                "Bile acid synthesis",
            ],
            HepatocyteZone::Zone2Intermediate => vec![
                "Mixed metabolic functions",
            ],
            HepatocyteZone::Zone3Pericentral => vec![
                "Glycolysis",
                "Lipogenesis",
                "Ketogenesis",
                "Biotransformation (CYP450)",
                "Glutamine synthesis",
            ],
        }
    }

    pub fn vulnerability(&self) -> &str {
        match self {
            HepatocyteZone::Zone1Periportal => "Viral hepatitis",
            HepatocyteZone::Zone2Intermediate => "Moderate vulnerability",
            HepatocyteZone::Zone3Pericentral => "Ischemic injury, toxic injury",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinSynthesis {
    pub albumin_g_per_day: f64,
    pub globulins_g_per_day: f64,
    pub fibrinogen_g_per_day: f64,
    pub transferrin_g_per_day: f64,
    pub complement_proteins: bool,
    pub acute_phase_proteins: Vec<String>,
}

impl ProteinSynthesis {
    pub fn new() -> Self {
        ProteinSynthesis {
            albumin_g_per_day: 12.0,
            globulins_g_per_day: 8.0,
            fibrinogen_g_per_day: 2.0,
            transferrin_g_per_day: 0.2,
            complement_proteins: true,
            acute_phase_proteins: vec![
                "C-reactive protein".to_string(),
                "Serum amyloid A".to_string(),
                "Fibrinogen".to_string(),
                "Haptoglobin".to_string(),
            ],
        }
    }

    pub fn total_protein_synthesis(&self) -> f64 {
        self.albumin_g_per_day + self.globulins_g_per_day +
        self.fibrinogen_g_per_day + self.transferrin_g_per_day
    }

    pub fn acute_phase_response(&mut self) {
        self.albumin_g_per_day *= 0.7;
        self.fibrinogen_g_per_day *= 2.0;
    }
}

impl Default for ProteinSynthesis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liver_creation() {
        let liver = Liver::new_adult();
        assert_eq!(liver.mass_g, 1500.0);
        assert!(liver.bile_production_ml_per_day > 0.0);
    }

    #[test]
    fn test_albumin_synthesis() {
        let liver = Liver::new_adult();
        let albumin = liver.synthesize_albumin();
        assert_eq!(albumin, 12.0);
    }

    #[test]
    fn test_clotting_factors() {
        let liver = Liver::new_adult();
        let factors = liver.synthesize_clotting_factors();
        assert_eq!(factors.len(), 7);
    }

    #[test]
    fn test_glucose_regulation() {
        let mut liver = Liver::new_adult();

        liver.glucose_regulation(8.0).unwrap();
        assert!(liver.metabolic_function.glycogenesis_active);

        liver.glucose_regulation(3.5).unwrap();
        assert!(liver.metabolic_function.glycogenolysis_active);
    }

    #[test]
    fn test_detoxification() {
        let mut detox = DetoxificationCapacity::new();

        let urea = detox.detoxify_ammonia(1.0).unwrap();
        assert_eq!(urea, 1.0);

        let conjugated = detox.conjugate_bilirubin(2.0);
        assert_eq!(conjugated, 2.0);
    }

    #[test]
    fn test_hepatic_metabolism_states() {
        let mut metabolism = HepaticMetabolism::new();

        metabolism.switch_to_fed();
        assert!(metabolism.glycogenesis_active);
        assert_eq!(metabolism.metabolic_state(), "Fed state - storing glucose as glycogen");

        metabolism.switch_to_fasting();
        assert!(metabolism.glycogenolysis_active);

        metabolism.switch_to_ketogenic();
        assert!(metabolism.ketogenesis_active);
        assert_eq!(metabolism.beta_oxidation_rate, 2.0);
    }

    #[test]
    fn test_bile_production() {
        let bile = BileProduction::new();
        assert_eq!(bile.daily_production(), 600.0);
        assert_eq!(bile.recirculation_efficiency(), 0.95);
        assert_eq!(bile.primary_bile_acids().len(), 2);
    }

    #[test]
    fn test_liver_function_assessment() {
        let assessment = LiverFunctionAssessment {
            albumin_g_dl: 3.5,
            bilirubin_mg_dl: 1.0,
            inr: 1.1,
            child_pugh_score: 5,
            meld_score: 8,
        };

        assert!(!assessment.is_cirrhotic());
        assert_eq!(assessment.severity(), "Child-Pugh A: Well-compensated");
        assert_eq!(assessment.transplant_priority(), "Low priority");
    }

    #[test]
    fn test_hepatocyte_zones() {
        let zone1 = HepatocyteZone::Zone1Periportal;
        let zone3 = HepatocyteZone::Zone3Pericentral;

        assert!(zone1.primary_functions().contains(&"Gluconeogenesis"));
        assert!(zone3.primary_functions().contains(&"Glycolysis"));
        assert_eq!(zone3.vulnerability(), "Ischemic injury, toxic injury");
    }

    #[test]
    fn test_protein_synthesis() {
        let mut synthesis = ProteinSynthesis::new();
        let total = synthesis.total_protein_synthesis();
        assert!(total > 20.0);

        let baseline_albumin = synthesis.albumin_g_per_day;
        synthesis.acute_phase_response();
        assert!(synthesis.albumin_g_per_day < baseline_albumin);
        assert_eq!(synthesis.fibrinogen_g_per_day, 4.0);
    }
}
