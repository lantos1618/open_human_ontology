use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLevel {
    SystematicReview,
    MetaAnalysis,
    RandomizedControlledTrial,
    CohortStudy,
    CaseControlStudy,
    CaseSeries,
    ExpertOpinion,
}

impl EvidenceLevel {
    pub fn quality_score(&self) -> f64 {
        match self {
            EvidenceLevel::SystematicReview => 1.0,
            EvidenceLevel::MetaAnalysis => 1.0,
            EvidenceLevel::RandomizedControlledTrial => 0.9,
            EvidenceLevel::CohortStudy => 0.7,
            EvidenceLevel::CaseControlStudy => 0.6,
            EvidenceLevel::CaseSeries => 0.4,
            EvidenceLevel::ExpertOpinion => 0.3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalReference {
    pub pmid: Option<String>,
    pub doi: Option<String>,
    pub citation: String,
    pub year: u32,
    pub evidence_level: EvidenceLevel,
    pub sample_size: Option<usize>,
    pub population: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTruthDataPoint {
    pub parameter_name: String,
    pub expected_value: f64,
    pub standard_deviation: Option<f64>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub reference: ClinicalReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTruthData {
    pub category: String,
    pub description: String,
    pub data_points: Vec<GroundTruthDataPoint>,
}

impl GroundTruthData {
    pub fn new(category: String, description: String) -> Self {
        Self {
            category,
            description,
            data_points: Vec::new(),
        }
    }

    pub fn add_data_point(&mut self, data_point: GroundTruthDataPoint) {
        self.data_points.push(data_point);
    }

    pub fn get_expected_value(&self, parameter_name: &str) -> Option<f64> {
        self.data_points
            .iter()
            .find(|dp| dp.parameter_name == parameter_name)
            .map(|dp| dp.expected_value)
    }

    pub fn is_within_expected_range(&self, parameter_name: &str, value: f64) -> bool {
        if let Some(dp) = self.data_points.iter().find(|dp| dp.parameter_name == parameter_name) {
            if let (Some(min), Some(max)) = (dp.min_value, dp.max_value) {
                return value >= min && value <= max;
            }
            if let Some(sd) = dp.standard_deviation {
                let lower = dp.expected_value - 2.0 * sd;
                let upper = dp.expected_value + 2.0 * sd;
                return value >= lower && value <= upper;
            }
        }
        false
    }
}

pub struct GroundTruthDatabase {
    datasets: HashMap<String, GroundTruthData>,
}

impl GroundTruthDatabase {
    pub fn new() -> Self {
        let mut db = Self {
            datasets: HashMap::new(),
        };
        db.initialize_cardiovascular_data();
        db.initialize_metabolic_data();
        db.initialize_aldh2_data();
        db
    }

    fn initialize_cardiovascular_data(&mut self) {
        let mut cv_data = GroundTruthData::new(
            "Cardiovascular".to_string(),
            "Normal resting cardiovascular parameters in healthy adults".to_string(),
        );

        cv_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "resting_heart_rate_bpm".to_string(),
            expected_value: 70.0,
            standard_deviation: Some(10.0),
            min_value: Some(60.0),
            max_value: Some(100.0),
            reference: ClinicalReference {
                pmid: Some("25910639".to_string()),
                doi: Some("10.1161/JAHA.114.001377".to_string()),
                citation: "Reimers AK et al. (2015) J Am Heart Assoc 4(5):e001377".to_string(),
                year: 2015,
                evidence_level: EvidenceLevel::CohortStudy,
                sample_size: Some(92757),
                population: "General adult population".to_string(),
            },
        });

        cv_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "systolic_bp_mmhg".to_string(),
            expected_value: 120.0,
            standard_deviation: Some(10.0),
            min_value: Some(90.0),
            max_value: Some(140.0),
            reference: ClinicalReference {
                pmid: Some("24222015".to_string()),
                doi: Some("10.1001/jama.2013.282543".to_string()),
                citation: "James PA et al. (2014) JAMA 311(5):507-520".to_string(),
                year: 2014,
                evidence_level: EvidenceLevel::SystematicReview,
                sample_size: None,
                population: "Adults >18 years".to_string(),
            },
        });

        cv_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "diastolic_bp_mmhg".to_string(),
            expected_value: 80.0,
            standard_deviation: Some(8.0),
            min_value: Some(60.0),
            max_value: Some(90.0),
            reference: ClinicalReference {
                pmid: Some("24222015".to_string()),
                doi: Some("10.1001/jama.2013.282543".to_string()),
                citation: "James PA et al. (2014) JAMA 311(5):507-520".to_string(),
                year: 2014,
                evidence_level: EvidenceLevel::SystematicReview,
                sample_size: None,
                population: "Adults >18 years".to_string(),
            },
        });

