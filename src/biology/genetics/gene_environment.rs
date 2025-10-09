use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneEnvironmentInteraction {
    pub gene: String,
    pub environmental_factor: EnvironmentalFactor,
    pub affected_trait: String,
    pub interaction_strength: f64,
    pub direction: InteractionDirection,
    pub evidence_level: EvidenceLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnvironmentalFactor {
    Diet(DietaryFactor),
    Exercise(ExerciseType),
    Smoking,
    Alcohol,
    Stress,
    Sleep(SleepQuality),
    Pollution(PollutionType),
    Temperature,
    UVExposure,
    Medication(String),
    Microbiome,
    SocialEnvironment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DietaryFactor {
    HighFat,
    LowCarb,
    Mediterranean,
    VeganVegetarian,
    HighSodium,
    LowSodium,
    HighSugar,
    CaffeinIntake,
    FiberIntake,
    OmegaThreeFattyAcids,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExerciseType {
    Aerobic,
    Resistance,
    HighIntensityInterval,
    Sedentary,
    Moderate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SleepQuality {
    Excellent,
    Good,
    Fair,
    Poor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PollutionType {
    AirPollution,
    WaterContamination,
    HeavyMetals,
    Pesticides,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionDirection {
    Protective,
    RiskIncreasing,
    Neutral,
    Conditional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLevel {
    Strong,
    Moderate,
    Weak,
    Experimental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneEnvironmentProfile {
    pub interactions: Vec<GeneEnvironmentInteraction>,
    pub lifestyle_factors: HashMap<EnvironmentalFactor, f64>,
}

impl GeneEnvironmentProfile {
    pub fn new() -> Self {
        Self {
            interactions: Vec::new(),
            lifestyle_factors: HashMap::new(),
        }
    }

    pub fn add_interaction(&mut self, interaction: GeneEnvironmentInteraction) {
        self.interactions.push(interaction);
    }

    pub fn set_lifestyle_factor(&mut self, factor: EnvironmentalFactor, level: f64) {
        self.lifestyle_factors.insert(factor, level);
    }

    pub fn calculate_gene_environment_risk(
        &self,
        gene: &str,
        trait_name: &str,
    ) -> f64 {
        let relevant_interactions: Vec<_> = self
            .interactions
            .iter()
            .filter(|i| i.gene == gene && i.affected_trait == trait_name)
            .collect();

        let mut total_risk = 1.0;

        for interaction in relevant_interactions {
            if let Some(&exposure_level) = self.lifestyle_factors.get(&interaction.environmental_factor) {
                let effect = match interaction.direction {
                    InteractionDirection::Protective => 1.0 - (interaction.interaction_strength * exposure_level),
                    InteractionDirection::RiskIncreasing => 1.0 + (interaction.interaction_strength * exposure_level),
                    InteractionDirection::Neutral => 1.0,
                    InteractionDirection::Conditional => 1.0 + (interaction.interaction_strength * exposure_level * 0.5),
                };
                total_risk *= effect;
            }
        }

        total_risk
    }

    pub fn load_known_interactions() -> Vec<GeneEnvironmentInteraction> {
        vec![
            GeneEnvironmentInteraction {
                gene: "FTO".to_string(),
                environmental_factor: EnvironmentalFactor::Exercise(ExerciseType::Aerobic),
                affected_trait: "Obesity risk".to_string(),
                interaction_strength: 0.4,
                direction: InteractionDirection::Protective,
                evidence_level: EvidenceLevel::Strong,
            },
            GeneEnvironmentInteraction {
                gene: "TCF7L2".to_string(),
                environmental_factor: EnvironmentalFactor::Diet(DietaryFactor::Mediterranean),
                affected_trait: "Type 2 diabetes".to_string(),
                interaction_strength: 0.35,
                direction: InteractionDirection::Protective,
                evidence_level: EvidenceLevel::Strong,
            },
            GeneEnvironmentInteraction {
                gene: "APOE4".to_string(),
                environmental_factor: EnvironmentalFactor::Exercise(ExerciseType::Aerobic),
                affected_trait: "Alzheimer's disease".to_string(),
                interaction_strength: 0.5,
                direction: InteractionDirection::Protective,
                evidence_level: EvidenceLevel::Moderate,
            },
            GeneEnvironmentInteraction {
                gene: "MC1R".to_string(),
                environmental_factor: EnvironmentalFactor::UVExposure,
                affected_trait: "Melanoma risk".to_string(),
                interaction_strength: 2.5,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Strong,
            },
            GeneEnvironmentInteraction {
                gene: "ALDH2".to_string(),
                environmental_factor: EnvironmentalFactor::Alcohol,
                affected_trait: "Esophageal cancer".to_string(),
                interaction_strength: 8.0,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Strong,
            },
            GeneEnvironmentInteraction {
                gene: "GST".to_string(),
                environmental_factor: EnvironmentalFactor::Smoking,
                affected_trait: "Lung cancer".to_string(),
                interaction_strength: 3.2,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Strong,
            },
            GeneEnvironmentInteraction {
                gene: "MTHFR".to_string(),
                environmental_factor: EnvironmentalFactor::Diet(DietaryFactor::HighSugar),
                affected_trait: "Cardiovascular disease".to_string(),
                interaction_strength: 1.8,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Moderate,
            },
            GeneEnvironmentInteraction {
                gene: "CLOCK".to_string(),
                environmental_factor: EnvironmentalFactor::Sleep(SleepQuality::Poor),
                affected_trait: "Metabolic syndrome".to_string(),
                interaction_strength: 1.5,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Moderate,
            },
            GeneEnvironmentInteraction {
                gene: "ACE".to_string(),
                environmental_factor: EnvironmentalFactor::Diet(DietaryFactor::HighSodium),
                affected_trait: "Hypertension".to_string(),
                interaction_strength: 1.6,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Strong,
            },
            GeneEnvironmentInteraction {
                gene: "PPARG".to_string(),
                environmental_factor: EnvironmentalFactor::Exercise(ExerciseType::Resistance),
                affected_trait: "Insulin sensitivity".to_string(),
                interaction_strength: 0.45,
                direction: InteractionDirection::Protective,
                evidence_level: EvidenceLevel::Moderate,
            },
            GeneEnvironmentInteraction {
                gene: "IL6".to_string(),
                environmental_factor: EnvironmentalFactor::Stress,
                affected_trait: "Chronic inflammation".to_string(),
                interaction_strength: 2.0,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Moderate,
            },
            GeneEnvironmentInteraction {
                gene: "COMT".to_string(),
                environmental_factor: EnvironmentalFactor::Stress,
                affected_trait: "Anxiety".to_string(),
                interaction_strength: 1.7,
                direction: InteractionDirection::RiskIncreasing,
                evidence_level: EvidenceLevel::Moderate,
            },
            GeneEnvironmentInteraction {
                gene: "VDR".to_string(),
                environmental_factor: EnvironmentalFactor::UVExposure,
                affected_trait: "Bone density".to_string(),
                interaction_strength: 0.3,
                direction: InteractionDirection::Protective,
                evidence_level: EvidenceLevel::Moderate,
            },
        ]
    }

    pub fn get_personalized_recommendations(
        &self,
        gene: &str,
    ) -> Vec<LifestyleRecommendation> {
        let mut recommendations = Vec::new();

        for interaction in &self.interactions {
            if interaction.gene == gene {
                let priority = match interaction.evidence_level {
                    EvidenceLevel::Strong => RecommendationPriority::High,
                    EvidenceLevel::Moderate => RecommendationPriority::Medium,
                    _ => RecommendationPriority::Low,
                };

                let recommendation_text = match (&interaction.environmental_factor, &interaction.direction) {
                    (EnvironmentalFactor::Exercise(_), InteractionDirection::Protective) => {
                        format!("Regular exercise can reduce your genetic risk for {}", interaction.affected_trait)
                    }
                    (EnvironmentalFactor::Diet(diet), InteractionDirection::Protective) => {
                        format!("A {:?} diet may help protect against {}", diet, interaction.affected_trait)
                    }
                    (EnvironmentalFactor::Smoking, InteractionDirection::RiskIncreasing) => {
                        format!("Avoid smoking - you have increased genetic susceptibility to {}", interaction.affected_trait)
                    }
                    (EnvironmentalFactor::Alcohol, InteractionDirection::RiskIncreasing) => {
                        format!("Limit alcohol consumption due to genetic risk for {}", interaction.affected_trait)
                    }
                    (EnvironmentalFactor::UVExposure, InteractionDirection::RiskIncreasing) => {
                        format!("Use sun protection - genetic variant increases {} risk", interaction.affected_trait)
                    }
                    (EnvironmentalFactor::Sleep(_), InteractionDirection::RiskIncreasing) => {
                        format!("Prioritize sleep quality to reduce genetic risk for {}", interaction.affected_trait)
                    }
                    _ => format!("Monitor {} for {}", format!("{:?}", interaction.environmental_factor), interaction.affected_trait),
                };

                recommendations.push(LifestyleRecommendation {
                    gene: gene.to_string(),
                    environmental_factor: interaction.environmental_factor.clone(),
                    recommendation: recommendation_text,
                    priority,
                    expected_benefit: interaction.interaction_strength,
                });
            }
        }

        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        recommendations
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifestyleRecommendation {
    pub gene: String,
    pub environmental_factor: EnvironmentalFactor,
    pub recommendation: String,
    pub priority: RecommendationPriority,
    pub expected_benefit: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene_environment_interaction() {
        let interactions = GeneEnvironmentProfile::load_known_interactions();
        assert!(interactions.len() > 0);

        let fto_interaction = interactions.iter().find(|i| i.gene == "FTO");
        assert!(fto_interaction.is_some());
    }

    #[test]
    fn test_risk_calculation() {
        let mut profile = GeneEnvironmentProfile::new();
        profile.interactions = GeneEnvironmentProfile::load_known_interactions();
        profile.set_lifestyle_factor(
            EnvironmentalFactor::Exercise(ExerciseType::Aerobic),
            0.8,
        );

        let risk = profile.calculate_gene_environment_risk("FTO", "Obesity risk");
        assert!(risk < 1.0);
    }

    #[test]
    fn test_recommendations() {
        let mut profile = GeneEnvironmentProfile::new();
        profile.interactions = GeneEnvironmentProfile::load_known_interactions();

        let recommendations = profile.get_personalized_recommendations("ALDH2");
        assert!(recommendations.len() > 0);
    }

    #[test]
    fn test_protective_effect() {
        let interaction = GeneEnvironmentInteraction {
            gene: "TEST".to_string(),
            environmental_factor: EnvironmentalFactor::Exercise(ExerciseType::Aerobic),
            affected_trait: "Test condition".to_string(),
            interaction_strength: 0.5,
            direction: InteractionDirection::Protective,
            evidence_level: EvidenceLevel::Strong,
        };

        assert_eq!(interaction.direction, InteractionDirection::Protective);
    }
}
