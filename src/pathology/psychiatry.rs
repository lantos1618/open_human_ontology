use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychiatricProfile {
    pub diagnoses: Vec<PsychiatricDiagnosis>,
    pub symptom_severity: SymptomSeverity,
    pub cognitive_function: CognitiveFunction,
    pub mood_state: MoodState,
    pub risk_assessments: RiskAssessments,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychiatricDiagnosis {
    pub disorder: DisorderType,
    pub onset_age: Option<f64>,
    pub duration_months: Option<u32>,
    pub severity: PsychiatricSeverity,
    pub episode_type: Option<EpisodeType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisorderType {
    MajorDepressiveDisorder,
    BipolarI,
    BipolarII,
    GeneralizedAnxietyDisorder,
    PanicDisorder,
    SocialAnxietyDisorder,
    PTSD,
    OCD,
    Schizophrenia,
    SchizoaffectiveDisorder,
    ADHD,
    AutismSpectrumDisorder,
    EatingDisorderAnorexia,
    EatingDisorderBulimia,
    SubstanceUseDisorder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PsychiatricSeverity {
    Mild,
    Moderate,
    Severe,
    Remission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EpisodeType {
    Depressive,
    Manic,
    Hypomanic,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymptomSeverity {
    pub depression_score: DepressionScore,
    pub anxiety_score: AnxietyScore,
    pub psychosis_score: PsychosisScore,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DepressionScore {
    pub phq9_score: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AnxietyScore {
    pub gad7_score: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PsychosisScore {
    pub positive_symptoms: u32,
    pub negative_symptoms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveFunction {
    pub attention: CognitiveScore,
    pub memory: CognitiveScore,
    pub executive_function: CognitiveScore,
    pub processing_speed: CognitiveScore,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CognitiveScore {
    pub percentile: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodState {
    pub current_mood: Mood,
    pub affect: Affect,
    pub energy_level: EnergyLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mood {
    Euthymic,
    Depressed,
    Elevated,
    Irritable,
    Anxious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Affect {
    Appropriate,
    Flat,
    Blunted,
    Labile,
    Restricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnergyLevel {
    VeryLow,
    Low,
    Normal,
    Elevated,
    VeryElevated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessments {
    pub suicide_risk: SuicideRisk,
    pub violence_risk: ViolenceRisk,
    pub self_harm_risk: SelfHarmRisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuicideRisk {
    Low,
    Moderate,
    High,
    Imminent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolenceRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfHarmRisk {
    Low,
    Moderate,
    High,
}

impl PsychiatricProfile {
    pub fn new() -> Self {
        Self {
            diagnoses: Vec::new(),
            symptom_severity: SymptomSeverity::normal(),
            cognitive_function: CognitiveFunction::normal(),
            mood_state: MoodState::euthymic(),
            risk_assessments: RiskAssessments::low_risk(),
        }
    }

    pub fn add_diagnosis(&mut self, diagnosis: PsychiatricDiagnosis) {
        self.diagnoses.push(diagnosis);
        self.update_risk_assessments();
    }

    fn update_risk_assessments(&mut self) {
        let has_depression = self.diagnoses.iter()
            .any(|d| matches!(d.disorder, DisorderType::MajorDepressiveDisorder));

        let has_bipolar = self.diagnoses.iter()
            .any(|d| matches!(d.disorder, DisorderType::BipolarI | DisorderType::BipolarII));

        if has_depression && self.symptom_severity.depression_score.phq9_score > 15 {
            self.risk_assessments.suicide_risk = SuicideRisk::Moderate;
        }

        if has_bipolar && matches!(self.mood_state.current_mood, Mood::Elevated | Mood::Irritable) {
            self.risk_assessments.self_harm_risk = SelfHarmRisk::Moderate;
        }
    }

    pub fn overall_functioning(&self) -> GlobalFunctioning {
        let mut score = 70;

        for diagnosis in &self.diagnoses {
            score -= match diagnosis.severity {
                PsychiatricSeverity::Mild => 5,
                PsychiatricSeverity::Moderate => 15,
                PsychiatricSeverity::Severe => 30,
                PsychiatricSeverity::Remission => 0,
            };
        }

        if self.symptom_severity.depression_score.phq9_score > 15 {
            score -= 10;
        }

        if self.symptom_severity.anxiety_score.gad7_score > 10 {
            score -= 10;
        }

        GlobalFunctioning::from_score(score.max(0) as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlobalFunctioning {
    Superior,
    Good,
    Fair,
    Poor,
    Serious,
    VerySerious,
}

impl GlobalFunctioning {
    fn from_score(score: u32) -> Self {
        match score {
            91..=100 => GlobalFunctioning::Superior,
            71..=90 => GlobalFunctioning::Good,
            51..=70 => GlobalFunctioning::Fair,
            31..=50 => GlobalFunctioning::Poor,
            11..=30 => GlobalFunctioning::Serious,
            _ => GlobalFunctioning::VerySerious,
        }
    }
}

impl Default for PsychiatricProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl SymptomSeverity {
    pub fn normal() -> Self {
        Self {
            depression_score: DepressionScore { phq9_score: 0 },
            anxiety_score: AnxietyScore { gad7_score: 0 },
            psychosis_score: PsychosisScore {
                positive_symptoms: 0,
                negative_symptoms: 0,
            },
        }
    }
}

impl DepressionScore {
    pub fn severity(&self) -> &'static str {
        match self.phq9_score {
            0..=4 => "None-minimal",
            5..=9 => "Mild",
            10..=14 => "Moderate",
            15..=19 => "Moderately severe",
            _ => "Severe",
        }
    }
}

impl AnxietyScore {
    pub fn severity(&self) -> &'static str {
        match self.gad7_score {
            0..=4 => "Minimal",
            5..=9 => "Mild",
            10..=14 => "Moderate",
            _ => "Severe",
        }
    }
}

impl CognitiveFunction {
    pub fn normal() -> Self {
        Self {
            attention: CognitiveScore { percentile: 50.0 },
            memory: CognitiveScore { percentile: 50.0 },
            executive_function: CognitiveScore { percentile: 50.0 },
            processing_speed: CognitiveScore { percentile: 50.0 },
        }
    }

    pub fn is_impaired(&self) -> bool {
        self.attention.percentile < 16.0 ||
        self.memory.percentile < 16.0 ||
        self.executive_function.percentile < 16.0 ||
        self.processing_speed.percentile < 16.0
    }
}

impl MoodState {
    pub fn euthymic() -> Self {
        Self {
            current_mood: Mood::Euthymic,
            affect: Affect::Appropriate,
            energy_level: EnergyLevel::Normal,
        }
    }
}

impl RiskAssessments {
    pub fn low_risk() -> Self {
        Self {
            suicide_risk: SuicideRisk::Low,
            violence_risk: ViolenceRisk::Low,
            self_harm_risk: SelfHarmRisk::Low,
        }
    }

    pub fn requires_immediate_intervention(&self) -> bool {
        matches!(self.suicide_risk, SuicideRisk::Imminent) ||
        matches!(self.violence_risk, ViolenceRisk::High)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psychiatric_profile_creation() {
        let profile = PsychiatricProfile::new();
        assert!(profile.diagnoses.is_empty());
        assert_eq!(profile.mood_state.current_mood, Mood::Euthymic);
    }

    #[test]
    fn test_depression_score_severity() {
        let score = DepressionScore { phq9_score: 16 };
        assert_eq!(score.severity(), "Moderately severe");
    }

    #[test]
    fn test_anxiety_score_severity() {
        let score = AnxietyScore { gad7_score: 12 };
        assert_eq!(score.severity(), "Moderate");
    }

    #[test]
    fn test_global_functioning() {
        let mut profile = PsychiatricProfile::new();

        profile.add_diagnosis(PsychiatricDiagnosis {
            disorder: DisorderType::MajorDepressiveDisorder,
            onset_age: Some(25.0),
            duration_months: Some(6),
            severity: PsychiatricSeverity::Severe,
            episode_type: Some(EpisodeType::Depressive),
        });

        profile.symptom_severity.depression_score.phq9_score = 20;

        let functioning = profile.overall_functioning();
        assert!(matches!(functioning, GlobalFunctioning::Poor | GlobalFunctioning::Serious));
    }

    #[test]
    fn test_cognitive_impairment() {
        let mut cognition = CognitiveFunction::normal();
        cognition.memory.percentile = 10.0;

        assert!(cognition.is_impaired());
    }

    #[test]
    fn test_risk_assessment() {
        let mut profile = PsychiatricProfile::new();

        profile.symptom_severity.depression_score.phq9_score = 18;
        profile.add_diagnosis(PsychiatricDiagnosis {
            disorder: DisorderType::MajorDepressiveDisorder,
            onset_age: Some(30.0),
            duration_months: Some(3),
            severity: PsychiatricSeverity::Severe,
            episode_type: Some(EpisodeType::Depressive),
        });

        assert!(matches!(profile.risk_assessments.suicide_risk, SuicideRisk::Moderate | SuicideRisk::High));
    }
}
