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
