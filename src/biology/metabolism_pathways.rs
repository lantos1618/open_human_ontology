use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicPathway {
    pub name: String,
    pub enzymes: Vec<Enzyme>,
    pub substrates: Vec<Substrate>,
    pub products: Vec<Product>,
    pub atp_yield: Option<ATPYield>,
    pub location: CellularLocation,
    pub regulation: PathwayRegulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enzyme {
    pub name: String,
    pub ec_number: String,
    pub km: f64,
    pub vmax: f64,
    pub cofactors: Vec<Cofactor>,
    pub activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substrate {
    pub name: String,
    pub concentration: f64,
    pub molecular_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub concentration: f64,
    pub molecular_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ATPYield {
    pub gross: f64,
    pub net: f64,
    pub nadh: f64,
    pub fadh2: f64,
    pub gtp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CellularLocation {
    Cytoplasm,
    Mitochondria,
    MitochondrialMatrix,
    InnerMitochondrialMembrane,
    EndoplasmicReticulum,
    Peroxisome,
    Lysosome,
    Nucleus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathwayRegulation {
    pub activators: Vec<String>,
    pub inhibitors: Vec<String>,
    pub feedback_loops: Vec<FeedbackLoop>,
    pub hormonal_control: Vec<HormonalRegulation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackLoop {
    pub molecule: String,
    pub loop_type: FeedbackType,
    pub target_enzyme: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FeedbackType {
    Negative,
    Positive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormonalRegulation {
    pub hormone: String,
    pub effect: RegulationEffect,
    pub mechanism: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RegulationEffect {
    Activation,
    Inhibition,
    Upregulation,
    Downregulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cofactor {
    NAD,
    NADH,
    NADP,
    NADPH,
    FAD,
    FADH2,
    CoA,
    ATP,
    ADP,
    ThiaminePyrophosphate,
    Biotin,
    PyridoxalPhosphate,
    Magnesium,
    Zinc,
    Iron,
    Copper,
    Manganese,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Glycolysis {
    pub glucose_concentration: f64,
    pub rate: f64,
    pub steps: Vec<GlycolysisStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlycolysisStep {
    pub step_number: u8,
    pub enzyme: String,
    pub substrate: String,
    pub product: String,
    pub atp_change: i8,
    pub nadh_produced: u8,
}

impl Glycolysis {
    pub fn new() -> Self {
        Self {
            glucose_concentration: 5.0,
            rate: 1.0,
            steps: vec![
                GlycolysisStep {
                    step_number: 1,
                    enzyme: "Hexokinase".to_string(),
                    substrate: "Glucose".to_string(),
                    product: "Glucose-6-phosphate".to_string(),
                    atp_change: -1,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 2,
                    enzyme: "Phosphoglucose isomerase".to_string(),
                    substrate: "Glucose-6-phosphate".to_string(),
                    product: "Fructose-6-phosphate".to_string(),
                    atp_change: 0,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 3,
                    enzyme: "Phosphofructokinase-1".to_string(),
                    substrate: "Fructose-6-phosphate".to_string(),
                    product: "Fructose-1,6-bisphosphate".to_string(),
                    atp_change: -1,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 4,
                    enzyme: "Aldolase".to_string(),
                    substrate: "Fructose-1,6-bisphosphate".to_string(),
                    product: "DHAP + G3P".to_string(),
                    atp_change: 0,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 5,
                    enzyme: "Triose phosphate isomerase".to_string(),
                    substrate: "DHAP".to_string(),
                    product: "G3P".to_string(),
                    atp_change: 0,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 6,
                    enzyme: "Glyceraldehyde-3-phosphate dehydrogenase".to_string(),
                    substrate: "G3P".to_string(),
                    product: "1,3-BPG".to_string(),
                    atp_change: 0,
                    nadh_produced: 2,
                },
                GlycolysisStep {
                    step_number: 7,
                    enzyme: "Phosphoglycerate kinase".to_string(),
                    substrate: "1,3-BPG".to_string(),
                    product: "3-Phosphoglycerate".to_string(),
                    atp_change: 2,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 8,
                    enzyme: "Phosphoglycerate mutase".to_string(),
                    substrate: "3-Phosphoglycerate".to_string(),
                    product: "2-Phosphoglycerate".to_string(),
                    atp_change: 0,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 9,
                    enzyme: "Enolase".to_string(),
                    substrate: "2-Phosphoglycerate".to_string(),
                    product: "Phosphoenolpyruvate".to_string(),
                    atp_change: 0,
                    nadh_produced: 0,
                },
                GlycolysisStep {
                    step_number: 10,
                    enzyme: "Pyruvate kinase".to_string(),
                    substrate: "Phosphoenolpyruvate".to_string(),
                    product: "Pyruvate".to_string(),
                    atp_change: 2,
                    nadh_produced: 0,
                },
            ],
        }
    }

    pub fn net_atp_yield(&self) -> i8 {
        self.steps.iter().map(|s| s.atp_change as i32).sum::<i32>() as i8
    }

    pub fn total_nadh(&self) -> u8 {
        self.steps.iter().map(|s| s.nadh_produced).sum()
    }
}

impl Default for Glycolysis {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitricAcidCycle {
    pub acetyl_coa_concentration: f64,
    pub rate: f64,
    pub steps: Vec<TCAStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCAStep {
    pub step_number: u8,
    pub enzyme: String,
    pub substrate: String,
    pub product: String,
    pub nadh_produced: u8,
    pub fadh2_produced: u8,
    pub gtp_produced: u8,
    pub co2_released: u8,
}

impl CitricAcidCycle {
    pub fn new() -> Self {
        Self {
            acetyl_coa_concentration: 1.0,
            rate: 1.0,
            steps: vec![
                TCAStep {
                    step_number: 1,
                    enzyme: "Citrate synthase".to_string(),
                    substrate: "Acetyl-CoA + Oxaloacetate".to_string(),
                    product: "Citrate".to_string(),
                    nadh_produced: 0,
                    fadh2_produced: 0,
                    gtp_produced: 0,
                    co2_released: 0,
                },
                TCAStep {
                    step_number: 2,
                    enzyme: "Aconitase".to_string(),
                    substrate: "Citrate".to_string(),
                    product: "Isocitrate".to_string(),
                    nadh_produced: 0,
                    fadh2_produced: 0,
                    gtp_produced: 0,
                    co2_released: 0,
                },
                TCAStep {
                    step_number: 3,
                    enzyme: "Isocitrate dehydrogenase".to_string(),
                    substrate: "Isocitrate".to_string(),
                    product: "α-Ketoglutarate".to_string(),
                    nadh_produced: 1,
                    fadh2_produced: 0,
                    gtp_produced: 0,
                    co2_released: 1,
                },
                TCAStep {
                    step_number: 4,
                    enzyme: "α-Ketoglutarate dehydrogenase".to_string(),
                    substrate: "α-Ketoglutarate".to_string(),
                    product: "Succinyl-CoA".to_string(),
                    nadh_produced: 1,
                    fadh2_produced: 0,
                    gtp_produced: 0,
                    co2_released: 1,
                },
                TCAStep {
                    step_number: 5,
                    enzyme: "Succinyl-CoA synthetase".to_string(),
                    substrate: "Succinyl-CoA".to_string(),
                    product: "Succinate".to_string(),
                    nadh_produced: 0,
                    fadh2_produced: 0,
                    gtp_produced: 1,
                    co2_released: 0,
                },
                TCAStep {
                    step_number: 6,
                    enzyme: "Succinate dehydrogenase".to_string(),
                    substrate: "Succinate".to_string(),
                    product: "Fumarate".to_string(),
                    nadh_produced: 0,
                    fadh2_produced: 1,
                    gtp_produced: 0,
                    co2_released: 0,
                },
                TCAStep {
                    step_number: 7,
                    enzyme: "Fumarase".to_string(),
                    substrate: "Fumarate".to_string(),
                    product: "Malate".to_string(),
                    nadh_produced: 0,
                    fadh2_produced: 0,
                    gtp_produced: 0,
                    co2_released: 0,
                },
                TCAStep {
                    step_number: 8,
                    enzyme: "Malate dehydrogenase".to_string(),
                    substrate: "Malate".to_string(),
                    product: "Oxaloacetate".to_string(),
                    nadh_produced: 1,
                    fadh2_produced: 0,
                    gtp_produced: 0,
                    co2_released: 0,
                },
            ],
        }
    }

    pub fn total_nadh(&self) -> u8 {
        self.steps.iter().map(|s| s.nadh_produced).sum()
    }

    pub fn total_fadh2(&self) -> u8 {
        self.steps.iter().map(|s| s.fadh2_produced).sum()
    }

    pub fn total_gtp(&self) -> u8 {
        self.steps.iter().map(|s| s.gtp_produced).sum()
    }

    pub fn total_co2(&self) -> u8 {
        self.steps.iter().map(|s| s.co2_released).sum()
    }
}

impl Default for CitricAcidCycle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxidativePhosphorylation {
    pub nadh_input: f64,
    pub fadh2_input: f64,
    pub atp_output: f64,
    pub complexes: Vec<ETCComplex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ETCComplex {
    pub name: String,
    pub complex_number: u8,
    pub protons_pumped: u8,
    pub electrons_transferred: u8,
}

impl OxidativePhosphorylation {
    pub fn new() -> Self {
        Self {
            nadh_input: 0.0,
            fadh2_input: 0.0,
            atp_output: 0.0,
            complexes: vec![
                ETCComplex {
                    name: "NADH dehydrogenase".to_string(),
                    complex_number: 1,
                    protons_pumped: 4,
                    electrons_transferred: 2,
                },
                ETCComplex {
                    name: "Succinate dehydrogenase".to_string(),
                    complex_number: 2,
                    protons_pumped: 0,
                    electrons_transferred: 2,
                },
                ETCComplex {
                    name: "Cytochrome bc1".to_string(),
                    complex_number: 3,
                    protons_pumped: 4,
                    electrons_transferred: 2,
                },
                ETCComplex {
                    name: "Cytochrome c oxidase".to_string(),
                    complex_number: 4,
                    protons_pumped: 2,
                    electrons_transferred: 2,
                },
            ],
        }
    }

    pub fn calculate_atp_from_nadh(&self, nadh: f64) -> f64 {
        nadh * 2.5
    }

    pub fn calculate_atp_from_fadh2(&self, fadh2: f64) -> f64 {
        fadh2 * 1.5
    }

    pub fn total_atp(&mut self, nadh: f64, fadh2: f64) -> f64 {
        self.nadh_input = nadh;
        self.fadh2_input = fadh2;
        self.atp_output = self.calculate_atp_from_nadh(nadh) + self.calculate_atp_from_fadh2(fadh2);
        self.atp_output
    }
}

impl Default for OxidativePhosphorylation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gluconeogenesis {
    pub pyruvate_concentration: f64,
    pub glucose_output: f64,
    pub rate: f64,
}

impl Gluconeogenesis {
    pub fn new() -> Self {
        Self {
            pyruvate_concentration: 0.0,
            glucose_output: 0.0,
            rate: 0.0,
        }
    }

    pub fn atp_cost_per_glucose(&self) -> u8 {
        6
    }

    pub fn gtp_cost_per_glucose(&self) -> u8 {
        2
    }
}

impl Default for Gluconeogenesis {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlycogenMetabolism {
    pub glycogen_stores: f64,
    pub synthesis_rate: f64,
    pub breakdown_rate: f64,
}

impl GlycogenMetabolism {
    pub fn new(initial_stores: f64) -> Self {
        Self {
            glycogen_stores: initial_stores,
            synthesis_rate: 0.0,
            breakdown_rate: 0.0,
        }
    }

    pub fn synthesize(&mut self, glucose_units: f64) {
        self.glycogen_stores += glucose_units;
        self.synthesis_rate = glucose_units;
    }

    pub fn breakdown(&mut self, glucose_units: f64) -> f64 {
        let released = glucose_units.min(self.glycogen_stores);
        self.glycogen_stores -= released;
        self.breakdown_rate = released;
        released
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FattyAcidOxidation {
    pub fatty_acid_length: u8,
    pub cycles: u8,
    pub acetyl_coa_produced: u8,
}

impl FattyAcidOxidation {
    pub fn new(carbon_atoms: u8) -> Self {
        let cycles = (carbon_atoms / 2) - 1;
        let acetyl_coa_produced = carbon_atoms / 2;

        Self {
            fatty_acid_length: carbon_atoms,
            cycles,
            acetyl_coa_produced,
        }
    }

    pub fn nadh_produced(&self) -> u8 {
        self.cycles
    }

    pub fn fadh2_produced(&self) -> u8 {
        self.cycles
    }

    pub fn total_atp_yield(&self) -> f64 {
        let nadh_atp = self.nadh_produced() as f64 * 2.5;
        let fadh2_atp = self.fadh2_produced() as f64 * 1.5;
        let acetyl_coa_atp = self.acetyl_coa_produced as f64 * 10.0;
        let activation_cost = 2.0;

        nadh_atp + fadh2_atp + acetyl_coa_atp - activation_cost
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PentosePhosphatePathway {
    pub oxidative_phase_active: bool,
    pub non_oxidative_phase_active: bool,
    pub nadph_produced: f64,
    pub ribose_5p_produced: f64,
}

impl PentosePhosphatePathway {
    pub fn new() -> Self {
        Self {
            oxidative_phase_active: true,
            non_oxidative_phase_active: true,
            nadph_produced: 0.0,
            ribose_5p_produced: 0.0,
        }
    }

    pub fn oxidative_phase_nadph_per_glucose(&self) -> u8 {
        2
    }
}

impl Default for PentosePhosphatePathway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glycolysis_atp_yield() {
        let glycolysis = Glycolysis::new();
        assert_eq!(glycolysis.net_atp_yield(), 2);
    }

    #[test]
    fn test_glycolysis_nadh() {
        let glycolysis = Glycolysis::new();
        assert_eq!(glycolysis.total_nadh(), 2);
    }

    #[test]
    fn test_tca_cycle() {
        let tca = CitricAcidCycle::new();
        assert_eq!(tca.total_nadh(), 3);
        assert_eq!(tca.total_fadh2(), 1);
        assert_eq!(tca.total_gtp(), 1);
        assert_eq!(tca.total_co2(), 2);
    }

    #[test]
    fn test_oxidative_phosphorylation() {
        let mut oxphos = OxidativePhosphorylation::new();
        let atp = oxphos.total_atp(10.0, 2.0);
        assert_eq!(atp, 28.0);
    }

    #[test]
    fn test_complete_glucose_oxidation() {
        let glycolysis = Glycolysis::new();
        let tca = CitricAcidCycle::new();
        let mut oxphos = OxidativePhosphorylation::new();

        assert_eq!(glycolysis.net_atp_yield(), 2);
        assert_eq!(glycolysis.total_nadh(), 2);
        assert_eq!(tca.total_nadh(), 3);
        assert_eq!(tca.total_fadh2(), 1);
        assert_eq!(tca.total_gtp(), 1);

        let glycolysis_atp = glycolysis.net_atp_yield() as f64;
        let glycolysis_nadh = glycolysis.total_nadh() as f64;

        let tca_nadh = tca.total_nadh() as f64 * 2.0;
        let tca_fadh2 = tca.total_fadh2() as f64 * 2.0;
        let tca_gtp = tca.total_gtp() as f64 * 2.0;

        let oxphos_atp = oxphos.total_atp(glycolysis_nadh + tca_nadh, tca_fadh2);

        let total_atp = glycolysis_atp + tca_gtp + oxphos_atp;

        assert!(total_atp > 25.0);
    }

    #[test]
    fn test_fatty_acid_oxidation() {
        let palmitate = FattyAcidOxidation::new(16);
        assert_eq!(palmitate.cycles, 7);
        assert_eq!(palmitate.acetyl_coa_produced, 8);
        assert_eq!(palmitate.nadh_produced(), 7);
        assert_eq!(palmitate.fadh2_produced(), 7);
        assert!((palmitate.total_atp_yield() - 106.0).abs() < 1.0);
    }

    #[test]
    fn test_glycogen_metabolism() {
        let mut glycogen = GlycogenMetabolism::new(100.0);
        glycogen.synthesize(50.0);
        assert_eq!(glycogen.glycogen_stores, 150.0);

        let released = glycogen.breakdown(30.0);
        assert_eq!(released, 30.0);
        assert_eq!(glycogen.glycogen_stores, 120.0);
    }

    #[test]
    fn test_gluconeogenesis_costs() {
        let gluconeo = Gluconeogenesis::new();
        assert_eq!(gluconeo.atp_cost_per_glucose(), 6);
        assert_eq!(gluconeo.gtp_cost_per_glucose(), 2);
    }
}
