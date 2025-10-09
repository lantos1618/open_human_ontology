use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BloodType {
    APositive,
    ANegative,
    BPositive,
    BNegative,
    ABPositive,
    ABNegative,
    OPositive,
    ONegative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BloodCell {
    RedBloodCell,
    Neutrophil,
    Lymphocyte,
    Monocyte,
    Eosinophil,
    Basophil,
    Platelet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoagulationFactor {
    pub factor: ClottingFactor,
    pub concentration_ug_ml: f64,
    pub activity_percent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClottingFactor {
    FactorI,
    FactorII,
    FactorV,
    FactorVII,
    FactorVIII,
    FactorIX,
    FactorX,
    FactorXI,
    FactorXII,
    FactorXIII,
    VonWillebrandFactor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodProtein {
    pub name: String,
    pub concentration_g_l: f64,
    pub function: ProteinFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProteinFunction {
    Transport,
    Immune,
    Coagulation,
    Enzymatic,
    Structural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Immunoglobulin {
    pub ig_type: ImmunoglobulinType,
    pub concentration_mg_dl: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImmunoglobulinType {
    IgG,
    IgA,
    IgM,
    IgE,
    IgD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lipoproteins {
    pub total_cholesterol_mg_dl: f64,
    pub ldl_mg_dl: f64,
    pub hdl_mg_dl: f64,
    pub vldl_mg_dl: f64,
    pub triglycerides_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiverEnzymes {
    pub alt_u_l: f64,
    pub ast_u_l: f64,
    pub alp_u_l: f64,
    pub ggt_u_l: f64,
    pub ldh_u_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiacMarkers {
    pub troponin_i_ng_ml: f64,
    pub troponin_t_ng_ml: f64,
    pub ck_mb_ng_ml: f64,
    pub bnp_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflammatoryMarkers {
    pub crp_mg_l: f64,
    pub esr_mm_hr: f64,
    pub procalcitonin_ng_ml: f64,
    pub ferritin_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Electrolytes {
    pub sodium_meq_l: f64,
    pub potassium_meq_l: f64,
    pub chloride_meq_l: f64,
    pub bicarbonate_meq_l: f64,
    pub calcium_mg_dl: f64,
    pub magnesium_mg_dl: f64,
    pub phosphate_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaComposition {
    pub volume_ml: f64,
    pub protein_g_dl: f64,
    pub albumin_g_dl: f64,
    pub globulin_g_dl: f64,
    pub fibrinogen_mg_dl: f64,
    pub glucose_mg_dl: f64,
    pub sodium_meq_l: f64,
    pub potassium_meq_l: f64,
    pub calcium_mg_dl: f64,
    pub chloride_meq_l: f64,
    pub bicarbonate_meq_l: f64,
    pub urea_nitrogen_mg_dl: f64,
    pub creatinine_mg_dl: f64,
    pub bilirubin_mg_dl: f64,
    pub electrolytes: Electrolytes,
    pub lipoproteins: Lipoproteins,
    pub liver_enzymes: LiverEnzymes,
    pub cardiac_markers: CardiacMarkers,
    pub inflammatory_markers: InflammatoryMarkers,
    pub immunoglobulins: Vec<Immunoglobulin>,
    pub coagulation_factors: Vec<CoagulationFactor>,
    pub proteins: Vec<BloodProtein>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellCount {
    pub cell_type: BloodCell,
    pub count_per_ul: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BloodComponent {
    Plasma(PlasmaComposition),
    Cells(CellCount),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blood {
    pub blood_type: BloodType,
    pub volume_ml: f64,
    pub hematocrit: f64,
    pub hemoglobin_g_dl: f64,
    pub ph: f64,
    pub oxygen_saturation: f64,
    pub components: Vec<BloodComponent>,
}

impl Blood {
    pub fn new(blood_type: BloodType) -> Self {
        Blood {
            blood_type,
            volume_ml: 5000.0,
            hematocrit: 0.45,
            hemoglobin_g_dl: 15.0,
            ph: 7.4,
            oxygen_saturation: 0.97,
            components: vec![
                BloodComponent::Plasma(PlasmaComposition::new_normal()),
                BloodComponent::Cells(CellCount {
                    cell_type: BloodCell::RedBloodCell,
                    count_per_ul: 5_000_000,
                }),
                BloodComponent::Cells(CellCount {
                    cell_type: BloodCell::Neutrophil,
                    count_per_ul: 4_000,
                }),
                BloodComponent::Cells(CellCount {
                    cell_type: BloodCell::Lymphocyte,
                    count_per_ul: 2_500,
                }),
                BloodComponent::Cells(CellCount {
                    cell_type: BloodCell::Monocyte,
                    count_per_ul: 500,
                }),
                BloodComponent::Cells(CellCount {
                    cell_type: BloodCell::Eosinophil,
                    count_per_ul: 200,
                }),
                BloodComponent::Cells(CellCount {
                    cell_type: BloodCell::Basophil,
                    count_per_ul: 50,
                }),
                BloodComponent::Cells(CellCount {
                    cell_type: BloodCell::Platelet,
                    count_per_ul: 250_000,
                }),
            ],
        }
    }

    pub fn oxygen_content_ml(&self) -> f64 {
        let hb_oxygen_capacity = 1.34;
        let oxygen_ml_per_dl = self.hemoglobin_g_dl * hb_oxygen_capacity * self.oxygen_saturation;
        (oxygen_ml_per_dl * self.volume_ml) / 100.0
    }

    pub fn is_anemic(&self) -> bool {
        self.hemoglobin_g_dl < 12.0
    }

    pub fn is_polycythemic(&self) -> bool {
        self.hematocrit > 0.55
    }

    pub fn white_blood_cell_count(&self) -> u64 {
        self.components
            .iter()
            .filter_map(|c| match c {
                BloodComponent::Cells(cell_count) => {
                    match cell_count.cell_type {
                        BloodCell::Neutrophil |
                        BloodCell::Lymphocyte |
                        BloodCell::Monocyte |
                        BloodCell::Eosinophil |
                        BloodCell::Basophil => Some(cell_count.count_per_ul),
                        _ => None,
                    }
                }
                _ => None,
            })
            .sum()
    }

    pub fn get_plasma_composition(&self) -> Option<&PlasmaComposition> {
        self.components.iter().find_map(|c| match c {
            BloodComponent::Plasma(plasma) => Some(plasma),
            _ => None,
        })
    }

    pub fn get_cell_count(&self, cell_type: BloodCell) -> u64 {
        self.components
            .iter()
            .filter_map(|c| match c {
                BloodComponent::Cells(cell_count) if cell_count.cell_type == cell_type => {
                    Some(cell_count.count_per_ul)
                }
                _ => None,
            })
            .sum()
    }

    pub fn platelet_count(&self) -> u64 {
        self.get_cell_count(BloodCell::Platelet)
    }

    pub fn has_thrombocytopenia(&self) -> bool {
        self.platelet_count() < 150_000
    }

    pub fn has_leukocytosis(&self) -> bool {
        self.white_blood_cell_count() > 11_000
    }

    pub fn has_leukopenia(&self) -> bool {
        self.white_blood_cell_count() < 4_000
    }

    pub fn can_donate_to(&self, recipient: BloodType) -> bool {
        match (self.blood_type, recipient) {
            (BloodType::ONegative, _) => true,
            (BloodType::OPositive, BloodType::OPositive) => true,
            (BloodType::OPositive, BloodType::APositive) => true,
            (BloodType::OPositive, BloodType::BPositive) => true,
            (BloodType::OPositive, BloodType::ABPositive) => true,
            (BloodType::ANegative, BloodType::ANegative) => true,
            (BloodType::ANegative, BloodType::APositive) => true,
            (BloodType::ANegative, BloodType::ABNegative) => true,
            (BloodType::ANegative, BloodType::ABPositive) => true,
            (BloodType::APositive, BloodType::APositive) => true,
            (BloodType::APositive, BloodType::ABPositive) => true,
            (BloodType::BNegative, BloodType::BNegative) => true,
            (BloodType::BNegative, BloodType::BPositive) => true,
            (BloodType::BNegative, BloodType::ABNegative) => true,
            (BloodType::BNegative, BloodType::ABPositive) => true,
            (BloodType::BPositive, BloodType::BPositive) => true,
            (BloodType::BPositive, BloodType::ABPositive) => true,
            (BloodType::ABNegative, BloodType::ABNegative) => true,
            (BloodType::ABNegative, BloodType::ABPositive) => true,
            (BloodType::ABPositive, BloodType::ABPositive) => true,
            _ => false,
        }
    }
}

impl PlasmaComposition {
    pub fn new_normal() -> Self {
        Self {
            volume_ml: 2750.0,
            protein_g_dl: 7.0,
            albumin_g_dl: 4.0,
            globulin_g_dl: 2.5,
            fibrinogen_mg_dl: 300.0,
            glucose_mg_dl: 90.0,
            sodium_meq_l: 140.0,
            potassium_meq_l: 4.0,
            calcium_mg_dl: 9.5,
            chloride_meq_l: 103.0,
            bicarbonate_meq_l: 24.0,
            urea_nitrogen_mg_dl: 14.0,
            creatinine_mg_dl: 1.0,
            bilirubin_mg_dl: 0.8,
            electrolytes: Electrolytes::new_normal(),
            lipoproteins: Lipoproteins::new_normal(),
            liver_enzymes: LiverEnzymes::new_normal(),
            cardiac_markers: CardiacMarkers::new_normal(),
            inflammatory_markers: InflammatoryMarkers::new_normal(),
            immunoglobulins: Immunoglobulin::new_normal_panel(),
            coagulation_factors: CoagulationFactor::new_normal_panel(),
            proteins: BloodProtein::new_normal_panel(),
        }
    }

    pub fn osmolality_mosm_kg(&self) -> f64 {
        2.0 * self.sodium_meq_l + self.glucose_mg_dl / 18.0 + self.urea_nitrogen_mg_dl / 2.8
    }

    pub fn anion_gap(&self) -> f64 {
        self.sodium_meq_l - (self.chloride_meq_l + self.bicarbonate_meq_l)
    }

    pub fn is_hyponatremic(&self) -> bool {
        self.sodium_meq_l < 135.0
    }

    pub fn is_hypernatremic(&self) -> bool {
        self.sodium_meq_l > 145.0
    }

    pub fn is_hypokalemic(&self) -> bool {
        self.potassium_meq_l < 3.5
    }

    pub fn is_hyperkalemic(&self) -> bool {
        self.potassium_meq_l > 5.0
    }

    pub fn has_elevated_creatinine(&self) -> bool {
        self.creatinine_mg_dl > 1.3
    }

    pub fn has_hyperglycemia(&self) -> bool {
        self.glucose_mg_dl > 125.0
    }

    pub fn has_hypoglycemia(&self) -> bool {
        self.glucose_mg_dl < 70.0
    }
}

impl Electrolytes {
    pub fn new_normal() -> Self {
        Self {
            sodium_meq_l: 140.0,
            potassium_meq_l: 4.0,
            chloride_meq_l: 103.0,
            bicarbonate_meq_l: 24.0,
            calcium_mg_dl: 9.5,
            magnesium_mg_dl: 2.0,
            phosphate_mg_dl: 3.5,
        }
    }

    pub fn anion_gap(&self) -> f64 {
        self.sodium_meq_l - (self.chloride_meq_l + self.bicarbonate_meq_l)
    }
}

impl Lipoproteins {
    pub fn new_normal() -> Self {
        Self {
            total_cholesterol_mg_dl: 180.0,
            ldl_mg_dl: 100.0,
            hdl_mg_dl: 50.0,
            vldl_mg_dl: 30.0,
            triglycerides_mg_dl: 150.0,
        }
    }

    pub fn calculate_vldl(&self) -> f64 {
        self.triglycerides_mg_dl / 5.0
    }

    pub fn ldl_hdl_ratio(&self) -> f64 {
        self.ldl_mg_dl / self.hdl_mg_dl
    }

    pub fn has_dyslipidemia(&self) -> bool {
        self.total_cholesterol_mg_dl > 200.0 || self.ldl_mg_dl > 130.0 || self.hdl_mg_dl < 40.0
    }
}

impl LiverEnzymes {
    pub fn new_normal() -> Self {
        Self {
            alt_u_l: 30.0,
            ast_u_l: 30.0,
            alp_u_l: 70.0,
            ggt_u_l: 25.0,
            ldh_u_l: 150.0,
        }
    }

    pub fn ast_alt_ratio(&self) -> f64 {
        self.ast_u_l / self.alt_u_l
    }

    pub fn has_hepatocellular_injury(&self) -> bool {
        self.alt_u_l > 100.0 || self.ast_u_l > 100.0
    }

    pub fn has_cholestasis(&self) -> bool {
        self.alp_u_l > 120.0 || self.ggt_u_l > 50.0
    }
}

impl CardiacMarkers {
    pub fn new_normal() -> Self {
        Self {
            troponin_i_ng_ml: 0.01,
            troponin_t_ng_ml: 0.01,
            ck_mb_ng_ml: 3.0,
            bnp_pg_ml: 50.0,
        }
    }

    pub fn has_myocardial_injury(&self) -> bool {
        self.troponin_i_ng_ml > 0.04 || self.troponin_t_ng_ml > 0.01
    }

    pub fn has_heart_failure(&self) -> bool {
        self.bnp_pg_ml > 100.0
    }
}

impl InflammatoryMarkers {
    pub fn new_normal() -> Self {
        Self {
            crp_mg_l: 1.0,
            esr_mm_hr: 10.0,
            procalcitonin_ng_ml: 0.05,
            ferritin_ng_ml: 100.0,
        }
    }

    pub fn has_inflammation(&self) -> bool {
        self.crp_mg_l > 3.0 || self.esr_mm_hr > 20.0
    }

    pub fn has_severe_inflammation(&self) -> bool {
        self.crp_mg_l > 10.0
    }

    pub fn has_bacterial_infection(&self) -> bool {
        self.procalcitonin_ng_ml > 0.5
    }
}

impl Immunoglobulin {
    pub fn new_normal_panel() -> Vec<Self> {
        vec![
            Self { ig_type: ImmunoglobulinType::IgG, concentration_mg_dl: 1000.0 },
            Self { ig_type: ImmunoglobulinType::IgA, concentration_mg_dl: 200.0 },
            Self { ig_type: ImmunoglobulinType::IgM, concentration_mg_dl: 100.0 },
            Self { ig_type: ImmunoglobulinType::IgE, concentration_mg_dl: 0.05 },
            Self { ig_type: ImmunoglobulinType::IgD, concentration_mg_dl: 3.0 },
        ]
    }
}

impl CoagulationFactor {
    pub fn new_normal_panel() -> Vec<Self> {
        vec![
            Self { factor: ClottingFactor::FactorI, concentration_ug_ml: 3000.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorII, concentration_ug_ml: 100.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorV, concentration_ug_ml: 10.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorVII, concentration_ug_ml: 0.5, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorVIII, concentration_ug_ml: 0.1, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorIX, concentration_ug_ml: 5.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorX, concentration_ug_ml: 10.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorXI, concentration_ug_ml: 5.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorXII, concentration_ug_ml: 30.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::FactorXIII, concentration_ug_ml: 20.0, activity_percent: 100.0 },
            Self { factor: ClottingFactor::VonWillebrandFactor, concentration_ug_ml: 10.0, activity_percent: 100.0 },
        ]
    }
}

impl BloodProtein {
    pub fn new_normal_panel() -> Vec<Self> {
        vec![
            Self {
                name: "Albumin".to_string(),
                concentration_g_l: 40.0,
                function: ProteinFunction::Transport,
            },
            Self {
                name: "Transferrin".to_string(),
                concentration_g_l: 2.5,
                function: ProteinFunction::Transport,
            },
            Self {
                name: "Ceruloplasmin".to_string(),
                concentration_g_l: 0.3,
                function: ProteinFunction::Transport,
            },
            Self {
                name: "Haptoglobin".to_string(),
                concentration_g_l: 1.0,
                function: ProteinFunction::Transport,
            },
            Self {
                name: "Alpha-1-antitrypsin".to_string(),
                concentration_g_l: 1.5,
                function: ProteinFunction::Enzymatic,
            },
            Self {
                name: "C3 Complement".to_string(),
                concentration_g_l: 1.2,
                function: ProteinFunction::Immune,
            },
            Self {
                name: "C4 Complement".to_string(),
                concentration_g_l: 0.3,
                function: ProteinFunction::Immune,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blood_creation() {
        let blood = Blood::new(BloodType::OPositive);
        assert_eq!(blood.blood_type, BloodType::OPositive);
        assert!(blood.volume_ml > 0.0);
    }

    #[test]
    fn test_oxygen_content() {
        let blood = Blood::new(BloodType::OPositive);
        let o2 = blood.oxygen_content_ml();
        assert!(o2 > 0.0);
    }

    #[test]
    fn test_anemia() {
        let mut blood = Blood::new(BloodType::OPositive);
        blood.hemoglobin_g_dl = 10.0;
        assert!(blood.is_anemic());
    }

    #[test]
    fn test_wbc_count() {
        let blood = Blood::new(BloodType::OPositive);
        let wbc = blood.white_blood_cell_count();
        assert!(wbc > 0);
    }

    #[test]
    fn test_blood_donation() {
        let o_neg = Blood::new(BloodType::ONegative);
        assert!(o_neg.can_donate_to(BloodType::APositive));
        assert!(o_neg.can_donate_to(BloodType::ONegative));

        let ab_pos = Blood::new(BloodType::ABPositive);
        assert!(ab_pos.can_donate_to(BloodType::ABPositive));
        assert!(!ab_pos.can_donate_to(BloodType::OPositive));
    }
}
