use super::disease::Disease;
use super::symptom::Symptom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnosis {
    pub disease: Disease,
    pub confidence: f64,
    pub differential_diagnoses: Vec<DifferentialDiagnosis>,
    pub supporting_evidence: Vec<Evidence>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialDiagnosis {
    pub disease: Disease,
    pub probability: f64,
    pub distinguishing_features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Evidence {
    ClinicalFindings(Vec<Symptom>),
    LabResults {
        test_name: String,
        value: f64,
        reference_range: (f64, f64),
        abnormal: bool,
    },
    ImagingFindings {
        modality: String,
        findings: Vec<String>,
    },
    GeneticTest {
        gene: String,
        variant: String,
        pathogenic: bool,
    },
    FamilyHistory {
        condition: String,
        relationship: String,
    },
}

impl Diagnosis {
    pub fn new(disease: Disease, confidence: f64) -> Self {
        Diagnosis {
            disease,
            confidence: confidence.clamp(0.0, 1.0),
            differential_diagnoses: Vec::new(),
            supporting_evidence: Vec::new(),
            date: None,
        }
    }

    pub fn add_differential(&mut self, diff: DifferentialDiagnosis) {
        self.differential_diagnoses.push(diff);
        self.differential_diagnoses
            .sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
    }

    pub fn add_evidence(&mut self, evidence: Evidence) {
        self.supporting_evidence.push(evidence);
    }

    pub fn is_confirmed(&self) -> bool {
        self.confidence >= 0.9
    }

    pub fn is_probable(&self) -> bool {
        self.confidence >= 0.7 && self.confidence < 0.9
    }

    pub fn is_possible(&self) -> bool {
        self.confidence >= 0.5 && self.confidence < 0.7
    }

    pub fn has_genetic_confirmation(&self) -> bool {
        self.supporting_evidence.iter().any(|e| {
            matches!(
                e,
                Evidence::GeneticTest {
                    pathogenic: true,
                    ..
                }
            )
        })
    }

    pub fn abnormal_lab_count(&self) -> usize {
        self.supporting_evidence
            .iter()
            .filter(|e| matches!(e, Evidence::LabResults { abnormal: true, .. }))
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticCriteria {
    pub condition_name: String,
    pub major_criteria: Vec<Criterion>,
    pub minor_criteria: Vec<Criterion>,
    pub required_major: usize,
    pub required_minor: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criterion {
    pub description: String,
    pub met: bool,
    pub evidence: Vec<String>,
}

impl DiagnosticCriteria {
    pub fn new(condition_name: String, required_major: usize, required_minor: usize) -> Self {
        DiagnosticCriteria {
            condition_name,
            major_criteria: Vec::new(),
            minor_criteria: Vec::new(),
            required_major,
            required_minor,
        }
    }

    pub fn add_major_criterion(&mut self, criterion: Criterion) {
        self.major_criteria.push(criterion);
    }

    pub fn add_minor_criterion(&mut self, criterion: Criterion) {
        self.minor_criteria.push(criterion);
    }

    pub fn major_met(&self) -> usize {
        self.major_criteria.iter().filter(|c| c.met).count()
    }

    pub fn minor_met(&self) -> usize {
        self.minor_criteria.iter().filter(|c| c.met).count()
    }

    pub fn is_diagnosis_met(&self) -> bool {
        self.major_met() >= self.required_major && self.minor_met() >= self.required_minor
    }

    pub fn completion_percentage(&self) -> f64 {
        let major_progress = self.major_met() as f64 / self.required_major as f64;
        let minor_progress = self.minor_met() as f64 / self.required_minor as f64;
        ((major_progress + minor_progress) / 2.0 * 100.0).min(100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::super::disease::{Disease, DiseaseCategory};
    use super::*;

    #[test]
    fn test_diagnosis_creation() {
        let disease = Disease::new(
            "Diabetes Mellitus Type 2".to_string(),
            DiseaseCategory::Metabolic,
        );

        let mut diagnosis = Diagnosis::new(disease, 0.95);

        diagnosis.add_evidence(Evidence::LabResults {
            test_name: "HbA1c".to_string(),
            value: 7.5,
            reference_range: (4.0, 5.6),
            abnormal: true,
        });

        assert!(diagnosis.is_confirmed());
        assert!(!diagnosis.is_probable());
        assert_eq!(diagnosis.abnormal_lab_count(), 1);
    }

    #[test]
    fn test_differential_diagnosis() {
        let primary_disease = Disease::new("Migraine".to_string(), DiseaseCategory::Neurological);

        let mut diagnosis = Diagnosis::new(primary_disease, 0.75);

        let diff = DifferentialDiagnosis {
            disease: Disease::new(
                "Tension Headache".to_string(),
                DiseaseCategory::Neurological,
            ),
            probability: 0.20,
            distinguishing_features: vec!["Bilateral pain".to_string()],
        };

        diagnosis.add_differential(diff);
        assert!(diagnosis.is_probable());
        assert_eq!(diagnosis.differential_diagnoses.len(), 1);
    }

    #[test]
    fn test_diagnostic_criteria() {
        let mut criteria = DiagnosticCriteria::new("Rheumatoid Arthritis".to_string(), 2, 1);

        criteria.add_major_criterion(Criterion {
            description: "Positive RF".to_string(),
            met: true,
            evidence: vec!["Lab result".to_string()],
        });

        criteria.add_major_criterion(Criterion {
            description: "Joint swelling".to_string(),
            met: true,
            evidence: vec!["Physical exam".to_string()],
        });

        criteria.add_minor_criterion(Criterion {
            description: "Morning stiffness".to_string(),
            met: true,
            evidence: vec!["Patient report".to_string()],
        });

        assert!(criteria.is_diagnosis_met());
        assert_eq!(criteria.major_met(), 2);
        assert_eq!(criteria.minor_met(), 1);
        assert_eq!(criteria.completion_percentage(), 100.0);
    }
}