        self.datasets.insert("cardiovascular".to_string(), cv_data);
    }

    fn initialize_metabolic_data(&mut self) {
        let mut metabolic_data = GroundTruthData::new(
            "Metabolic".to_string(),
            "Normal metabolic parameters in healthy adults".to_string(),
        );

        metabolic_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "fasting_glucose_mg_dl".to_string(),
            expected_value: 90.0,
            standard_deviation: Some(10.0),
            min_value: Some(70.0),
            max_value: Some(100.0),
            reference: ClinicalReference {
                pmid: Some("32657974".to_string()),
                doi: Some("10.2337/dc20-S002".to_string()),
                citation: "American Diabetes Association (2020) Diabetes Care 43(Suppl 1):S14-S31".to_string(),
                year: 2020,
                evidence_level: EvidenceLevel::SystematicReview,
                sample_size: None,
                population: "General adult population".to_string(),
            },
        });

        metabolic_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "bmi_healthy_range".to_string(),
            expected_value: 22.5,
            standard_deviation: Some(2.5),
            min_value: Some(18.5),
            max_value: Some(24.9),
            reference: ClinicalReference {
                pmid: Some("27216006".to_string()),
                doi: Some("10.1016/S0140-6736(16)30175-1".to_string()),
                citation: "GBD 2015 Obesity Collaborators (2017) Lancet 390(10113):2627-2642".to_string(),
                year: 2017,
                evidence_level: EvidenceLevel::MetaAnalysis,
                sample_size: Some(19200000),
                population: "Global population".to_string(),
            },
        });

        self.datasets.insert("metabolic".to_string(), metabolic_data);
    }

    fn initialize_aldh2_data(&mut self) {
        let mut aldh2_data = GroundTruthData::new(
            "ALDH2_Deficiency".to_string(),
            "Acetaldehyde metabolism in ALDH2*2 carriers".to_string(),
        );

        aldh2_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "aldh2_activity_heterozygous".to_string(),
            expected_value: 0.12,
            standard_deviation: Some(0.05),
            min_value: Some(0.05),
            max_value: Some(0.20),
            reference: ClinicalReference {
                pmid: Some("19320537".to_string()),
                doi: Some("10.1371/journal.pmed.1000050".to_string()),
                citation: "Brooks PJ et al. (2009) PLoS Med 6(3):e50".to_string(),
                year: 2009,
                evidence_level: EvidenceLevel::MetaAnalysis,
                sample_size: Some(44000),
                population: "East Asian populations".to_string(),
            },
        });

        aldh2_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "acetaldehyde_peak_multiplier_aldh2_het".to_string(),
            expected_value: 5.0,
            standard_deviation: Some(2.0),
            min_value: Some(2.0),
            max_value: Some(10.0),
            reference: ClinicalReference {
                pmid: Some("19320537".to_string()),
                doi: Some("10.1371/journal.pmed.1000050".to_string()),
                citation: "Brooks PJ et al. (2009) PLoS Med 6(3):e50".to_string(),
                year: 2009,
                evidence_level: EvidenceLevel::MetaAnalysis,
                sample_size: Some(44000),
                population: "East Asian populations".to_string(),
            },
        });

        aldh2_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "esophageal_cancer_risk_moderate_drinking".to_string(),
            expected_value: 5.0,
            standard_deviation: Some(1.5),
            min_value: Some(3.0),
            max_value: Some(8.0),
            reference: ClinicalReference {
                pmid: Some("19320537".to_string()),
                doi: Some("10.1371/journal.pmed.1000050".to_string()),
                citation: "Brooks PJ et al. (2009) PLoS Med 6(3):e50".to_string(),
                year: 2009,
                evidence_level: EvidenceLevel::MetaAnalysis,
                sample_size: Some(44000),
                population: "East Asian ALDH2*1/*2 carriers".to_string(),
            },
        });

        aldh2_data.add_data_point(GroundTruthDataPoint {
            parameter_name: "aldh2_deficiency_frequency_east_asian".to_string(),
            expected_value: 0.36,
            standard_deviation: Some(0.05),
            min_value: Some(0.28),
            max_value: Some(0.45),
            reference: ClinicalReference {
                pmid: Some("30158283".to_string()),
                doi: Some("10.1038/s41436-018-0290-2".to_string()),
                citation: "Chen CH et al. (2018) Genet Med 21(8):1657-1659".to_string(),
                year: 2018,
                evidence_level: EvidenceLevel::CohortStudy,
                sample_size: Some(550000000),
                population: "East Asian population".to_string(),
            },
        });

        self.datasets.insert("aldh2".to_string(), aldh2_data);
    }

    pub fn get_dataset(&self, category: &str) -> Option<&GroundTruthData> {
        self.datasets.get(category)
    }

    pub fn all_categories(&self) -> Vec<String> {
        self.datasets.keys().cloned().collect()
    }
}

impl Default for GroundTruthDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_truth_database() {
        let db = GroundTruthDatabase::new();
        assert!(db.get_dataset("cardiovascular").is_some());
        assert!(db.get_dataset("metabolic").is_some());
        assert!(db.get_dataset("aldh2").is_some());
    }

    #[test]
    fn test_expected_values() {
        let db = GroundTruthDatabase::new();
        let cv = db.get_dataset("cardiovascular").unwrap();

        assert_eq!(cv.get_expected_value("resting_heart_rate_bpm"), Some(70.0));
        assert_eq!(cv.get_expected_value("systolic_bp_mmhg"), Some(120.0));
    }

    #[test]
    fn test_range_validation() {
        let db = GroundTruthDatabase::new();
        let cv = db.get_dataset("cardiovascular").unwrap();

        assert!(cv.is_within_expected_range("resting_heart_rate_bpm", 70.0));
        assert!(cv.is_within_expected_range("resting_heart_rate_bpm", 80.0));
        assert!(!cv.is_within_expected_range("resting_heart_rate_bpm", 150.0));
    }

    #[test]
    fn test_evidence_levels() {
        assert_eq!(EvidenceLevel::SystematicReview.quality_score(), 1.0);
        assert_eq!(EvidenceLevel::RandomizedControlledTrial.quality_score(), 0.9);
        assert!(EvidenceLevel::CohortStudy.quality_score() > EvidenceLevel::CaseSeries.quality_score());
    }
}
