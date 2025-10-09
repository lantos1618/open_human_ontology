use serde::{Deserialize, Serialize};
use crate::biology::genetics::{
    AncestryProfile, GeneEnvironmentProfile,
    PolygeneticRiskScore,
};
use crate::pharmacology::pharmacogenomics::PharmacogeneticProfile;
use crate::human::BiologicalSex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedMedicineProfile {
    pub patient_id: String,
    pub ancestry: AncestryProfile,
    pub pharmacogenomics: PharmacogeneticProfile,
    pub disease_risks: Vec<DiseaseRiskAssessment>,
    pub drug_recommendations: Vec<DrugRecommendation>,
    pub lifestyle_interventions: Vec<LifestyleIntervention>,
    pub preventive_strategies: Vec<PreventiveStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRiskAssessment {
    pub condition: String,
    pub genetic_risk: f64,
    pub environmental_risk: f64,
    pub combined_risk: f64,
    pub risk_category: RiskCategory,
    pub contributing_genes: Vec<String>,
    pub modifiable_factors: Vec<String>,
    pub surveillance_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    VeryLow,
    Low,
    Average,
    Elevated,
    High,
    VeryHigh,
}

impl RiskCategory {
    pub fn from_risk_score(score: f64) -> Self {
        match score {
            s if s < 0.5 => RiskCategory::VeryLow,
            s if s < 1.0 => RiskCategory::Low,
            s if s < 1.5 => RiskCategory::Average,
            s if s < 2.5 => RiskCategory::Elevated,
            s if s < 4.0 => RiskCategory::High,
            _ => RiskCategory::VeryHigh,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugRecommendation {
    pub drug_name: String,
    pub indication: String,
    pub metabolism_prediction: MetabolismPrediction,
    pub dosage_adjustment: DosageAdjustment,
    pub adverse_reaction_risk: f64,
    pub efficacy_prediction: f64,
    pub genetic_factors: Vec<String>,
    pub alternative_drugs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolismPrediction {
    UltraRapid,
    Normal,
    Intermediate,
    Poor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DosageAdjustment {
    Increase50Percent,
    Increase25Percent,
    Standard,
    Decrease25Percent,
    Decrease50Percent,
    Contraindicated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifestyleIntervention {
    pub intervention_type: InterventionType,
    pub target_condition: String,
    pub expected_risk_reduction: f64,
    pub priority: Priority,
    pub specific_recommendations: Vec<String>,
    pub monitoring_parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionType {
    Diet,
    Exercise,
    StressManagement,
    SleepHygiene,
    SmokingCessation,
    AlcoholModeration,
    WeightManagement,
    SupplementationHere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreventiveStrategy {
    pub target_condition: String,
    pub strategy_type: StrategyType,
    pub recommended_frequency: String,
    pub rationale: String,
    pub genetic_basis: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrategyType {
    Screening,
    Prophylaxis,
    Vaccination,
    SurveillanceImaging,
    GeneticCounseling,
    PreemptiveTreatment,
}

impl PersonalizedMedicineProfile {
    pub fn new(patient_id: String, ancestry: AncestryProfile) -> Self {
        Self {
            patient_id,
            ancestry,
            pharmacogenomics: PharmacogeneticProfile::new(),
            disease_risks: Vec::new(),
            drug_recommendations: Vec::new(),
            lifestyle_interventions: Vec::new(),
            preventive_strategies: Vec::new(),
        }
    }

    pub fn assess_disease_risk(
        &mut self,
        condition: String,
        prs: PolygeneticRiskScore,
        gene_env_profile: &GeneEnvironmentProfile,
    ) {
        let genetic_risk = prs.total_score;
        let environmental_risk = self.calculate_environmental_risk(&condition, gene_env_profile);
        let combined_risk = genetic_risk * environmental_risk;

        let contributing_genes = prs
            .contributing_genes
            .iter()
            .map(|gc| gc.gene.clone())
            .collect();

        let modifiable_factors = self.identify_modifiable_factors(&condition, gene_env_profile);
        let surveillance = self.generate_surveillance_recommendations(&condition, combined_risk);

        self.disease_risks.push(DiseaseRiskAssessment {
            condition,
            genetic_risk,
            environmental_risk,
            combined_risk,
            risk_category: RiskCategory::from_risk_score(combined_risk),
            contributing_genes,
            modifiable_factors,
            surveillance_recommendations: surveillance,
        });
    }

    fn calculate_environmental_risk(
        &self,
        condition: &str,
        gene_env_profile: &GeneEnvironmentProfile,
    ) -> f64 {
        let mut env_risk = 1.0;

        for interaction in &gene_env_profile.interactions {
            if interaction.affected_trait == condition {
                if let Some(&exposure) = gene_env_profile.lifestyle_factors.get(&interaction.environmental_factor) {
                    env_risk *= 1.0 + (interaction.interaction_strength * exposure * 0.1);
                }
            }
        }

        env_risk
    }

    fn identify_modifiable_factors(
        &self,
        condition: &str,
        gene_env_profile: &GeneEnvironmentProfile,
    ) -> Vec<String> {
        gene_env_profile
            .interactions
            .iter()
            .filter(|i| i.affected_trait == condition)
            .map(|i| format!("{:?}", i.environmental_factor))
            .collect()
    }

    fn generate_surveillance_recommendations(
        &self,
        condition: &str,
        risk: f64,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if condition.contains("cancer") || condition.contains("Cancer") {
            if risk > 2.0 {
                recommendations.push("Annual screening recommended".to_string());
                recommendations.push("Consider genetic counseling".to_string());
            } else if risk > 1.5 {
                recommendations.push("Biennial screening recommended".to_string());
            }
        }

        if (condition.contains("diabetes") || condition.contains("Diabetes")) && risk > 1.5 {
            recommendations.push("Annual HbA1c testing".to_string());
            recommendations.push("Quarterly fasting glucose monitoring".to_string());
        }

        if (condition.contains("cardiovascular") || condition.contains("heart")) && risk > 2.0 {
            recommendations.push("Annual cardiac stress test".to_string());
            recommendations.push("Lipid panel every 6 months".to_string());
        }

        recommendations
    }

    pub fn add_drug_recommendation(&mut self, recommendation: DrugRecommendation) {
        self.drug_recommendations.push(recommendation);
    }

    pub fn add_lifestyle_intervention(&mut self, intervention: LifestyleIntervention) {
        self.lifestyle_interventions.push(intervention);
    }

    pub fn add_preventive_strategy(&mut self, strategy: PreventiveStrategy) {
        self.preventive_strategies.push(strategy);
    }

    pub fn get_high_risk_conditions(&self) -> Vec<&DiseaseRiskAssessment> {
        self.disease_risks
            .iter()
            .filter(|r| matches!(r.risk_category, RiskCategory::High | RiskCategory::VeryHigh))
            .collect()
    }

    pub fn get_critical_interventions(&self) -> Vec<&LifestyleIntervention> {
        self.lifestyle_interventions
            .iter()
            .filter(|i| i.priority == Priority::Critical || i.priority == Priority::High)
            .collect()
    }

    pub fn generate_personalized_report(&self) -> PersonalizedHealthReport {
        PersonalizedHealthReport {
            patient_id: self.patient_id.clone(),
            ancestry_summary: self.generate_ancestry_summary(),
            top_disease_risks: self.get_top_risks(5),
            pharmacogenomic_insights: self.generate_pharmacogenomic_insights(),
            lifestyle_priorities: self.get_lifestyle_priorities(),
            preventive_care_plan: self.preventive_strategies.clone(),
        }
    }

    fn generate_ancestry_summary(&self) -> String {
        if let Some(primary) = self.ancestry.primary_ancestry() {
            format!("Primary ancestry: {:?}", primary)
        } else {
            "Mixed ancestry".to_string()
        }
    }

    fn get_top_risks(&self, n: usize) -> Vec<DiseaseRiskAssessment> {
        let mut risks = self.disease_risks.clone();
        risks.sort_by(|a, b| b.combined_risk.partial_cmp(&a.combined_risk).unwrap());
        risks.into_iter().take(n).collect()
    }

    fn generate_pharmacogenomic_insights(&self) -> Vec<String> {
        vec![
            format!(
                "Pharmacogenomic profile includes {} gene variants",
                self.pharmacogenomics.phenotypes.len()
            ),
            "Drug recommendations personalized based on genetic metabolism".to_string(),
        ]
    }

    fn get_lifestyle_priorities(&self) -> Vec<LifestyleIntervention> {
        let mut interventions = self.lifestyle_interventions.clone();
        interventions.sort_by(|a, b| b.priority.cmp(&a.priority));
        interventions
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedHealthReport {
    pub patient_id: String,
    pub ancestry_summary: String,
    pub top_disease_risks: Vec<DiseaseRiskAssessment>,
    pub pharmacogenomic_insights: Vec<String>,
    pub lifestyle_priorities: Vec<LifestyleIntervention>,
    pub preventive_care_plan: Vec<PreventiveStrategy>,
}

pub struct PersonalizedMedicineEngine;

impl PersonalizedMedicineEngine {
    pub fn create_comprehensive_profile(
        patient_id: String,
        ancestry: AncestryProfile,
        _sex: BiologicalSex,
        _age: f64,
    ) -> PersonalizedMedicineProfile {
        

        PersonalizedMedicineProfile::new(patient_id, ancestry)
    }

    pub fn optimize_drug_therapy(
        _profile: &PersonalizedMedicineProfile,
        drug_name: &str,
        indication: &str,
    ) -> DrugRecommendation {
        let metabolism = MetabolismPrediction::Normal;
        let dosage = DosageAdjustment::Standard;

        DrugRecommendation {
            drug_name: drug_name.to_string(),
            indication: indication.to_string(),
            metabolism_prediction: metabolism,
            dosage_adjustment: dosage,
            adverse_reaction_risk: 0.05,
            efficacy_prediction: 0.85,
            genetic_factors: vec![],
            alternative_drugs: vec![],
        }
    }

    pub fn generate_lifestyle_plan(
        disease_risks: &[DiseaseRiskAssessment],
    ) -> Vec<LifestyleIntervention> {
        let mut interventions = Vec::new();

        for risk in disease_risks {
            if risk.combined_risk > 2.0 {
                for modifiable in &risk.modifiable_factors {
                    if modifiable.contains("Exercise") || modifiable.contains("Sedentary") {
                        interventions.push(LifestyleIntervention {
                            intervention_type: InterventionType::Exercise,
                            target_condition: risk.condition.clone(),
                            expected_risk_reduction: 0.3,
                            priority: Priority::High,
                            specific_recommendations: vec![
                                "150 minutes moderate aerobic activity per week".to_string(),
                                "Resistance training 2-3 times per week".to_string(),
                            ],
                            monitoring_parameters: vec![
                                "Weekly exercise logs".to_string(),
                                "Quarterly fitness assessments".to_string(),
                            ],
                        });
                    }

                    if modifiable.contains("Diet") {
                        interventions.push(LifestyleIntervention {
                            intervention_type: InterventionType::Diet,
                            target_condition: risk.condition.clone(),
                            expected_risk_reduction: 0.25,
                            priority: Priority::High,
                            specific_recommendations: vec![
                                "Mediterranean-style diet".to_string(),
                                "Reduce processed foods".to_string(),
                            ],
                            monitoring_parameters: vec![
                                "Food diary".to_string(),
                                "Quarterly nutritional assessment".to_string(),
                            ],
                        });
                    }
                }
            }
        }

        interventions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personalized_medicine_profile() {
        let ancestry = AncestryProfile::new();
        let profile = PersonalizedMedicineProfile::new("TEST001".to_string(), ancestry);
        assert_eq!(profile.patient_id, "TEST001");
    }

    #[test]
    fn test_risk_category() {
        assert_eq!(RiskCategory::from_risk_score(0.3), RiskCategory::VeryLow);
        assert_eq!(RiskCategory::from_risk_score(1.2), RiskCategory::Average);
        assert_eq!(RiskCategory::from_risk_score(3.0), RiskCategory::High);
    }

    #[test]
    fn test_get_high_risk_conditions() {
        let ancestry = AncestryProfile::new();
        let mut profile = PersonalizedMedicineProfile::new("TEST002".to_string(), ancestry);

        profile.disease_risks.push(DiseaseRiskAssessment {
            condition: "Test Condition".to_string(),
            genetic_risk: 3.0,
            environmental_risk: 1.5,
            combined_risk: 4.5,
            risk_category: RiskCategory::VeryHigh,
            contributing_genes: vec![],
            modifiable_factors: vec![],
            surveillance_recommendations: vec![],
        });

        let high_risks = profile.get_high_risk_conditions();
        assert_eq!(high_risks.len(), 1);
    }

    #[test]
    fn test_lifestyle_intervention_priority() {
        let intervention = LifestyleIntervention {
            intervention_type: InterventionType::Exercise,
            target_condition: "Test".to_string(),
            expected_risk_reduction: 0.3,
            priority: Priority::High,
            specific_recommendations: vec![],
            monitoring_parameters: vec![],
        };

        assert_eq!(intervention.priority, Priority::High);
    }
}
