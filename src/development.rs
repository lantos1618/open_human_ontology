use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentalStage {
    pub age_years: f64,
    pub stage: LifeStage,
    pub growth_metrics: GrowthMetrics,
    pub developmental_milestones: Vec<Milestone>,
    pub maturation: MaturationStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifeStage {
    Fetal,
    Neonatal,
    Infant,
    Toddler,
    EarlyChildhood,
    MiddleChildhood,
    Adolescent,
    YoungAdult,
    MiddleAged,
    Elderly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetrics {
    pub height_cm: f64,
    pub weight_kg: f64,
    pub head_circumference_cm: f64,
    pub height_percentile: f64,
    pub weight_percentile: f64,
    pub bmi: f64,
    pub bmi_percentile: f64,
    pub growth_velocity_cm_year: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub category: MilestoneCategory,
    pub description: String,
    pub typical_age_months: f64,
    pub achieved: bool,
    pub age_achieved_months: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MilestoneCategory {
    Motor,
    Language,
    Cognitive,
    Social,
    Emotional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaturationStatus {
    pub skeletal_age_years: f64,
    pub dental_age_years: f64,
    pub sexual_maturity_rating: TannerStage,
    pub bone_age_years: f64,
    pub predicted_adult_height_cm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TannerStage {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthChart {
    pub measurements: Vec<GrowthMeasurement>,
    pub percentile_curves: PercentileCurves,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMeasurement {
    pub age_months: f64,
    pub height_cm: f64,
    pub weight_kg: f64,
    pub head_circumference_cm: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentileCurves {
    pub height_p3: Vec<(f64, f64)>,
    pub height_p50: Vec<(f64, f64)>,
    pub height_p97: Vec<(f64, f64)>,
    pub weight_p3: Vec<(f64, f64)>,
    pub weight_p50: Vec<(f64, f64)>,
    pub weight_p97: Vec<(f64, f64)>,
}

impl DevelopmentalStage {
    pub fn from_age(age_years: f64, is_male: bool) -> Self {
        let stage = LifeStage::from_age(age_years);

        Self {
            age_years,
            stage,
            growth_metrics: GrowthMetrics::default_for_age(age_years, is_male),
            developmental_milestones: Vec::new(),
            maturation: MaturationStatus::default_for_age(age_years, is_male),
        }
    }

    pub fn assess_development(&self) -> DevelopmentalAssessment {
        let mut concerns = Vec::new();
        let mut strengths = Vec::new();

        if self.growth_metrics.height_percentile < 3.0 {
            concerns.push("Height below 3rd percentile - evaluate for growth disorders".to_string());
        } else if self.growth_metrics.height_percentile > 97.0 {
            strengths.push("Height above 97th percentile".to_string());
        }

        if self.growth_metrics.weight_percentile < 3.0 {
            concerns.push("Weight below 3rd percentile - evaluate for failure to thrive".to_string());
        } else if self.growth_metrics.weight_percentile > 95.0 {
            concerns.push("Weight above 95th percentile - obesity risk".to_string());
        }

        let milestones_delayed = self
            .developmental_milestones
            .iter()
            .filter(|m| !m.achieved && self.age_years * 12.0 > m.typical_age_months + 6.0)
            .count();

        if milestones_delayed > 0 {
            concerns.push(format!("{} developmental milestones delayed", milestones_delayed));
        }

        let overall_status = if concerns.is_empty() {
            DevelopmentalStatus::Normal
        } else if concerns.len() <= 1 {
            DevelopmentalStatus::MildConcern
        } else {
            DevelopmentalStatus::SignificantConcern
        };

        DevelopmentalAssessment {
            status: overall_status,
            concerns,
            strengths,
            recommendations: self.generate_recommendations(&concerns),
        }
    }

    fn generate_recommendations(&self, concerns: &[String]) -> Vec<String> {
        let mut recs = Vec::new();

        if concerns.iter().any(|c| c.contains("Height")) {
            recs.push("Refer to pediatric endocrinologist for growth evaluation".to_string());
            recs.push("Check IGF-1 and growth hormone levels".to_string());
        }

        if concerns.iter().any(|c| c.contains("Weight")) {
            recs.push("Nutritional assessment and counseling".to_string());
            recs.push("Rule out underlying medical conditions".to_string());
        }

        if concerns.iter().any(|c| c.contains("milestones")) {
            recs.push("Developmental screening with standardized tools".to_string());
            recs.push("Consider early intervention services".to_string());
        }

        if recs.is_empty() {
            recs.push("Continue routine developmental surveillance".to_string());
        }

        recs
    }
}

impl LifeStage {
    pub fn from_age(age_years: f64) -> Self {
        if age_years < 0.0 {
            LifeStage::Fetal
        } else if age_years < 0.1 {
            LifeStage::Neonatal
        } else if age_years < 1.0 {
            LifeStage::Infant
        } else if age_years < 3.0 {
            LifeStage::Toddler
        } else if age_years < 6.0 {
            LifeStage::EarlyChildhood
        } else if age_years < 12.0 {
            LifeStage::MiddleChildhood
        } else if age_years < 20.0 {
            LifeStage::Adolescent
        } else if age_years < 40.0 {
            LifeStage::YoungAdult
        } else if age_years < 65.0 {
            LifeStage::MiddleAged
        } else {
            LifeStage::Elderly
        }
    }
}

impl GrowthMetrics {
    pub fn default_for_age(age_years: f64, is_male: bool) -> Self {
        let (height, weight, head_circ) = if is_male {
            Self::male_reference_values(age_years)
        } else {
            Self::female_reference_values(age_years)
        };

        let bmi = weight / ((height / 100.0).powi(2));

        Self {
            height_cm: height,
            weight_kg: weight,
            head_circumference_cm: head_circ,
            height_percentile: 50.0,
            weight_percentile: 50.0,
            bmi,
            bmi_percentile: 50.0,
            growth_velocity_cm_year: Self::expected_growth_velocity(age_years),
        }
    }

    fn male_reference_values(age_years: f64) -> (f64, f64, f64) {
        if age_years < 1.0 {
            (75.0, 9.0, 45.0)
        } else if age_years < 2.0 {
            (85.0, 12.0, 48.0)
        } else if age_years < 5.0 {
            (105.0, 17.0, 50.0)
        } else if age_years < 10.0 {
            (130.0, 28.0, 52.0)
        } else if age_years < 15.0 {
            (155.0, 48.0, 54.0)
        } else if age_years < 20.0 {
            (175.0, 68.0, 56.0)
        } else {
            (177.0, 77.0, 57.0)
        }
    }

    fn female_reference_values(age_years: f64) -> (f64, f64, f64) {
        if age_years < 1.0 {
            (73.0, 8.5, 44.0)
        } else if age_years < 2.0 {
            (83.0, 11.0, 47.0)
        } else if age_years < 5.0 {
            (103.0, 16.0, 49.0)
        } else if age_years < 10.0 {
            (128.0, 27.0, 51.0)
        } else if age_years < 15.0 {
            (160.0, 50.0, 54.0)
        } else if age_years < 20.0 {
            (163.0, 58.0, 55.0)
        } else {
            (163.0, 65.0, 55.0)
        }
    }

    fn expected_growth_velocity(age_years: f64) -> f64 {
        if age_years < 1.0 {
            25.0
        } else if age_years < 2.0 {
            12.0
        } else if age_years < 4.0 {
            8.0
        } else if age_years < 10.0 {
            6.0
        } else if age_years < 14.0 {
            8.0
        } else if age_years < 18.0 {
            3.0
        } else {
            0.0
        }
    }

    pub fn calculate_percentile(value: f64, mean: f64, sd: f64) -> f64 {
        let z = (value - mean) / sd;
        let percentile = 50.0 + 50.0 * (z / (1.0 + z.powi(2)).sqrt());
        percentile.max(0.1).min(99.9)
    }
}

impl MaturationStatus {
    pub fn default_for_age(age_years: f64, is_male: bool) -> Self {
        let tanner = if age_years < 8.0 {
            TannerStage::Stage1
        } else if age_years < 10.0 {
            TannerStage::Stage2
        } else if age_years < 12.0 {
            TannerStage::Stage3
        } else if age_years < 15.0 {
            TannerStage::Stage4
        } else {
            TannerStage::Stage5
        };

        let predicted_height = if is_male { 177.0 } else { 163.0 };

        Self {
            skeletal_age_years: age_years,
            dental_age_years: age_years,
            sexual_maturity_rating: tanner,
            bone_age_years: age_years,
            predicted_adult_height_cm: predicted_height,
        }
    }

    pub fn is_advanced(&self, chronological_age: f64) -> bool {
        self.bone_age_years > chronological_age + 2.0
    }

    pub fn is_delayed(&self, chronological_age: f64) -> bool {
        self.bone_age_years < chronological_age - 2.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentalAssessment {
    pub status: DevelopmentalStatus,
    pub concerns: Vec<String>,
    pub strengths: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevelopmentalStatus {
    Normal,
    MildConcern,
    SignificantConcern,
}

impl Milestone {
    pub fn new_motor(description: &str, typical_age_months: f64) -> Self {
        Self {
            category: MilestoneCategory::Motor,
            description: description.to_string(),
            typical_age_months,
            achieved: false,
            age_achieved_months: None,
        }
    }

    pub fn new_language(description: &str, typical_age_months: f64) -> Self {
        Self {
            category: MilestoneCategory::Language,
            description: description.to_string(),
            typical_age_months,
            achieved: false,
            age_achieved_months: None,
        }
    }

    pub fn new_cognitive(description: &str, typical_age_months: f64) -> Self {
        Self {
            category: MilestoneCategory::Cognitive,
            description: description.to_string(),
            typical_age_months,
            achieved: false,
            age_achieved_months: None,
        }
    }

    pub fn standard_milestones() -> Vec<Self> {
        vec![
            Self::new_motor("Rolls over", 4.0),
            Self::new_motor("Sits without support", 6.0),
            Self::new_motor("Crawls", 8.0),
            Self::new_motor("Walks independently", 12.0),
            Self::new_motor("Runs", 18.0),
            Self::new_language("Babbles", 6.0),
            Self::new_language("First words", 12.0),
            Self::new_language("Two-word phrases", 18.0),
            Self::new_language("Speaks in sentences", 24.0),
            Self::new_cognitive("Object permanence", 9.0),
            Self::new_cognitive("Points to body parts", 18.0),
            Self::new_cognitive("Knows colors", 36.0),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_life_stage_classification() {
        assert_eq!(LifeStage::from_age(0.05), LifeStage::Neonatal);
        assert_eq!(LifeStage::from_age(0.5), LifeStage::Infant);
        assert_eq!(LifeStage::from_age(2.0), LifeStage::Toddler);
        assert_eq!(LifeStage::from_age(8.0), LifeStage::MiddleChildhood);
        assert_eq!(LifeStage::from_age(15.0), LifeStage::Adolescent);
        assert_eq!(LifeStage::from_age(30.0), LifeStage::YoungAdult);
        assert_eq!(LifeStage::from_age(70.0), LifeStage::Elderly);
    }

    #[test]
    fn test_developmental_stage_creation() {
        let stage = DevelopmentalStage::from_age(5.0, true);
        assert_eq!(stage.stage, LifeStage::EarlyChildhood);
        assert!(stage.growth_metrics.height_cm > 0.0);
        assert!(stage.growth_metrics.weight_kg > 0.0);
    }

    #[test]
    fn test_growth_metrics_male() {
        let metrics = GrowthMetrics::default_for_age(10.0, true);
        assert!(metrics.height_cm > 100.0);
        assert!(metrics.weight_kg > 20.0);
        assert!(metrics.bmi > 0.0);
    }

    #[test]
    fn test_growth_metrics_female() {
        let metrics = GrowthMetrics::default_for_age(10.0, false);
        assert!(metrics.height_cm > 100.0);
        assert!(metrics.weight_kg > 20.0);
    }

    #[test]
    fn test_growth_velocity() {
        let infant_velocity = GrowthMetrics::expected_growth_velocity(0.5);
        let child_velocity = GrowthMetrics::expected_growth_velocity(6.0);
        let adolescent_velocity = GrowthMetrics::expected_growth_velocity(13.0);

        assert!(infant_velocity > child_velocity);
        assert!(adolescent_velocity > child_velocity);
    }

    #[test]
    fn test_maturation_status() {
        let maturation = MaturationStatus::default_for_age(12.0, true);
        assert_eq!(maturation.sexual_maturity_rating, TannerStage::Stage3);
        assert!(maturation.predicted_adult_height_cm > 0.0);
    }

    #[test]
    fn test_maturation_advanced() {
        let mut maturation = MaturationStatus::default_for_age(10.0, true);
        maturation.bone_age_years = 13.0;
        assert!(maturation.is_advanced(10.0));
        assert!(!maturation.is_delayed(10.0));
    }

    #[test]
    fn test_maturation_delayed() {
        let mut maturation = MaturationStatus::default_for_age(12.0, true);
        maturation.bone_age_years = 9.0;
        assert!(maturation.is_delayed(12.0));
        assert!(!maturation.is_advanced(12.0));
    }

    #[test]
    fn test_standard_milestones() {
        let milestones = Milestone::standard_milestones();
        assert!(!milestones.is_empty());
        assert!(milestones.iter().any(|m| m.category == MilestoneCategory::Motor));
        assert!(milestones.iter().any(|m| m.category == MilestoneCategory::Language));
        assert!(milestones.iter().any(|m| m.category == MilestoneCategory::Cognitive));
    }

    #[test]
    fn test_developmental_assessment_normal() {
        let stage = DevelopmentalStage::from_age(5.0, true);
        let assessment = stage.assess_development();
        assert_eq!(assessment.status, DevelopmentalStatus::Normal);
    }

    #[test]
    fn test_developmental_assessment_growth_concern() {
        let mut stage = DevelopmentalStage::from_age(5.0, true);
        stage.growth_metrics.height_percentile = 1.0;
        stage.growth_metrics.weight_percentile = 2.0;

        let assessment = stage.assess_development();
        assert_ne!(assessment.status, DevelopmentalStatus::Normal);
        assert!(!assessment.concerns.is_empty());
        assert!(!assessment.recommendations.is_empty());
    }

    #[test]
    fn test_percentile_calculation() {
        let p50 = GrowthMetrics::calculate_percentile(100.0, 100.0, 10.0);
        assert!((p50 - 50.0).abs() < 5.0);

        let p84 = GrowthMetrics::calculate_percentile(110.0, 100.0, 10.0);
        assert!(p84 > 70.0);
    }
}
