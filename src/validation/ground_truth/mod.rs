use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Module declarations
pub mod cardiovascular;
pub mod metabolic;
pub mod aldh2;
pub mod respiratory;
pub mod renal;
pub mod endocrine;
pub mod hematology;
pub mod neurological;
pub mod gastrointestinal;
pub mod musculoskeletal;
pub mod immunology;
pub mod hepatic;
pub mod dermatology;
pub mod ophthalmology;
pub mod auditory;
pub mod dental;
pub mod pulmonary;
pub mod rheumatology;
pub mod urology;
pub mod oncology;
pub mod alzheimers;

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
        if let Some(dp) = self
            .data_points
            .iter()
            .find(|dp| dp.parameter_name == parameter_name)
        {
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
        cardiovascular::initialize_cardiovascular_data(&mut db);
        metabolic::initialize_metabolic_data(&mut db);
        aldh2::initialize_aldh2_data(&mut db);
        respiratory::initialize_respiratory_data(&mut db);
        renal::initialize_renal_data(&mut db);
        endocrine::initialize_endocrine_data(&mut db);
        hematology::initialize_hematology_data(&mut db);
        neurological::initialize_neurological_data(&mut db);
        gastrointestinal::initialize_gastrointestinal_data(&mut db);
        musculoskeletal::initialize_musculoskeletal_data(&mut db);
        immunology::initialize_immunology_data(&mut db);
        hepatic::initialize_hepatic_data(&mut db);
        dermatology::initialize_dermatology_data(&mut db);
        ophthalmology::initialize_ophthalmology_data(&mut db);
        auditory::initialize_auditory_data(&mut db);
        dental::initialize_dental_data(&mut db);
        pulmonary::initialize_pulmonary_data(&mut db);
        rheumatology::initialize_rheumatology_data(&mut db);
        urology::initialize_urology_data(&mut db);
        db.add_dataset("cancer_biomarkers".to_string(), oncology::get_cancer_biomarkers());
        db.add_dataset("inflammation_markers".to_string(), oncology::get_inflammation_markers());
        db.add_dataset("alzheimers_biomarkers".to_string(), alzheimers::get_alzheimers_biomarkers());
        db
    }

    pub fn get_dataset(&self, name: &str) -> Option<&GroundTruthData> {
        self.datasets.get(name)
    }

    pub fn get_all_datasets(&self) -> &HashMap<String, GroundTruthData> {
        &self.datasets
    }

    pub fn add_dataset(&mut self, name: String, dataset: GroundTruthData) {
        self.datasets.insert(name, dataset);
    }

    pub fn validate_completeness(&self) {
        let categories: Vec<_> = self.datasets.keys().collect();
        let total_params: usize = self.datasets.values().map(|d| d.data_points.len()).sum();

        println!("\n=== Validation Database Statistics ===");
        println!("Total Systems: {}", categories.len());
        println!("Total Parameters: {}", total_params);

        println!("Categories: {}", categories.len());
        println!("Total parameters: {}", total_params);
    }
}