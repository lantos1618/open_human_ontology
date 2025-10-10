use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeurologicalCondition {
    Migraine(MigraineType),
    ClusterHeadache,
    TensionHeadache,
    Epilepsy(EpilepsyType),
    Parkinsons,
    Alzheimers,
    MultipleSclerosis,
    PeripheralNeuropathy(NeuropathyType),
    Stroke(StrokeType),
    EssentialTremor,
    RestlessLegSyndrome,
    NarcolepsyType1,
    NarcolepsyType2,
    ChronicPain(PainType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigraineType {
    WithoutAura,
    WithAura,
    Chronic,
    Hemiplegic,
    Basilar,
    Retinal,
    Vestibular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EpilepsyType {
    FocalOnset,
    GeneralizedOnset,
    UnknownOnset,
    TonicClonic,
    Absence,
    Myoclonic,
    Atonic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuropathyType {
    Diabetic,
    Alcoholic,
    ChemotherapyInduced,
    Idiopathic,
    Hereditary,
    AutoimmuneInflammatory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrokeType {
    Ischemic,
    Hemorrhagic,
    TransientIschemicAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PainType {
    Neuropathic,
    Nociceptive,
    Mixed,
    Fibromyalgia,
    ComplexRegionalPainSyndrome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurologicalProfile {
    pub conditions: Vec<NeurologicalCondition>,
    pub genetic_risk_factors: HashMap<String, f64>,
    pub symptoms: Vec<NeurologicalSymptom>,
    pub treatments: Vec<String>,
    pub quality_of_life_score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeurologicalSymptom {
    Headache,
    Dizziness,
    Numbness,
    Tingling,
    Weakness,
    MemoryLoss,
    Confusion,
    Tremor,
    Seizure,
    VisionChanges,
    SpeechDifficulty,
    BalanceProblems,
    Coordination,
    Fatigue,
    SleepDisturbances,
    CognitiveImpairment,
}

impl NeurologicalProfile {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            genetic_risk_factors: HashMap::new(),
            symptoms: Vec::new(),
            treatments: Vec::new(),
            quality_of_life_score: 100.0,
        }
    }

    pub fn add_condition(&mut self, condition: NeurologicalCondition) {
        if !self.conditions.contains(&condition) {
            self.conditions.push(condition);
        }
    }

    pub fn add_genetic_risk(&mut self, gene: String, risk_score: f64) {
        self.genetic_risk_factors.insert(gene, risk_score);
    }

    pub fn add_symptom(&mut self, symptom: NeurologicalSymptom) {
        if !self.symptoms.contains(&symptom) {
            self.symptoms.push(symptom);
        }
    }

    pub fn has_condition(&self, condition_type: &NeurologicalCondition) -> bool {
        self.conditions
            .iter()
            .any(|c| std::mem::discriminant(c) == std::mem::discriminant(condition_type))
    }

    pub fn calculate_burden_score(&self) -> f64 {
        let condition_weight = self.conditions.len() as f64 * 10.0;
        let symptom_weight = self.symptoms.len() as f64 * 2.0;
        let qol_impact = (100.0 - self.quality_of_life_score) / 100.0 * 50.0;

        (condition_weight + symptom_weight + qol_impact).min(100.0)
    }

    pub fn get_genetic_insights(&self) -> Vec<String> {
        let mut insights = Vec::new();

        for (gene, risk_score) in &self.genetic_risk_factors {
            let risk_level = if *risk_score > 3.0 {
                "High"
            } else if *risk_score > 1.5 {
                "Moderate"
            } else {
                "Low"
            };

            insights.push(format!(
                "{}: {} risk (OR: {:.2})",
                gene, risk_level, risk_score
            ));
        }

        insights
    }
}

impl Default for NeurologicalProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticNeurologicalRisk {
    pub alzheimers_apoe_status: APOEStatus,
    pub parkinsons_lrrk2: bool,
    pub parkinsons_snca: bool,
    pub parkinsons_gba: bool,
    pub migraine_susceptibility: Vec<String>,
    pub epilepsy_susceptibility: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum APOEStatus {
    E2E2,
    E2E3,
    E2E4,
    E3E3,
    E3E4,
    E4E4,
}

impl APOEStatus {
    pub fn alzheimers_risk(&self) -> f64 {
        match self {
            APOEStatus::E2E2 | APOEStatus::E2E3 => 0.6,
            APOEStatus::E3E3 => 1.0,
            APOEStatus::E2E4 | APOEStatus::E3E4 => 3.0,
            APOEStatus::E4E4 => 12.0,
        }
    }

    pub fn cardiovascular_risk(&self) -> f64 {
        match self {
            APOEStatus::E2E2 | APOEStatus::E2E3 => 0.8,
            APOEStatus::E3E3 => 1.0,
            APOEStatus::E2E4 | APOEStatus::E3E4 => 1.3,
            APOEStatus::E4E4 => 1.7,
        }
    }

    pub fn is_protective(&self) -> bool {
        matches!(self, APOEStatus::E2E2 | APOEStatus::E2E3)
    }

    pub fn is_high_risk(&self) -> bool {
        matches!(self, APOEStatus::E4E4 | APOEStatus::E3E4)
    }
}

impl GeneticNeurologicalRisk {
    pub fn new() -> Self {
        Self {
            alzheimers_apoe_status: APOEStatus::E3E3,
            parkinsons_lrrk2: false,
            parkinsons_snca: false,
            parkinsons_gba: false,
            migraine_susceptibility: Vec::new(),
            epilepsy_susceptibility: Vec::new(),
        }
    }

    pub fn alzheimers_lifetime_risk(&self) -> f64 {
        let base_risk = 0.10;
        base_risk * self.alzheimers_apoe_status.alzheimers_risk()
    }

    pub fn parkinsons_risk(&self) -> f64 {
        let mut risk = 1.0;

        if self.parkinsons_lrrk2 {
            risk *= 2.5;
        }
        if self.parkinsons_snca {
            risk *= 3.0;
        }
        if self.parkinsons_gba {
            risk *= 5.0;
        }

        risk
    }

    pub fn has_parkinsons_genetic_risk(&self) -> bool {
        self.parkinsons_lrrk2 || self.parkinsons_snca || self.parkinsons_gba
    }

    pub fn has_migraine_genetic_risk(&self) -> bool {
        !self.migraine_susceptibility.is_empty()
    }

    pub fn comprehensive_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!(
            "APOE Status: {:?} (Alzheimer's Risk: {:.1}x)\n",
            self.alzheimers_apoe_status,
            self.alzheimers_apoe_status.alzheimers_risk()
        ));

        if self.has_parkinsons_genetic_risk() {
            report.push_str(&format!(
                "Parkinson's Disease Risk: {:.1}x baseline\n",
                self.parkinsons_risk()
            ));
        }

        if self.has_migraine_genetic_risk() {
            report.push_str(&format!(
                "Migraine Susceptibility Variants: {}\n",
                self.migraine_susceptibility.len()
            ));
        }

        report
    }
}

impl Default for GeneticNeurologicalRisk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neurological_profile_creation() {
        let profile = NeurologicalProfile::new();
        assert!(profile.conditions.is_empty());
        assert_eq!(profile.quality_of_life_score, 100.0);
    }

    #[test]
    fn test_add_condition() {
        let mut profile = NeurologicalProfile::new();
        profile.add_condition(NeurologicalCondition::Migraine(MigraineType::WithAura));
        assert_eq!(profile.conditions.len(), 1);
    }

    #[test]
    fn test_burden_score() {
        let mut profile = NeurologicalProfile::new();
        profile.add_condition(NeurologicalCondition::Migraine(MigraineType::Chronic));
        profile.add_symptom(NeurologicalSymptom::Headache);
        profile.add_symptom(NeurologicalSymptom::Fatigue);
        profile.quality_of_life_score = 70.0;

        let score = profile.calculate_burden_score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_apoe_e4e4_high_risk() {
        let apoe = APOEStatus::E4E4;
        assert!(apoe.is_high_risk());
        assert!(apoe.alzheimers_risk() > 10.0);
    }

    #[test]
    fn test_apoe_e2_protective() {
        let apoe = APOEStatus::E2E2;
        assert!(apoe.is_protective());
        assert!(apoe.alzheimers_risk() < 1.0);
    }

    #[test]
    fn test_parkinsons_risk_calculation() {
        let mut risk = GeneticNeurologicalRisk::new();
        risk.parkinsons_lrrk2 = true;
        risk.parkinsons_gba = true;

        let total_risk = risk.parkinsons_risk();
        assert!(total_risk > 10.0);
    }

    #[test]
    fn test_genetic_neurological_risk_report() {
        let mut risk = GeneticNeurologicalRisk::new();
        risk.alzheimers_apoe_status = APOEStatus::E4E4;
        risk.parkinsons_lrrk2 = true;

        let report = risk.comprehensive_report();
        assert!(report.contains("APOE"));
        assert!(report.contains("Parkinson"));
    }
}
