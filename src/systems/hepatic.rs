use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Liver {
    pub weight_g: f64,
    pub lobes: Vec<HepaticLobe>,
    pub lobules: Vec<LiverLobule>,
    pub blood_flow: HepaticBloodFlow,
    pub bile_production: BileProduction,
    pub metabolic_functions: HepaticMetabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HepaticLobe {
    Right,
    Left,
    Caudate,
    Quadrate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiverLobule {
    pub diameter_mm: f64,
    pub hepatocytes: Vec<Hepatocyte>,
    pub sinusoids: Vec<HepaticSinusoid>,
    pub central_vein: CentralVein,
    pub portal_triad: PortalTriad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hepatocyte {
    pub diameter_um: f64,
    pub glycogen_content: f64,
    pub lipid_content: f64,
    pub protein_synthesis_rate: f64,
    pub zone: HepaticZone,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HepaticZone {
    Zone1,
    Zone2,
    Zone3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticSinusoid {
    pub diameter_um: f64,
    pub fenestrations: bool,
    pub kupffer_cells: Vec<KupfferCell>,
    pub stellate_cells: Vec<HepaticStellateCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KupfferCell {
    pub phagocytic_activity: f64,
    pub cytokine_production: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticStellateCell {
    pub activated: bool,
    pub collagen_production: f64,
    pub vitamin_a_storage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralVein {
    pub diameter_um: f64,
    pub blood_flow_ml_per_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalTriad {
    pub hepatic_artery: HepaticArteriole,
    pub portal_vein: PortalVenule,
    pub bile_duct: BileDuct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticArteriole {
    pub diameter_um: f64,
    pub blood_flow_ml_per_min: f64,
    pub oxygen_content: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalVenule {
    pub diameter_um: f64,
    pub blood_flow_ml_per_min: f64,
    pub nutrient_content: NutrientContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientContent {
    pub glucose_mg_per_dl: f64,
    pub amino_acids_mg_per_dl: f64,
    pub fatty_acids_mg_per_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BileDuct {
    pub diameter_um: f64,
    pub bile_flow_ml_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticBloodFlow {
    pub total_ml_per_min: f64,
    pub hepatic_artery_percent: f64,
    pub portal_vein_percent: f64,
    pub oxygen_extraction: f64,
}

impl HepaticBloodFlow {
    pub fn new() -> Self {
        Self {
            total_ml_per_min: 1500.0,
            hepatic_artery_percent: 25.0,
            portal_vein_percent: 75.0,
            oxygen_extraction: 0.3,
        }
    }

    pub fn hepatic_artery_flow(&self) -> f64 {
        self.total_ml_per_min * self.hepatic_artery_percent / 100.0
    }

    pub fn portal_vein_flow(&self) -> f64 {
        self.total_ml_per_min * self.portal_vein_percent / 100.0
    }
}

impl Default for HepaticBloodFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BileProduction {
    pub rate_ml_per_day: f64,
    pub composition: BileComposition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BileComposition {
    pub bile_salts_percent: f64,
    pub bilirubin_mg_per_dl: f64,
    pub cholesterol_mg_per_dl: f64,
    pub phospholipids_mg_per_dl: f64,
    pub ph: f64,
}

impl BileComposition {
    pub fn new() -> Self {
        Self {
            bile_salts_percent: 67.0,
            bilirubin_mg_per_dl: 0.3,
            cholesterol_mg_per_dl: 100.0,
            phospholipids_mg_per_dl: 300.0,
            ph: 7.8,
        }
    }
}

impl Default for BileComposition {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticMetabolism {
    pub glucose_regulation: GlucoseRegulation,
    pub protein_synthesis: ProteinSynthesis,
    pub lipid_metabolism: LipidMetabolism,
    pub detoxification: Detoxification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucoseRegulation {
    pub glycogen_storage_g: f64,
    pub gluconeogenesis_rate: f64,
    pub glycolysis_rate: f64,
}

impl GlucoseRegulation {
    pub fn new() -> Self {
        Self {
            glycogen_storage_g: 100.0,
            gluconeogenesis_rate: 1.0,
            glycolysis_rate: 1.0,
        }
    }
}

impl Default for GlucoseRegulation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinSynthesis {
    pub albumin_production_g_per_day: f64,
    pub clotting_factors: Vec<ClottingFactor>,
    pub acute_phase_proteins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClottingFactor {
    pub name: String,
    pub plasma_concentration: f64,
    pub half_life_hours: f64,
}

impl ProteinSynthesis {
    pub fn new() -> Self {
        Self {
            albumin_production_g_per_day: 12.0,
            clotting_factors: vec![
                ClottingFactor {
                    name: "Factor II (Prothrombin)".to_string(),
                    plasma_concentration: 100.0,
                    half_life_hours: 60.0,
                },
                ClottingFactor {
                    name: "Factor VII".to_string(),
                    plasma_concentration: 0.5,
                    half_life_hours: 6.0,
                },
                ClottingFactor {
                    name: "Factor IX".to_string(),
                    plasma_concentration: 4.0,
                    half_life_hours: 24.0,
                },
                ClottingFactor {
                    name: "Factor X".to_string(),
                    plasma_concentration: 10.0,
                    half_life_hours: 40.0,
                },
            ],
            acute_phase_proteins: vec![
                "C-reactive protein".to_string(),
                "Fibrinogen".to_string(),
                "Haptoglobin".to_string(),
            ],
        }
    }
}

impl Default for ProteinSynthesis {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidMetabolism {
    pub fatty_acid_oxidation_rate: f64,
    pub cholesterol_synthesis_mg_per_day: f64,
    pub lipoprotein_production: Vec<String>,
}

impl LipidMetabolism {
    pub fn new() -> Self {
        Self {
            fatty_acid_oxidation_rate: 1.0,
            cholesterol_synthesis_mg_per_day: 800.0,
            lipoprotein_production: vec!["VLDL".to_string(), "HDL".to_string()],
        }
    }
}

impl Default for LipidMetabolism {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detoxification {
    pub phase1_enzymes: Vec<CytochromeP450>,
    pub phase2_enzymes: Vec<ConjugationEnzyme>,
    pub ammonia_conversion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytochromeP450 {
    pub isoform: String,
    pub activity: f64,
    pub substrates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjugationEnzyme {
    pub name: String,
    pub activity: f64,
    pub conjugate: String,
}

impl Detoxification {
    pub fn new() -> Self {
        Self {
            phase1_enzymes: vec![
                CytochromeP450 {
                    isoform: "CYP3A4".to_string(),
                    activity: 1.0,
                    substrates: vec!["Drugs".to_string(), "Steroids".to_string()],
                },
                CytochromeP450 {
                    isoform: "CYP2D6".to_string(),
                    activity: 1.0,
                    substrates: vec!["Antidepressants".to_string()],
                },
            ],
            phase2_enzymes: vec![
                ConjugationEnzyme {
                    name: "UGT".to_string(),
                    activity: 1.0,
                    conjugate: "Glucuronide".to_string(),
                },
                ConjugationEnzyme {
                    name: "GST".to_string(),
                    activity: 1.0,
                    conjugate: "Glutathione".to_string(),
                },
            ],
            ammonia_conversion_rate: 1.0,
        }
    }
}

impl Default for Detoxification {
    fn default() -> Self {
        Self::new()
    }
}

impl Liver {
    pub fn new_adult() -> Self {
        Self {
            weight_g: 1500.0,
            lobes: vec![
                HepaticLobe::Right,
                HepaticLobe::Left,
                HepaticLobe::Caudate,
                HepaticLobe::Quadrate,
            ],
            lobules: Vec::new(),
            blood_flow: HepaticBloodFlow::new(),
            bile_production: BileProduction {
                rate_ml_per_day: 600.0,
                composition: BileComposition::new(),
            },
            metabolic_functions: HepaticMetabolism {
                glucose_regulation: GlucoseRegulation::new(),
                protein_synthesis: ProteinSynthesis::new(),
                lipid_metabolism: LipidMetabolism::new(),
                detoxification: Detoxification::new(),
            },
        }
    }

    pub fn percent_cardiac_output(&self) -> f64 {
        (self.blood_flow.total_ml_per_min / 5000.0) * 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiverFunctionTests {
    pub alt_u_per_l: f64,
    pub ast_u_per_l: f64,
    pub alkaline_phosphatase_u_per_l: f64,
    pub ggt_u_per_l: f64,
    pub bilirubin_total_mg_per_dl: f64,
    pub bilirubin_direct_mg_per_dl: f64,
    pub albumin_g_per_dl: f64,
    pub prothrombin_time_sec: f64,
    pub inr: f64,
}

impl LiverFunctionTests {
    pub fn new_normal() -> Self {
        Self {
            alt_u_per_l: 25.0,
            ast_u_per_l: 25.0,
            alkaline_phosphatase_u_per_l: 80.0,
            ggt_u_per_l: 30.0,
            bilirubin_total_mg_per_dl: 0.8,
            bilirubin_direct_mg_per_dl: 0.2,
            albumin_g_per_dl: 4.0,
            prothrombin_time_sec: 12.0,
            inr: 1.0,
        }
    }

    pub fn has_hepatocellular_injury(&self) -> bool {
        self.alt_u_per_l > 40.0 || self.ast_u_per_l > 40.0
    }

    pub fn has_cholestasis(&self) -> bool {
        self.alkaline_phosphatase_u_per_l > 120.0 || self.ggt_u_per_l > 50.0
    }

    pub fn has_synthetic_dysfunction(&self) -> bool {
        self.albumin_g_per_dl < 3.5 || self.inr > 1.2
    }

    pub fn bilirubin_indirect(&self) -> f64 {
        self.bilirubin_total_mg_per_dl - self.bilirubin_direct_mg_per_dl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liver_creation() {
        let liver = Liver::new_adult();
        assert_eq!(liver.weight_g, 1500.0);
        assert_eq!(liver.lobes.len(), 4);
    }

    #[test]
    fn test_hepatic_blood_flow() {
        let flow = HepaticBloodFlow::new();
        assert_eq!(flow.hepatic_artery_flow(), 375.0);
        assert_eq!(flow.portal_vein_flow(), 1125.0);
    }

    #[test]
    fn test_liver_function_tests() {
        let lft = LiverFunctionTests::new_normal();
        assert!(!lft.has_hepatocellular_injury());
        assert!(!lft.has_cholestasis());
        assert!(!lft.has_synthetic_dysfunction());
    }

    #[test]
    fn test_abnormal_lfts() {
        let mut lft = LiverFunctionTests::new_normal();
        lft.alt_u_per_l = 200.0;
        lft.ast_u_per_l = 180.0;
        assert!(lft.has_hepatocellular_injury());
    }

    #[test]
    fn test_liver_percent_cardiac_output() {
        let liver = Liver::new_adult();
        assert!((liver.percent_cardiac_output() - 30.0).abs() < 1.0);
    }
}
