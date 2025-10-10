use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Biomarker {
    Glucose,
    HbA1c,
    TotalCholesterol,
    LDLCholesterol,
    HDLCholesterol,
    Triglycerides,
    CReactiveProtein,
    ALT,
    AST,
    Creatinine,
    BUN,
    TSH,
    FreeT4,
    VitaminD,
    VitaminB12,
    Hemoglobin,
    WBC,
    Platelets,
    Albumin,
    BNP,
    Troponin,
    PSA,
    Ferritin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerValue {
    pub value: f64,
    pub unit: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceRange {
    pub low: f64,
    pub high: f64,
    pub optimal_low: Option<f64>,
    pub optimal_high: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerPanel {
    pub name: String,
    pub markers: HashMap<Biomarker, BiomarkerValue>,
}

impl Biomarker {
    pub fn reference_range(&self) -> ReferenceRange {
        match self {
            Biomarker::Glucose => ReferenceRange {
                low: 70.0,
                high: 100.0,
                optimal_low: Some(80.0),
                optimal_high: Some(95.0),
            },
            Biomarker::HbA1c => ReferenceRange {
                low: 4.0,
                high: 5.6,
                optimal_low: Some(4.5),
                optimal_high: Some(5.3),
            },
            Biomarker::TotalCholesterol => ReferenceRange {
                low: 125.0,
                high: 200.0,
                optimal_low: Some(150.0),
                optimal_high: Some(180.0),
            },
            Biomarker::LDLCholesterol => ReferenceRange {
                low: 0.0,
                high: 100.0,
                optimal_low: Some(50.0),
                optimal_high: Some(80.0),
            },
            Biomarker::HDLCholesterol => ReferenceRange {
                low: 40.0,
                high: 200.0,
                optimal_low: Some(60.0),
                optimal_high: Some(80.0),
            },
            Biomarker::Triglycerides => ReferenceRange {
                low: 0.0,
                high: 150.0,
                optimal_low: Some(50.0),
                optimal_high: Some(100.0),
            },
            Biomarker::CReactiveProtein => ReferenceRange {
                low: 0.0,
                high: 3.0,
                optimal_low: Some(0.0),
                optimal_high: Some(1.0),
            },
            Biomarker::ALT => ReferenceRange {
                low: 7.0,
                high: 56.0,
                optimal_low: Some(10.0),
                optimal_high: Some(30.0),
            },
            Biomarker::AST => ReferenceRange {
                low: 10.0,
                high: 40.0,
                optimal_low: Some(15.0),
                optimal_high: Some(30.0),
            },
            Biomarker::Creatinine => ReferenceRange {
                low: 0.7,
                high: 1.3,
                optimal_low: Some(0.8),
                optimal_high: Some(1.1),
            },
            Biomarker::BUN => ReferenceRange {
                low: 7.0,
                high: 20.0,
                optimal_low: Some(10.0),
                optimal_high: Some(16.0),
            },
            Biomarker::TSH => ReferenceRange {
                low: 0.4,
                high: 4.0,
                optimal_low: Some(1.0),
                optimal_high: Some(2.5),
            },
            Biomarker::VitaminD => ReferenceRange {
                low: 20.0,
                high: 100.0,
                optimal_low: Some(40.0),
                optimal_high: Some(60.0),
            },
            Biomarker::VitaminB12 => ReferenceRange {
                low: 200.0,
                high: 900.0,
                optimal_low: Some(400.0),
                optimal_high: Some(700.0),
            },
            Biomarker::Hemoglobin => ReferenceRange {
                low: 12.0,
                high: 17.0,
                optimal_low: Some(13.5),
                optimal_high: Some(16.0),
            },
            Biomarker::WBC => ReferenceRange {
                low: 4000.0,
                high: 11000.0,
                optimal_low: Some(5000.0),
                optimal_high: Some(9000.0),
            },
            Biomarker::Platelets => ReferenceRange {
                low: 150000.0,
                high: 400000.0,
                optimal_low: Some(200000.0),
                optimal_high: Some(350000.0),
            },
            Biomarker::Albumin => ReferenceRange {
                low: 3.5,
                high: 5.5,
                optimal_low: Some(4.0),
                optimal_high: Some(5.0),
            },
            Biomarker::BNP => ReferenceRange {
                low: 0.0,
                high: 100.0,
                optimal_low: Some(0.0),
                optimal_high: Some(50.0),
            },
            Biomarker::Troponin => ReferenceRange {
                low: 0.0,
                high: 0.04,
                optimal_low: Some(0.0),
                optimal_high: Some(0.01),
            },
            Biomarker::PSA => ReferenceRange {
                low: 0.0,
                high: 4.0,
                optimal_low: Some(0.0),
                optimal_high: Some(2.5),
            },
            Biomarker::Ferritin => ReferenceRange {
                low: 20.0,
                high: 300.0,
                optimal_low: Some(50.0),
                optimal_high: Some(150.0),
            },
            Biomarker::FreeT4 => ReferenceRange {
                low: 0.8,
                high: 1.8,
                optimal_low: Some(1.0),
                optimal_high: Some(1.5),
            },
        }
    }

    pub fn default_unit(&self) -> &str {
        match self {
            Biomarker::Glucose => "mg/dL",
            Biomarker::HbA1c => "%",
            Biomarker::TotalCholesterol => "mg/dL",
            Biomarker::LDLCholesterol => "mg/dL",
            Biomarker::HDLCholesterol => "mg/dL",
            Biomarker::Triglycerides => "mg/dL",
            Biomarker::CReactiveProtein => "mg/L",
            Biomarker::ALT => "U/L",
            Biomarker::AST => "U/L",
            Biomarker::Creatinine => "mg/dL",
            Biomarker::BUN => "mg/dL",
            Biomarker::TSH => "mIU/L",
            Biomarker::FreeT4 => "ng/dL",
            Biomarker::VitaminD => "ng/mL",
            Biomarker::VitaminB12 => "pg/mL",
            Biomarker::Hemoglobin => "g/dL",
            Biomarker::WBC => "cells/μL",
            Biomarker::Platelets => "cells/μL",
            Biomarker::Albumin => "g/dL",
            Biomarker::BNP => "pg/mL",
            Biomarker::Troponin => "ng/mL",
            Biomarker::PSA => "ng/mL",
            Biomarker::Ferritin => "ng/mL",
        }
    }
}

impl BiomarkerValue {
    pub fn new(value: f64, unit: String) -> Self {
        Self {
            value,
            unit,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    pub fn is_in_range(&self, range: &ReferenceRange) -> bool {
        self.value >= range.low && self.value <= range.high
    }

    pub fn is_optimal(&self, range: &ReferenceRange) -> bool {
        if let (Some(opt_low), Some(opt_high)) = (range.optimal_low, range.optimal_high) {
            self.value >= opt_low && self.value <= opt_high
        } else {
            self.is_in_range(range)
        }
    }

    pub fn is_low(&self, range: &ReferenceRange) -> bool {
        self.value < range.low
    }

    pub fn is_high(&self, range: &ReferenceRange) -> bool {
        self.value > range.high
    }
}

impl BiomarkerPanel {
    pub fn new(name: String) -> Self {
        Self {
            name,
            markers: HashMap::new(),
        }
    }

    pub fn basic_metabolic_panel() -> Self {
        let mut panel = Self::new("Basic Metabolic Panel".to_string());
        panel.add_marker(Biomarker::Glucose, 90.0);
        panel.add_marker(Biomarker::Creatinine, 1.0);
        panel.add_marker(Biomarker::BUN, 14.0);
        panel
    }

    pub fn lipid_panel() -> Self {
        let mut panel = Self::new("Lipid Panel".to_string());
        panel.add_marker(Biomarker::TotalCholesterol, 180.0);
        panel.add_marker(Biomarker::LDLCholesterol, 90.0);
        panel.add_marker(Biomarker::HDLCholesterol, 60.0);
        panel.add_marker(Biomarker::Triglycerides, 100.0);
        panel
    }

    pub fn complete_blood_count() -> Self {
        let mut panel = Self::new("Complete Blood Count".to_string());
        panel.add_marker(Biomarker::Hemoglobin, 14.5);
        panel.add_marker(Biomarker::WBC, 7000.0);
        panel.add_marker(Biomarker::Platelets, 250000.0);
        panel
    }

    pub fn add_marker(&mut self, biomarker: Biomarker, value: f64) {
        let unit = biomarker.default_unit().to_string();
        self.markers
            .insert(biomarker, BiomarkerValue::new(value, unit));
    }

    pub fn get_marker(&self, biomarker: &Biomarker) -> Option<&BiomarkerValue> {
        self.markers.get(biomarker)
    }

    pub fn abnormal_markers(&self) -> Vec<(Biomarker, &BiomarkerValue)> {
        self.markers
            .iter()
            .filter(|(marker, value)| {
                let range = marker.reference_range();
                !value.is_in_range(&range)
            })
            .map(|(marker, value)| (*marker, value))
            .collect()
    }

    pub fn suboptimal_markers(&self) -> Vec<(Biomarker, &BiomarkerValue)> {
        self.markers
            .iter()
            .filter(|(marker, value)| {
                let range = marker.reference_range();
                !value.is_optimal(&range)
            })
            .map(|(marker, value)| (*marker, value))
            .collect()
    }

    pub fn all_in_range(&self) -> bool {
        self.abnormal_markers().is_empty()
    }

    pub fn all_optimal(&self) -> bool {
        self.suboptimal_markers().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biomarker_reference_range() {
        let glucose_range = Biomarker::Glucose.reference_range();
        assert_eq!(glucose_range.low, 70.0);
        assert_eq!(glucose_range.high, 100.0);
    }

    #[test]
    fn test_biomarker_value_in_range() {
        let value = BiomarkerValue::new(85.0, "mg/dL".to_string());
        let range = Biomarker::Glucose.reference_range();
        assert!(value.is_in_range(&range));
        assert!(value.is_optimal(&range));
    }

    #[test]
    fn test_biomarker_value_out_of_range() {
        let value = BiomarkerValue::new(150.0, "mg/dL".to_string());
        let range = Biomarker::Glucose.reference_range();
        assert!(!value.is_in_range(&range));
        assert!(value.is_high(&range));
    }

    #[test]
    fn test_biomarker_panel() {
        let mut panel = BiomarkerPanel::new("Test Panel".to_string());
        panel.add_marker(Biomarker::Glucose, 90.0);
        panel.add_marker(Biomarker::HbA1c, 5.2);

        assert!(panel.all_in_range());
    }

    #[test]
    fn test_lipid_panel() {
        let panel = BiomarkerPanel::lipid_panel();
        assert!(panel.markers.contains_key(&Biomarker::TotalCholesterol));
        assert!(panel.markers.contains_key(&Biomarker::HDLCholesterol));
    }

    #[test]
    fn test_abnormal_markers() {
        let mut panel = BiomarkerPanel::new("Test".to_string());
        panel.add_marker(Biomarker::Glucose, 200.0);
        panel.add_marker(Biomarker::HbA1c, 5.0);

        let abnormal = panel.abnormal_markers();
        assert_eq!(abnormal.len(), 1);
        assert_eq!(abnormal[0].0, Biomarker::Glucose);
    }
}
