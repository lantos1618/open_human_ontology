use serde::{Deserialize, Serialize};
use crate::biology::genetics::snp::SNP;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiseaseCategory {
    Infectious,
    Genetic,
    Autoimmune,
    Cardiovascular,
    Metabolic,
    Neurological,
    Oncological,
    Respiratory,
    Gastrointestinal,
    Renal,
    Endocrine,
    Musculoskeletal,
    Dermatological,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disease {
    pub name: String,
    pub icd10_code: Option<String>,
    pub category: DiseaseCategory,
    pub prevalence: f64,
    pub genetic_component: Option<GeneticComponent>,
    pub environmental_factors: Vec<String>,
    pub risk_factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneticComponent {
    pub heritability: f64,
    pub associated_genes: Vec<String>,
    pub associated_snps: Vec<SNP>,
    pub inheritance_pattern: Option<InheritancePattern>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InheritancePattern {
    AutosomalDominant,
    AutosomalRecessive,
    XLinkedDominant,
    XLinkedRecessive,
    Mitochondrial,
    Multifactorial,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor: String,
    pub odds_ratio: f64,
    pub modifiable: bool,
}

impl Disease {
    pub fn new(name: String, category: DiseaseCategory) -> Self {
        Disease {
            name,
            icd10_code: None,
            category,
            prevalence: 0.0,
            genetic_component: None,
            environmental_factors: Vec::new(),
            risk_factors: Vec::new(),
        }
    }

    pub fn with_icd10(mut self, code: String) -> Self {
        self.icd10_code = Some(code);
        self
    }

    pub fn with_prevalence(mut self, prevalence: f64) -> Self {
        self.prevalence = prevalence.clamp(0.0, 1.0);
        self
    }

    pub fn add_risk_factor(&mut self, risk_factor: RiskFactor) {
        self.risk_factors.push(risk_factor);
    }

    pub fn is_genetic(&self) -> bool {
        self.genetic_component.is_some() ||
        matches!(self.category, DiseaseCategory::Genetic)
    }

    pub fn is_rare(&self) -> bool {
        self.prevalence < 0.001
    }

    pub fn is_common(&self) -> bool {
        self.prevalence > 0.05
    }

    pub fn modifiable_risk_factors(&self) -> Vec<&RiskFactor> {
        self.risk_factors.iter()
            .filter(|rf| rf.modifiable)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseProgression {
    pub disease: Disease,
    pub stages: Vec<DiseaseStage>,
    pub current_stage: usize,
    pub progression_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiseaseStage {
    pub name: String,
    pub severity: Severity,
    pub duration_days: f64,
    pub biomarkers: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Asymptomatic,
    Mild,
    Moderate,
    Severe,
    Critical,
    Terminal,
}

impl DiseaseProgression {
    pub fn new(disease: Disease) -> Self {
        DiseaseProgression {
            disease,
            stages: Vec::new(),
            current_stage: 0,
            progression_rate: 1.0,
        }
    }

    pub fn add_stage(&mut self, stage: DiseaseStage) {
        self.stages.push(stage);
    }

    pub fn advance_stage(&mut self) -> bool {
        if self.current_stage < self.stages.len() - 1 {
            self.current_stage += 1;
            true
        } else {
            false
        }
    }

    pub fn current_severity(&self) -> Option<Severity> {
        self.stages.get(self.current_stage)
            .map(|stage| stage.severity)
    }

    pub fn is_terminal(&self) -> bool {
        self.current_severity()
            .map(|s| matches!(s, Severity::Terminal))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disease_creation() {
        let disease = Disease::new(
            "Type 2 Diabetes".to_string(),
            DiseaseCategory::Metabolic,
        )
        .with_icd10("E11".to_string())
        .with_prevalence(0.095);

        assert_eq!(disease.name, "Type 2 Diabetes");
        assert!(disease.is_common());
        assert!(!disease.is_rare());
    }

    #[test]
    fn test_risk_factors() {
        let mut disease = Disease::new(
            "Coronary Artery Disease".to_string(),
            DiseaseCategory::Cardiovascular,
        );

        disease.add_risk_factor(RiskFactor {
            factor: "Smoking".to_string(),
            odds_ratio: 2.5,
            modifiable: true,
        });

        disease.add_risk_factor(RiskFactor {
            factor: "Age".to_string(),
            odds_ratio: 1.5,
            modifiable: false,
        });

        assert_eq!(disease.modifiable_risk_factors().len(), 1);
    }

    #[test]
    fn test_disease_progression() {
        let disease = Disease::new(
            "Cancer".to_string(),
            DiseaseCategory::Oncological,
        );

        let mut progression = DiseaseProgression::new(disease);

        progression.add_stage(DiseaseStage {
            name: "Stage I".to_string(),
            severity: Severity::Mild,
            duration_days: 365.0,
            biomarkers: vec!["CEA".to_string()],
        });

        progression.add_stage(DiseaseStage {
            name: "Stage IV".to_string(),
            severity: Severity::Terminal,
            duration_days: 180.0,
            biomarkers: vec!["CEA".to_string(), "CA19-9".to_string()],
        });

        assert_eq!(progression.current_severity(), Some(Severity::Mild));
        progression.advance_stage();
        assert_eq!(progression.current_severity(), Some(Severity::Terminal));
        assert!(progression.is_terminal());
    }
}
